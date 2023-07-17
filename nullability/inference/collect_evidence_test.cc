// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/collect_evidence.h"

#include <string>
#include <utility>
#include <vector>

#include "nullability/inference/inference.proto.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/Basic/LLVM.h"
#include "clang/Testing/TestAST.h"
#include "third_party/llvm/llvm-project/clang/unittests/Analysis/FlowSensitive/TestingSupport.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/raw_ostream.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"  // IWYU pragma: keep
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {
using ::testing::ElementsAre;
using ::testing::IsEmpty;
using ::testing::UnorderedElementsAre;

MATCHER_P3(isEvidenceMatcher, S, Kind, SymbolMatcher, "") {
  return arg.slot() == S && arg.kind() == Kind &&
         SymbolMatcher.Matches(arg.symbol());
}

testing::Matcher<const Evidence&> evidence(
    Slot S, Evidence::Kind Kind,
    testing::Matcher<const Symbol&> SymbolMatcher = testing::_) {
  return isEvidenceMatcher(S, Kind, SymbolMatcher);
}

MATCHER_P(functionNamed, Name, "") {
  return llvm::StringRef(arg.usr()).contains(
      ("@" + llvm::StringRef(Name) + "#").str());
}

std::vector<Evidence> collectEvidenceFromTargetFunction(
    llvm::StringRef Source) {
  std::vector<Evidence> Results;
  clang::TestAST AST(Source);
  auto Err = collectEvidenceFromImplementation(
      cast<FunctionDecl>(
          *dataflow::test::findValueDecl(AST.context(), "target")),
      evidenceEmitter([&](const Evidence& E) { Results.push_back(E); }));
  if (Err) ADD_FAILURE() << toString(std::move(Err));
  return Results;
}

std::vector<Evidence> collectEvidenceFromTargetDecl(llvm::StringRef Source) {
  std::vector<Evidence> Results;
  clang::TestInputs Inputs = Source;
  Inputs.ExtraFiles["nullability.h"] = R"cc(
    template <typename T>
    using Nullable [[clang::annotate("Nullable")]] = T;
    template <typename T>
    using Nonnull [[clang::annotate("Nonnull")]] = T;
  )cc";
  Inputs.ExtraArgs.push_back("-include");
  Inputs.ExtraArgs.push_back("nullability.h");
  clang::TestAST AST(Inputs);
  collectEvidenceFromTargetDeclaration(
      *dataflow::test::findValueDecl(AST.context(), "target"),
      evidenceEmitter([&](const Evidence& E) { Results.push_back(E); }));
  return Results;
}

TEST(InferAnnotationsTest, NoParams) {
  static constexpr llvm::StringRef Src = R"cc(
    void target() {}
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src), IsEmpty());
}

TEST(InferAnnotationsTest, OneParamUnused) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p0) {}
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src), IsEmpty());
}

TEST(InferAnnotationsTest, OneParamUsedWithoutRestriction) {
  static constexpr llvm::StringRef Src = R"cc(
    void takesUnknown(int *unknown) {}

    void target(int *p0) { takesUnknown(p0); }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src), IsEmpty());
}

TEST(InferAnnotationsTest, Deref) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p0, int *p1) {
      int a = *p0;
      if (p1 != nullptr) {
        int b = *p1;
      }
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(InferAnnotationsTest, DereferenceBeforeAssignment) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p) {
      *p;
      int i = 1;
      p = &i;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(InferAnnotationsTest, DereferenceAfterAssignment) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p) {
      int i = 1;
      p = &i;
      *p;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src), IsEmpty());
}

TEST(InferAnnotationsTest, DerefOfPtrRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *&p0, int *&p1) {
      int a = *p0;
      if (p1 != nullptr) {
        int b = *p1;
      }
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(InferAnnotationsTest, UnrelatedCondition) {
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
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE),
                  evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE),
                  // We collect two Evidence values for two dereferences of p0
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE),
                  evidence(paramSlot(2), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(InferAnnotationsTest, LaterDeref) {
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
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(InferAnnotationsTest, DerefBeforeGuardedDeref) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p0) {
      int a = *p0;
      if (p0 != nullptr) {
        int b = *p0;
      }
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(InferAnnotationsTest, EarlyReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p0) {
      if (!p0) {
        return;
      }
      int a = *p0;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src), IsEmpty());
}

TEST(InferAnnotationsTest, UnreachableCode) {
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
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(InferAnnotationsTest, VariableDeclIgnored) {
  llvm::StringLiteral Src = "Nullable<int *> target;";
  EXPECT_THAT(collectEvidenceFromTargetDecl(Src), IsEmpty());
}

TEST(InferAnnotationsTest, FunctionDeclReturnType) {
  llvm::StringLiteral Src = "Nonnull<int *> target();";
  EXPECT_THAT(
      collectEvidenceFromTargetDecl(Src),
      ElementsAre(evidence(SLOT_RETURN_TYPE, Evidence::ANNOTATED_NONNULL,
                           functionNamed("target"))));
}

TEST(InferAnnotationsTest, FunctionDeclParams) {
  llvm::StringLiteral Src = "void target(Nullable<int*>, int*, Nonnull<int*>);";
  EXPECT_THAT(collectEvidenceFromTargetDecl(Src),
              ElementsAre(evidence(paramSlot(0), Evidence::ANNOTATED_NULLABLE),
                          evidence(paramSlot(2), Evidence::ANNOTATED_NONNULL)));
}

TEST(InferAnnotationsTest, FunctionDeclNonTopLevel) {
  llvm::StringLiteral Src = "Nonnull<int*>** target(Nullable<int*>*);";
  EXPECT_THAT(collectEvidenceFromTargetDecl(Src), IsEmpty());
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
    auto Lambda = []() {};  // Not analyzed yet.

    struct S {
      void member();
    };
    void S::member() {}
  )cc");
  auto Sites = EvidenceSites::discover(AST.context());
  EXPECT_THAT(Sites.Declarations,
              ElementsAre(declNamed("foo"), declNamed("bar"), declNamed("bar"),
                          declNamed("baz"), declNamed("S::member"),
                          declNamed("S::member")));
  EXPECT_THAT(
      Sites.Implementations,
      ElementsAre(declNamed("bar"), declNamed("baz"), declNamed("S::member")));
}

TEST(EvidenceSitesTest, Variables) {
  TestAST AST(R"cc(
    int* x = true ? nullptr : nullptr;
    struct S {
      int* s;
    };
  )cc");
  auto Sites = EvidenceSites::discover(AST.context());
  // For now, variables are not inferrable.
  EXPECT_THAT(Sites.Declarations, IsEmpty());
  // For now, we don't examine variable initializers.
  EXPECT_THAT(Sites.Implementations, IsEmpty());
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

    auto Unused = f<0>() + f<1>() + S{}.f<0>() + T<0>{}.f();
  )cc");
  auto Sites = EvidenceSites::discover(AST.context());

  // Relevant declarations are the written ones.
  EXPECT_THAT(Sites.Declarations,
              ElementsAre(declNamed("f"), declNamed("f<1>"), declNamed("S::f"),
                          declNamed("T::f")));
  // Instantiations are relevant inference targets.
  EXPECT_THAT(Sites.Implementations,
              ElementsAre(declNamed("f<0>"), declNamed("f<1>"),
                          declNamed("S::f<0>"), declNamed("T<0>::f")));
}

}  // namespace
}  // namespace clang::tidy::nullability
