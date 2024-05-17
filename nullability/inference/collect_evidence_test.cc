// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/collect_evidence.h"

#include <cassert>
#include <memory>
#include <string>
#include <vector>

#include "nullability/inference/ctn_replacement_macros.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/inference/replace_macros.h"
#include "nullability/inference/slot_fingerprint.h"
#include "nullability/test/test_headers.h"
#include "nullability/type_nullability.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/ASTMatchers/ASTMatchersMacros.h"
#include "clang/Basic/LLVM.h"
#include "clang/Testing/CommandLineArgs.h"
#include "clang/Testing/TestAST.h"
#include "third_party/llvm/llvm-project/clang/unittests/Analysis/FlowSensitive/TestingSupport.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/DenseSet.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/raw_ostream.h"
#include "llvm/Testing/Support/Error.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"  // IWYU pragma: keep
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {
using ::clang::ast_matchers::cxxConstructorDecl;
using ::clang::ast_matchers::functionDecl;
using ::clang::ast_matchers::hasName;
using ::clang::ast_matchers::isDefaultConstructor;
using ::clang::ast_matchers::isTemplateInstantiation;
using ::clang::ast_matchers::match;
using ::clang::ast_matchers::parameterCountIs;
using ::clang::ast_matchers::selectFirst;
using ::clang::ast_matchers::varDecl;
using ::testing::_;
using ::testing::AllOf;
using ::testing::Contains;
using ::testing::ElementsAre;
using ::testing::IsEmpty;
using ::testing::IsSupersetOf;
using ::testing::Not;
using ::testing::ResultOf;
using ::testing::SizeIs;
using ::testing::UnorderedElementsAre;

test::EnableSmartPointers Enable;

constexpr llvm::StringRef CheckMacroDefinitions = R"cc(
  // Bodies must reference the first param so that args are in the AST, but
  // otherwise don't matter.
#define CHECK(x) (x)
#define CHECK_NE(a, b) (a, b)
)cc";

MATCHER_P3(isEvidenceMatcher, SlotMatcher, KindMatcher, SymbolMatcher, "") {
  return SlotMatcher.Matches(static_cast<Slot>(arg.slot())) &&
         KindMatcher.Matches(arg.kind()) && SymbolMatcher.Matches(arg.symbol());
}

MATCHER_P(functionNamed, Name, "") {
  return llvm::StringRef(arg.usr()).contains(
      ("@" + llvm::StringRef(Name) + "#").str());
}

/// Matches a non-static field with the given name.
/// The name should be of the form "MyStruct::field", but it should be qualified
/// only by the enclosing type, not any namespaces.
MATCHER_P(fieldNamed, TypeQualifiedFieldName, "") {
  const auto [TypeName, FieldName] =
      llvm::StringRef(TypeQualifiedFieldName).split("::");
  return arg.usr().ends_with(("@S@" + TypeName + "@FI@" + FieldName).str());
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
  return arg.usr() == ("c:@" + llvm::StringRef(Name)).str();
}

testing::Matcher<const Evidence&> evidence(
    testing::Matcher<Slot> S, testing::Matcher<Evidence::Kind> Kind,
    testing::Matcher<const Symbol&> SymbolMatcher = functionNamed("target")) {
  return isEvidenceMatcher(S, Kind, SymbolMatcher);
}

clang::TestInputs getInputsWithAnnotationDefinitions(llvm::StringRef Source) {
  clang::TestInputs Inputs = Source;
  Inputs.Language = TestLanguage::Lang_CXX20;
  for (const auto& Entry :
       llvm::ArrayRef(test_headers_create(), test_headers_size()))
    Inputs.ExtraFiles.try_emplace(Entry.name, Entry.data);
  for (const auto& Entry : llvm::ArrayRef(ctn_replacement_macros_create(),
                                          ctn_replacement_macros_size()))
    Inputs.ExtraFiles.try_emplace(Entry.name, Entry.data);
  Inputs.ExtraArgs.push_back("-I.");
  Inputs.ExtraArgs.push_back("-include");
  Inputs.ExtraArgs.push_back("nullability_annotations.h");
  Inputs.ExtraArgs.push_back("-include");
  Inputs.ExtraArgs.push_back(std::string(ReplacementMacrosHeaderFileName));

  Inputs.MakeAction = [&]() { return std::make_unique<ReplaceMacrosAction>(); };
  return Inputs;
}

std::vector<Evidence> collectFromDefinition(
    clang::TestAST& AST, const Decl& Definition,
    PreviousInferences InputInferences = {}) {
  std::vector<Evidence> Results;
  USRCache UsrCache;
  // Can't assert from within a non-void helper function, so only EXPECT.
  EXPECT_THAT_ERROR(
      collectEvidenceFromDefinition(
          Definition,
          evidenceEmitter([&](const Evidence& E) { Results.push_back(E); },
                          UsrCache),
          UsrCache, InputInferences),
      llvm::Succeeded());
  return Results;
}

std::vector<Evidence> collectFromDefinitionNamed(
    llvm::StringRef TargetName, llvm::StringRef Source,
    PreviousInferences InputInferences = {}) {
  clang::TestAST AST(getInputsWithAnnotationDefinitions(Source));
  const Decl& Definition =
      *dataflow::test::findValueDecl(AST.context(), TargetName);
  return collectFromDefinition(AST, Definition, InputInferences);
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
  clang::TestAST AST(getInputsWithAnnotationDefinitions(Source));
  const Decl& Definition =
      *selectFirst<Decl>("d", match(Matcher.bind("d"), AST.context()));
  return collectFromDefinition(AST, Definition, InputInferences);
}

std::vector<Evidence> collectFromTargetDecl(llvm::StringRef Source) {
  std::vector<Evidence> Results;
  clang::TestAST AST(getInputsWithAnnotationDefinitions(Source));
  USRCache USRCache;
  collectEvidenceFromTargetDeclaration(
      *dataflow::test::findValueDecl(AST.context(), "target"),
      evidenceEmitter([&](const Evidence& E) { Results.push_back(E); },
                      USRCache));
  return Results;
}

TEST(CollectEvidenceFromDefinitionTest, Location) {
  llvm::StringRef Code = "void target(int *p) { *p; }";
  //                      12345678901234567890123456
  //                      0        1         2

  auto Evidence = collectFromTargetFuncDefinition(Code);
  ASSERT_THAT(Evidence, ElementsAre(evidence(paramSlot(0),
                                             Evidence::UNCHECKED_DEREFERENCE)));
  EXPECT_EQ("input.cc:1:23", Evidence.front().location());
}

