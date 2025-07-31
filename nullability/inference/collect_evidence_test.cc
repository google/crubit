// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/collect_evidence.h"

#include <cassert>
#include <memory>
#include <string>
#include <vector>

#include "gmock/gmock.h"
#include "nullability/inference/augmented_test_inputs.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/inference/slot_fingerprint.h"
#include "nullability/inference/usr_cache.h"
#include "nullability/pragma.h"
#include "clang/AST/ASTConsumer.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/ASTMatchers/ASTMatchersMacros.h"
#include "clang/Analysis/FlowSensitive/WatchedLiteralsSolver.h"
#include "clang/Basic/LLVM.h"
#include "clang/Frontend/CompilerInstance.h"
#include "clang/Frontend/FrontendActions.h"
#include "clang/Testing/TestAST.h"
#include "third_party/llvm/llvm-project/clang/unittests/Analysis/FlowSensitive/TestingSupport.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Regex.h"
#include "llvm/Support/raw_ostream.h"
#include "llvm/Testing/Support/Error.h"
#include "external/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"  // IWYU pragma: keep
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {
using ::clang::ast_matchers::anything;
using ::clang::ast_matchers::asString;
using ::clang::ast_matchers::booleanType;
using ::clang::ast_matchers::cxxConstructorDecl;
using ::clang::ast_matchers::cxxMethodDecl;
using ::clang::ast_matchers::functionDecl;
using ::clang::ast_matchers::hasAncestor;
using ::clang::ast_matchers::hasBody;
using ::clang::ast_matchers::hasName;
using ::clang::ast_matchers::hasParameter;
using ::clang::ast_matchers::hasTemplateArgument;
using ::clang::ast_matchers::hasType;
using ::clang::ast_matchers::isDefaultConstructor;
using ::clang::ast_matchers::isImplicit;
using ::clang::ast_matchers::isTemplateInstantiation;
using ::clang::ast_matchers::lambdaExpr;
using ::clang::ast_matchers::match;
using ::clang::ast_matchers::ofClass;
using ::clang::ast_matchers::parameterCountIs;
using ::clang::ast_matchers::refersToType;
using ::clang::ast_matchers::selectFirst;
using ::clang::ast_matchers::unless;
using ::clang::ast_matchers::varDecl;
using ::testing::_;
using ::testing::AllOf;
using ::testing::Contains;
using ::testing::ElementsAre;
using ::testing::HasSubstr;
using ::testing::IsEmpty;
using ::testing::IsSupersetOf;
using ::testing::Not;
using ::testing::ResultOf;
using ::testing::SizeIs;
using ::testing::UnorderedElementsAre;

constexpr llvm::StringRef CheckMacroDefinitions = R"cc(
  // Bodies must reference the first param so that args are in the AST, but
  // otherwise don't matter.
#define CHECK(X) (X)
#define CHECK_NE(A, B) (A, B)
)cc";

MATCHER_P3(isEvidenceMatcher, SlotMatcher, KindMatcher, SymbolMatcher, "") {
  return SlotMatcher.Matches(static_cast<Slot>(arg.slot())) &&
         KindMatcher.Matches(arg.kind()) && SymbolMatcher.Matches(arg.symbol());
}

MATCHER_P(functionNamed, Name, "") {
  return llvm::StringRef(arg.usr()).contains(
      ("@" + llvm::Twine(Name) + "#").str());
}

MATCHER_P(functionTemplateNamed, Name, "") {
  return llvm::Regex((".*@FT@>[0-9]+(#.*)*" + llvm::Twine(Name) + "#.*").str())
      .match(arg.usr());
}

/// Matches a non-static field with the given name.
/// The name should be of the form "MyStruct::field", but it should be qualified
/// only by the enclosing type, not any namespaces.
MATCHER_P(fieldNamed, TypeQualifiedFieldName, "") {
  const auto [TypeName, FieldName] =
      llvm::StringRef(TypeQualifiedFieldName).split("::");
  return arg.usr().ends_with(("@S@" + TypeName + "@FI@" + FieldName).str()) ||
         arg.usr().ends_with(("@U@" + TypeName + "@FI@" + FieldName).str());
}

/// Matches a static field with the given name.
/// The name should be of the form "MyStruct::field" (see also comment for
/// `fieldNamed()`).
MATCHER_P(staticFieldNamed, TypeQualifiedFieldName, "") {
  const auto [TypeName, FieldName] =
      llvm::StringRef(TypeQualifiedFieldName).split("::");
  return arg.usr().ends_with(("@S@" + TypeName + "@" + FieldName).str());
}

MATCHER_P(globalVarNamed, Name, "") {
  return arg.usr() == ("c:@" + llvm::Twine(Name)).str();
}

MATCHER_P2(localVarNamedImpl, VarName, FunctionName, "") {
  return llvm::StringRef(arg.usr()).contains(
             ("@F@" + llvm::Twine(FunctionName) + "#").str()) &&
         arg.usr().ends_with(("@" + llvm::Twine(VarName)).str());
}

auto localVarNamed(llvm::StringRef VarName,
                   llvm::StringRef FunctionName = "target") {
  return localVarNamedImpl(VarName, FunctionName);
}

testing::Matcher<const Evidence&> evidence(
    testing::Matcher<Slot> S, testing::Matcher<Evidence::Kind> Kind,
    testing::Matcher<const Symbol&> SymbolMatcher = functionNamed("target")) {
  return isEvidenceMatcher(S, Kind, SymbolMatcher);
}

std::vector<Evidence> collectFromDefinition(
    clang::TestAST& AST, const Decl& Definition,
    const NullabilityPragmas& Pragmas,
    PreviousInferences InputInferences = {}) {
  std::vector<Evidence> Results;
  USRCache UsrCache;
  // Can't assert from within a non-void helper function, so only EXPECT.
  EXPECT_THAT_ERROR(
      collectEvidenceFromDefinition(
          Definition,
          evidenceEmitter([&](const Evidence& E) { Results.push_back(E); },
                          UsrCache, AST.context()),
          UsrCache, Pragmas, InputInferences),
      llvm::Succeeded());
  return Results;
}

std::vector<Evidence> collectFromDefinitionNamed(
    llvm::StringRef TargetName, llvm::StringRef Source,
    PreviousInferences InputInferences = {}) {
  NullabilityPragmas Pragmas;
  clang::TestAST AST(getAugmentedTestInputs(Source, Pragmas));
  const Decl& Definition =
      *dataflow::test::findValueDecl(AST.context(), TargetName);
  return collectFromDefinition(AST, Definition, Pragmas, InputInferences);
}

/// Provides a default function-name-cased value for TargetName in
/// collectEvidenceFromDefinitionNamed, which puts TargetName first for
/// readability.
std::vector<Evidence> collectFromTargetFuncDefinition(
    llvm::StringRef Source, PreviousInferences InputInferences = {}) {
  return collectFromDefinitionNamed("target", Source, InputInferences);
}

template <typename MatcherT>
std::vector<Evidence> collectFromDefinitionMatching(
    MatcherT Matcher, llvm::StringRef Source,
    PreviousInferences InputInferences = {}) {
  NullabilityPragmas Pragmas;
  clang::TestAST AST(getAugmentedTestInputs(Source, Pragmas));
  const Decl& Definition =
      *selectFirst<Decl>("d", match(Matcher.bind("d"), AST.context()));
  return collectFromDefinition(AST, Definition, Pragmas, InputInferences);
}

std::vector<Evidence> collectFromDecl(llvm::StringRef Source,
                                      llvm::StringRef DeclName) {
  std::vector<Evidence> Results;
  NullabilityPragmas Pragmas;
  clang::TestAST AST(getAugmentedTestInputs(Source, Pragmas));
  USRCache USRCache;
  collectEvidenceFromTargetDeclaration(
      *dataflow::test::findValueDecl(AST.context(), DeclName),
      evidenceEmitter([&](const Evidence& E) { Results.push_back(E); },
                      USRCache, AST.context()),
      Pragmas);
  return Results;
}

auto collectFromTargetVarDecl(llvm::StringRef Source) {
  return collectFromDecl(Source, "Target");
}

auto collectFromTargetFuncDecl(llvm::StringRef Source) {
  return collectFromDecl(Source, "target");
}

TEST(CollectEvidenceFromDefinitionTest, Location) {
  llvm::StringRef Code = "void target(int *P) { *P; }";
  //                      12345678901234567890123456
  //                      0        1         2

  auto Evidence = collectFromTargetFuncDefinition(Code);
  ASSERT_THAT(Evidence, ElementsAre(evidence(paramSlot(0),
                                             Evidence::UNCHECKED_DEREFERENCE)));
  EXPECT_EQ("input.cc:1:23", Evidence.front().location());
}

TEST(SmartPointerCollectEvidenceFromDefinitionTest, Location) {
  llvm::StringRef Code =
      "#include <memory>\nvoid target(std::unique_ptr<int> P) { *P; }";
  //                      123456789012345678901234567890123456789012
  //                      0        1         2         3         4

  auto Evidence = collectFromTargetFuncDefinition(Code);
  ASSERT_THAT(Evidence, ElementsAre(evidence(paramSlot(0),
                                             Evidence::UNCHECKED_DEREFERENCE)));
  EXPECT_EQ("input.cc:2:39", Evidence.front().location());
}

TEST(CollectEvidenceFromDefinitionTest, NoParams) {
  static constexpr llvm::StringRef Src = R"cc(
    void target() {}
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, OneParamUnused) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *P) {}
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, OneParamUsedWithoutRestriction) {
  static constexpr llvm::StringRef Src = R"cc(
    void takesUnknown(int *Unknown) {}

    void target(int *P) { takesUnknown(P); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromDefinitionTest, Deref) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *P0, int *P1) {
      int A = *P0;
      if (P1 != nullptr) {
        int B = *P1;
      }
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest, DerefArrow) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      int X;
      int y();
    };
    void target(S *A, S *B) {
      A->X;
      B->y();
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE),
                  evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(SmartPointerCollectEvidenceFromDefinitionTest, Deref) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    struct S {
      int X;
      int y();
    };
    void target(std::unique_ptr<S> P) {
      *P;
      P->X;
      P->y();
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE),
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE),
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest, DerefOfNonnull) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(Nonnull<int *> P) {
      *P;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, DereferenceBeforeAssignment) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *P) {
      *P;
      int I = 1;
      P = &I;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      Contains(evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest, DereferenceAfterAssignment) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *P) {
      int I = 1;
      P = &I;
      *P;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Not(Contains(evidence(_, Evidence::UNCHECKED_DEREFERENCE))));
}

TEST(CollectEvidenceFromDefinitionTest, DereferenceAfterAssignmentFromReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    int& getIntRef();
    int* getIntPtr();
    void target(int* P) {
      P = &getIntRef();
      *P;
      P = getIntPtr();
      *P;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Not(Contains(evidence(_, Evidence::UNCHECKED_DEREFERENCE,
                                    functionNamed("target")))));
}

TEST(CollectEvidenceFromDefinitionTest, DerefOfPtrRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *&P0, int *&P1) {
      int A = *P0;
      if (P1 != nullptr) {
        int B = *P1;
      }
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest, UnrelatedCondition) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *P0, int *P1, int *P2, bool B) {
      if (B) {
        int A = *P0;
        int B = *P1;
      } else {
        int A = *P0;
        int C = *P2;
      }
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE),
                  evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE),
                  // We collect two Evidence values for two dereferences of P0
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE),
                  evidence(paramSlot(2), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest, LaterDeref) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *P) {
      if (P == nullptr) {
        (void)0;
      } else {
        (void)0;
      }
      int A = *P;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest, DerefBeforeGuardedDeref) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *P) {
      int A = *P;
      if (P != nullptr) {
        int B = *P;
      }
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest, DerefAndOrCheckOfCopiedPtr) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P, int* Q) {
      int* A = P;
      *A;
      int* B = Q;
      if (Q) {
        *B;
      }
      if (B) {
        *Q;
      }
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE),
                  evidence(Slot(0), Evidence::ASSIGNED_FROM_UNKNOWN,
                           localVarNamed("A")),
                  evidence(Slot(0), Evidence::ASSIGNED_FROM_UNKNOWN,
                           localVarNamed("B"))));
}

TEST(CollectEvidenceFromDefinitionTest, FirstSufficientSlotOnly) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P, int* Q) {
      // Marking either of P or Q Nonnull is sufficient to avoid dereferencing
      // without a check. We choose to record evidence only for the first
      // sufficient slot which can be Nonnull without the dereference becoming
      // dead code.
      int* A;
      if (P) {
        A = P;
      } else {
        A = Q;
      }
      *A;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE),
                  evidence(Slot(0), Evidence::ASSIGNED_FROM_NONNULL,
                           localVarNamed("A")),
                  evidence(Slot(0), Evidence::ASSIGNED_FROM_UNKNOWN,
                           localVarNamed("A"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     FirstSufficientSlotNotContradictingFlowConditions) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P, int* Q) {
      // Marking P Nonnull would make the dereference dead, so we collect
      // evidence for Q being Nonnull instead, since it is also sufficient.
      if (!P) {
        *Q;
      }
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest, EarlyReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *P) {
      if (!P) {
        return;
      }
      int A = *P;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, UnreachableCode) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *P0, int *P1, int *P2, int *P3) {
      if (true) {
        int A = *P0;
      } else {
        int A = *P1;
      }

      if (false) {
        int A = *P2;
      }

      return;
      int A = *P3;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest, PointerToMemberField) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {};

    void target(int S::*P) {
      S AnS;
      AnS.*P;
      (&AnS)->*P;
    }
  )cc";
  // Pointers to members are not supported pointer types, so no evidence is
  // collected. If they become a supported pointer type, this test should start
  // failing.
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, PointerToMemberMethod) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {};

    void target(void (S::*P)()) {
      S AnS;
      (AnS.*P)();
      ((&AnS)->*P)();
    }
  )cc";

  // Pointers to members are not supported pointer types, so no evidence is
  // collected. If they become a supported pointer type, this test should start
  // failing.
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, PointerToMemberMethodArgs) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {};

    void target(void (S::*P)(Nonnull<int*> I, int* J), int* Q) {
      S AnS;
      (AnS.*P)(Q, nullptr);
      ((&AnS)->*P)(Q, nullptr);
    }
  )cc";

  // Pointers to members are not supported pointer types, so no evidence is
  // collected for `P` or `J`. If they become a supported pointer type, this
  // test should start failing.
  // TODO(b/309625642) We should still collect evidence for the use of `Q` as an
  // argument for param `I`.
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, CheckMacro) {
  llvm::Twine Src = CheckMacroDefinitions + R"cc(
    void target(int* P, int* Q, int* R, int* S, int* T, int* U, int* V) {
      // should collect evidence for params from these calls
      CHECK(P);
      CHECK(Q != nullptr);
      int* A = nullptr;
      CHECK(R != A);
      CHECK(A != S);
      bool B = T != nullptr;
      CHECK(B);

      // should not crash when analyzing these calls
      CHECK(U == V);
      CHECK(U != V);
      CHECK(1);
      struct ConvertibleToBool {
        operator bool() const { return true; }
      };
      CHECK(ConvertibleToBool());
      CHECK(true);
      CHECK(false);  // must come last because it's detected as causing the rest
                     // of the function to be dead.
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src.str()),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ABORT_IF_NULL),
                           evidence(paramSlot(1), Evidence::ABORT_IF_NULL),
                           evidence(paramSlot(2), Evidence::ABORT_IF_NULL),
                           evidence(paramSlot(3), Evidence::ABORT_IF_NULL),
                           evidence(paramSlot(4), Evidence::ABORT_IF_NULL),
                           evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    localVarNamed("A"))));
}

TEST(SmartPointerCollectEvidenceFromDefinitionTest, CheckMacro) {
  llvm::Twine Src = CheckMacroDefinitions + R"cc(
#include <memory>
    void target(std::unique_ptr<int> P, std::unique_ptr<int> Q,
                std::unique_ptr<int> R) {
      CHECK(P);
      CHECK(!!Q);
      CHECK(R.get());
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src.str()),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ABORT_IF_NULL),
                           evidence(paramSlot(1), Evidence::ABORT_IF_NULL),
                           evidence(paramSlot(2), Evidence::ABORT_IF_NULL)));
}

// This is a crash repro; see b/370737278.
TEST(SmartPointerCollectEvidenceFromDefinitionTest,
     CheckMacroSmartPointerToPointer) {
  llvm::Twine Src = CheckMacroDefinitions + R"cc(
#include <memory>

    class Target {
      std::shared_ptr<int*> Shared;

      Target(int* Raw) : Shared(std::make_shared<int*>(Raw)) { CHECK(*Shared); }
    };
  )cc";
  EXPECT_THAT(
      collectFromDefinitionMatching(functionDecl(hasName("Target")), Src.str()),
      IsSupersetOf({(evidence(Slot(0), Evidence::ASSIGNED_FROM_NONNULL,
                              fieldNamed("Target::Shared")),
                     evidence(paramSlot(0), Evidence::ABORT_IF_NULL,
                              functionNamed("Target")))}));
}

