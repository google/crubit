// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/collect_evidence.h"

#include <utility>
#include <vector>

#include "nullability/inference/inference.proto.h"
#include "clang/AST/Decl.h"
#include "clang/Basic/LLVM.h"
#include "clang/Testing/TestAST.h"
#include "third_party/llvm/llvm-project/clang/unittests/Analysis/FlowSensitive/TestingSupport.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"  // IWYU pragma: keep
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {
using ::testing::HasSubstr;
using ::testing::IsEmpty;
using ::testing::UnorderedElementsAre;

MATCHER_P3(isEvidenceMatcher, SlotMatcher, ConstraintMatcher, SymbolMatcher,
           "") {
  return SlotMatcher.Matches(arg.slot()) &&
         ConstraintMatcher.Matches(arg.constraint()) &&
         SymbolMatcher.Matches(arg.symbol());
}

testing::Matcher<const Evidence&> evidence(
    testing::Matcher<const Slot&> SlotMatcher,
    testing::Matcher<const NullabilityConstraint&> ConstraintMatcher,
    testing::Matcher<const Symbol&> SymbolMatcher = testing::_) {
  return isEvidenceMatcher(SlotMatcher, ConstraintMatcher, SymbolMatcher);
}

MATCHER_P(functionNamed, Name, "") {
  return llvm::StringRef(arg.usr()).contains(
      ("@" + llvm::StringRef(Name) + "#").str());
}
MATCHER(returnType, "") { return arg.return_type(); }
MATCHER_P(param, I, "") { return arg.parameter() == I; }
MATCHER(mustBeNullable, "") { return arg.must_be_nullable(); }
MATCHER(mustBeNonnull, "") { return arg.must_be_nonnull(); }

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
              UnorderedElementsAre(evidence(param(0), mustBeNonnull())));
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
              UnorderedElementsAre(evidence(param(0), mustBeNonnull())));
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
              UnorderedElementsAre(evidence(param(0), mustBeNonnull())));
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
                  evidence(param(0), mustBeNonnull()),
                  evidence(param(1), mustBeNonnull()),
                  // We collect two Evidence values for two dereferences of p0
                  evidence(param(0), mustBeNonnull()),
                  evidence(param(2), mustBeNonnull())));
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
              UnorderedElementsAre(evidence(param(0), mustBeNonnull())));
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
              UnorderedElementsAre(evidence(param(0), mustBeNonnull())));
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
              UnorderedElementsAre(evidence(param(0), mustBeNonnull())));
}

TEST(InferAnnotationsTest, VariableDeclIgnored) {
  llvm::StringLiteral Src = "Nullable<int *> target;";
  EXPECT_THAT(collectEvidenceFromTargetDecl(Src), IsEmpty());
}

TEST(InferAnnotationsTest, FunctionDeclReturnType) {
  llvm::StringLiteral Src = "Nonnull<int *> target();";
  EXPECT_THAT(collectEvidenceFromTargetDecl(Src),
              ElementsAre(evidence(returnType(), mustBeNonnull(),
                                   functionNamed("target"))));
}

TEST(InferAnnotationsTest, FunctionDeclParams) {
  llvm::StringLiteral Src = "void target(Nullable<int*>, int*, Nonnull<int*>);";
  EXPECT_THAT(collectEvidenceFromTargetDecl(Src),
              ElementsAre(evidence(param(0), mustBeNullable()),
                          evidence(param(2), mustBeNonnull())));
}

TEST(InferAnnotationsTest, FunctionDeclNonTopLevel) {
  llvm::StringLiteral Src = "Nonnull<int*>** target(Nullable<int*>*);";
  EXPECT_THAT(collectEvidenceFromTargetDecl(Src), IsEmpty());
}

}  // namespace
}  // namespace clang::tidy::nullability