TEST(CollectEvidenceFromDefinitionTest, LocationSmart) {
  llvm::StringRef Code =
      "#include <memory>\nvoid target(std::unique_ptr<int> p) { *p; }";
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
    void target(int *p0) {}
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, OneParamUsedWithoutRestriction) {
  static constexpr llvm::StringRef Src = R"cc(
    void takesUnknown(int *unknown) {}

    void target(int *p0) { takesUnknown(p0); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromDefinitionTest, Deref) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p0, int *p1) {
      int a = *p0;
      if (p1 != nullptr) {
        int b = *p1;
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
      int x;
      int y();
    };
    void target(S *a, S *b) {
      a->x;
      b->y();
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE),
                  evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest, DerefSmart) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    struct S {
      int x;
      int y();
    };
    void target(std::unique_ptr<S> p) {
      *p;
      p->x;
      p->y();
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
    void target(Nonnull<int *> p) {
      *p;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, DereferenceBeforeAssignment) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p) {
      *p;
      int i = 1;
      p = &i;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest, DereferenceAfterAssignment) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p) {
      int i = 1;
      p = &i;
      *p;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, DereferenceAfterAssignmentFromReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    int& getIntRef();
    int* getIntPtr();
    void target(int* p) {
      p = &getIntRef();
      *p;
      p = getIntPtr();
      *p;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromDefinitionTest, DerefOfPtrRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *&p0, int *&p1) {
      int a = *p0;
      if (p1 != nullptr) {
        int b = *p1;
      }
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest, UnrelatedCondition) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p0, int *p1, int *p2, bool b) {
      if (b) {
        int a = *p0;
        int b = *p1;
      } else {
        int a = *p0;
        int c = *p2;
      }
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE),
                  evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE),
                  // We collect two Evidence values for two dereferences of p0
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE),
                  evidence(paramSlot(2), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest, LaterDeref) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p0) {
      if (p0 == nullptr) {
        (void)0;
      } else {
        (void)0;
      }
      int a = *p0;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest, DerefBeforeGuardedDeref) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p0) {
      int a = *p0;
      if (p0 != nullptr) {
        int b = *p0;
      }
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest, DerefAndOrCheckOfCopiedPtr) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p, int* q) {
      int* a = p;
      *a;
      int* b = q;
      if (q) {
        *b;
      }
      if (b) {
        *q;
      }
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest, FirstSufficientSlotOnly) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p, int* q) {
      // Marking either of p or q Nonnull is sufficient to avoid dereferencing
      // without a check. We choose to record evidence only for the first
      // sufficient slot which can be Nonnull without the dereference becoming
      // dead code.
      int* a;
      if (p) {
        a = p;
      } else {
        a = q;
      }
      *a;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest,
     FirstSufficientSlotNotContradictingFlowConditions) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p, int* q) {
      // Marking p Nonnull would make the dereference dead, so we collect
      // evidence for q being Nonnull instead, since it is also sufficient.
      if (!p) {
        *q;
      }
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest, EarlyReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p0) {
      if (!p0) {
        return;
      }
      int a = *p0;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, UnreachableCode) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p0, int *p1, int *p2, int *p3) {
      if (true) {
        int a = *p0;
      } else {
        int a = *p1;
      }

      if (false) {
        int a = *p2;
      }

      return;
      int a = *p3;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromDefinitionTest, PointerToMemberField) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {};

    void target(int S::*p) {
      S s;
      s.*p;
      (&s)->*p;
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

    void target(void (S::*p)()) {
      S s;
      (s.*p)();
      ((&s)->*p)();
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

    void target(void (S::*p)(Nonnull<int*> i, int* j), int* q) {
      S s;
      (s.*p)(q, nullptr);
      ((&s)->*p)(q, nullptr);
    }
  )cc";

  // Pointers to members are not supported pointer types, so no evidence is
  // collected for `p` or `j`. If they become a supported pointer type, this
  // test should start failing.
  // TODO(b/309625642) We should still collect evidence for the use of `q` as an
  // argument for param `i`.
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, CheckMacro) {
  llvm::Twine Src = CheckMacroDefinitions + R"cc(
    void target(int* p, int* q, int* r, int* s, int* t, int* u, int* v) {
      // should collect evidence for params from these calls
      CHECK(p);
      CHECK(q != nullptr);
      int* a = nullptr;
      CHECK(r != a);
      CHECK(a != s);
      bool b = t != nullptr;
      CHECK(b);

      // should not crash when analyzing these calls
      CHECK(u == v);
      CHECK(u != v);
      CHECK(1);
      struct S {
        operator bool() const { return true; }
      };
      CHECK(S());
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
                           evidence(paramSlot(4), Evidence::ABORT_IF_NULL)));
}

TEST(CollectEvidenceFromDefinitionTest, CheckMacroSmart) {
  llvm::Twine Src = CheckMacroDefinitions + R"cc(
#include <memory>
    void target(std::unique_ptr<int> p, std::unique_ptr<int> q,
                std::unique_ptr<int> r) {
      CHECK(p);
      CHECK(!!q);
      CHECK(r.get());
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src.str()),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ABORT_IF_NULL),
                           evidence(paramSlot(1), Evidence::ABORT_IF_NULL),
                           evidence(paramSlot(2), Evidence::ABORT_IF_NULL)));
}

TEST(CollectEvidenceFromDefinitionTest, CheckNEMacro) {
  llvm::Twine Src = CheckMacroDefinitions + R"cc(
    void target(int* p, int* q, int* r, int* s) {
      // should collect evidence for params from these calls
      CHECK_NE(p, nullptr);
      CHECK_NE(nullptr, q);
      int* a = nullptr;
      CHECK_NE(a, r);
      CHECK_NE(s, a);

      // should not crash when analyzing these calls
      CHECK_NE(a, 0);
      int i = 1;
      CHECK_NE(i, 0);
      bool b = true;
      CHECK_NE(true, false);
      struct S {
        bool operator==(const S&) const { return false; }
      };
      CHECK_NE(S(), S());
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src.str()),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::ABORT_IF_NULL),
                           evidence(paramSlot(1), Evidence::ABORT_IF_NULL),
                           evidence(paramSlot(2), Evidence::ABORT_IF_NULL),
                           evidence(paramSlot(3), Evidence::ABORT_IF_NULL)));
}