TEST(CollectEvidenceFromDefinitionTest, CheckNEMacro) {
  llvm::Twine Src = CheckMacroDefinitions + R"cc(
    void target(int* P, int* Q, int* R, int* S) {
      // should collect evidence for params from these calls
      CHECK_NE(P, nullptr);
      CHECK_NE(nullptr, Q);
      int* A = nullptr;
      CHECK_NE(A, R);
      CHECK_NE(S, A);

      // should not crash when analyzing these calls
      CHECK_NE(A, 0);
      int I = 1;
      CHECK_NE(I, 0);
      bool B = true;
      CHECK_NE(true, false);
      struct ConvertibleToBool {
        bool operator==(const ConvertibleToBool&) const { return false; }
      };
      CHECK_NE(ConvertibleToBool(), ConvertibleToBool());
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src.str()),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ABORT_IF_NULL),
                           evidence(paramSlot(1), Evidence::ABORT_IF_NULL),
                           evidence(paramSlot(2), Evidence::ABORT_IF_NULL),
                           evidence(paramSlot(3), Evidence::ABORT_IF_NULL),
                           evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    localVarNamed("A"))));
}

TEST(SmartPointerCollectEvidenceFromDefinitionTest, CheckNEMacro) {
  llvm::Twine Src = CheckMacroDefinitions + R"cc(
#include <memory>
    void target(std::unique_ptr<int> P, std::unique_ptr<int> Q,
                std::unique_ptr<int> R, std::unique_ptr<int> S) {
      CHECK_NE(P, nullptr);
      CHECK_NE(nullptr, Q);
      if (!R) {
        CHECK_NE(S, R);
      }
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src.str()),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ABORT_IF_NULL),
                           evidence(paramSlot(1), Evidence::ABORT_IF_NULL),
                           evidence(paramSlot(3), Evidence::ABORT_IF_NULL)));
}

TEST(CollectEvidenceFromDefinitionTest, NullableArgPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int *Q);
    void target(Nullable<int *> P) { callee(P); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Contains(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest, NonnullArgPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int *Q);
    void target(Nonnull<int *> P) { callee(P); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Contains(evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT,
                                functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest, UnknownArgPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int *Q);
    void target(int *P) { callee(P); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Contains(evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest, UnknownButProvablyNullArgPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int *Q);
    void target(int *P) {
      if (P == nullptr) {
        callee(P);
      }
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Contains(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest, CheckedArgPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int *Q);
    void target(int *P) {
      if (P) callee(P);
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Contains(evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT,
                                functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest, NullptrPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int* Q);
    void target() {
      callee(nullptr);
      int* P = nullptr;
      callee(P);
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("callee")),
                           evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("callee")),
                           evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    localVarNamed("P"))));
}

TEST(CollectEvidenceFromDefinitionTest, NonPtrArgPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int Q);
    void target(int P) { callee(P); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, LValueReferenceArgsPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void constCallee(int* const& A, int* const& B, int* const& C);
    void mutableCallee(int*& A, int*& B, int*& C);
    void target(Nullable<int*> P, Nonnull<int*> Q, int* R) {
      constCallee(P, Q, R);
      mutableCallee(P, Q, R);
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(
          evidence(paramSlot(0), Evidence::NULLABLE_REFERENCE_ARGUMENT,
                   functionNamed("constCallee")),
          evidence(paramSlot(1), Evidence::NONNULL_REFERENCE_ARGUMENT_AS_CONST,
                   functionNamed("constCallee")),
          evidence(paramSlot(2), Evidence::UNKNOWN_REFERENCE_ARGUMENT,
                   functionNamed("constCallee")),
          evidence(paramSlot(0), Evidence::NULLABLE_REFERENCE_ARGUMENT,
                   functionNamed("mutableCallee")),
          evidence(paramSlot(1), Evidence::NONNULL_REFERENCE_ARGUMENT,
                   functionNamed("mutableCallee")),
          evidence(paramSlot(2), Evidence::UNKNOWN_REFERENCE_ARGUMENT,
                   functionNamed("mutableCallee"))));
}

TEST(CollectEvidenceFromDefinitionTest, RValueUniversalReferenceArgsPassed) {
  static constexpr llvm::StringRef Src = R"cc(
#include <utility>

    template <typename T>
    void universalRef(T&& p);

    void target(int* q) {
      if (!q) {
        universalRef(std::move(q));  // Nullable
        return;
      }
      universalRef(std::move(q));  // Nonnull
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  // RValue references don't have the same invariance as lvalue
                  // references, because accesses through the reference and
                  // through the original variable can't be interleaved. So, we
                  // treat them like non-reference arguments.
                  evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                           functionNamed("universalRef<#*I>")),
                  evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT,
                           functionNamed("universalRef<#*I>"))));
}

TEST(CollectEvidenceFromDefinitionTest, NoEvidenceForFullyAnnotatedFunctions) {
  static constexpr llvm::StringRef Src = R"cc(
    Nonnull<int *> callee(Nullable<int *> A, Nonnull<int *> B,
                          Nullable<int *> &C);
    Nonnull<int *> target(Nullable<int *> P, Nonnull<int *> Q, Nullable<int *> R) {
      return callee(P, Q, R);
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(SmartPointerCollectEvidenceFromDefinitionTest, ArgsAndParams) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
#include <utility>
    void callee(std::unique_ptr<int> P, Nonnull<std::unique_ptr<int>> Q,
                Nullable<std::unique_ptr<int>>& R,
                Nullable<std::unique_ptr<int>>* S);
    void target(Nullable<std::unique_ptr<int>> A, std::unique_ptr<int> B,
                std::unique_ptr<int> C, std::unique_ptr<int> D) {
      callee(std::move(A), std::move(B), C, &D);
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      AllOf(IsSupersetOf(
                {evidence(paramSlot(1), Evidence::ASSIGNED_TO_NONNULL,
                          functionNamed("target")),
                 evidence(paramSlot(2), Evidence::ASSIGNED_TO_MUTABLE_NULLABLE,
                          functionNamed("target")),
                 evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                          functionNamed("callee")),
                 evidence(paramSlot(3), Evidence::NONNULL_ARGUMENT,
                          functionNamed("callee"))}),
            Not(Contains(
                // We aspire to collect ASSIGNED_TO_MUTABLE_NULLABLE evidence
                // for `D` as the inner pointer passed to `S`, but don't yet.
                evidence(paramSlot(3), Evidence::ASSIGNED_TO_MUTABLE_NULLABLE,
                         functionNamed("target"))))));
}

TEST(CollectEvidenceFromDefinitionTest,
     DefaultArgumentsProduceNoEvidenceFromDefinition) {
  static constexpr llvm::StringRef Src = R"cc(
    int* getDefault();
    void hasDefaultUnannotatedFunc(int* = getDefault());
    int* Q = nullptr;
    void hasDefaultUnannotatedVariable(int* = getDefault());
    int I = 1;
    void hasDefaultExpressionOfVariable(int* = &I);
    void target() {
      hasDefaultUnannotatedFunc();
      hasDefaultUnannotatedVariable();
      hasDefaultExpressionOfVariable();
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, NullableReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target() { return nullptr; }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN)));
}

TEST(CollectEvidenceFromDefinitionTest, NullableButCheckedReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target(Nullable<int*> P) {
      if (P) return P;

      // no return in this path to avoid irrelevant evidence, and this still
      // compiles, as the lack of return in a path is only a warning.
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::NONNULL_RETURN)));
}

TEST(CollectEvidenceFromDefinitionTest, NonnullReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target(Nonnull<int*> P) {
      return P;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::NONNULL_RETURN)));
}

TEST(CollectEvidenceFromDefinitionTest, UnknownReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target(int* P) { return P; }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::UNKNOWN_RETURN)));
}

TEST(CollectEvidenceFromDefinitionTest, UnknownButProvablyNullReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target(int* P) {
      if (P == nullptr) {
        return P;
      }
      // no return in this path to avoid irrelevant evidence, and this still
      // compiles, as the lack of return in a path is only a warning.
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN)));
}

TEST(CollectEvidenceFromDefinitionTest, MultipleReturns) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target(Nonnull<int*> P, Nullable<int*> Q, bool B, bool C) {
      if (B) return Q;
      if (C) return nullptr;
      return P;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN),
                  evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN),
                  evidence(SLOT_RETURN_TYPE, Evidence::NONNULL_RETURN)));
}

TEST(CollectEvidenceFromDefinitionTest, MutableReferenceReturns) {
  static constexpr llvm::StringRef Src = R"cc(
    int*& target(Nonnull<int*>& P, Nullable<int*>& Q, int*& R, bool A, bool B) {
      if (A) return P;
      if (B) return Q;
      return R;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(
          evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_REFERENCE_RETURN),
          evidence(SLOT_RETURN_TYPE, Evidence::NONNULL_REFERENCE_RETURN),
          evidence(SLOT_RETURN_TYPE, Evidence::UNKNOWN_REFERENCE_RETURN)));
}

TEST(CollectEvidenceFromDefinitionTest, ConstReferenceReturns) {
  static constexpr llvm::StringRef Src = R"cc(
    int* const& target(Nonnull<int*>& P, Nullable<int*>& Q, int*& R, bool A,
                       bool B) {
      if (A) return P;
      if (B) return Q;
      return R;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(
          evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_REFERENCE_RETURN),
          evidence(SLOT_RETURN_TYPE,
                   Evidence::NONNULL_REFERENCE_RETURN_AS_CONST),
          evidence(SLOT_RETURN_TYPE, Evidence::UNKNOWN_REFERENCE_RETURN)));
}

TEST(CollectEvidenceFromDefinitionTest, FromReturnAnnotation) {
  static constexpr llvm::StringRef Src = R"cc(
    Nonnull<int*> target(int* A) {
      return A;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL)));
}

TEST(CollectEvidenceFromDefinitionTest,
     FromPreviouslyInferredReturnAnnotation) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target(int* A) { return A; }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(
          Src, {.Nonnull = std::make_shared<SortedFingerprintVector>(
                    std::vector<SlotFingerprint>{
                        fingerprint("c:@F@target#*I#", 0)})}),
      UnorderedElementsAre(
          evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL),
          // We still collect evidence for the return type in case iteration
          // turns up new evidence to contradict a previous inference. Only
          // nullabilities written in source code are considered unchangeable.
          evidence(SLOT_RETURN_TYPE, Evidence::UNKNOWN_RETURN)));
}

TEST(CollectEvidenceFromDefinitionTest, FromAutoReturnAnnotationByPragma) {
  static constexpr llvm::StringRef Src = R"cc(
#pragma nullability file_default nonnull
    int* getNonnull();

    // The pragma applies to the int* deduced for the `auto` return type,
    // making the return type Nonnull<int*>.
    auto target(NullabilityUnknown<int*> A, bool B) {
      if (B) return A;
      return getNonnull();
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL,
                                    functionNamed("target"))));
}

// This is a crash repro related to functions with AttributedTypeLocs.
TEST(CollectEvidenceFromDefinitionTest, FromReturnInAttributedFunction) {
  static constexpr llvm::StringRef Src = R"cc(
    struct AStruct {
      const char* target() [[clang::lifetimebound]] { return nullptr; }
    };
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN,
                                    functionNamed("target"))));
}

TEST(SmartPointerCollectEvidenceFromDefinitionTest, MultipleReturns) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    std::unique_ptr<int> target(Nonnull<std::unique_ptr<int>> P,
                                Nullable<std::unique_ptr<int>> Q,
                                std::unique_ptr<int> R, bool A, bool B,
                                bool C) {
      if (A) return nullptr;
      if (B) return P;
      if (C) return Q;
      return R;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(
          evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN),
          evidence(SLOT_RETURN_TYPE, Evidence::NONNULL_RETURN),
          evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN),
          evidence(SLOT_RETURN_TYPE, Evidence::UNKNOWN_RETURN),
          // evidence for the move constructor, which we don't care much about.
          evidence(_, _, functionNamed("unique_ptr")),
          evidence(_, _, functionNamed("unique_ptr")),
          evidence(_, _, functionNamed("unique_ptr"))));
}

TEST(SmartPointerCollectEvidenceFromDefinitionTest, FromReturnAnnotation) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    Nonnull<std::unique_ptr<int>> target(std::unique_ptr<int> A) {
      return A;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(
          evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL),
          // evidence for the move constructor, which we don't care much about.
          evidence(_, _, functionNamed("unique_ptr"))));
}

TEST(CollectEvidenceFromDefinitionTest, FunctionCallDereferenced) {
  static constexpr llvm::StringRef Src = R"cc(
    int* makePtr();
    void target() { *makePtr(); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      Contains(evidence(SLOT_RETURN_TYPE, Evidence::UNCHECKED_DEREFERENCE,
                        functionNamed("makePtr"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     FunctionCallResultDereferencedAfterAssignedLocally) {
  static constexpr llvm::StringRef Src = R"cc(
    int* makePtr();
    void target() {
      auto P = makePtr();
      *P;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      Contains(evidence(SLOT_RETURN_TYPE, Evidence::UNCHECKED_DEREFERENCE,
                        functionNamed("makePtr"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     FunctionCallResultDereferencedAfterAssignedLocallyAndChecked) {
  static constexpr llvm::StringRef Src = R"cc(
    int* makePtr();
    void target() {
      auto P = makePtr();
      if (P) *P;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      Not(Contains(evidence(SLOT_RETURN_TYPE, Evidence::UNCHECKED_DEREFERENCE,
                            functionNamed("makePtr")))));
}

TEST(CollectEvidenceFromDefinitionTest,
     FunctionCallResultDereferencedAfterUnrelatedConditionChecked) {
  static constexpr llvm::StringRef Src = R"cc(
    int* makePtr();
    void target(bool Cond) {
      auto P = makePtr();
      if (Cond) *P;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      Contains(evidence(SLOT_RETURN_TYPE, Evidence::UNCHECKED_DEREFERENCE,
                        functionNamed("makePtr"))));
}

TEST(CollectEvidenceFromDefinitionTest, FunctionCallDereferencedWithArrow) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      void member();
    };

    S* makePtr();
    void target() { makePtr()->member(); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      Contains(evidence(SLOT_RETURN_TYPE, Evidence::UNCHECKED_DEREFERENCE,
                        functionNamed("makePtr"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     AlreadyNonnullFunctionCallDereferenced) {
  static constexpr llvm::StringRef Src = R"cc(
    Nonnull<int*> makeNonnullPtr();
    void target() { *makeNonnullPtr(); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, FunctionPointerCall) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(void (*F)()) { F(); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

// This is a crash repro; see b/352043668.
TEST(CollectEvidenceFromDefinitionTest, FunctionPointerCallThroughBindingDecl) {
  static constexpr llvm::StringRef Src = R"cc(
    template <typename A, typename B>
    struct Pair {
      Pair();

      A AnA;
      B AB;
    };

    void target(int *I) {
      Pair<void (*)(Nonnull<int *>), bool> P;
      auto [FP, B] = P;
      FP(I);
    }
  )cc";
  // Ideally, we would see the Nonnull from `P`'s template parameter and collect
  // ASSIGNED_TO_NONNULL evidence for `I`, but the sugar doesn't carry through
  // the BindingDecl's `auto` type.
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, ConstAccessorDereferencedAfterCheck) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      int* accessor() const { return I; }
      int* I = nullptr;
    };
    void target() {
      S AnS;
      if (AnS.accessor() != nullptr) {
        *AnS.accessor();
      }
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest,
     ReferenceConstAccessorDereferencedAfterCheck) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      int* const& accessor() const { return I; }
      int* I = nullptr;
    };
    void target() {
      S AnS;
      if (AnS.accessor() != nullptr) {
        *AnS.accessor();
      }
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest,
     ConstAccessorOnTwoDifferentObjectsDereferencedAfterCheck) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      int* const& accessor() const { return I; }
      int* I = nullptr;
    };

    S makeS();

    void target() {
      if (makeS().accessor() != nullptr) {
        *makeS().accessor();
      }
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(evidence(SLOT_RETURN_TYPE,
                                            Evidence::UNCHECKED_DEREFERENCE,
                                            functionNamed("accessor"))));
}

TEST(CollectEvidenceFromDefinitionTest, MemberCallOperatorReturnDereferenced) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      int* operator()();
    };
    void target() {
      S AnS;
      *AnS();
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      Contains(evidence(SLOT_RETURN_TYPE, Evidence::UNCHECKED_DEREFERENCE,
                        functionNamed("operator()"))));
}

TEST(CollectEvidenceFromDefinitionTest, MemberOperatorCall) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      bool operator+(int*);
    };
    void target() { S{} + nullptr; }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Contains(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                functionNamed("operator+"))));
}

TEST(CollectEvidenceFromDefinitionTest, NonMemberOperatorCall) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {};
    bool operator+(const S&, int*);
    void target() { S{} + nullptr; }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Contains(evidence(paramSlot(1), Evidence::NULLABLE_ARGUMENT,
                                functionNamed("operator+"))));
}

TEST(CollectEvidenceFromDefinitionTest, VarArgs) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int*...);
    void target() { callee(nullptr, nullptr); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest, MemberOperatorCallVarArgs) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      bool operator()(int*...);
    };
    void target() { S{}(nullptr, nullptr); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("operator()"))));
}

TEST(CollectEvidenceFromDefinitionTest, ConstructorCall) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      S(Nonnull<int*> A, int* B);
    };
    void target(int* P) { S AnS(P, nullptr); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(paramSlot(1), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("S"))));
}

TEST(CollectEvidenceFromDefinitionTest, ConstructorCallThroughMakeUnique) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    struct S {
      S(Nonnull<int*> A, int* B);
    };
    void target(int* P) { std::make_unique<S>(P, nullptr); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(paramSlot(1), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("S"))));
}