TEST(CollectEvidenceFromDefinitionTest, CheckNEMacroSmart) {
  llvm::Twine Src = CheckMacroDefinitions + R"cc(
#include <memory>
    void target(std::unique_ptr<int> p, std::unique_ptr<int> q,
                std::unique_ptr<int> r, std::unique_ptr<int> s) {
      CHECK_NE(p, nullptr);
      CHECK_NE(nullptr, q);
      if (!r) {
        CHECK_NE(s, r);
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
    void callee(int *q);
    void target(Nullable<int *> p) { callee(p); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Contains(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest, NonnullArgPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int *q);
    void target(Nonnull<int *> p) { callee(p); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Contains(evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT,
                                functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest, UnknownArgPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int *q);
    void target(int *p) { callee(p); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Contains(evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest, UnknownButProvablyNullArgPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int *q);
    void target(int *p) {
      if (p == nullptr) {
        callee(p);
      }
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Contains(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest, CheckedArgPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int *q);
    void target(int *p) {
      if (p) callee(p);
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Contains(evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT,
                                functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest, NullptrPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int* q);
    void target() {
      callee(nullptr);
      int* p = nullptr;
      callee(nullptr);
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("callee")),
                           evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest, NonPtrArgPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int q);
    void target(int p) { callee(p); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, ArgsAndParamsSmart) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
#include <utility>
    void callee(std::unique_ptr<int> p, Nonnull<std::unique_ptr<int>> q,
                Nullable<std::unique_ptr<int>>& r);
    void target(Nullable<std::unique_ptr<int>> a, std::unique_ptr<int> b,
                std::unique_ptr<int> c) {
      callee(std::move(a), std::move(b), c);
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                           functionNamed("callee")),
                  evidence(paramSlot(1), Evidence::BOUND_TO_NONNULL,
                           functionNamed("target")),
                  evidence(paramSlot(2), Evidence::BOUND_TO_MUTABLE_NULLABLE,
                           functionNamed("target")),
                  evidence(paramSlot(1), Evidence::UNKNOWN_ARGUMENT,
                           functionNamed("callee")),
                  evidence(paramSlot(2), Evidence::UNKNOWN_ARGUMENT,
                           functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     DefaultArgumentsProduceNoEvidenceFromDefinition) {
  static constexpr llvm::StringRef Src = R"cc(
    int* getDefault();
    void hasDefaultUnannotatedFunc(int* = getDefault());
    int* q = nullptr;
    void hasDefaultUnannotatedVariable(int* = getDefault());
    int i = 1;
    void hasDefaultExpressionOfVariable(int* = &i);
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
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, NullableButCheckedReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target(Nullable<int*> p) {
      if (p) return p;

      // no return in this path to avoid irrelevant evidence, and this still
      // compiles, as the lack of return in a path is only a warning.
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(SLOT_RETURN_TYPE, Evidence::NONNULL_RETURN,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, NonnullReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target(Nonnull<int*> p) {
      return p;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(SLOT_RETURN_TYPE, Evidence::NONNULL_RETURN,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, UnknownReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target(int* p) { return p; }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(SLOT_RETURN_TYPE, Evidence::UNKNOWN_RETURN,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, UnknownButProvablyNullReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target(int* p) {
      if (p == nullptr) {
        return p;
      }
      // no return in this path to avoid irrelevant evidence, and this still
      // compiles, as the lack of return in a path is only a warning.
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, MultipleReturns) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target(Nonnull<int*> p, Nullable<int*> q, bool b, bool c) {
      if (b) return q;
      if (c) return nullptr;
      return p;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN,
                                    functionNamed("target")),
                           evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN,
                                    functionNamed("target")),
                           evidence(SLOT_RETURN_TYPE, Evidence::NONNULL_RETURN,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, FromReturnAnnotation) {
  static constexpr llvm::StringRef Src = R"cc(
    Nonnull<int*> target(int* a) {
      return a;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     FromPreviouslyInferredReturnAnnotation) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target(int* a) { return a; }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(
          Src, {.Nonnull = {fingerprint("c:@F@target#*I#", 0)}}),
      UnorderedElementsAre(
          evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                   functionNamed("target")),
          // We still collect evidence for the return type in case iteration
          // turns up new evidence to contradict a previous inference. Only
          // nullabilities written in source code are considered unchangeable.
          evidence(SLOT_RETURN_TYPE, Evidence::UNKNOWN_RETURN,
                   functionNamed("target"))));
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
      auto p = makePtr();
      *p;
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
      auto p = makePtr();
      if (p) *p;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest,
     FunctionCallResultDereferencedAfterUnrelatedConditionChecked) {
  static constexpr llvm::StringRef Src = R"cc(
    int* makePtr();
    void target(bool cond) {
      auto p = makePtr();
      if (cond) *p;
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
    void target(void (*f)()) { f(); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(evidence(paramSlot(0),
                                            Evidence::UNCHECKED_DEREFERENCE,
                                            functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, ConstAccessorDereferencedAfterCheck) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      int* accessor() const { return i; }
      int* i = nullptr;
    };
    void target() {
      S s;
      if (s.accessor() != nullptr) {
        *s.accessor();
      }
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

// Special modeling of accessors is not implemented for accessors returning
// references.
TEST(CollectEvidenceFromDefinitionTest,
     ReferenceConstAccessorDereferencedAfterCheck) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      int* const& accessor() const { return i; }
      int* i = nullptr;
    };
    void target() {
      S s;
      if (s.accessor() != nullptr) {
        *s.accessor();
      }
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(evidence(SLOT_RETURN_TYPE,
                                            Evidence::UNCHECKED_DEREFERENCE,
                                            functionNamed("accessor"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     ConstAccessorOnTwoDifferentObjectsDereferencedAfterCheck) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      int* const& accessor() const { return i; }
      int* i = nullptr;
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
      S s;
      *s();
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
      S(Nonnull<int*> a);
    };
    void target(int* p) { S s(p); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("S"))));
}

TEST(CollectEvidenceFromDefinitionTest, NonTargetConstructorCall) {
  static constexpr llvm::StringRef Src = R"cc(
    template <typename T>
    struct S {
      // Not a target due to templating, but the annotation here can still
      // provide evidence for `p` from the call in `target`'s body.
      S(Nonnull<T*> a);
    };
    void target(int* p) { S s(p); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, ConstructorWithBaseInitializer) {
  static constexpr llvm::StringRef Src = R"cc(
    struct TakeNonnull {
      explicit TakeNonnull(Nonnull<int *>);
    };
    struct target : TakeNonnull {
      target(int *i) : TakeNonnull(i) {}
    };
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Contains(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, ConstructorWithDelegatingConstructor) {
  static constexpr llvm::StringRef Src = R"cc(
    struct target {
      target(int* i);
      target() : target(nullptr){};
    };
  )cc";

  EXPECT_THAT(collectFromDefinitionMatching(
                  functionDecl(hasName("target"), parameterCountIs(0)), Src),
              Contains(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, VariadicConstructorCall) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      S(Nonnull<int*> i, ...);
    };
    void target(int* p, int* q) { S s(p, q); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("S"))));
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
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("Target"))));
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
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
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
    void foo() { Target t; }
  )cc";
  EXPECT_THAT(
      collectFromDefinitionMatching(
          cxxConstructorDecl(isDefaultConstructor(), hasName("Target")), Src),
      UnorderedElementsAre(
          evidence(Slot(0), Evidence::NULLABLE_DEFAULT_MEMBER_INITIALIZER,
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
                  evidence(SLOT_RETURN_TYPE, Evidence::BOUND_TO_NONNULL,
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
    void foo() { Target t; }
  )cc";
  EXPECT_THAT(collectFromDefinitionMatching(
                  cxxConstructorDecl(isDefaultConstructor()), Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::BOUND_TO_NONNULL,
                           functionNamed("getIntPtr")),
                  evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                           functionNamed("getIntPtr"))));
}

TEST(CollectEvidenceFromDefinitionTest, PassedToNonnull) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nonnull<int*> i);

    void target(int* p) { callee(p); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest, PassedToNonnullRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nonnull<int*>& i);

    void target(int* p) { callee(p); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest, PassedToNonnullInMemberFunction) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      void callee(Nonnull<int*> i);
    };

    void target(int* p) {
      S s;
      s.callee(p);
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest, PassedToNonnullInFunctionPointerParam) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p, void (*callee)(Nonnull<int*> i)) {
      callee(p);
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                           functionNamed("target")),
                  evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE,
                           functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     PassedToNonnullInFunctionPointerParamSmart) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
#include <utility>
    void target(std::unique_ptr<int*> p,
                void (*callee)(Nonnull<std::unique_ptr<int*>> i)) {
      callee(std::move(p));
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                           functionNamed("target")),
                  evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE,
                           functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, PassedToNonnullInFunctionPointerField) {
  static constexpr llvm::StringRef Src = R"cc(
    struct MyStruct {
      void (*callee)(Nonnull<int*>);
    };

    void target(int* p) { MyStruct().callee(p); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                                    fieldNamed("MyStruct::callee"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     PassedToNonnullInFunctionPointerFromAddressOfFunctionDecl) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nonnull<int*> i);

    void target(int* p) { (&callee)(p); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     PassedToNonnullInFunctionReferenceParam) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p, void (&callee)(Nonnull<int*> i)) {
      callee(p);
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     PassedToNonnullInFunctionPointerReferenceParam) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p, void (*&callee)(Nonnull<int*> i)) {
      callee(p);
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                           functionNamed("target")),
                  evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE,
                           functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, FunctionCallPassedToNonnull) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nonnull<int*> i);
    int* makeIntPtr();

    void target() { callee(makeIntPtr()); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::BOUND_TO_NONNULL,
                           functionNamed("makeIntPtr")),
                  evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                           functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     FunctionCallPassedToNonnullFunctionPointer) {
  static constexpr llvm::StringRef Src = R"cc(
    int* makeIntPtr();

    void target(void (*callee)(Nonnull<int*> i)) { callee(makeIntPtr()); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::BOUND_TO_NONNULL,
                           functionNamed("makeIntPtr")),
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                           functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     FunctionCallPassedToNonnullFunctionPointerTargetNotAnInferenceTarget) {
  static constexpr llvm::StringRef Src = R"cc(
    int* makeIntPtr();

    template <typename T>
    void target(void (*callee)(Nonnull<T*> i), int* a) {
      callee(makeIntPtr());
      *a;
    }

    void instantiate() {
      target<int>([](Nonnull<int*> i) {}, nullptr);
    }
  )cc";
  // Doesn't collect any evidence for target from target's body, only collects
  // some for makeIntPtr.
  EXPECT_THAT(
      collectFromDefinitionMatching(
          functionDecl(hasName("target"), isTemplateInstantiation()), Src),
      UnorderedElementsAre(evidence(SLOT_RETURN_TYPE,
                                    Evidence::BOUND_TO_NONNULL,
                                    functionNamed("makeIntPtr"))));
}