TEST(CollectEvidenceFromDefinitionTest, ConstructorWithBaseInitializer) {
  static constexpr llvm::StringRef Src = R"cc(
    struct TakeNonnull {
      explicit TakeNonnull(Nonnull<int *>);
    };
    struct Target : TakeNonnull {
      Target(int *I) : TakeNonnull(I) {}
    };
  )cc";
  EXPECT_THAT(collectFromDefinitionNamed("Target", Src),
              Contains(evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL,
                                functionNamed("Target"))));
}

TEST(CollectEvidenceFromDefinitionTest, ConstructorWithDelegatingConstructor) {
  static constexpr llvm::StringRef Src = R"cc(
    struct Target {
      Target(int* I);
      Target() : Target(nullptr) {};
    };
  )cc";

  EXPECT_THAT(collectFromDefinitionMatching(
                  functionDecl(hasName("Target"), parameterCountIs(0)), Src),
              Contains(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                functionNamed("Target"))));
}

TEST(CollectEvidenceFromDefinitionTest, VariadicConstructorCall) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      S(Nonnull<int*> I, ...);
    };
    void target(int* P, int* Q) { S AnS(P, Q); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     VariadicConstructorCallThroughMakeUnique) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    struct S {
      S(Nonnull<int*> I, ...);
    };
    void target(int* P, int* Q) { std::make_unique<S>(P, Q); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, ConstructorCallWithConversionOperator) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      S(Nonnull<int*> A);
    };
    struct ConvertibleToIntPtr {
      ConvertibleToIntPtr(int* p) : p_(p) {}
      operator int*() { return p_; }
      int* p_;
    };
    void target(int* P) { S AnS(ConvertibleToIntPtr{P}); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::ASSIGNED_TO_NONNULL,
                           functionNamed("operator int *")),
                  evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                           functionNamed("ConvertibleToIntPtr"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     ConstructorCallThroughMakeUniqueWithConversionOperator) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    struct S {
      S(Nonnull<int*> A);
    };
    struct ConvertibleToIntPtr {
      ConvertibleToIntPtr(int* p) : p_(p) {}
      operator int*() { return p_; }
      int* p_;
    };
    void target(int* P) { std::make_unique<S>(ConvertibleToIntPtr{P}); }
  )cc";

  // The implicit conversion from ConvertibleToIntPtr to int* happens within
  // make_unique instead of at the call site in target, so we don't collect that
  // evidence. However, we collect the evidence from the make_unique
  // instantiation and will do inference from that.
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("ConvertibleToIntPtr"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     MakeUniqueImplicitCastNothingToForward) {
  static constexpr llvm::StringRef Src =
      R"cc(
#include <memory>
    struct Foo {
      int *_Nonnull p;
    };

    struct Bar {
      int *_Nonnull q;
      operator Foo() { return Foo{q}; }
    };

    // No evidence to collect -- the make_unique just calls the user-defined
    // conversion operator Foo() with no arguments.
    void target(Bar b) { std::make_unique<Foo>(b); }
      )cc";

  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, FieldInitializerFromAssignmentToType) {
  static constexpr llvm::StringRef Src = R"cc(
    struct Target {
      Target(int *Input) : I(Input) {}
      Nonnull<int *> I;
    };
  )cc";
  EXPECT_THAT(
      collectFromDefinitionNamed("Target", Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL,
                                    functionNamed("Target"))));
}

TEST(CollectEvidenceFromDefinitionTest, DefaultFieldInitializerNullptr) {
  static constexpr llvm::StringRef Src = R"cc(
    struct Target {
      Target() {};
      int* I = nullptr;
    };
  )cc";
  EXPECT_THAT(collectFromDefinitionMatching(
                  cxxConstructorDecl(isDefaultConstructor()), Src),
              UnorderedElementsAre(evidence(
                  Slot(0), Evidence::NULLPTR_DEFAULT_MEMBER_INITIALIZER,
                  fieldNamed("Target::I"))));
}

TEST(CollectEvidenceFromDefinitionTest, DefaultFieldInitializerNullable) {
  static constexpr llvm::StringRef Src = R"cc(
    Nullable<int*> G;
    struct Target {
      Target() {};
      int* I = G;
    };
  )cc";
  EXPECT_THAT(
      collectFromDefinitionMatching(cxxConstructorDecl(isDefaultConstructor()),
                                    Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    fieldNamed("Target::I"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     IndirectFieldInitializerFromAssignmentToType) {
  static constexpr llvm::StringRef Src = R"cc(
    struct Target {
      Target(int *Input) : I(Input) {}
      struct {
        Nonnull<int *> I;
      };
    };
  )cc";
  EXPECT_THAT(
      collectFromDefinitionNamed("Target", Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL,
                                    functionNamed("Target"))));
}

TEST(CollectEvidenceFromDefinitionTest, IndirectFieldDefaultFieldInitializer) {
  static constexpr llvm::StringRef Src = R"cc(
    struct Target {
      Target() {}
      struct {
        int* I = nullptr;
      };
    };

    // Use the implicitly-declared default constructor, so that it will be
    // generated.
    Target T;
  )cc";
  EXPECT_THAT(
      collectFromDefinitionMatching(
          cxxConstructorDecl(isDefaultConstructor(), hasName("Target")), Src),
      UnorderedElementsAre(
          evidence(Slot(0), Evidence::NULLPTR_DEFAULT_MEMBER_INITIALIZER,
                   fieldNamed("Target@Sa::I"))));
}

TEST(CollectEvidenceFromDefinitionTest, FieldInitializedWithNullable) {
  static constexpr llvm::StringRef Src = R"cc(
    struct Target {
      Target(Nullable<int *> Input) : I(Input) {}
      int *I;
    };
  )cc";
  EXPECT_THAT(
      collectFromDefinitionNamed("Target", Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    fieldNamed("Target::I"))));
}

TEST(EvidenceEmitterTest, UnionFieldInitializedWithNullable) {
  static constexpr llvm::StringRef Src = R"cc(
    struct Target {
      Target() : Field{nullptr} {};

      union UnionType {
        int* I;
        bool* B;
      } Field;
    };
  )cc";

  EXPECT_THAT(
      collectFromDefinitionNamed("Target", Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    fieldNamed("UnionType::I"))));
}

TEST(CollectEvidenceFromDefinitionTest, FieldInitializerCallsFunction) {
  static constexpr llvm::StringRef Src = R"cc(
    int* getIntPtr(int*);
    struct Target {
      Target() : I(getIntPtr(nullptr)) {}
      Nonnull<int*> I;
    };
  )cc";
  EXPECT_THAT(collectFromDefinitionNamed("Target", Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::ASSIGNED_TO_NONNULL,
                           functionNamed("getIntPtr")),
                  evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                           functionNamed("getIntPtr"))));
}

TEST(CollectEvidenceFromDefinitionTest, DefaultFieldInitializerCallsFunction) {
  static constexpr llvm::StringRef Src = R"cc(
    int* getIntPtr(int*);
    struct Target {
      Target() = default;
      Nonnull<int*> I = getIntPtr(nullptr);
    };

    // Use the explicitly-declared but still implicitly-defined default
    // constructor, so that it will be generated.
    Target T;
  )cc";
  EXPECT_THAT(collectFromDefinitionMatching(
                  cxxConstructorDecl(isDefaultConstructor()), Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::ASSIGNED_TO_NONNULL,
                           functionNamed("getIntPtr")),
                  evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                           functionNamed("getIntPtr"))));
}

TEST(SmartPointerCollectEvidenceFromDefinitionTest,
     FieldInitializerFromAssignmentToType) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
#include <utility>
    struct Target {
      Target(std::unique_ptr<int> Input) : I(std::move(Input)) {}
      Nonnull<std::unique_ptr<int>> I;
    };
  )cc";
  EXPECT_THAT(
      collectFromDefinitionMatching(
          cxxConstructorDecl(unless(isImplicit()), hasName("Target")), Src),
      UnorderedElementsAre(
          evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL,
                   functionNamed("Target")),
          // evidence for the move constructor, which we don't care much about.
          evidence(_, _, functionNamed("unique_ptr"))));
}

TEST(SmartPointerCollectEvidenceFromDefinitionTest,
     FieldInitializedFromNullable) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
#include <utility>
    struct Target {
      Target(Nullable<std::unique_ptr<int>> Input) : I(std::move(Input)) {}
      std::unique_ptr<int> I;
    };
  )cc";
  EXPECT_THAT(
      collectFromDefinitionMatching(
          cxxConstructorDecl(unless(isImplicit()), hasName("Target")), Src),
      UnorderedElementsAre(
          evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                   fieldNamed("Target::I")),
          // evidence for the move constructor, which we don't care much about.
          evidence(_, _, functionNamed("unique_ptr"))));
}

TEST(SmartPointerCollectEvidenceFromDefinitionTest,
     DefaultFieldInitializerNullptr) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    struct Target {
      std::unique_ptr<int> I = nullptr;
    };

    // Use the implicitly-declared default constructor, so that it will be
    // generated.
    Target T;
  )cc";
  EXPECT_THAT(
      collectFromDefinitionMatching(
          cxxConstructorDecl(isDefaultConstructor(), hasName("Target")), Src),
      UnorderedElementsAre(
          evidence(Slot(0), Evidence::NULLPTR_DEFAULT_MEMBER_INITIALIZER,
                   fieldNamed("Target::I"))));
}

TEST(SmartPointerCollectEvidenceFromDefinitionTest,
     DefaultFieldInitializerAbsentOnlyImplicitConstructor) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    struct Target {
      std::unique_ptr<int> I;
    };

    // Use the implicitly-declared default constructor, so that it will be
    // generated.
    Target T;
  )cc";
  EXPECT_THAT(
      collectFromDefinitionMatching(
          cxxConstructorDecl(isDefaultConstructor(), hasName("Target")), Src),
      // By the end of the constructor body, the field is still only
      // default-initialized, which for smart pointers means it is null.
      UnorderedElementsAre(evidence(Slot(0),
                                    Evidence::LEFT_NULLABLE_BY_CONSTRUCTOR,
                                    fieldNamed("Target::I"))));
}

TEST(SmartPointerCollectEvidenceFromDefinitionTest,
     DefaultFieldInitializerAbsentInitializedInConstructor) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    struct Target {
      Target(int Input) { I = std::make_unique<int>(Input); }
      std::unique_ptr<int> I;
    };
  )cc";
  EXPECT_THAT(
      collectFromDefinitionMatching(
          cxxConstructorDecl(unless(isImplicit()), hasName("Target")), Src),
      // Evidence collected from constructor body, which assigns a Nonnull
      // value, but no evidence collected from *implicit* member initializer
      // which default constructs to null.
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_NONNULL,
                                    fieldNamed("Target::I")),
                           // evidence for the move assignment operator for
                           // unique_ptr, which we don't care much about.
                           evidence(_, _, functionNamed("operator="))));
}

TEST(SmartPointerCollectEvidenceFromDefinitionTest,
     DefaultFieldInitializerAbsentConditionalAssignmentInConstructor) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    struct Target {
      Target(int Input) {
        if (Input != 0) {
          I = std::make_unique<int>(Input);
        }
      }
      std::unique_ptr<int> I;
    };
  )cc";
  EXPECT_THAT(
      collectFromDefinitionMatching(
          cxxConstructorDecl(unless(isImplicit()), hasName("Target")), Src),
      // By the end of the constructor body, the field is still potentially
      // default-initialized, which for smart pointers means it may be null.
      // We also collect from the Nonnull value assignment in the body, though
      // this doesn't end up affecting the inferred annotation.
      UnorderedElementsAre(
          evidence(Slot(0), Evidence::LEFT_NULLABLE_BY_CONSTRUCTOR,
                   fieldNamed("Target::I")),
          evidence(Slot(0), Evidence::ASSIGNED_FROM_NONNULL,
                   fieldNamed("Target::I")),
          // evidence for the move assignment operator for
          // unique_ptr, which we don't care much about.
          evidence(_, _, functionNamed("operator="))));
}

TEST(SmartPointerCollectEvidenceFromDefinitionTest,
     DefaultFieldInitializerAbsentUnknownAssignmentInConstructor) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    std::unique_ptr<int> getUnknown();

    struct Target {
      Target() { I = getUnknown(); }
      std::unique_ptr<int> I;
    };
  )cc";

  EXPECT_THAT(
      collectFromDefinitionMatching(
          cxxConstructorDecl(unless(isImplicit()), hasName("Target")), Src),
      // By the end of the constructor body, the field is no longer default
      // initialized to null, but is assigned from an unknown.
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_UNKNOWN,
                                    fieldNamed("Target::I")),
                           // evidence for the move assignment operator for
                           // unique_ptr, which we don't care much about.
                           evidence(_, _, functionNamed("operator="))));
}

// This is a crash repro.
TEST(SmartPointerCollectEvidenceFromDefinitionTest,
     CopyConstructorExitingWithUnmodeledField) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    struct Target {
      // other.Field is not modeled and has no null state attached. Its value is
      // coped into this.Field, leaving it without null state at the end of the
      // constructor.
      Target(Target& other) { *this = other; }
      Target& operator=(const Target& other);
      std::unique_ptr<int> Field;
    };
  )cc";
  EXPECT_THAT(
      collectFromDefinitionMatching(
          cxxConstructorDecl(unless(isImplicit()), hasName("Target")), Src),
      IsEmpty());
}

// This is a crash repro; see b/369863079.
TEST(SmartPointerCollectEvidenceFromDefinitionTest,
     CustomConstructorExitingWithUnmodeledField) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    struct Target {
      // other.Field is not modeled and has no null state attached. Its value is
      // coped into this.Field, leaving it without null state at the end of the
      // constructor. Note that this is not technically a copy constructor.
      Target(Target& other, bool b) { *this = other; }
      Target& operator=(const Target& other);
      std::unique_ptr<int> Field;
    };
  )cc";
  EXPECT_THAT(
      collectFromDefinitionMatching(
          cxxConstructorDecl(unless(isImplicit()), hasName("Target")), Src),
      IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, LateInitializerDirectlyForTest) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    namespace testing {
    class Test {
     public:
      virtual void SetUp() = 0;
      virtual ~Test();
    };
    }  // namespace testing

    class Target : public ::testing::Test {
     protected:
      void SetUp() override { FieldInitializedInSetUp = std::make_unique<int>(0); }
      std::unique_ptr<int> FieldInitializedInSetUp;
      std::unique_ptr<int> NotInit;
    };
  )cc";
  EXPECT_THAT(
      collectFromDefinitionMatching(
          cxxMethodDecl(hasName("SetUp"), ofClass(hasName("Target"))), Src),
      AllOf(Contains(evidence(Slot(0),
                              Evidence::LEFT_NOT_NULLABLE_BY_LATE_INITIALIZER,
                              fieldNamed("Target::FieldInitializedInSetUp"))),
            Not(Contains(evidence(_, _, fieldNamed("Target::NotInit"))))));
}

TEST(CollectEvidenceFromDefinitionTest, LateInitializerThroughAliasForTest) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>

    namespace testing {
    class Test {
     public:
      virtual void SetUp() = 0;
      virtual ~Test();
    };
    }  // namespace testing

    using TestAlias = ::testing::Test;

    // Even though the base class is named through an alias, we detect that this
    // class inherits from testing::Test.
    class Target : public TestAlias {
     protected:
      void SetUp() override { FieldInitializedInSetUp = std::make_unique<int>(1); }

      std::unique_ptr<int> FieldInitializedInSetUp;
    };
  )cc";
  EXPECT_THAT(
      collectFromDefinitionMatching(
          cxxMethodDecl(hasName("SetUp"), ofClass(hasName("Target"))), Src),
      Contains(evidence(Slot(0),
                        Evidence::LEFT_NOT_NULLABLE_BY_LATE_INITIALIZER,
                        fieldNamed("Target::FieldInitializedInSetUp"))));
}

TEST(CollectEvidenceFromDefinitionTest, PassedToNonnull) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nonnull<int*> I);

    void target(int* P) { callee(P); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, PassedToNonnullRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nonnull<int*>& I, Nonnull<int*> const& J);

    void target(int* P, int* Q) { callee(P, Q); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(
          evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL_REFERENCE,
                   functionNamed("target")),
          evidence(paramSlot(1), Evidence::ASSIGNED_TO_NONNULL_REFERENCE,
                   functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, PassedToNonnullInMemberFunction) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      void callee(Nonnull<int*> I);
    };

    void target(int* P) {
      S AnS;
      AnS.callee(P);
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, PassedToNonnullInFunctionPointerParam) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P, void (*Callee)(Nonnull<int*> I)) {
      Callee(P);
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL,
                           functionNamed("target")),
                  evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE,
                           functionNamed("target"))));
}

TEST(SmartPointerCollectEvidenceFromDefinitionTest,
     PassedToNonnullInFunctionPointerParam) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
#include <utility>
    void target(std::unique_ptr<int*> P,
                void (*Callee)(Nonnull<std::unique_ptr<int*>> I)) {
      Callee(std::move(P));
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(
          evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL,
                   functionNamed("target")),
          evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE,
                   functionNamed("target")),
          // evidence for the move constructor, which we don't care much about.
          evidence(_, _, functionNamed("unique_ptr"))));
}