TEST(CollectEvidenceFromDefinitionTest, PassedToNullable) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nullable<int*> i);

    void target(int* p) { callee(p); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromDefinitionTest, PassedToNullableRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nullable<int*>& i);

    void target(int* p) { callee(p); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::BOUND_TO_MUTABLE_NULLABLE,
                           functionNamed("target")),
                  evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                           functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     PassedToNullableRefFromStoredFunctionCall) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nullable<int*>& i);
    int* producer();

    void target() {
      auto p = producer();
      callee(p);
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(
          evidence(SLOT_RETURN_TYPE, Evidence::BOUND_TO_MUTABLE_NULLABLE,
                   functionNamed("producer")),
          evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                   functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest, PassedToNullableRefFromFunctionCall) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nullable<int*>& i);
    int*& producer();

    void target() { callee(producer()); }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(
          evidence(SLOT_RETURN_TYPE, Evidence::BOUND_TO_MUTABLE_NULLABLE,
                   functionNamed("producer")),
          evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                   functionNamed("callee"))));
}

TEST(CollectEvidenceFromDefinitionTest, AssignedToNonnull) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p, int* q, int* r) {
      Nonnull<int*> a = p, b = q;
      a = r;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(paramSlot(1), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(paramSlot(2), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, RefAssignedToNonnullRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int*& p) {
      Nonnull<int*>& a = p;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     FunctionCallAssignedToNonnullTargetNotAnInferenceTarget) {
  static constexpr llvm::StringRef Src = R"cc(
    int* makeIntPtr();

    template <typename T>
    void target() {
      Nonnull<T*> p = makeIntPtr();
    }

    void instantiate() { target<int>(); }
  )cc";

  // Doesn't collect any evidence for target from target's body, only collects
  // some for makeIntPtr.
  EXPECT_THAT(
      collectFromDefinitionMatching(
          functionDecl(hasName("target"), isTemplateInstantiation()), Src),
      UnorderedElementsAre(evidence(SLOT_RETURN_TYPE,
                                    Evidence::BOUND_TO_NONNULL,
                                    functionNamed("makeIntPtr"))));
}

TEST(CollectEvidenceFromDefinitionTest, AssignedToNullableOrUnknown) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p, int* q, int* r) {
      Nullable<int*> a = p;
      int* b = q;
      NullabilityUnknown<int*> c = r;
      q = r;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, AssignedToNullableRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p) {
      Nullable<int*>& a = p;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(evidence(paramSlot(0),
                                            Evidence::BOUND_TO_MUTABLE_NULLABLE,
                                            functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, AssignedToNullableRefSmart) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>
    void target(std::unique_ptr<int> p) {
      Nullable<std::unique_ptr<int>>& a = p;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(evidence(paramSlot(0),
                                            Evidence::BOUND_TO_MUTABLE_NULLABLE,
                                            functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, RefAssignedToNullableRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int*& p) {
      Nullable<int*>& a = p;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(evidence(paramSlot(0),
                                            Evidence::BOUND_TO_MUTABLE_NULLABLE,
                                            functionNamed("target"))));
}

// Ternary expressions are not currently modeled correctly by the analysis, but
// are necessary to test the case of multiple connected decls.
//
// DISABLED until ternary expressions are handle.
TEST(CollectEvidenceFromDefinitionTest,
     DISABLED_AssignedToNullableRefAllConnectedDecls) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p, int* q, bool b) {
      Nullable<int*>& x = b ? p : q;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::BOUND_TO_MUTABLE_NULLABLE,
                           functionNamed("target")),
                  evidence(paramSlot(1), Evidence::BOUND_TO_MUTABLE_NULLABLE,
                           functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, AssignedFromNullptr) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p, int* q) {
      q = nullptr;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(evidence(paramSlot(1),
                                            Evidence::ASSIGNED_FROM_NULLABLE,
                                            functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, AssignedFromNullptrIndirect) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p) {
      int* a = nullptr;
      p = a;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(evidence(paramSlot(0),
                                            Evidence::ASSIGNED_FROM_NULLABLE,
                                            functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, AssignedFromZero) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p, int* q) {
      q = 0;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(evidence(paramSlot(1),
                                            Evidence::ASSIGNED_FROM_NULLABLE,
                                            functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, AssignedFromNullable) {
  static constexpr llvm::StringRef Src = R"cc(
    Nullable<int*> getNullable();
    void target(int* p) { p = getNullable(); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(evidence(paramSlot(0),
                                            Evidence::ASSIGNED_FROM_NULLABLE,
                                            functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, AssignedFromLocalNullable) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p) {
      Nullable<int*> a;
      p = a;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              UnorderedElementsAre(evidence(paramSlot(0),
                                            Evidence::ASSIGNED_FROM_NULLABLE,
                                            functionNamed("target"))));
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

TEST(CollectEvidenceFromDefinitionTest, IrrelevantAssignments) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      S(int* i);
    };

    void target(int* p) {
      int* a = p;  // No useful information.

      // We don't collect if types on either side are not a supported pointer
      // type.
      int* b = 0;
      int c = 4;
      bool d = false;
      S e = a;

      // We don't collect from compound assignments.
      b += 8;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      // From the constructor call constructing an S; no evidence from
      // assignments.
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("S"))));
}

TEST(CollectEvidenceFromDefinitionTest, Arithmetic) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* a, int* b, int* c, int* d, int* e, int* f, int* g,
                int* h) {
      a += 1;
      b -= 2;
      c + 3;
      d - 4;
      e++;
      ++f;
      g--;
      --h;
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(
          evidence(paramSlot(0), Evidence::ARITHMETIC, functionNamed("target")),
          evidence(paramSlot(1), Evidence::ARITHMETIC, functionNamed("target")),
          evidence(paramSlot(2), Evidence::ARITHMETIC, functionNamed("target")),
          evidence(paramSlot(3), Evidence::ARITHMETIC, functionNamed("target")),
          evidence(paramSlot(4), Evidence::ARITHMETIC, functionNamed("target")),
          evidence(paramSlot(5), Evidence::ARITHMETIC, functionNamed("target")),
          evidence(paramSlot(6), Evidence::ARITHMETIC, functionNamed("target")),
          evidence(paramSlot(7), Evidence::ARITHMETIC,
                   functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, Fields) {
  llvm::Twine Src = CheckMacroDefinitions + R"cc(
#include <memory>
    void takesNonnull(Nonnull<int*>);
    void takesMutableNullable(Nullable<int*>&);
    struct S {
      int* Deref;
      int* BoundToNonnull;
      int* BoundToMutableNullable;
      int* AbortIfNull;
      int* AbortIfNullBool;
      int* AbortIfNullNE;
      int* AssignedFromNullable;
      int* Arithmetic;
      std::unique_ptr<int> SmartDeref;
    };
    void target(S AnS) {
      *AnS.Deref;
      takesNonnull(AnS.BoundToNonnull);
      takesMutableNullable(AnS.BoundToMutableNullable);
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
           evidence(Slot(0), Evidence::BOUND_TO_NONNULL,
                    fieldNamed("S::BoundToNonnull")),
           evidence(Slot(0), Evidence::BOUND_TO_MUTABLE_NULLABLE,
                    fieldNamed("S::BoundToMutableNullable")),
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
      static int* BoundToNonnull;
      static int* BoundToMutableNullable;
      static int* AbortIfNull;
      static int* AbortIfNullBool;
      static int* AbortIfNullNE;
      static int* AssignedFromNullable;
      static int* Arithmetic;
      static std::unique_ptr<int> SmartDeref;
    };
    void target() {
      *MyStruct::Deref;
      takesNonnull(MyStruct::BoundToNonnull);
      takesMutableNullable(MyStruct::BoundToMutableNullable);
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
           evidence(Slot(0), Evidence::BOUND_TO_NONNULL,
                    staticFieldNamed("MyStruct::BoundToNonnull")),
           evidence(Slot(0), Evidence::BOUND_TO_MUTABLE_NULLABLE,
                    staticFieldNamed("MyStruct::BoundToMutableNullable")),
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
    int* BoundToNonnull;
    int* BoundToMutableNullable;
    int* AbortIfNull;
    int* AbortIfNullBool;
    int* AbortIfNullNE;
    int* AssignedFromNullable;
    int* Arithmetic;
    std::unique_ptr<int> SmartDeref;
    void target() {
      *Deref;
      takesNonnull(BoundToNonnull);
      takesMutableNullable(BoundToMutableNullable);
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
                    evidence(Slot(0), Evidence::BOUND_TO_NONNULL,
                             globalVarNamed("BoundToNonnull")),
                    evidence(Slot(0), Evidence::BOUND_TO_MUTABLE_NULLABLE,
                             globalVarNamed("BoundToMutableNullable")),
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
                  evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                           functionNamed("getNullableFromNonnull")),
                  evidence(SLOT_RETURN_TYPE, Evidence::BOUND_TO_NONNULL,
                           functionNamed("getPtr")),
                  evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                           globalVarNamed("Target"))));
}

TEST(CollectEvidenceFromDefinitionTest, GlobalInitFromGlobalAnnotation) {
  static constexpr llvm::StringRef Src = R"cc(
    int* foo();
    Nonnull<int*> Target = foo();
  )cc";
  EXPECT_THAT(
      collectFromDefinitionNamed("Target", Src),
      UnorderedElementsAre(evidence(
          SLOT_RETURN_TYPE, Evidence::BOUND_TO_NONNULL, functionNamed("foo"))));
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
  EXPECT_THAT(collectFromDefinitionMatching(varDecl(hasInit()), Src),
              UnorderedElementsAre(

                  evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                           staticFieldNamed("MyStruct::Target"))));
}

TEST(CollectEvidenceFromDefinitionTest, NoEvidenceForLocals) {
  static constexpr llvm::StringRef Src = R"cc(
    void target() {
      int* p = nullptr;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, FunctionCallInLoop) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p) {
      for (int i = 0; i < 3; ++i) {
        target(nullptr);
      }
      for (int i = 0; i < 3; ++i) {
        target(&i);
      }
      for (int i = 0; i < 3; ++i) {
        target(p);
      }
    }
  )cc";
  EXPECT_THAT(
      collectFromTargetFuncDefinition(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("target")),
                           evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("target")),
                           evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, OutputParameterPointerToPointer) {
  static constexpr llvm::StringRef Src = R"cc(
    void maybeModifyPtr(int** a);
    void target(int* p) {
      maybeModifyPtr(&p);
      *p;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromDefinitionTest, OutputParameterReferenceToPointer) {
  static constexpr llvm::StringRef Src = R"cc(
    void maybeModifyPtr(int*& a);
    void target(int* p) {
      maybeModifyPtr(p);
      *p;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromDefinitionTest,
     OutputParameterReferenceToConstPointer) {
  static constexpr llvm::StringRef Src = R"cc(
    void dontModifyPtr(int* const& a);
    void target(int* p) {
      dontModifyPtr(p);
      *p;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Contains(evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                                functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest,
     OutputParameterReferenceToPointerToPointer) {
  static constexpr llvm::StringRef Src = R"cc(
    void maybeModifyPtr(int**& a);
    void target(int** p) {
      maybeModifyPtr(p);
      *p;
      **p;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromDefinitionTest, OutputParameterPointerToConstPointer) {
  static constexpr llvm::StringRef Src = R"cc(
    void dontModifyPtr(int* const* a);
    void target(int* p) {
      dontModifyPtr(&p);
      *p;
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
    void maybeModifyPtr(const int** const a);
    void target(const int* p) {
      maybeModifyPtr(&p);
      *p;
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromDefinitionTest, PassAsOutputParameterOrDereference) {
  static constexpr llvm::StringRef Src = R"cc(
    void maybeModifyPtr(int** a);
    void target(int* p, bool b) {
      if (b) {
        maybeModifyPtr(&p);
      } else {
        *p;
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
    void maybeModifyPtr(int** a);
    void target(int* p, bool b) {
      if (b) maybeModifyPtr(&p);
      *p;  // Because we model p as Unknown post-output-parameter-use, adding an
           // annotation would not be considered sufficient to make this
           // dereference safe, so we do not collect evidence for p.
    }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromDefinitionTest, FromGlobalLabmdaBodyForGlobal) {
  static constexpr llvm::StringRef Src = R"cc(
    int* p;
    auto Lambda = []() { *p; };
  )cc";

  EXPECT_THAT(
      collectFromDefinitionNamed("operator()", Src),
      UnorderedElementsAre(evidence(Slot(0), Evidence::UNCHECKED_DEREFERENCE,
                                    globalVarNamed("p"))));
}

// TODO(b/315967534) Collect for captured function parameters, specifically from
// the unchecked dereference of `foo`'s parameter.
TEST(CollectEvidenceFromDefinitionTest, FromLocalLambdaForCapturedParam) {
  static constexpr llvm::StringRef Src = R"cc(
    void foo(int* p) {
      auto Lambda = [&p]() { *p; };
    }
  )cc";

  EXPECT_THAT(collectFromDefinitionNamed("operator()", Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, FromLocalLambdaForCalledFunction) {
  static constexpr llvm::StringRef Src = R"cc(
    int* bar();
    void foo() {
      auto Lambda = []() { *bar(); };
    }
  )cc";

  EXPECT_THAT(collectFromDefinitionNamed("operator()", Src),
              UnorderedElementsAre(evidence(SLOT_RETURN_TYPE,
                                            Evidence::UNCHECKED_DEREFERENCE,
                                            functionNamed("bar"))));
}

// TODO(b/315967535) If we collect evidence for lambda declarations, collect
// from the unchecked dereference of the lambda's parameter and/or the null
// return.
TEST(CollectEvidenceFromDefinitionTest, NoneForLambdaParamOrReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    auto Lambda = [](int* p) {
      *p;
      return nullptr;
    };
  )cc";

  EXPECT_THAT(collectFromDefinitionNamed("operator()", Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest, AggregateInitialization) {
  static constexpr llvm::StringRef Header = R"cc(
    struct Base {
      bool* BaseB;
      Nonnull<char*> BaseC;
    };
    struct MyStruct : public Base {
      int* I;
      bool* B;
    };
  )cc";
  const llvm::Twine BracesAggInit = Header + R"cc(
    void target(Nullable<bool*> Bool, char* Char) {
      MyStruct{Bool, Char, nullptr, Bool};
    }
  )cc";
  // New aggregate initialization syntax in C++20
  const llvm::Twine ParensAggInit = Header + R"cc(
    void target(Nullable<bool*> Bool, char* Char) {
      MyStruct(Base(Bool, Char), nullptr, Bool);
    }
  )cc";

  auto ExpectedEvidenceMatcher =
      UnorderedElementsAre(evidence(Slot(0), Evidence::ASSIGNED_FROM_NULLABLE,
                                    fieldNamed("Base::BaseB")),
                           evidence(paramSlot(1), Evidence::BOUND_TO_NONNULL,
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

// This is a crash repro related to aggregate initialization.
TEST(CollectEvidenceFromDefinitionTest, NonRecordInitListExpr) {
  static constexpr llvm::StringRef Src = R"cc(
    void target() { int a[3] = {}; }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src), IsEmpty());
}

TEST(CollectEvidenceFromDefinitionTest,
     SmartPointerAnalysisProvidesEvidenceForRawPointer) {
  static constexpr llvm::StringRef Src = R"cc(
#include <memory>

    void foo(int*);
    void target(Nullable<std::unique_ptr<int>> p) { foo(p.get()); }
  )cc";
  EXPECT_THAT(collectFromTargetFuncDefinition(Src),
              Contains(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                functionNamed("foo"))));
}

TEST(CollectEvidenceFromDefinitionTest, NotInferenceTarget) {
  static constexpr llvm::StringRef Src = R"cc(
    void isATarget(Nonnull<int*> a);
    template <typename T>
    T* target(T* p) {
      *p;
      Nonnull<int*> a = p;
      isATarget(p);
      target<T>(nullptr);
      target<int>(nullptr);
      return nullptr;
    }

    void instantiate() { target<int>(nullptr); }
  )cc";
  // Doesn't collect any evidence for target from target's body, only collects
  // some for isATarget.
  EXPECT_THAT(
      collectFromDefinitionMatching(
          functionDecl(hasName("target"), isTemplateInstantiation()), Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("isATarget"))));
}

TEST(CollectEvidenceFromDefinitionTest, PropagatesPreviousInferences) {
  static constexpr llvm::StringRef Src = R"cc(
    void calledWithToBeNullable(int* x);
    void calledWithToBeNonnull(int* a);
    void target(int* p, int* q) {
      target(nullptr, q);
      calledWithToBeNullable(p);
      *q;
      calledWithToBeNonnull(q);
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
                  Src, {/*Nullable=*/{fingerprint(TargetUsr, paramSlot(0))},
                        /*Nonnull=*/{fingerprint(TargetUsr, paramSlot(1))}}),
              AllOf(IsSupersetOf(ExpectedBothRoundResults),
                    IsSupersetOf(ExpectedSecondRoundResults)));
}

TEST(CollectEvidenceFromDefinitionTest,
     AnalysisUsesPreviousInferencesForSlotsOutsideTargetDefinition) {
  static constexpr llvm::StringRef Src = R"cc(
    int* returnsToBeNonnull(int* a) {
      return a;
    }
    int* target(int* q) {
      *q;
      return returnsToBeNonnull(q);
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
      Src, {.Nonnull = {fingerprint(TargetUsr, paramSlot(0))}});
  EXPECT_THAT(SecondRoundResults,
              AllOf(IsSupersetOf(ExpectedNewResultsPerRound.at(0)),
                    IsSupersetOf(ExpectedNewResultsPerRound.at(1))));
  for (const auto& E : ExpectedNewResultsPerRound.at(2)) {
    ASSERT_THAT(SecondRoundResults, Not(Contains(E)));
  }

  auto ThirdRoundResults = collectFromTargetFuncDefinition(
      Src, {.Nonnull = {fingerprint(TargetUsr, paramSlot(0)),
                        fingerprint(ReturnsToBeNonnullUsr, paramSlot(0))}});
  EXPECT_THAT(ThirdRoundResults,
              AllOf(IsSupersetOf(ExpectedNewResultsPerRound.at(0)),
                    IsSupersetOf(ExpectedNewResultsPerRound.at(1)),
                    IsSupersetOf(ExpectedNewResultsPerRound.at(2))));
  for (const auto& E : ExpectedNewResultsPerRound.at(3)) {
    ASSERT_THAT(ThirdRoundResults, Not(Contains(E)));
  }

  auto FourthRoundResults = collectFromTargetFuncDefinition(
      Src, {.Nonnull = {
                fingerprint(TargetUsr, paramSlot(0)),
                fingerprint(ReturnsToBeNonnullUsr, paramSlot(0)),
                // As noted in the Evidence matcher list above, we don't infer
                // the return type of returnsToBeNonnull from only collecting
                // evidence from target's definition, but for the sake of this
                // test, let's pretend we collected evidence from the entire TU.
                fingerprint(ReturnsToBeNonnullUsr, SLOT_RETURN_TYPE)}});
  EXPECT_THAT(FourthRoundResults,
              AllOf(IsSupersetOf(ExpectedNewResultsPerRound.at(0)),
                    IsSupersetOf(ExpectedNewResultsPerRound.at(1)),
                    IsSupersetOf(ExpectedNewResultsPerRound.at(2)),
                    IsSupersetOf(ExpectedNewResultsPerRound.at(3))));
}

TEST(CollectEvidenceFromDefinitionTest,
     PreviousInferencesOfNonTargetParameterNullabilitiesPropagate) {
  static constexpr llvm::StringRef Src = R"cc(
    void takesToBeNonnull(int* a) {
      // Not read when collecting evidence only from Target, but corresponding
      // inference is explicitly input below.
      *a;
    }
    void target(int* q) { takesToBeNonnull(q); }
  )cc";
  std::string TakesToBeNonnullUsr = "c:@F@takesToBeNonnull#*I#";

  // Pretend that in a first round of inferring for all functions, we made this
  // inference about takesToBeNonnull's first parameter.
  // This test confirms that we use that information when collecting from
  // target's definition.
  EXPECT_THAT(
      collectFromTargetFuncDefinition(
          Src, {.Nonnull = {fingerprint(TakesToBeNonnullUsr, paramSlot(0))}}),
      Contains(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                        functionNamed("target"))));
}