TEST(CollectEvidenceFromDefinitionTest, PassedToNonnullInFunctionPointerField) {
  static constexpr llvm::StringRef Src = R"cc(
    struct MyStruct {
      void (*Callee)(Nonnull<int*>);
    };

    void target(int* P) { MyStruct().Callee(P); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                                    fieldNamed("MyStruct::Callee"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     PassedToNonnullInFunctionPointerFromAddressOfFunctionDecl) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nonnull<int*> I);

    void target(int* P) { (&callee)(P); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     PassedToNonnullInFunctionReferenceParam) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P, void (&Callee)(Nonnull<int*> I)) {
      Callee(P);
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     PassedToNonnullInFunctionPointerReferenceParam) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P, void (*&Callee)(Nonnull<int*> I)) {
      Callee(P);
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL,
                           functionNamed("target")),
                  evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE,
                           functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, FunctionCallPassedToNonnull) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nonnull<int*> I);
    int* makeIntPtr();

    void target() { callee(makeIntPtr()); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(evidence(SLOT_RETURN_TYPE,
                                            Evidence::ASSIGNED_TO_NONNULL,
                                            functionNamed("makeIntPtr"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     FunctionCallPassedToNonnullFunctionPointer) {
  static constexpr llvm::StringRef Src = R"cc(
    int* makeIntPtr();

    void target(void (*Callee)(Nonnull<int*> I)) { Callee(makeIntPtr()); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::ASSIGNED_TO_NONNULL,
                           functionNamed("makeIntPtr")),
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                           functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, PassedToNullable) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nullable<int*> I);

    void target(int* P) { callee(P); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromDefinitionTest, PassedToNullableRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nullable<int*>& I);

    void target(int* P) { callee(P); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_TO_MUTABLE_NULLABLE,
                           functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     PassedToNullableRefFromStoredFunctionCall) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nullable<int*>& I);
    int* producer();

    void target() {
      auto P = producer();
      callee(P);
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  // The object taken by reference (P) needs to be nullable, not
                  // necessarily the source of its value (producer).
                  evidence(Slot(0), Evidence::ASSIGNED_TO_MUTABLE_NULLABLE,
                           localVarNamed("P", "target")),
                  evidence(Slot(0), Evidence::ASSIGNED_FROM_UNKNOWN,
                           localVarNamed("P", "target"))));
}

TEST(CollectEvidenceFromDefinitionTest, PassedToNullableRefFromFunctionCall) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nullable<int*>& I);
    int*& producer();

    void target() { callee(producer()); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(evidence(
                  SLOT_RETURN_TYPE, Evidence::ASSIGNED_TO_MUTABLE_NULLABLE,
                  functionNamed("producer"))));
}

TEST(CollectEvidenceFromDefinitionTest, PassedToPtrToNullable) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nullable<int*>* I);
    void target(int* P) { callee(&P); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      AllOf(UnorderedElementsAre(evidence(paramSlot(0),
                                          Evidence::NONNULL_ARGUMENT,
                                          functionNamed("callee"))),
            Not(Contains(
                // We aspire to collect ASSIGNED_TO_MUTABLE_NULLABLE evidence
                // for `P` as the inner pointer passed to `I`, but don't yet.
                evidence(paramSlot(0), Evidence::ASSIGNED_TO_MUTABLE_NULLABLE,
                         functionNamed("target"))))));
}

TEST(CollectEvidenceFromDefinitionTest,
     InitializationOfAndAssignmentToNonnull) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P, int* Q, int* R) {
      Nonnull<int*> A = P, B = Q;
      A = R;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL),
                  evidence(paramSlot(1), Evidence::ASSIGNED_TO_NONNULL),
                  evidence(paramSlot(2), Evidence::ASSIGNED_TO_NONNULL)));
}

TEST(CollectEvidenceFromDefinitionTest,
     InitializationOfAndAssignmentToNonnullFromTernary) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(bool B, int* P, int* Q, int* R, int* S) {
      Nonnull<int*> A = B ? P : Q;
      A = B ? R : S;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
  // TODO(b/293609145) When value nullability for conditional operators is
  // carried through for glvalues, this should collect the following:
  // UnorderedElementsAre(evidence(paramSlot(1), Evidence::ASSIGNED_TO_NONNULL),
  //                      evidence(paramSlot(2), Evidence::ASSIGNED_TO_NONNULL),
  //                      evidence(paramSlot(3, Evidence::ASSIGNED_TO_NONNULL)),
  //                      evidence(paramSlot(4,
  //                      Evidence::ASSIGNED_TO_NONNULL)));
}

TEST(SmartPointerCollectEvidenceFromDefinitionTest,
     InitializationOfAndAssignmentToNonnull) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
#include <utility>
    struct SomeType {
      Nonnull<std::unique_ptr<int>> Field;
      Nonnull<std::unique_ptr<int>>& getRef();
    };

    void target(std::unique_ptr<int> P, Nonnull<std::unique_ptr<int>> Q,
                std::unique_ptr<int> R, std::unique_ptr<int> S,
                std::unique_ptr<int> T) {
      Q = std::move(P);
      SomeType SomeObject;
      SomeObject.Field = std::move(R);
      SomeObject.getRef() = std::move(S);
      Nonnull<std::unique_ptr<int>> nonnull = std::move(T);
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL),
                  evidence(paramSlot(2), Evidence::ASSIGNED_TO_NONNULL),
                  evidence(paramSlot(3), Evidence::ASSIGNED_TO_NONNULL),
                  evidence(paramSlot(4), Evidence::ASSIGNED_TO_NONNULL),
                  // evidence for the move constructor and move assignment
                  // operator, which we don't care much about.
                  evidence(_, _, functionNamed("unique_ptr")),
                  evidence(_, _, functionNamed("operator=")),
                  evidence(_, _, functionNamed("operator=")),
                  evidence(_, _, functionNamed("operator="))));
}

TEST(CollectEvidenceFromDefinitionTest,
     InitializationOfAndAssignmentToNonnullRefFromRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int*& P, int*& Q, int*& R) {
      Nonnull<int*>& A = P;
      A = Q;
      Nonnull<int*> const& B = R;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(
          evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL_REFERENCE),
          // `A = Q;` copies Q into P; it doesn't make a reference to Q,
          // so only ASSIGNED_TO_NONNULL.
          evidence(paramSlot(1), Evidence::ASSIGNED_TO_NONNULL),
          evidence(paramSlot(2), Evidence::ASSIGNED_TO_NONNULL_REFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest,
     InitializationOfAndAssignmentToNullableOrUnknown) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P, int* Q, int* R) {
      Nullable<int*> A = P;
      int* B = Q;
      NullabilityUnknown<int*> C = R;
      Q = R;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(1), Evidence::ASSIGNED_FROM_UNKNOWN),
                  evidence(Slot(0), Evidence::ASSIGNED_FROM_UNKNOWN,
                           localVarNamed("B")),
                  evidence(Slot(0), Evidence::ASSIGNED_FROM_UNKNOWN,
                           localVarNamed("C"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     InitializationOfAndAssignmentToNullableRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P, int*& Q) {
      Nullable<int*>& A = P;
      A = Q;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_TO_MUTABLE_NULLABLE)
                  // `A = Q;` copies Q into P; it doesn't make a reference to Q,
                  // so no evidence for Q.
                  ));
}

TEST(SmartPointerCollectEvidenceFromDefinitionTest,
     InitializationOfNullableRef) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    void target(std::unique_ptr<int> P) {
      Nullable<std::unique_ptr<int>>& A = P;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_TO_MUTABLE_NULLABLE,
                           functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, InitializationOfNullableRefFromRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int*& P) {
      Nullable<int*>& A = P;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(evidence(
                  paramSlot(0), Evidence::ASSIGNED_TO_MUTABLE_NULLABLE)));
}

// Ternary expressions are not currently modeled correctly by the analysis, but
// are necessary to test the case of multiple connected decls.
//
// DISABLED until ternary expressions are handle.
TEST(CollectEvidenceFromDefinitionTest,
     DISABLED_InitializationOfNullableRefAllConnectedDecls) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P, int* Q, bool B) {
      Nullable<int*>& X = B ? P : Q;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(
          evidence(paramSlot(0), Evidence::ASSIGNED_TO_MUTABLE_NULLABLE),
          evidence(paramSlot(1), Evidence::ASSIGNED_TO_MUTABLE_NULLABLE)));
}

TEST(CollectEvidenceFromDefinitionTest, AssignedFromNullptr) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P) {
      P = nullptr;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_FROM_NULLABLE)));
}

TEST(CollectEvidenceFromDefinitionTest, AssignedFromNullptrIndirect) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P) {
      int* A = nullptr;
      P = A;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_FROM_NULLABLE),
                  evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                           localVarNamed("A"))));
}

TEST(CollectEvidenceFromDefinitionTest, AssignedFromZero) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P) { P = 0; }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_FROM_NULLABLE)));
}

TEST(CollectEvidenceFromDefinitionTest, AssignedFromNullable) {
  static constexpr llvm::StringRef Src = R"cc(
    Nullable<int*> getNullable();
    void target(int* P) { P = getNullable(); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(evidence(paramSlot(0),
                                            Evidence::ASSIGNED_FROM_NULLABLE,
                                            functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, AssignedFromLocalNullable) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P) {
      Nullable<int*> A;
      P = A;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_FROM_NULLABLE)));
}

TEST(CollectEvidenceFromDefinitionTest, AssignedFromNullableMemberCallExpr) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      int*& getPtrRef();
    };

    void target(S AnS) { AnS.getPtrRef() = nullptr; }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(evidence(SLOT_RETURN_TYPE,
                                            Evidence::ASSIGNED_FROM_NULLABLE,
                                            functionNamed("getPtrRef"))));
}

TEST(CollectEvidenceFromDefinitionTest, AssignedFromNullptrMultipleOperators) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P) {
      *&P = nullptr;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_FROM_NULLABLE)));
}

// This is a regression test for a bug where we collected ASSIGNED_FROM_NULLABLE
// evidence for the return type of `foo`, because the LHS type of the assignment
// was already nullable, and so any formula does imply that the LHS type of the
// assignment is nullable.
TEST(CollectEvidenceFromDefinitionTest,
     AnnotatedLocalAssignedFromNullableAfterFunctionCallAssignment) {
  static constexpr llvm::StringRef Src = R"cc(
    int* foo();
    void target() {
      Nullable<int*> P = foo();
      P = nullptr;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, AssignedFromNonnull) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P) {
      int A = 0;
      P = &A;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_FROM_NONNULL)));
}

TEST(CollectEvidenceFromDefinitionTest, AssignedFromUnknown) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P, int* Q) {
      P = Q;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_FROM_UNKNOWN)));
}

TEST(CollectEvidenceFromDefinitionTest,
     IrrelevantAssignmentsAndInitializations) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      S(int* I);
    };

    void target(int* P) {
      // We don't collect if types on either side are not a supported pointer
      // type.
      int A = 4;
      bool B = false;
      S AnS = P;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      // From the constructor call constructing an S; no evidence from
      // assignments or initializations.
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("S"))));
}

// This is a crash repro; see b/370031684 and b/293609145.
TEST(CollectEvidenceFromDefinitionTest, ConditionalOperatorAssignment) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* A, int* B, bool C) {
      (C ? A : B) = nullptr;
    }
  )cc";
  // Could in theory collect evidence for both A and B as nullable, but we don't
  // track null state through the conditional operator, so we don't collect
  // evidence for either.
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, Arithmetic) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* A, int* B, int* C, int* D, int* E, int* F, int* G,
                int* H) {
      A += 1;
      B -= 2;
      C + 3;
      D - 4;
      E++;
      ++F;
      G--;
      --H;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ARITHMETIC),
                           evidence(paramSlot(1), Evidence::ARITHMETIC),
                           evidence(paramSlot(2), Evidence::ARITHMETIC),
                           evidence(paramSlot(3), Evidence::ARITHMETIC),
                           evidence(paramSlot(4), Evidence::ARITHMETIC),
                           evidence(paramSlot(5), Evidence::ARITHMETIC),
                           evidence(paramSlot(6), Evidence::ARITHMETIC),
                           evidence(paramSlot(7), Evidence::ARITHMETIC)));
}

TEST(CollectEvidenceFromDefinitionTest, Fields) {
  llvm::Twine Src = CheckMacroDefinitions + R"cc(
#include <memory>
    void takesNonnull(Nonnull<int*>);
    void takesMutableNullable(Nullable<int*>&);
    struct S {
      int* Deref;
      int* AssignedToNonnull;
      int* AssignedToMutableNullable;
      int* AbortIfNull;
      int* AbortIfNullBool;
      int* AbortIfNullNE;
      int* AssignedFromNullable;
      int* Arithmetic;
      std::unique_ptr<int> SmartDeref;
    };
    void target(S AnS) {
      *AnS.Deref;
      takesNonnull(AnS.AssignedToNonnull);
      takesMutableNullable(AnS.AssignedToMutableNullable);
      CHECK(AnS.AbortIfNull);
      CHECK(AnS.AbortIfNullBool != nullptr);
      CHECK_NE(AnS.AbortIfNullNE, nullptr);
      AnS.AssignedFromNullable = nullptr;
      AnS.Arithmetic += 4;
      *AnS.SmartDeref;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src.str()),
      IsSupersetOf(
          {evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                    fieldNamed("S::Deref")),
           evidence(Slot(0), Evidence::ASSIGNED_TO_NONNULL,
                    fieldNamed("S::AssignedToNonnull")),
           evidence(Slot(0), Evidence::ASSIGNED_TO_MUTABLE_NULLABLE,
                    fieldNamed("S::AssignedToMutableNullable")),
           evidence(Slot(0), Evidence::ABORT_IF_NULL,
                    fieldNamed("S::AbortIfNull")),
           evidence(Slot(0), Evidence::ABORT_IF_NULL,
                    fieldNamed("S::AbortIfNullBool")),
           evidence(Slot(0), Evidence::ABORT_IF_NULL,
                    fieldNamed("S::AbortIfNullNE")),
           evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                    fieldNamed("S::AssignedFromNullable")),
           evidence(Slot(0), Evidence::ARITHMETIC, fieldNamed("S::Arithmetic")),
           evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                    fieldNamed("S::SmartDeref"))}));
}

TEST(CollectEvidenceFromDefinitionTest, StaticMemberVariables) {
  llvm::Twine Src = CheckMacroDefinitions + R"cc(
#include <memory>
    void takesNonnull(Nonnull<int*>);
    void takesMutableNullable(Nullable<int*>&);
    struct MyStruct {
      static int* Deref;
      static int* AssignedToNonnull;
      static int* AssignedToMutableNullable;
      static int* AbortIfNull;
      static int* AbortIfNullBool;
      static int* AbortIfNullNE;
      static int* AssignedFromNullable;
      static int* Arithmetic;
      static std::unique_ptr<int> SmartDeref;
    };
    void target() {
      *MyStruct::Deref;
      takesNonnull(MyStruct::AssignedToNonnull);
      takesMutableNullable(MyStruct::AssignedToMutableNullable);
      CHECK(MyStruct::AbortIfNull);
      CHECK(MyStruct::AbortIfNullBool != nullptr);
      CHECK_NE(MyStruct::AbortIfNullNE, nullptr);
      MyStruct::AssignedFromNullable = nullptr;
      MyStruct::Arithmetic += 4;
      *MyStruct::SmartDeref;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src.str()),
      IsSupersetOf(
          {evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                    staticFieldNamed("MyStruct::Deref")),
           evidence(Slot(0), Evidence::ASSIGNED_TO_NONNULL,
                    staticFieldNamed("MyStruct::AssignedToNonnull")),
           evidence(Slot(0), Evidence::ASSIGNED_TO_MUTABLE_NULLABLE,
                    staticFieldNamed("MyStruct::AssignedToMutableNullable")),
           evidence(Slot(0), Evidence::ABORT_IF_NULL,
                    staticFieldNamed("MyStruct::AbortIfNull")),
           evidence(Slot(0), Evidence::ABORT_IF_NULL,
                    staticFieldNamed("MyStruct::AbortIfNullBool")),
           evidence(Slot(0), Evidence::ABORT_IF_NULL,
                    staticFieldNamed("MyStruct::AbortIfNullNE")),
           evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                    staticFieldNamed("MyStruct::AssignedFromNullable")),
           evidence(Slot(0), Evidence::ARITHMETIC,
                    staticFieldNamed("MyStruct::Arithmetic")),
           evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                    staticFieldNamed("MyStruct::SmartDeref"))}));
}

TEST(CollectEvidenceFromDefinitionTest, Globals) {
  llvm::Twine Src = CheckMacroDefinitions + R"cc(
#include <memory>
    void takesNonnull(Nonnull<int*>);
    void takesMutableNullable(Nullable<int*>&);
    int* Deref;
    int* AssignedToNonnull;
    int* AssignedToMutableNullable;
    int* AbortIfNull;
    int* AbortIfNullBool;
    int* AbortIfNullNE;
    int* AssignedFromNullable;
    int* Arithmetic;
    std::unique_ptr<int> SmartDeref;
    void target() {
      *Deref;
      takesNonnull(AssignedToNonnull);
      takesMutableNullable(AssignedToMutableNullable);
      CHECK(AbortIfNull);
      CHECK(AbortIfNullBool != nullptr);
      CHECK_NE(AbortIfNullNE, nullptr);
      AssignedFromNullable = nullptr;
      Arithmetic += 4;
      *SmartDeref;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src.str()),
      IsSupersetOf({evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                             globalVarNamed("Deref")),
                    evidence(Slot(0), Evidence::ASSIGNED_TO_NONNULL,
                             globalVarNamed("AssignedToNonnull")),
                    evidence(Slot(0), Evidence::ASSIGNED_TO_MUTABLE_NULLABLE,
                             globalVarNamed("AssignedToMutableNullable")),
                    evidence(Slot(0), Evidence::ABORT_IF_NULL,
                             globalVarNamed("AbortIfNull")),
                    evidence(Slot(0), Evidence::ABORT_IF_NULL,
                             globalVarNamed("AbortIfNullBool")),
                    evidence(Slot(0), Evidence::ABORT_IF_NULL,
                             globalVarNamed("AbortIfNullNE")),
                    evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                             globalVarNamed("AssignedFromNullable")),
                    evidence(Slot(0), Evidence::ARITHMETIC,
                             globalVarNamed("Arithmetic")),
                    evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                             globalVarNamed("SmartDeref"))}));
}

TEST(CollectEvidenceFromDefinitionTest, GlobalInit) {
  static constexpr llvm::StringRef Src = R"cc(
    int* getPtr();
    Nullable<int*> getNullableFromNonnull(Nonnull<int*>);
    int* Target = static_cast<int*>(getNullableFromNonnull(getPtr()));
  )cc";

  EXPECT_THAT(collectFromDefinitionNamed("Target", Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::ASSIGNED_TO_NONNULL,
                           functionNamed("getPtr")),
                  evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                           globalVarNamed("Target"))));
}

TEST(CollectEvidenceFromDefinitionTest, GlobalInitFromGlobalAnnotation) {
  static constexpr llvm::StringRef Src = R"cc(
    int* foo();
    Nonnull<int*> Target = foo();
  )cc";
  EXPECT_THAT(collectFromDefinitionNamed("Target", Src),
              UnorderedElementsAre(evidence(SLOT_RETURN_TYPE,
                                            Evidence::ASSIGNED_TO_NONNULL,
                                            functionNamed("foo"))));
}

TEST(CollectEvidenceFromDefinitionTest, GlobalSmartImplicitInit) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    // This has an implicit init because of default construction.
    std::unique_ptr<int> Target;
  )cc";
  EXPECT_THAT(
      collectFromDefinitionNamed("Target", Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    globalVarNamed("Target"))));
}

TEST(CollectEvidenceFromDefinitionTest, GlobalSmartExplicitInit) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    std::unique_ptr<int> Target = nullptr;
  )cc";
  EXPECT_THAT(
      collectFromDefinitionNamed("Target", Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    globalVarNamed("Target"))));
}

TEST(CollectEvidenceFromDefinitionTest, GlobalInitWithCtor) {
  llvm::StringLiteral Src = R"cc(
#include <memory>
    struct S {
      S(int *P, Nonnull<int *> Q);
    };

    int *Foo();
    int GInt;
    int *AssignedToNonnull = Foo();
    S Target(&GInt, AssignedToNonnull);
  )cc";
  EXPECT_THAT(
      collectFromDefinitionNamed("Target", Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT,
                                    functionNamed("S")),
                           evidence(Slot(0), Evidence::ASSIGNED_TO_NONNULL,
                                    globalVarNamed("AssignedToNonnull"))));
}

TEST(CollectEvidenceFromDefinitionTest, GlobalSmartInitWithMakeUniqueCtor) {
  llvm::StringLiteral Src = R"cc(
#include <memory>
    struct S {
      S(int *P, Nonnull<int *> Q);
    };

    int *Foo();
    int GInt;
    int *AssignedToNonnull = Foo();
    std::unique_ptr<S> Target = std::make_unique<S>(&GInt, AssignedToNonnull);
  )cc";
  EXPECT_THAT(
      collectFromDefinitionNamed("Target", Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_NONNULL,
                                    globalVarNamed("Target")),
                           evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT,
                                    functionNamed("S")),
                           evidence(Slot(0), Evidence::ASSIGNED_TO_NONNULL,
                                    globalVarNamed("AssignedToNonnull"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     GlobalSmartInitWithMakeUniqueAggregate) {
  llvm::StringLiteral Src = R"cc(
#include <memory>
    struct S {
      int *P;
      Nonnull<int *> Q;
    };

    int *Foo();
    int GInt;
    int *AssignedToNonnull = Foo();
    std::unique_ptr<S> Target = std::make_unique<S>(&GInt, AssignedToNonnull);
  )cc";
  EXPECT_THAT(
      collectFromDefinitionNamed("Target", Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_NONNULL,
                                    globalVarNamed("Target")),
                           evidence(Slot(0), Evidence::ASSIGNED_FROM_NONNULL,
                                    fieldNamed("S::P")),
                           evidence(Slot(0), Evidence::ASSIGNED_TO_NONNULL,
                                    globalVarNamed("AssignedToNonnull"))));
}

TEST(CollectEvidenceFromDefinitionTest, StaticInitInClass) {
  static constexpr llvm::StringRef Src = R"cc(
    struct MyStruct {
      inline static int* Target = nullptr;
    };
  )cc";
  EXPECT_THAT(
      collectFromDefinitionNamed("Target", Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    staticFieldNamed("MyStruct::Target"))));
}

AST_MATCHER(VarDecl, hasInit) { return Node.hasInit(); }

TEST(CollectEvidenceFromDefinitionTest, StaticInitOutOfClass) {
  static constexpr llvm::StringRef Src = R"cc(
    struct MyStruct {
      static int* Target;
    };
    int* MyStruct::Target = nullptr;
  )cc";
  EXPECT_THAT(
      collectFromDefinitionMatching(varDecl(hasInit()), Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    staticFieldNamed("MyStruct::Target"))));
}

TEST(CollectEvidenceFromDefinitionTest, LocalVariable) {
  static constexpr llvm::StringRef Src = R"cc(
    void target() {
      int* P = nullptr;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    localVarNamed("P"))));
}

TEST(CollectEvidenceFromDefinitionTest, FunctionCallInLoop) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P) {
      for (int I = 0; I < 3; ++I) {
        target(nullptr);
      }
      for (int I = 0; I < 3; ++I) {
        target(&I);
      }
      for (int I = 0; I < 3; ++I) {
        target(P);
      }
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT),
                           evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT),
                           evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT)));
}

TEST(CollectEvidenceFromDefinitionTest, OutputParameterPointerToPointer) {
  static constexpr llvm::StringRef Src = R"cc(
    void maybeModifyPtr(int** A);
    void target(int* P) {
      maybeModifyPtr(&P);
      *P;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromDefinitionTest, OutputParameterReferenceToPointer) {
  static constexpr llvm::StringRef Src = R"cc(
    void maybeModifyPtr(int*& A);
    void target(int* P) {
      maybeModifyPtr(P);
      *P;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromDefinitionTest,
     OutputParameterReferenceToConstPointer) {
  static constexpr llvm::StringRef Src = R"cc(
    void dontModifyPtr(int* const& A);
    void target(int* P) {
      dontModifyPtr(P);
      *P;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Contains(evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                                functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     OutputParameterReferenceToPointerToPointer) {
  static constexpr llvm::StringRef Src = R"cc(
    void maybeModifyPtr(int**& A);
    void target(int** P) {
      maybeModifyPtr(P);
      *P;
      **P;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromDefinitionTest, OutputParameterPointerToConstPointer) {
  static constexpr llvm::StringRef Src = R"cc(
    void dontModifyPtr(int* const* A);
    void target(int* P) {
      dontModifyPtr(&P);
      *P;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Contains(evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                                functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     OutputParameterConstPointerToPointerToConst) {
  static constexpr llvm::StringRef Src = R"cc(
    // Outer pointer and int are const, but inner pointer can still be modified.
    void maybeModifyPtr(const int** const A);
    void target(const int* P) {
      maybeModifyPtr(&P);
      *P;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromDefinitionTest, PassAsOutputParameterOrDereference) {
  static constexpr llvm::StringRef Src = R"cc(
    void maybeModifyPtr(int** A);
    void target(int* P, bool B) {
      if (B) {
        maybeModifyPtr(&P);
      } else {
        *P;
      }
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Contains(evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                                functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     ConditionallyPassAsOutputParameterAlwaysDereference) {
  static constexpr llvm::StringRef Src = R"cc(
    void maybeModifyPtr(int** A);
    void target(int* P, bool B) {
      if (B) maybeModifyPtr(&P);
      *P;  // Because we model P as Unknown post-output-parameter-use, adding an
           // annotation would not be considered sufficient to make this
           // dereference safe, so we do not collect evidence for P.
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromDefinitionTest, FromGlobalLabmdaBodyForGlobal) {
  static constexpr llvm::StringRef Src = R"cc(
    int* P;
    auto Lambda = []() { *P; };
  )cc";

  EXPECT_THAT(
      collectFromDefinitionNamed("operator()", Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                                    globalVarNamed("P"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     FromLocalLambdaBodyForCapturedRefLocal) {
  static constexpr llvm::StringRef Src = R"cc(
    void foo() {
      int* P;
      auto Lambda = [&P]() { *P; };
    }
  )cc";

  EXPECT_THAT(
      collectFromDefinitionNamed("operator()", Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                                    localVarNamed("P", "foo"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     FromLocalLambdaBodyForCapturedValueLocal) {
  static constexpr llvm::StringRef Src = R"cc(
    void foo() {
      int* P;
      auto Lambda = [P]() { *P; };
    }
  )cc";

  EXPECT_THAT(
      collectFromDefinitionNamed("operator()", Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                                    localVarNamed("P", "foo"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     FromLocalLambdaBodyForRefCapturedParam) {
  static constexpr llvm::StringRef Src = R"cc(
    void foo(int* P, Nonnull<int*> Q) {
      auto Lambda = [&P, &Q]() {
        *P;
        P = nullptr;
        *Q;
      };
    }
  )cc";

  EXPECT_THAT(collectFromDefinitionNamed("operator()", Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                           functionNamed("foo")),
                  evidence(paramSlot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                           functionNamed("foo"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     FromLocalLambdaBodyForValueCapturedParam) {
  static constexpr llvm::StringRef Src = R"cc(
    void foo(int* P, Nonnull<int*> Q) {
      auto Lambda = [P, Q]() mutable {
        // In theory, the captured variable's value could have been modified
        // from it's argument to foo before the lambda is declared, and we don't
        // track that state when analyzing the lambda body, so this dereference
        // could be safe without the variable being declared Nonnull. However,
        // because we don't track the state, the only way we can assert safety
        // is by annotating the variable Nonnull, so we collect
        // UNCHECKED_DEREFERENCE evidence if the variable hasn't been checked or
        // made Nonnull within the lambda body.
        *P;

        // Similarly, this assignment to null could be safe, because the capture
        // here is much like the declaration of a new variable that is simply
        // initialized to P's value at the time of this lambda's declaration.
        // However, since we can't annotate this capture variable separately, we
        // will treat this as relevant for the declaration of `P` as a parameter
        // and collect ASSIGNED_FROM_NULLABLE evidence.
        P = nullptr;
        // Since Q is already annotated, we collect no evidence for it from
        // lambda bodies.
        *Q;
      };
    }
  )cc";

  EXPECT_THAT(collectFromDefinitionNamed("operator()", Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                           functionNamed("foo")),
                  evidence(paramSlot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                           functionNamed("foo"))));
}

TEST(CollectEvidenceFromDefinitionTest, FromLocalLambdaBodyForField) {
  static constexpr llvm::StringRef Src = R"cc(
    struct A {
      int* P;
    };
    struct B {
      bool* Q;
    };
    struct C {
      char* R;
    };
    void foo(B MyB) {
      C MyC;
      auto Lambda = [&MyB, MyC]() {
        A MyA;
        *MyA.P;
        *MyB.Q;
        *MyC.R;
      };
    }
  )cc";

  EXPECT_THAT(
      collectFromDefinitionNamed("operator()", Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                                    fieldNamed("A::P")),
                           evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                                    fieldNamed("B::Q")),
                           evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                                    fieldNamed("C::R"))));
}

TEST(CollectEvidenceFromDefinitionTest, FromLocalLambdaBodyForCalledFunction) {
  static constexpr llvm::StringRef Src = R"cc(
    int* bar(bool* B);
    void foo() {
      auto Lambda = []() { *bar(nullptr); };
    }
  )cc";

  EXPECT_THAT(collectFromDefinitionNamed("operator()", Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::UNCHECKED_DEREFERENCE,
                           functionNamed("bar")),
                  evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                           functionNamed("bar"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     FromLocalLambdaBodyForDefaultRefCaptures) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      int* F;

      void method(bool* P) {
        char* L;
        auto Lambda = [&]() {
          *P;
          *F;
          *L;
        };
      }
    };
  )cc";
  EXPECT_THAT(collectFromDefinitionNamed("operator()", Src),
              UnorderedElementsAre(
                  evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                           fieldNamed("S::F")),
                  evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                           localVarNamed("L", "method")),
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                           functionNamed("method"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     FromLocalLambdaBodyForDefaultValueCaptures) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      int* F;

      void method(bool* P) {
        char* L;
        auto Lambda = [=]() {
          *P;
          *F;
          *L;
        };
      }
    };
  )cc";
  EXPECT_THAT(collectFromDefinitionNamed("operator()", Src),
              UnorderedElementsAre(
                  evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                           fieldNamed("S::F")),
                  evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                           localVarNamed("L", "method")),
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                           functionNamed("method"))));
}

TEST(CollectEvidenceFromDefinitionTest, FromNestedLambdaBody) {
  static constexpr llvm::StringRef Src = R"cc(
    void foo() {
      int *A;
      int *B;
      auto OuterLambda = [&A, &B]() {
        auto InnerLambda = [&A, &B]() {
          *A;
          *B;
        };
      };
    }
  )cc";

  EXPECT_THAT(
      collectFromDefinitionMatching(
          cxxMethodDecl(hasName("operator()"),
                        hasAncestor(lambdaExpr(hasAncestor(lambdaExpr())))),
          Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                                    localVarNamed("A", "foo")),
                           evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                                    localVarNamed("B", "foo"))));
}

TEST(CollectEvidenceFromDefinitionTest, ForLambdaInitCapture) {
  static constexpr llvm::StringRef Src = R"cc(
    void foo() {
      int* P;
      auto Lambda = [Q = P]() { *Q; };
    }
  )cc";

  EXPECT_THAT(
      collectFromDefinitionNamed("operator()", Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                                    localVarNamed("Q", "operator()"))));
}

TEST(CollectEvidenceFromDefinitionTest, ForLambdaInitCaptureFromInit) {
  static constexpr llvm::StringRef Src = R"cc(
    void foo() {
      auto Lambda = [Q = static_cast<int*>(nullptr)]() {};
    }
  )cc";

  EXPECT_THAT(
      collectFromDefinitionMatching(varDecl(hasName("Q")), Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    localVarNamed("Q", "operator()"))));
}

TEST(CollectEvidenceFromDefinitionTest, ForLambdaParamOrReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    auto Lambda = [](int* P) -> int* {
      *P;
      return nullptr;
    };
  )cc";

  EXPECT_THAT(collectFromDefinitionNamed("operator()", Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN,
                           functionNamed("operator()")),
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                           functionNamed("operator()"))));
}

TEST(CollectEvidenceFromDefinitionTest, AggregateInitialization) {
  static constexpr llvm::StringRef Header = R"cc(
    struct Base {
      int BaseNonPtr;
      bool* BaseB;
      Nonnull<char*> BaseC;
    };
    struct MyStruct : public Base {
      float NonPtr;
      int* I;
      bool* B;
    };
  )cc";
  const llvm::Twine BracesAggInit = Header + R"cc(
    void target(Nullable<bool*> Bool, char* Char) {
      MyStruct{0, Bool, Char, 1.0f, nullptr, Bool};
    }
  )cc";
  // New aggregate initialization syntax in C++20
  const llvm::Twine ParensAggInit = Header + R"cc(
    void target(Nullable<bool*> Bool, char* Char) {
      MyStruct(Base(0, Bool, Char), 1.0f, nullptr, Bool);
    }
  )cc";

  auto ExpectedEvidenceMatcher =
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    fieldNamed("Base::BaseB")),
                           evidence(paramSlot(1), Evidence::ASSIGNED_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    fieldNamed("MyStruct::I")),
                           evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    fieldNamed("MyStruct::B")));

  EXPECT_THAT(collectFromTargetFuncDefinition(BracesAggInit.str()),
              ExpectedEvidenceMatcher);
  EXPECT_THAT(collectFromTargetFuncDefinition(ParensAggInit.str()),
              ExpectedEvidenceMatcher);
}

TEST(CollectEvidenceFromDefinitionTest,
     AggregateInitializationThroughMakeUnique) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    struct Base {
      int BaseNonPtr;
      bool* BaseB;
      Nonnull<char*> BaseC;
    };
    struct MyStruct : public Base {
      float NonPtr;
      int* I;
      bool* B;
    };
    // New aggregate initialization syntax in C++20, which allows make_unique
    // without a constructor.
    void target(Nullable<bool*> Bool, char* Char) {
      std::make_unique<MyStruct>(Base(0, Bool, Char), 1.0f, nullptr, Bool);
    }
  )cc";

  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    fieldNamed("Base::BaseB")),
                           evidence(paramSlot(1), Evidence::ASSIGNED_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    fieldNamed("MyStruct::I")),
                           evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    fieldNamed("MyStruct::B"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     AggregateInitializationWithConversionOperator) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    struct S {
      Nonnull<int*> I;
    };
    struct ConvertibleToIntPtr {
      ConvertibleToIntPtr(int* p) : p_(p) {}
      operator int*() { return p_; }
      int* p_;
    };
    void target(int* Int) { S AnS(ConvertibleToIntPtr{Int}); }
  )cc";

  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::ASSIGNED_TO_NONNULL,
                           functionNamed("operator int *")),
                  evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                           functionNamed("ConvertibleToIntPtr"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     AggregateInitializationThroughMakeUniqueWithConversionOperator) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    struct S {
      Nonnull<int*> I;
    };
    struct ConvertibleToIntPtr {
      ConvertibleToIntPtr(int* p) : p_(p) {}
      operator int*() { return p_; }
      int* p_;
    };
    void target(int* Int) { std::make_unique<S>(ConvertibleToIntPtr{Int}); }
  )cc";

  // The implicit conversion from ConvertibleToIntPtr to int* happens within
  // make_unique instead of at the call site in target, so we don't collect that
  // evidence. However, we collect the evidence from the make_unique
  // instantiation and will do inference from that.
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("ConvertibleToIntPtr"))));
}