TEST(CollectEvidenceFromDefinitionTest, SolverLimitReached) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p, int* q) {
      *p;
      *q;
    }
  )cc";
  clang::TestAST AST(getInputsWithAnnotationDefinitions(Src));
  std::vector<Evidence> Results;
  USRCache UsrCache;
  EXPECT_THAT_ERROR(
      collectEvidenceFromDefinition(
          *cast<FunctionDecl>(
              dataflow::test::findValueDecl(AST.context(), "target")),
          evidenceEmitter([&](const Evidence& E) { Results.push_back(E); },
                          UsrCache),
          UsrCache, /*PreviousInferences=*/{},
          // Enough iterations to collect one piece of evidence but not both.
          /*MaxSATIterations=*/100),
      llvm::FailedWithMessage("SAT solver reached iteration limit"));
  EXPECT_THAT(Results, SizeIs(1));
}

TEST(CollectEvidenceFromDeclarationTest, GlobalVariable) {
  llvm::StringLiteral Src = R"cc(
    Nullable<int *> target;
  )cc";
  EXPECT_THAT(collectFromTargetDecl(Src),
              ElementsAre(evidence(Slot(0), Evidence::ANNOTATED_NULLABLE,
                                   globalVarNamed("target"))));
}

TEST(CollectEvidenceFromDeclarationTest, GlobalVariableSmart) {
  llvm::StringLiteral Src = R"cc(
#include <memory>
    Nullable<std::unique_ptr<int>> target;
  )cc";
  EXPECT_THAT(collectFromTargetDecl(Src),
              ElementsAre(evidence(Slot(0), Evidence::ANNOTATED_NULLABLE,
                                   globalVarNamed("target"))));
}

TEST(CollectEvidenceFromDeclarationTest, StaticMemberVariable) {
  llvm::StringLiteral Src = R"cc(
    struct S {
      static Nonnull<int*> target;
    };
  )cc";
  EXPECT_THAT(collectFromTargetDecl(Src),
              ElementsAre(evidence(Slot(0), Evidence::ANNOTATED_NONNULL,
                                   staticFieldNamed("S::target"))));
}

TEST(CollectEvidenceFromDeclarationTest, Field) {
  llvm::StringLiteral Src = R"cc(
    struct S {
      Nonnull<int*> target;
    };
  )cc";
  EXPECT_THAT(collectFromTargetDecl(Src),
              ElementsAre(evidence(Slot(0), Evidence::ANNOTATED_NONNULL,
                                   fieldNamed("S::target"))));
}

TEST(CollectEvidenceFromDeclarationTest, FieldSmart) {
  llvm::StringLiteral Src = R"cc(
#include <memory>
    struct S {
      Nonnull<std::unique_ptr<int>> target;
    };
  )cc";
  EXPECT_THAT(collectFromTargetDecl(Src),
              ElementsAre(evidence(Slot(0), Evidence::ANNOTATED_NONNULL,
                                   fieldNamed("S::target"))));
}

TEST(CollectEvidenceFromDeclarationTest, FunctionDeclReturnType) {
  llvm::StringLiteral Src = R"cc(
    Nonnull<int *> target();
  )cc";
  EXPECT_THAT(
      collectFromTargetDecl(Src),
      ElementsAre(evidence(SLOT_RETURN_TYPE, Evidence::ANNOTATED_NONNULL,
                           functionNamed("target"))));
}

TEST(CollectEvidenceFromDeclarationTest, FunctionDeclParams) {
  llvm::StringLiteral Src = R"cc(
    void target(Nullable<int*>, int*, Nonnull<int*>);
  )cc";
  EXPECT_THAT(collectFromTargetDecl(Src),
              ElementsAre(evidence(paramSlot(0), Evidence::ANNOTATED_NULLABLE),
                          evidence(paramSlot(2), Evidence::ANNOTATED_NONNULL)));
}

TEST(CollectEvidenceFromDeclarationTest, FunctionDeclNonTopLevel) {
  llvm::StringLiteral Src = R"cc(
    Nonnull<int*>** target(Nullable<int*>*);
  )cc";
  EXPECT_THAT(collectFromTargetDecl(Src), IsEmpty());
}