TEST(SmartPointerCollectEvidenceFromDefinitionTest, AggregateInitialization) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
#include <utility>
    struct MyStruct {
      std::unique_ptr<int> P;
      Nonnull<std::unique_ptr<int>> Q;
      std::unique_ptr<int> R;
    };

    void target(Nullable<std::unique_ptr<int>> A, std::unique_ptr<int> B) {
      MyStruct{std::move(A), std::move(B), nullptr};
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(
          evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                   fieldNamed("MyStruct::P")),
          evidence(paramSlot(1), Evidence::ASSIGNED_TO_NONNULL,
                   functionNamed("target")),
          evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                   fieldNamed("MyStruct::R")),
          // evidence for the move constructor, which we don't care much about
          evidence(_, _, functionNamed("unique_ptr")),
          evidence(_, _, functionNamed("unique_ptr"))));
}

// This is a crash repro related to aggregate initialization.
TEST(CollectEvidenceFromDefinitionTest, NonRecordInitListExpr) {
  static constexpr llvm::StringRef Src = R"cc(
    void target() { int A[3] = {}; }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest,
     SmartPointerAnalysisProvidesEvidenceForRawPointer) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>

    void foo(int*);
    void target(Nullable<std::unique_ptr<int>> P) { foo(P.get()); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Contains(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                functionNamed("foo"))));
}

// This is a crash repro related to non-aggregate initialization using an
// InitListExpr.
TEST(CollectEvidenceFromDefinitionTest, TransparentInitListExpr) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {};
    void foo(S P) {}
    S get();

    void target() { foo({get()}); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, ArraySubscript) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P) { P[0]; }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ARRAY_SUBSCRIPT)));
}

TEST(SmartPointerCollectEvidenceFromDefinitionTest, ArraySubscript) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    void target(std::unique_ptr<int[]> P) {
      P[0];
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ARRAY_SUBSCRIPT)));
}

// Evidence for return type nonnull-ness should flow only from derived to base,
// so we collect evidence for the base but not the derived.
TEST(CollectEvidenceFromDefinitionTest, FromVirtualDerivedForReturnNonnull) {
  static constexpr llvm::StringRef Src = R"cc(
    struct Base {
      virtual int* foo();
    };

    struct Derived : public Base {
      int* foo() override {
        static int I;
        return &I;
      }
    };

    void target() {
      Derived D;
      *D.foo();
    }
  )cc";
  EXPECT_THAT(
      collectFromDefinitionNamed("Derived::foo", Src),
      UnorderedElementsAre(evidence(SLOT_RETURN_TYPE, Evidence::NONNULL_RETURN,
                                    functionNamed("Derived@F@foo"))));

  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(evidence(SLOT_RETURN_TYPE,
                                            Evidence::UNCHECKED_DEREFERENCE,
                                            functionNamed("Derived@F@foo"))));
}

TEST(CollectEvidenceFromDefinitionTest, FromVirtualDerivedForReturnNullable) {
  static constexpr llvm::StringRef Src = R"cc(
    struct Base {
      virtual int* foo();
    };

    struct Derived : public Base {
      int* foo() override { return nullptr; }
    };
  )cc";
  EXPECT_THAT(
      collectFromDefinitionNamed("Derived::foo", Src),
      UnorderedElementsAre(evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN,
                                    functionNamed("Derived@F@foo")),
                           evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN,
                                    functionNamed("Base@F@foo"))));

  // We don't currently have any evidence kinds that can force a non-reference
  // top-level pointer return type to be nullable from its usage, so no other
  // expectation.
}

TEST(CollectEvidenceFromDefinitionTest, FromVirtualDerivedForParamNonnull) {
  static constexpr llvm::StringRef Src = R"cc(
    struct Base {
      virtual void foo(int* P);
    };

    struct Derived : public Base {
      void foo(int* P) override { *P; }
    };

    void target() {
      int I;
      Derived D;
      D.foo(&I);
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT,
                                    functionNamed("Derived@F@foo")),
                           evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT,
                                    functionNamed("Base@F@foo"))));

  EXPECT_THAT(collectFromDefinitionNamed("Derived::foo", Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                           functionNamed("Derived@F@foo")),
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                           functionNamed("Base@F@foo"))));
}

// Evidence for parameter nullable-ness should flow only from base to derived,
// so we collect evidence for the derived but not the base.
TEST(CollectEvidenceFromDefinitionTest, FromVirtualDerivedForParamNullable) {
  static constexpr llvm::StringRef Src = R"cc(
    struct Base {
      virtual void foo(int* P);
    };

    struct Derived : public Base {
      void foo(int* P) override { P = nullptr; }
    };

    void target() {
      Derived D;
      D.foo(nullptr);
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("Derived@F@foo"))));

  EXPECT_THAT(collectFromDefinitionNamed("Derived::foo", Src),
              UnorderedElementsAre(evidence(paramSlot(0),
                                            Evidence::ASSIGNED_FROM_NULLABLE,
                                            functionNamed("Derived@F@foo"))));
}

TEST(CollectEvidenceFromDefinitionTest, FromVirtualBaseForReturnNonnull) {
  static constexpr llvm::StringRef Src = R"cc(
    struct Base {
      virtual int* foo() {
        static int I;
        return &I;
      }
    };

    struct Derived : public Base {
      int* foo() override;
    };

    void target() {
      Base B;
      *B.foo();
    }
  )cc";
  EXPECT_THAT(
      collectFromDefinitionNamed("Base::foo", Src),
      UnorderedElementsAre(evidence(SLOT_RETURN_TYPE, Evidence::NONNULL_RETURN,
                                    functionNamed("Base@F@foo")),
                           evidence(SLOT_RETURN_TYPE, Evidence::NONNULL_RETURN,
                                    functionNamed("Derived@F@foo"))));

  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::UNCHECKED_DEREFERENCE,
                           functionNamed("Base@F@foo")),
                  evidence(SLOT_RETURN_TYPE, Evidence::UNCHECKED_DEREFERENCE,
                           functionNamed("Derived@F@foo"))));
}

// Evidence for return type nullable-ness should flow only from derived to base,
// so we collect evidence for the base but not the derived.
TEST(CollectEvidenceFromDefinitionTest, FromVirtualBaseForReturnNullable) {
  static constexpr llvm::StringRef Src = R"cc(
    struct Base {
      virtual int* foo() { return nullptr; }
    };

    struct Derived : public Base {
      int* foo() override;
    };
  )cc";
  EXPECT_THAT(
      collectFromDefinitionNamed("Base::foo", Src),
      UnorderedElementsAre(evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN,
                                    functionNamed("Base@F@foo"))));

  // We don't currently have any evidence kinds that can force a non-reference
  // top-level pointer return type to be nullable from its usage, so no other
  // expectation.
}

// Evidence for parameter nonnull-ness should flow only from derived to base, so
// we collect evidence for the base but not the derived.
TEST(CollectEvidenceFromDefinitionTest, FromVirtualBaseForParamNonnull) {
  static constexpr llvm::StringRef Src = R"cc(
    struct Base {
      virtual void foo(int* P) { *P; }
    };

    struct Derived : public Base {
      void foo(int* P) override;
    };

    void target() {
      int I;
      Base B;
      B.foo(&I);
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT,
                                    functionNamed("Base@F@foo"))));

  EXPECT_THAT(collectFromDefinitionNamed("Base::foo", Src),
              UnorderedElementsAre(evidence(paramSlot(0),
                                            Evidence::UNCHECKED_DEREFERENCE,
                                            functionNamed("Base@F@foo"))));
}

TEST(CollectEvidenceFromDefinitionTest, FromVirtualBaseForParamNullable) {
  static constexpr llvm::StringRef Src = R"cc(
    struct Base {
      virtual void foo(int* P) { P = nullptr; }
    };

    struct Derived : public Base {
      void foo(int* P) override;
    };

    void target() {
      Base B;
      B.foo(nullptr);
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("Base@F@foo")),
                           evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("Derived@F@foo"))));

  EXPECT_THAT(collectFromDefinitionNamed("Base::foo", Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                           functionNamed("Base@F@foo")),
                  evidence(paramSlot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                           functionNamed("Derived@F@foo"))));
}

TEST(CollectEvidenceFromDefinitionTest, FromVirtualDerivedMultipleLayers) {
  static constexpr llvm::StringRef Src = R"cc(
    struct Base {
      virtual int* foo();
    };

    struct Derived : public Base {
      virtual int* foo();
    };

    struct DerivedDerived : public Derived {
      int* foo() override { return nullptr; };
    };
  )cc";

  EXPECT_THAT(
      collectFromDefinitionNamed("DerivedDerived::foo", Src),
      UnorderedElementsAre(evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN,
                                    functionNamed("DerivedDerived@F@foo")),
                           evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN,
                                    functionNamed("Derived@F@foo")),
                           evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN,
                                    functionNamed("Base@F@foo"))));
}

TEST(CollectEvidenceFromDefinitionTest, FromVirtualBaseMultipleLayers) {
  static constexpr llvm::StringRef Src = R"cc(
    struct Base {
      virtual void foo(int* P) { P = nullptr; }
    };

    struct Derived : public Base {
      virtual void foo(int*);
    };

    struct DerivedDerived : public Derived {
      void foo(int*) override;
    };
  )cc";

  EXPECT_THAT(collectFromDefinitionNamed("Base::foo", Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                           functionNamed("DerivedDerived@F@foo")),
                  evidence(paramSlot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                           functionNamed("Derived@F@foo")),
                  evidence(paramSlot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                           functionNamed("Base@F@foo"))));
}

TEST(CollectEvidenceFromDefinitionTest, FunctionTemplate) {
  static constexpr llvm::StringRef Src = R"cc(
    template <typename T>
    void tmpl(T* P, T Q) {
      *P;
    }

    void usage() {
      tmpl<int>(nullptr, 1);
      tmpl<bool>(nullptr, true);
      tmpl<char*>(nullptr, nullptr);
    }
  )cc";
  EXPECT_THAT(
      collectFromDefinitionNamed("usage", Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("tmpl<#I>")),
                           evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("tmpl<#b>")),
                           evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("tmpl<#*C>")),
                           evidence(paramSlot(1), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("tmpl<#*C>"))));

  EXPECT_THAT(
      collectFromDefinitionMatching(
          functionDecl(hasTemplateArgument(0, refersToType(asString("int")))),
          Src),
      UnorderedElementsAre(evidence(paramSlot(0),
                                    Evidence::UNCHECKED_DEREFERENCE,
                                    functionNamed("tmpl<#I>"))));
  EXPECT_THAT(
      collectFromDefinitionMatching(
          functionDecl(hasTemplateArgument(0, refersToType(booleanType()))),
          Src),
      UnorderedElementsAre(evidence(paramSlot(0),
                                    Evidence::UNCHECKED_DEREFERENCE,
                                    functionNamed("tmpl<#b>"))));
  EXPECT_THAT(
      collectFromDefinitionMatching(functionDecl(hasTemplateArgument(
                                        0, refersToType(asString("char *")))),
                                    Src),
      UnorderedElementsAre(evidence(paramSlot(0),
                                    Evidence::UNCHECKED_DEREFERENCE,
                                    functionNamed("tmpl<#*C>"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     FunctionTemplateExplicitSpecialization) {
  static constexpr llvm::StringRef Src = R"cc(
    template <typename T>
    void tmpl(T* P, T Q) {
      *P;
    }

    template <>
    void tmpl<int*>(int** P, int* Q) {
      *P;
      *Q;
    }

    void usage() { tmpl<int*>(nullptr, nullptr); }
  )cc";
  EXPECT_THAT(
      collectFromDefinitionNamed("usage", Src),
      // Evidence is emitted for the explicit specialization, not the template.
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("tmpl<#*I>")),
                           evidence(paramSlot(1), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("tmpl<#*I>"))));
  EXPECT_THAT(
      collectFromDefinitionMatching(
          functionDecl(hasTemplateArgument(0, refersToType(asString("int *")))),
          Src),
      // Evidence is emitted for the explicit specialization, not the template.
      UnorderedElementsAre(
          evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                   functionNamed("tmpl<#*I>")),
          evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE,
                   functionNamed("tmpl<#*I>"))));
}

TEST(CollectEvidenceFromDefinitionTest, LocalVariableInFunctionTemplate) {
  static constexpr llvm::StringRef Src = R"cc(
    template <typename T>
    void tmpl() {
      int* A = nullptr;
      T* B = nullptr;
    }

    void usage() { tmpl<int>(); }
  )cc";
  EXPECT_THAT(
      collectFromDefinitionMatching(functionDecl(isTemplateInstantiation()),
                                    Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    localVarNamed("A", "tmpl<#I>")),
                           evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    localVarNamed("B", "tmpl<#I>"))));
}

TEST(CollectEvidenceFromDefinitionTest, ClassTemplate) {
  static constexpr llvm::StringRef Src = R"cc(
    template <typename T>
    class C {
     public:
      void method(T* P) {
        *P;
        *Field;
      }
      T* Field;
    };

    void usage() {
      C<int> CInt;
      CInt.Field = nullptr;
      CInt.method(nullptr);
      C<char*> CCharPtr;
      CCharPtr.Field = nullptr;
      CCharPtr.method(nullptr);
    }
  )cc";
  EXPECT_THAT(collectFromDefinitionNamed("usage", Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                           AllOf(functionNamed("method"),
                                 ResultOf([](Symbol S) { return S.usr(); },
                                          HasSubstr("@S@C>#I@")))),
                  evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                           AllOf(functionNamed("method"),
                                 ResultOf([](Symbol S) { return S.usr(); },
                                          HasSubstr("@S@C>#*C@")))),
                  evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                           fieldNamed("C>#I::Field")),
                  evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                           fieldNamed("C>#*C::Field"))));

  EXPECT_THAT(collectFromDefinitionMatching(
                  functionDecl(isTemplateInstantiation(),
                               hasParameter(0, hasType(asString("int *")))),
                  Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                           AllOf(functionNamed("method"),
                                 ResultOf([](Symbol S) { return S.usr(); },
                                          HasSubstr("@S@C>#I@")))),
                  evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                           fieldNamed("C>#I::Field"))));
}

TEST(CollectEvidenceFromDefinitionTest, InClassInsideClassTemplate) {
  static constexpr llvm::StringRef Src = R"cc(
    template <typename T>
    class Tmpl {
     public:
      class C {
       public:
        void method(T* P) {
          *P;
          *Field;
        }
        T* Field;
      };
    };

    void usage() {
      Tmpl<int>::C CInt;
      CInt.Field = nullptr;
      CInt.method(nullptr);
      Tmpl<bool*>::C CBoolPtr;
      CBoolPtr.Field = nullptr;
      CBoolPtr.method(nullptr);
    }
  )cc";
  EXPECT_THAT(collectFromDefinitionNamed("usage", Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                           AllOf(functionNamed("method"),
                                 ResultOf([](Symbol S) { return S.usr(); },
                                          HasSubstr("@S@Tmpl>#I@S@C@")))),
                  evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                           AllOf(functionNamed("method"),
                                 ResultOf([](Symbol S) { return S.usr(); },
                                          HasSubstr("@S@Tmpl>#*b@S@C@")))),
                  evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                           AllOf(fieldNamed("C::Field"),
                                 ResultOf([](Symbol S) { return S.usr(); },
                                          HasSubstr("@S@Tmpl>#I@S@C@")))),
                  evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                           AllOf(fieldNamed("C::Field"),
                                 ResultOf([](Symbol S) { return S.usr(); },
                                          HasSubstr("@S@Tmpl>#*b@S@C@"))))));

  EXPECT_THAT(collectFromDefinitionMatching(
                  functionDecl(isTemplateInstantiation(),
                               hasParameter(0, hasType(asString("int *")))),
                  Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                           AllOf(functionNamed("method"),
                                 ResultOf([](Symbol S) { return S.usr(); },
                                          HasSubstr("@S@Tmpl>#I@S@C@")))),
                  evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                           AllOf(fieldNamed("C::Field"),
                                 ResultOf([](Symbol S) { return S.usr(); },
                                          HasSubstr("@S@Tmpl>#I@S@"))))));
}

TEST(CollectEvidenceFromDefinitionTest, ClassTemplateExplicitSpecialization) {
  static constexpr llvm::StringRef Src = R"cc(
    template <typename T>
    class C {
     public:
      void method(T* P) {}
      T* Field;
    };

    template <>
    class C<int> {
     public:
      void method(int* P) {
        *P;
        *Field;
      }
      int* Field;
    };

    void usage() {
      C<int> CInt;
      CInt.method(nullptr);
      CInt.Field = nullptr;
    }
  )cc";
  EXPECT_THAT(collectFromDefinitionNamed("usage", Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                           AllOf(functionNamed("method"),
                                 ResultOf([](Symbol S) { return S.usr(); },
                                          HasSubstr("@S@C>#I@")))),
                  evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                           fieldNamed("C>#I::Field"))));
}

TEST(CollectEvidenceFromDefinitionTest, ClassTemplatePartialSpecialization) {
  static constexpr llvm::StringRef Src = R"cc(
    template <typename T, typename U>
    class C {
     public:
      void method(T* P) {}
      U* Field;
    };

    template <typename U>
    class C<int, U> {
     public:
      void method(int* P) {
        *P;
        *Field;
      }
      U* Field;
    };

    void usage() {
      C<int, bool> CIntBool;
      CIntBool.method(nullptr);
      CIntBool.Field = nullptr;
    }
  )cc";
  EXPECT_THAT(collectFromDefinitionNamed("usage", Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                           AllOf(functionNamed("method"),
                                 ResultOf([](Symbol S) { return S.usr(); },
                                          HasSubstr("@S@C>#I#b@")))),
                  evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                           fieldNamed("C>#I#b::Field"))));
}

TEST(CollectEvidenceFromDefinitionTest, GlobalVariableTemplate) {
  static constexpr llvm::StringRef Src = R"cc(
    template <typename T>
    T* Global = nullptr;

    void usage() { Global<int>; }
  )cc";
  EXPECT_THAT(
      collectFromDefinitionMatching(varDecl(isTemplateInstantiation()), Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    globalVarNamed("Global>#I"))));
}

AST_MATCHER(VarDecl, isVarTemplateCompleteSpecializationDecl) {
  return isa<VarTemplateSpecializationDecl>(Node) &&
         !isa<VarTemplatePartialSpecializationDecl>(Node);
}

TEST(CollectEvidenceFromDefinitionTest,
     GlobalVariableTemplateExplicitSpecialization) {
  static constexpr llvm::StringRef Src = R"cc(
    template <typename T>
    T* Global = nullptr;

    template <>
    int* Global<int> = nullptr;
  )cc";
  EXPECT_THAT(
      collectFromDefinitionMatching(
          varDecl(isVarTemplateCompleteSpecializationDecl()), Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    globalVarNamed("Global>#I"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     GlobalVariableTemplatePartialSpecialization) {
  static constexpr llvm::StringRef Src = R"cc(
    template <typename T, typename U>
    T* Global = U{};

    template <typename U>
    int* Global<int, U> = nullptr;

    void usage() { Global<int, bool>; }
  )cc";
  EXPECT_THAT(
      collectFromDefinitionMatching(
          varDecl(isVarTemplateCompleteSpecializationDecl()), Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    globalVarNamed("Global>#I#b"))));
}

TEST(CollectEvidenceFromDefinitionTest, PropagatesPreviousInferences) {
  static constexpr llvm::StringRef Src = R"cc(
    void calledWithToBeNullable(int* X);
    void calledWithToBeNonnull(int* A);
    void target(int* P, int* Q) {
      target(nullptr, Q);
      calledWithToBeNullable(P);
      *Q;
      calledWithToBeNonnull(Q);
    }
  )cc";
  std::string TargetUsr = "c:@F@target#*I#S0_#";
  std::vector ExpectedBothRoundResults = {
      evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
               AllOf(functionNamed("target"),
                     // Double-check that target's usr is as expected before we
                     // use it to create SlotFingerprints.
                     ResultOf([](Symbol S) { return S.usr(); }, TargetUsr))),
      evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE,
               functionNamed("target")),
  };
  std::vector ExpectedSecondRoundResults = {
      evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
               functionNamed("calledWithToBeNullable")),
      evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT,
               functionNamed("calledWithToBeNonnull"))};

  // Only proceed if we have the correct USR for target and the first round
  // results contain the evidence needed to produce our expected inferences and
  // do not contain the evidence only found from propagating inferences from the
  // first round.
  auto FirstRoundResults = collectFromTargetFuncDefinition(Src);
  ASSERT_THAT(FirstRoundResults, IsSupersetOf(ExpectedBothRoundResults));
  for (const auto& E : ExpectedSecondRoundResults) {
    ASSERT_THAT(FirstRoundResults, Not(Contains(E)));
  }

  EXPECT_THAT(collectFromTargetFuncDefinition(
                  Src, {.Nullable = std::make_shared<SortedFingerprintVector>(
                            std::vector<SlotFingerprint>{
                                fingerprint(TargetUsr, paramSlot(0))}),
                        .Nonnull = std::make_shared<SortedFingerprintVector>(
                            std::vector<SlotFingerprint>{
                                fingerprint(TargetUsr, paramSlot(1))})}),
              AllOf(IsSupersetOf(ExpectedBothRoundResults),
                    IsSupersetOf(ExpectedSecondRoundResults)));
}

TEST(CollectEvidenceFromDefinitionTest,
     AnalysisUsesPreviousInferencesForSlotsOutsideTargetDefinition) {
  static constexpr llvm::StringRef Src = R"cc(
    int* returnsToBeNonnull(int* A) {
      return A;
    }
    int* target(int* Q) {
      *Q;
      return returnsToBeNonnull(Q);
    }
  )cc";
  std::string TargetUsr = "c:@F@target#*I#";
  std::string ReturnsToBeNonnullUsr = "c:@F@returnsToBeNonnull#*I#";
  const llvm::DenseMap<int, std::vector<testing::Matcher<const Evidence&>>>
      ExpectedNewResultsPerRound = {
          {0,
           {evidence(
               paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
               AllOf(functionNamed("target"),
                     // Double-check that target's usr is as expected before
                     // we use it to create SlotFingerprints.
                     ResultOf([](Symbol S) { return S.usr(); }, TargetUsr)))}},
          {1,
           {evidence(
               paramSlot(0), Evidence::NONNULL_ARGUMENT,
               AllOf(functionNamed("returnsToBeNonnull"),
                     // Double-check that returnsToBeNonnull's usr is as
                     // expected before we use it to create SlotFingerprints.
                     ResultOf([](Symbol S) { return S.usr(); },
                              ReturnsToBeNonnullUsr)))}},
          {2,
           {
               // No new evidence from target's definition in this round, but in
               // a full-TU analysis, this would be the round where we decide
               // returnsToBeNonnull returns Nonnull, based on the now-Nonnull
               // argument that is the only return value.
           }},
          {3,
           {evidence(SLOT_RETURN_TYPE, Evidence::NONNULL_RETURN,
                     functionNamed("target"))}}};

  // Assert first round results because they don't rely on previous inference
  // propagation at all and in this case are test setup and preconditions.
  auto FirstRoundResults = collectFromTargetFuncDefinition(Src);
  ASSERT_THAT(FirstRoundResults,
              IsSupersetOf(ExpectedNewResultsPerRound.at(0)));
  for (const auto& E : ExpectedNewResultsPerRound.at(1)) {
    ASSERT_THAT(FirstRoundResults, Not(Contains(E)));
  }

  auto SecondRoundResults = collectFromTargetFuncDefinition(
      Src, {.Nonnull = std::make_shared<SortedFingerprintVector>(
                std::vector<SlotFingerprint>{
                    fingerprint(TargetUsr, paramSlot(0))})});
  EXPECT_THAT(SecondRoundResults,
              AllOf(IsSupersetOf(ExpectedNewResultsPerRound.at(0)),
                    IsSupersetOf(ExpectedNewResultsPerRound.at(1))));
  for (const auto& E : ExpectedNewResultsPerRound.at(2)) {
    ASSERT_THAT(SecondRoundResults, Not(Contains(E)));
  }

  auto ThirdRoundResults = collectFromTargetFuncDefinition(
      Src, {.Nonnull = std::make_shared<SortedFingerprintVector>(
                std::vector<SlotFingerprint>{
                    fingerprint(TargetUsr, paramSlot(0)),
                    fingerprint(ReturnsToBeNonnullUsr, paramSlot(0))})});
  EXPECT_THAT(ThirdRoundResults,
              AllOf(IsSupersetOf(ExpectedNewResultsPerRound.at(0)),
                    IsSupersetOf(ExpectedNewResultsPerRound.at(1)),
                    IsSupersetOf(ExpectedNewResultsPerRound.at(2))));
  for (const auto& E : ExpectedNewResultsPerRound.at(3)) {
    ASSERT_THAT(ThirdRoundResults, Not(Contains(E)));
  }

  auto FourthRoundResults = collectFromTargetFuncDefinition(
      Src,
      {.Nonnull = std::make_shared<SortedFingerprintVector>(
           std::vector<SlotFingerprint>{
               fingerprint(TargetUsr, paramSlot(0)),
               fingerprint(ReturnsToBeNonnullUsr, paramSlot(0)),
               // As noted in the Evidence matcher list above, we don't infer
               // the return type of returnsToBeNonnull from only collecting
               // evidence from target's definition, but for the sake of this
               // test, let's pretend we collected evidence from the entire TU.
               fingerprint(ReturnsToBeNonnullUsr, SLOT_RETURN_TYPE)})});
  EXPECT_THAT(FourthRoundResults,
              AllOf(IsSupersetOf(ExpectedNewResultsPerRound.at(0)),
                    IsSupersetOf(ExpectedNewResultsPerRound.at(1)),
                    IsSupersetOf(ExpectedNewResultsPerRound.at(2)),
                    IsSupersetOf(ExpectedNewResultsPerRound.at(3))));
}

TEST(CollectEvidenceFromDefinitionTest,
     PreviousInferencesOfNonFocusParameterNullabilitiesPropagate) {
  static constexpr llvm::StringRef Src = R"cc(
    void takesToBeNonnull(int* A);
    void target(int* Q) { takesToBeNonnull(Q); }
  )cc";
  std::string TakesToBeNonnullUsr = "c:@F@takesToBeNonnull#*I#";

  // Pretend that in a first round of inferring for all functions, we made this
  // inference about takesToBeNonnull's first parameter.
  // This test confirms that we use that information when collecting from
  // target's definition.
  EXPECT_THAT(collectFromTargetFuncDefinition(
                  Src, {.Nonnull = std::make_shared<SortedFingerprintVector>(
                            std::vector<SlotFingerprint>{fingerprint(
                                TakesToBeNonnullUsr, paramSlot(0))})}),
              Contains(evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL,
                                functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, Pragma) {
  static constexpr llvm::StringRef Src = R"cc(
#pragma nullability file_default nonnull
    int* target(NullabilityUnknown<int*> P) {
      return P;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL)));
}

TEST(CollectEvidenceFromDefinitionTest, PragmaLocalTopLevelPointer) {
  static constexpr llvm::StringRef Src = R"cc(
#pragma nullability file_default nonnull
    void target(NullabilityUnknown<int*> P) {
      int* local_top_level_pointer = P;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL)));
}

// Just confirming that the test setup to run both FrontendActions is working.
TEST(CollectEvidenceFromDefinitionTest, PragmaAndMacroReplace) {
  llvm::Twine Src = CheckMacroDefinitions + R"cc(
#pragma nullability file_default nonnull
    int* target(NullabilityUnknown<int*> P) {
      CHECK(P);
      return P;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src.str()),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ASSIGNED_TO_NONNULL),
                  evidence(paramSlot(0), Evidence::ABORT_IF_NULL)));
}

TEST(CollectEvidenceFromDefinitionTest,
     UnsupportedVarTemplateSpecializationWithInitListExpr) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      int Field;
    };

    template <int I>
    S AnS = {.Field = I};

    constexpr int getInt(const char* s) { return 0; }

    // Not entirely sure why, but sufficient complexity in the template argument
    // is needed to produce the crash conditions. Check carefully that the crash
    // would still occur without the fix if modifying this test case.
    S usage() { return AnS<getInt("foo")>; }
  )cc";
  NullabilityPragmas Pragmas;
  clang::TestAST AST(getAugmentedTestInputs(Src, Pragmas));
  std::vector<Evidence> Results;
  USRCache UsrCache;

  auto& Decl = *selectFirst<VarTemplateSpecializationDecl>(
      "d", match(varDecl(isTemplateInstantiation()).bind("d"), AST.context()));
  EXPECT_THAT_ERROR(
      collectEvidenceFromDefinition(
          Decl,
          evidenceEmitter([&](const Evidence& E) { Results.push_back(E); },
                          UsrCache, AST.context()),
          UsrCache, Pragmas),
      llvm::FailedWithMessage(
          "Variable template specializations with InitListExprs in their "
          "initializers are currently unsupported."));
  EXPECT_THAT(Results, IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest,
     UnsupportedVarTemplateSpecializationContainingInitListExpr) {
  static constexpr llvm::StringRef Src = R"cc(
    template <typename T>
    class AClassTemplate {
     public:
      struct ABaseClass {};

      struct ADerivedClass : ABaseClass {};

      template <typename U>
      static constexpr ADerivedClass AVariableTemplate = {
          ABaseClass{},
      };
    };

    using AnyPointerType = int*;
    using AnyType = char;

    auto t = AClassTemplate<AnyPointerType>::AVariableTemplate<AnyType>;
  )cc";
  NullabilityPragmas Pragmas;
  clang::TestAST AST(getAugmentedTestInputs(Src, Pragmas));
  std::vector<Evidence> Results;
  USRCache UsrCache;

  auto& Decl = *selectFirst<VarTemplateSpecializationDecl>(
      "d", match(varDecl(isTemplateInstantiation()).bind("d"), AST.context()));
  EXPECT_THAT_ERROR(
      collectEvidenceFromDefinition(
          Decl,
          evidenceEmitter([&](const Evidence& E) { Results.push_back(E); },
                          UsrCache, AST.context()),
          UsrCache, Pragmas),
      llvm::FailedWithMessage(
          "Variable template specializations with InitListExprs in their "
          "initializers are currently unsupported."));
  EXPECT_THAT(Results, IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, SolverLimitReached) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* P, int* Q) {
      *P;
      *Q;
    }
  )cc";
  NullabilityPragmas Pragmas;
  clang::TestAST AST(getAugmentedTestInputs(Src, Pragmas));
  std::vector<Evidence> Results;
  USRCache UsrCache;
  EXPECT_THAT_ERROR(
      collectEvidenceFromDefinition(
          *cast<FunctionDecl>(
              dataflow::test::findValueDecl(AST.context(), "target")),
          evidenceEmitter([&](const Evidence& E) { Results.push_back(E); },
                          UsrCache, AST.context()),
          UsrCache, Pragmas, /*PreviousInferences=*/{},
          // Enough iterations to collect one piece of evidence but not both.
          []() {
            return std::make_unique<dataflow::WatchedLiteralsSolver>(
                /*MaxSATIterations=*/100);
          }),
      llvm::FailedWithMessage("SAT solver reached iteration limit"));
  EXPECT_THAT(Results, SizeIs(1));
}

TEST(CollectEvidenceFromDeclarationTest, GlobalVariable) {
  llvm::StringLiteral Src = R"cc(
    Nullable<int *> Target;
  )cc";
  EXPECT_THAT(collectFromTargetVarDecl(Src),
              ElementsAre(evidence(Slot(0), Evidence::ANNOTATED_NULLABLE,
                                   globalVarNamed("Target"))));
}

TEST(SmartPointerCollectEvidenceFromDeclarationTest, GlobalVariable) {
  llvm::StringLiteral Src = R"cc(
#include <memory>
    Nullable<std::unique_ptr<int>> Target;
  )cc";
  EXPECT_THAT(collectFromTargetVarDecl(Src),
              ElementsAre(evidence(Slot(0), Evidence::ANNOTATED_NULLABLE,
                                   globalVarNamed("Target"))));
}

TEST(CollectEvidenceFromDeclarationTest, StaticMemberVariable) {
  llvm::StringLiteral Src = R"cc(
    struct S {
      static Nonnull<int*> Target;
    };
  )cc";
  EXPECT_THAT(collectFromTargetVarDecl(Src),
              ElementsAre(evidence(Slot(0), Evidence::ANNOTATED_NONNULL,
                                   staticFieldNamed("S::Target"))));
}

TEST(CollectEvidenceFromDeclarationTest, Field) {
  llvm::StringLiteral Src = R"cc(
    struct S {
      Nonnull<int*> Target;
    };
  )cc";
  EXPECT_THAT(collectFromTargetVarDecl(Src),
              ElementsAre(evidence(Slot(0), Evidence::ANNOTATED_NONNULL,
                                   fieldNamed("S::Target"))));
}

TEST(SmartPointerCollectEvidenceFromDeclarationTest, Field) {
  llvm::StringLiteral Src = R"cc(
#include <memory>
    struct S {
      Nonnull<std::unique_ptr<int>> Target;
    };
  )cc";
  EXPECT_THAT(collectFromTargetVarDecl(Src),
              ElementsAre(evidence(Slot(0), Evidence::ANNOTATED_NONNULL,
                                   fieldNamed("S::Target"))));
}

TEST(CollectEvidenceFromDeclarationTest, FunctionDeclReturnType) {
  llvm::StringLiteral Src = R"cc(
    Nonnull<int *> target();
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDecl(Src),
      ElementsAre(evidence(SLOT_RETURN_TYPE, Evidence::ANNOTATED_NONNULL)));
}

TEST(CollectEvidenceFromDeclarationTest, FunctionDeclParams) {
  llvm::StringLiteral Src = R"cc(
    void target(Nullable<int*>, int*, Nonnull<int*>);
  )cc";
  EXPECT_THAT(collectFromTargetFuncDecl(Src),
              ElementsAre(evidence(paramSlot(0), Evidence::ANNOTATED_NULLABLE),
                          evidence(paramSlot(2), Evidence::ANNOTATED_NONNULL)));
}