TEST(CollectEvidenceFromDeclarationTest, FunctionDeclSmart) {
  llvm::StringLiteral Src = R"cc(
#include <memory>
    Nullable<std::unique_ptr<int>> target(Nonnull<std::unique_ptr<int>>,
                                          Nullable<std::unique_ptr<int>>);
  )cc";
  EXPECT_THAT(
      collectFromTargetDecl(Src),
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
  EXPECT_THAT(collectFromTargetDecl(Src), IsEmpty());
}

TEST(CollectEvidenceFromDeclarationTest, DefaultArgumentNullptrLiteral) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* = nullptr);
  )cc";
  EXPECT_THAT(
      collectFromTargetDecl(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDeclarationTest, DefaultArgumentZeroLiteral) {
  static constexpr llvm::StringRef Src =
      R"cc(
    void target(int* = 0);
      )cc";
  EXPECT_THAT(
      collectFromTargetDecl(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDeclarationTest, DefaultArgumentAnnotatedVariable) {
  static constexpr llvm::StringRef Src = R"cc(
    Nonnull<int*> q;
    void target(int* = q);
  )cc";
  EXPECT_THAT(
      collectFromTargetDecl(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDeclarationTest,
     DefaultArgumentCallingAnnotatedFunction) {
  static constexpr llvm::StringRef Src = R"cc(
    Nullable<int*> getDefault();
    void target(int* = getDefault());
  )cc";
  EXPECT_THAT(
      collectFromTargetDecl(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDeclarationTest,
     DefaultArgumentUnannotatedNonLiteralExpressionsUnknown) {
  static constexpr llvm::StringRef Src = R"cc(
    int* getDefault();
    int* q = nullptr;
    int i = 1;
    void target(int* = getDefault(), int* = q, int* = &i);
  )cc";
  EXPECT_THAT(
      collectFromTargetDecl(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("target")),
                           evidence(paramSlot(1), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("target")),
                           evidence(paramSlot(2), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("target"))));
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

TEST(EvidenceSitesTest, Functions) {
  TestAST AST(R"cc(
    void foo();
    void bar();
    void bar() {}
    void baz() {}

    struct S {
      S() {}
      void member();
    };
    void S::member() {}
  )cc");
  auto Sites = EvidenceSites::discover(AST.context());
  EXPECT_THAT(Sites.Declarations,
              ElementsAre(declNamed("foo"), declNamed("bar"), declNamed("bar"),
                          declNamed("baz"), declNamed("S::S"),
                          declNamed("S::member"), declNamed("S::member")));
  EXPECT_THAT(Sites.Definitions,
              ElementsAre(declNamed("bar"), declNamed("baz"), declNamed("S::S"),
                          declNamed("S::member")));
}

TEST(EvidenceSitesTest, Lambdas) {
  TestAST AST(R"cc(
    auto Lambda = []() {};
  )cc");
  auto Sites = EvidenceSites::discover(AST.context());
  // TODO(b/315967535) If we collect for lambda declarations, add them to
  // `Sites.Declarations`.
  EXPECT_THAT(Sites.Declarations, IsEmpty());
  EXPECT_THAT(Sites.Definitions,
              UnorderedElementsAre(declNamed("(anonymous class)::operator()"),
                                   declNamed("Lambda")));
}

TEST(EvidenceSitesTest, GlobalVariables) {
  TestAST AST = getInputsWithAnnotationDefinitions(R"cc(
#include <memory>
    int* x = true ? nullptr : nullptr;
    int* y;
    int a;
    int b = *y;
    std::unique_ptr<int> p;
    std::unique_ptr<int> q = nullptr;
  )cc");

  auto Sites = EvidenceSites::discover(AST.context());
  EXPECT_THAT(Sites.Declarations,
              UnorderedElementsAre(declNamed("x"), declNamed("y"),
                                   declNamed("p"), declNamed("q")));
  EXPECT_THAT(
      Sites.Definitions,
      UnorderedElementsAre(
          declNamed("x"), declNamed("b"),
          // unique_ptr p has an initializer because of default construction.
          declNamed("p"), declNamed("q")));
}

TEST(EvidenceSitesTest, StaticMemberVariables) {
  TestAST AST(R"cc(
    struct S {
      inline static int* a = nullptr;
      static int* b;
      static int* c;
    };

    int* S::c = nullptr;
  )cc");
  auto Sites = EvidenceSites::discover(AST.context());
  EXPECT_THAT(
      Sites.Declarations,
      UnorderedElementsAre(
          declNamed("S::a"), declNamed("S::b"),
          // one for in-class declaration and one for out-of-class definition
          declNamed("S::c"), declNamed("S::c")));
  EXPECT_THAT(Sites.Definitions,
              UnorderedElementsAre(declNamed("S::a"), declNamed("S::c")));
}

TEST(EvidenceSitesTest, NonStaticMemberVariables) {
  TestAST AST = getInputsWithAnnotationDefinitions(R"cc(
#include <memory>
    struct S {
      int* a = nullptr;
      int* b;
      std::unique_ptr<int> p = nullptr;
      std::unique_ptr<int> q;
    };
  )cc");
  auto Sites = EvidenceSites::discover(AST.context());
  EXPECT_THAT(Sites.Declarations,
              UnorderedElementsAre(declNamed("S::a"), declNamed("S::b"),
                                   declNamed("S::p"), declNamed("S::q")));
  EXPECT_THAT(Sites.Definitions, IsEmpty());
}

TEST(EvidenceSitesTest, LocalVariablesNotIncluded) {
  TestAST AST(R"cc(
    void foo() {
      int* p = nullptr;
      static int* q = nullptr;
    }
  )cc");
  auto Sites = EvidenceSites::discover(AST.context());
  EXPECT_THAT(Sites.Declarations, UnorderedElementsAre(declNamed("foo")));
  EXPECT_THAT(Sites.Definitions, UnorderedElementsAre(declNamed("foo")));
}

TEST(EvidenceSitesTest, Templates) {
  TestAST AST(R"cc(
    template <int I>
    int f() {
      return I;
    }
    template <>
    int f<1>() {
      return 1;
    }

    struct S {
      template <int I>
      int f() {
        return I;
      }
    };

    template <int I>
    struct T {
      int f() { return I; }
    };

    int Unused = f<0>() + f<1>() + S{}.f<0>() + T<0>{}.f();
  )cc");
  auto Sites = EvidenceSites::discover(AST.context());

  // Relevant declarations are the written ones that are not templates.
  EXPECT_THAT(Sites.Declarations, ElementsAre(declNamed("f<1>")));
  // Instantiations are relevant definitions, as is the global variable.
  EXPECT_THAT(
      Sites.Definitions,
      ElementsAre(declNamed("f<0>"), declNamed("f<1>"), declNamed("S::f<0>"),
                  declNamed("T<0>::f"), declNamed("Unused")));
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
  EXPECT_DEATH(evidenceEmitter([](const Evidence& E) {}, USRCache)(
                   *TargetDecl, Slot{}, Evidence::ANNOTATED_UNKNOWN,
                   TargetDecl->getLocation()),
               "not an inference target");
}

}  // namespace
}  // namespace clang::tidy::nullability