TEST(CollectEvidenceFromDeclarationTest, FunctionDeclNonTopLevel) {
  llvm::StringLiteral Src = R"cc(
    Nonnull<int*>** target(Nullable<int*>*);
  )cc";
  EXPECT_THAT(collectFromTargetFuncDecl(Src), IsEmpty());
}

TEST(SmartPointerCollectEvidenceFromDeclarationTest, FunctionDecl) {
  llvm::StringLiteral Src = R"cc(
#include <memory>
    Nullable<std::unique_ptr<int>> target(Nonnull<std::unique_ptr<int>>,
                                          Nullable<std::unique_ptr<int>>);
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDecl(Src),
      ElementsAre(evidence(SLOT_RETURN_TYPE, Evidence::ANNOTATED_NULLABLE),
                  evidence(paramSlot(0), Evidence::ANNOTATED_NONNULL),
                  evidence(paramSlot(1), Evidence::ANNOTATED_NULLABLE)));
}

TEST(CollectEvidenceFromDeclarationTest, FunctionTemplateIgnored) {
  // We used to inspect the type of `target` and crash.
  llvm::StringLiteral Src = R"cc(
    template <class A>
    struct S {
      template <class B>
      static void target(const S<B>&) {}
    };
  )cc";
  EXPECT_THAT(collectFromTargetFuncDecl(Src), IsEmpty());
}

TEST(CollectEvidenceFromDeclarationTest, DefaultArgumentNullptrLiteral) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* = nullptr);
  )cc";
  EXPECT_THAT(collectFromTargetFuncDecl(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT)));
}

TEST(CollectEvidenceFromDeclarationTest, DefaultArgumentZeroLiteral) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* = 0);
  )cc";
  EXPECT_THAT(collectFromTargetFuncDecl(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT)));
}

TEST(CollectEvidenceFromDeclarationTest, DefaultArgumentAnnotatedVariable) {
  static constexpr llvm::StringRef Src = R"cc(
    Nonnull<int*> Q;
    void target(int* = Q);
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDecl(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT)));
}

TEST(CollectEvidenceFromDeclarationTest,
     DefaultArgumentCallingAnnotatedFunction) {
  static constexpr llvm::StringRef Src = R"cc(
    Nullable<int*> getDefault();
    void target(int* = getDefault());
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDecl(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDeclarationTest,
     DefaultArgumentUnannotatedNonLiteralExpressionsUnknown) {
  static constexpr llvm::StringRef Src = R"cc(
    int* getDefault();
    int* Q = nullptr;
    int I = 1;
    void target(int* = getDefault(), int* = Q, int* = &I);
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDecl(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("target")),
                           evidence(paramSlot(1), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("target")),
                           evidence(paramSlot(2), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("target"))));
}

TEST(SmartPointerCollectEvidenceFromDeclarationTest,
     DefaultArgumentNullptrLiteral) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    void target(std::unique_ptr<int> = nullptr);
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDecl(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("target"))));
}

TEST(SmartPointerCollectEvidenceFromDeclarationTest,
     DefaultArgumentMakeUniqueTooComplex) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    void target(std::unique_ptr<int> = std::make_unique<int>(1));
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDecl(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("target"))));
}

TEST(SmartPointerCollectEvidenceFromDeclarationTest,
     DefaultArgumentReferenceTypesNullptrLiteral) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    void target(const std::unique_ptr<int>& PL = nullptr,
                std::unique_ptr<int>&& PR = nullptr);
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDecl(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("target")),
                           evidence(paramSlot(1), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDeclarationTest, NonnullAttributeOnFunction) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target(int* P, int** Q, int*& R, bool B) __attribute__((nonnull));
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDecl(Src),
      // attribute applies to top-level non-reference raw pointer
      // parameter types only, not return type or other params.
      ElementsAre(evidence(paramSlot(0), Evidence::GCC_NONNULL_ATTRIBUTE),
                  evidence(paramSlot(1), Evidence::GCC_NONNULL_ATTRIBUTE)));
}

TEST(CollectEvidenceFromDeclarationTest, NonnullAttributeOnFunctionWithArgs) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target(int* P, int** Q, int*& R, bool B, int* NotIndicated)
        __attribute__((nonnull(1, 2, 3, 4)));
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDecl(Src),
      // attribute applies to the indicated and eligible parameters only.
      ElementsAre(evidence(paramSlot(0), Evidence::GCC_NONNULL_ATTRIBUTE),
                  evidence(paramSlot(1), Evidence::GCC_NONNULL_ATTRIBUTE)));
}

TEST(CollectEvidenceFromDeclarationTest, NonnullAttributeOnMethodWithArgs) {
  static constexpr llvm::StringRef Src = R"cc(
    struct T {
      // Index 1 on a non-static method is for the implicit `this` parameter.
      int* target(int* P, int* NotIndicated) __attribute__((nonnull(2)));
    };
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDecl(Src),
      ElementsAre(evidence(paramSlot(0), Evidence::GCC_NONNULL_ATTRIBUTE)));
}

TEST(CollectEvidenceFromDeclarationTest,
     NonnullAttributeOnStaticMethodWithArgs) {
  static constexpr llvm::StringRef Src = R"cc(
    struct T {
      // no implicit `this` parameter for static methods.
      static int* target(int* P, int* Q) __attribute__((nonnull(2)));
    };
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDecl(Src),
      ElementsAre(evidence(paramSlot(1), Evidence::GCC_NONNULL_ATTRIBUTE)));
}

TEST(CollectEvidenceFromDeclarationTest, NonnullAttributeOnParam) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target(int* P __attribute__((nonnull())), int* NotIndicated);
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDecl(Src),
      ElementsAre(evidence(paramSlot(0), Evidence::GCC_NONNULL_ATTRIBUTE)));
}

TEST(SmartPointerCollectEvidenceFromDeclarationTest,
     NonnullAttributeOnFunction) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    void target(std::unique_ptr<int> P, std::unique_ptr<int>* Q,
                std::unique_ptr<int*> R) __attribute__((nonnull));
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDecl(Src),
      // attribute applies to top-level non-reference *raw* pointer
      // parameter types only.
      ElementsAre(evidence(paramSlot(1), Evidence::GCC_NONNULL_ATTRIBUTE)));
}

TEST(CollectEvidenceFromDeclarationTest, ReturnsNonnullAttribute) {
  static constexpr llvm::StringRef Src = R"cc(
    int** target() __attribute__((returns_nonnull));
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDecl(Src),
      // Affects the top-level pointer.
      ElementsAre(evidence(SLOT_RETURN_TYPE, Evidence::GCC_NONNULL_ATTRIBUTE)));
}

TEST(CollectEvidenceFromDeclarationTest, ReturnsNonnullAttributeReference) {
  static constexpr llvm::StringRef Src = R"cc(
    int*& target() __attribute__((returns_nonnull));
  )cc";
  // No effect on reference types.
  EXPECT_THAT(collectFromTargetFuncDecl(Src), IsEmpty());
}

TEST(SmartPointerCollectEvidenceFromDeclarationTest, ReturnsNonnullAttribute) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    std::unique_ptr<int> target() __attribute__((returns_nonnull));
  )cc";
  // No effect on smart pointers.
  EXPECT_THAT(collectFromTargetFuncDecl(Src), IsEmpty());
}

TEST(CollectEvidenceFromDeclarationTest, MainNoParams) {
  static constexpr llvm::StringRef Src = R"cc(
    int main() {}
  )cc";
  EXPECT_THAT(collectFromDecl(Src, "main"), IsEmpty());
}

TEST(CollectEvidenceFromDeclarationTest, MainTwoParamsNestedPointer) {
  static constexpr llvm::StringRef Src = R"cc(
    int main(int argc, char** argv) {}
  )cc";
  EXPECT_THAT(collectFromDecl(Src, "main"),
              ElementsAre(evidence(paramSlot(1), Evidence::WELL_KNOWN_NONNULL,
                                   functionNamed("main"))));
}

TEST(CollectEvidenceFromDeclarationTest, MainTwoParamsPointerToArray) {
  static constexpr llvm::StringRef Src = R"cc(
    int main(int argc, char* argv[]) {}
  )cc";
  EXPECT_THAT(collectFromDecl(Src, "main"),
              ElementsAre(evidence(paramSlot(1), Evidence::WELL_KNOWN_NONNULL,
                                   functionNamed("main"))));
}

TEST(CollectEvidenceFromDeclarationTest, Pragma) {
  static constexpr llvm::StringRef Src = R"cc(
#pragma nullability file_default nonnull
    void target(int* P);
  )cc";
  EXPECT_THAT(collectFromTargetFuncDecl(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::ANNOTATED_NONNULL)));
}

MATCHER_P(declNamed, Name, "") {
  std::string Actual;
  llvm::raw_string_ostream OS(Actual);
  if (auto* ND = dyn_cast<NamedDecl>(arg))
    ND->getNameForDiagnostic(
        OS, arg->getDeclContext()->getParentASTContext().getPrintingPolicy(),
        /*Qualified=*/true);
  return ::testing::ExplainMatchResult(Name, Actual, result_listener);
}

TEST(EvidenceSitesTest, FastMode) {
  TestInputs Inputs;
  NullabilityPragmas Pragmas;
  Inputs.FileName = "input.cc";
  Inputs.MakeAction = [&] {
    struct Action : public SyntaxOnlyAction {
      NullabilityPragmas& Pragmas;
      Action(NullabilityPragmas& Pragmas) : Pragmas(Pragmas) {}
      std::unique_ptr<ASTConsumer> CreateASTConsumer(
          CompilerInstance& CI, llvm::StringRef File) override {
        registerPragmaHandler(CI.getPreprocessor(), Pragmas);
        return SyntaxOnlyAction::CreateASTConsumer(CI, File);
      }
    };
    return std::make_unique<Action>(Pragmas);
  };
  Inputs.Code = R"cc(
#include "input.h"
#include "header.h"
    int* foo();
  )cc";
  Inputs.ExtraFiles["input.h"] = R"cc(
    int* bar();
  )cc";
  Inputs.ExtraFiles["header.h"] = R"cc(
    int* baz();
  )cc";

  TestAST AST(Inputs);
  EXPECT_THAT(EvidenceSites::discover(AST.context(),
                                      /*RestrictToMainFileOrHeader=*/true)
                  .Declarations,
              UnorderedElementsAre(declNamed("foo"), declNamed("bar")));
  EXPECT_THAT(EvidenceSites::discover(AST.context(),
                                      /*RestrictToMainFileOrHeader=*/false)
                  .Declarations,
              UnorderedElementsAre(declNamed("foo"), declNamed("bar"),
                                   declNamed("baz")));
}

TEST(EvidenceSitesTest, Functions) {
  TestAST AST(R"cc(
    void foo();
    int* bar();
    int* bar() {}
    void baz(int*) {}
    void def() {}

    struct S {
      S() {}
      S(int*) {}
      void member(int*);
    };
    void S::member(int*) {}
  )cc");
  auto Sites = EvidenceSites::discover(AST.context());
  EXPECT_THAT(
      Sites.Declarations,
      UnorderedElementsAre(declNamed("bar"), declNamed("bar"), declNamed("baz"),
                           declNamed("S::S"), declNamed("S::member"),
                           declNamed("S::member")));
  EXPECT_THAT(Sites.Definitions,
              UnorderedElementsAre(declNamed("bar"), declNamed("baz"),
                                   declNamed("def"), declNamed("S::S"),
                                   declNamed("S::S"), declNamed("S::member")));
}

TEST(EvidenceSitesTest, LambdaNoPtr) {
  TestAST AST(R"cc(
    auto NoPtrs = []() {};
  )cc");
  auto Sites = EvidenceSites::discover(AST.context());
  EXPECT_THAT(Sites.Declarations, IsEmpty());
  EXPECT_THAT(Sites.Definitions,
              UnorderedElementsAre(declNamed("(anonymous class)::operator()"),
                                   declNamed("NoPtrs")));
}

TEST(EvidenceSitesTest, LambdaWithPtr) {
  TestAST AST(R"cc(
    auto Ptr = [](int*) {};
  )cc");
  auto Sites = EvidenceSites::discover(AST.context());
  EXPECT_THAT(Sites.Declarations,
              UnorderedElementsAre(declNamed("(anonymous class)::operator()")));
  EXPECT_THAT(Sites.Definitions,
              UnorderedElementsAre(declNamed("(anonymous class)::operator()"),
                                   declNamed("Ptr")));
}

TEST(EvidenceSitesTest, GlobalVariables) {
  NullabilityPragmas Pragmas;
  TestAST AST = getAugmentedTestInputs(
      R"cc(
#include <memory>
        int* X = true ? nullptr : nullptr;
        int* Y;
        int A;
        int B = *Y;
        std::unique_ptr<int> P;
        std::unique_ptr<int> Q = nullptr;
      )cc",
      Pragmas);

  auto Sites = EvidenceSites::discover(AST.context(),
                                       /*RestrictToMainFileOrHeader=*/true);
  EXPECT_THAT(Sites.Declarations,
              UnorderedElementsAre(declNamed("X"), declNamed("Y"),
                                   declNamed("P"), declNamed("Q")));
  EXPECT_THAT(
      Sites.Definitions,
      UnorderedElementsAre(
          declNamed("X"), declNamed("B"),
          // unique_ptr P has an initializer because of default construction.
          declNamed("P"), declNamed("Q")));
}

TEST(EvidenceSitesTest, StaticMemberVariables) {
  TestAST AST(R"cc(
    struct S {
      inline static int* A = nullptr;
      static int* B;
      static int* C;
    };

    int* S::C = nullptr;
  )cc");
  auto Sites = EvidenceSites::discover(AST.context());
  EXPECT_THAT(
      Sites.Declarations,
      UnorderedElementsAre(
          declNamed("S::A"), declNamed("S::B"),
          // one for in-class declaration and one for out-of-class definition
          declNamed("S::C"), declNamed("S::C")));
  EXPECT_THAT(Sites.Definitions,
              UnorderedElementsAre(declNamed("S::A"), declNamed("S::C")));
}

TEST(EvidenceSitesTest, NonStaticMemberVariables) {
  NullabilityPragmas Pragmas;
  TestAST AST = getAugmentedTestInputs(
      R"cc(
#include <memory>
        struct S {
          int* A = nullptr;
          int* B;
          std::unique_ptr<int> P = nullptr;
          std::unique_ptr<int> Q;
        };
      )cc",
      Pragmas);
  auto Sites = EvidenceSites::discover(AST.context(),
                                       /*RestrictToMainFileOrHeader=*/true);
  EXPECT_THAT(Sites.Declarations,
              UnorderedElementsAre(declNamed("S::A"), declNamed("S::B"),
                                   declNamed("S::P"), declNamed("S::Q")));
  EXPECT_THAT(Sites.Definitions, IsEmpty());
}

TEST(EvidenceSitesTest, LocalVariables) {
  TestAST AST(R"cc(
    void foo() {
      int* P = nullptr;
      static int* Q = nullptr;
      static int* R;
    }
  )cc");
  auto Sites = EvidenceSites::discover(AST.context());
  EXPECT_THAT(
      Sites.Declarations,
      UnorderedElementsAre(declNamed("P"), declNamed("Q"), declNamed("R")));
  EXPECT_THAT(Sites.Definitions, UnorderedElementsAre(declNamed("foo")));
}

TEST(EvidenceSitesTest, Templates) {
  TestAST AST(R"cc(
    template <int I>
    int f(int*) {
      return I;
    }
    template <>
    int f<1>(int*) {
      return 1;
    }

    struct S {
      template <int I>
      int f(int*) {
        return I;
      }
    };

    template <int I>
    struct T {
      int f(int*) { return I; }
    };

    template <int I>
    int* V = nullptr;

    template <>
    int* V<1> = nullptr;

    int Unused = f<0>(V<0>) + f<1>(V<1>) + S{}.f<0>(nullptr) + T<0>{}.f(nullptr);
  )cc");
  auto Sites = EvidenceSites::discover(AST.context());

  // Relevant declarations are the written ones that aren't template-related
  // plus the template instantiations.
  EXPECT_THAT(Sites.Declarations,
              UnorderedElementsAre(declNamed("f<0>"), declNamed("V<0>"),
                                   declNamed("f<1>"), declNamed("V<1>"),
                                   declNamed("S::f<0>"), declNamed("T<0>::f")));
  // Instantiations are relevant definitions, as is the global variable Unused.
  EXPECT_THAT(Sites.Definitions,
              UnorderedElementsAre(declNamed("f<0>"), declNamed("V<0>"),
                                   declNamed("f<1>"), declNamed("V<1>"),
                                   declNamed("S::f<0>"), declNamed("T<0>::f"),
                                   declNamed("Unused")));
}

TEST(EvidenceEmitterTest, NotInferenceTarget) {
  TestAST AST(R"cc(
    template <int I>
    int target() {
      return I;
    })cc");

  const auto* TargetDecl =
      dataflow::test::findValueDecl(AST.context(), "target");
  ASSERT_NE(TargetDecl, nullptr);

  USRCache USRCache;
  EXPECT_DEATH(
      evidenceEmitter([](const Evidence& E) {}, USRCache, AST.context())(
          *TargetDecl, Slot{}, Evidence::ANNOTATED_UNKNOWN,
          TargetDecl->getLocation()),
      "not an inference target");
}

}  // namespace
}  // namespace clang::tidy::nullability
