// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/infer_nullability_constraints.h"

#include <string>
#include <utility>

#include "absl/log/check.h"
#include "nullability/inference/analyze_target_for_test.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/proto_matchers.h"
#include "clang/AST/Decl.h"
#include "clang/AST/Expr.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/Analysis/CFG.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/STLFunctionalExtras.h"
#include "llvm/ADT/StringRef.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

using ::clang::ast_matchers::MatchFinder;
using ::testing::IsEmpty;
using ::testing::Pair;
using ::testing::UnorderedElementsAre;

using InferredNullabilityConstraints =
    llvm::DenseMap<const clang::Decl *, NullabilityConstraint>;

InferredNullabilityConstraints inferAnnotationsForTargetFunction(
    llvm::StringRef Source) {
  InferredNullabilityConstraints Constraints;
  analyzeTargetForTest(
      Source, [&Constraints](const clang::FunctionDecl &Func,
                             const MatchFinder::MatchResult &Result) mutable {
        auto Results = inferNullabilityConstraints(Func, *Result.Context);
        CHECK(Results);
        Constraints = std::move(*Results);
      });
  return Constraints;
}

MATCHER_P(Parameter, Name, std::string("is a parameter named ") + Name) {
  const auto *Param = clang::dyn_cast_or_null<clang::ParmVarDecl>(arg);
  if (!Param) {
    return false;
  }

  const auto *Identifier = Param->getIdentifier();
  if (!Identifier) {
    return false;
  }

  return testing::ExplainMatchResult(Name, Identifier->getName(),
                                     result_listener);
}

TEST(InferAnnotationsTest, NoParams) {
  static constexpr llvm::StringRef Src = R"cc(
    void Target() {}
  )cc";
  EXPECT_THAT(inferAnnotationsForTargetFunction(Src), IsEmpty());
}

TEST(InferAnnotationsTest, OneParamUnused) {
  static constexpr llvm::StringRef Src = R"cc(
    void Target(int *p0) {}
  )cc";
  EXPECT_THAT(inferAnnotationsForTargetFunction(Src), IsEmpty());
}

TEST(InferAnnotationsTest, OneParamUsedWithoutRestriction) {
  static constexpr llvm::StringRef Src = R"cc(
    void TakesUnknown(int *unknown) {}

    void Target(int *p0) { TakesUnknown(p0); }
  )cc";
  EXPECT_THAT(
      inferAnnotationsForTargetFunction(Src),
      UnorderedElementsAre(
          Pair(Parameter("p0"), EqualsProto(R"pb(must_be_nonnull: false)pb"))));
}

TEST(InferAnnotationsTest, Deref) {
  static constexpr llvm::StringRef Src = R"cc(
    void Target(int *p0, int *p1) {
      int a = *p0;
      if (p1 != nullptr) {
        int b = *p1;
      }
    }
  )cc";
  EXPECT_THAT(
      inferAnnotationsForTargetFunction(Src),
      UnorderedElementsAre(
          Pair(Parameter("p0"), EqualsProto(R"pb(must_be_nonnull: true)pb")),
          Pair(Parameter("p1"), EqualsProto(R"pb(must_be_nonnull: false)pb"))));
}

TEST(InferAnnotationsTest, DereferenceBeforeAssignment) {
  static constexpr llvm::StringRef Src = R"cc(
    void Target(int *p) {
      *p;
      int i = 1;
      p = &i;
    }
  )cc";
  EXPECT_THAT(
      inferAnnotationsForTargetFunction(Src),
      UnorderedElementsAre(
          Pair(Parameter("p"), EqualsProto(R"pb(must_be_nonnull: true)pb"))));
}

TEST(InferAnnotationsTest, DereferenceAfterAssignment) {
  static constexpr llvm::StringRef Src = R"cc(
    void Target(int *p) {
      int i = 1;
      p = &i;
      *p;
    }
  )cc";
  EXPECT_THAT(
      inferAnnotationsForTargetFunction(Src),
      UnorderedElementsAre(
          Pair(Parameter("p"), EqualsProto(R"pb(must_be_nonnull: false)pb"))));
}

TEST(InferAnnotationsTest, DerefOfPtrRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void Target(int *&p0, int *&p1) {
      int a = *p0;
      if (p1 != nullptr) {
        int b = *p1;
      }
    }
  )cc";
  EXPECT_THAT(
      inferAnnotationsForTargetFunction(Src),
      UnorderedElementsAre(
          Pair(Parameter("p0"), EqualsProto(R"pb(must_be_nonnull: true)pb")),
          Pair(Parameter("p1"), EqualsProto(R"pb(must_be_nonnull: false)pb"))));
}

TEST(InferAnnotationsTest, UnrelatedCondition) {
  static constexpr llvm::StringRef Src = R"cc(
    void Target(int *p0, int *p1, int *p2, bool b) {
      if (b) {
        int a = *p0;
        int b = *p1;
      } else {
        int a = *p0;
        int c = *p2;
      }
    }
  )cc";
  EXPECT_THAT(
      inferAnnotationsForTargetFunction(Src),
      UnorderedElementsAre(
          Pair(Parameter("p0"), EqualsProto(R"pb(must_be_nonnull: true)pb")),
          Pair(Parameter("p1"), EqualsProto(R"pb(must_be_nonnull: true)pb")),
          Pair(Parameter("p2"), EqualsProto(R"pb(must_be_nonnull: true)pb"))));
}

TEST(InferAnnotationsTest, LaterDeref) {
  static constexpr llvm::StringRef Src = R"cc(
    void Target(int *p0) {
      if (p0 == nullptr) {
        (void)0;
      } else {
        (void)0;
      }
      int a = *p0;
    }
  )cc";
  EXPECT_THAT(
      inferAnnotationsForTargetFunction(Src),
      UnorderedElementsAre(
          Pair(Parameter("p0"), EqualsProto(R"pb(must_be_nonnull: true)pb"))));
}

TEST(InferAnnotationsTest, DerefBeforeGuardedDeref) {
  static constexpr llvm::StringRef Src = R"cc(
    void Target(int *p0) {
      int a = *p0;
      if (p0 != nullptr) {
        int b = *p0;
      }
    }
  )cc";
  EXPECT_THAT(
      inferAnnotationsForTargetFunction(Src),
      UnorderedElementsAre(
          Pair(Parameter("p0"), EqualsProto(R"pb(must_be_nonnull: true)pb"))));
}

TEST(InferAnnotationsTest, EarlyReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    void Target(int *p0) {
      if (!p0) {
        return;
      }
      int a = *p0;
    }
  )cc";
  EXPECT_THAT(
      inferAnnotationsForTargetFunction(Src),
      UnorderedElementsAre(
          Pair(Parameter("p0"), EqualsProto(R"pb(must_be_nonnull: false)pb"))));
}

TEST(InferAnnotationsTest, UnreachableCode) {
  static constexpr llvm::StringRef Src = R"cc(
    void Target(int *p0, int *p1, int *p2, int *p3) {
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
  EXPECT_THAT(
      inferAnnotationsForTargetFunction(Src),
      UnorderedElementsAre(
          Pair(Parameter("p0"), EqualsProto(R"pb(must_be_nonnull: true)pb"))));
}

TEST(InferAnnotationsTest,
     RequiresNonNullWhenAnnotatedWithClangNullabilityAttributeAtDefinition) {
  static constexpr llvm::StringRef Src = R"cc(
    void Target(int *unannotated, int *_Nonnull non_null,
                int *_Nonnull *only_inner_layer_nonnull) {
      unannotated;
      non_null;
      only_inner_layer_nonnull;
    }
  )cc";
  EXPECT_THAT(inferAnnotationsForTargetFunction(Src),
              UnorderedElementsAre(
                  Pair(Parameter("unannotated"),
                       EqualsProto(R"pb(must_be_nonnull: false)pb")),
                  Pair(Parameter("non_null"), EqualsProto(R"pb(must_be_nonnull:
                                                                   true)pb")),
                  Pair(Parameter("only_inner_layer_nonnull"),
                       EqualsProto(R"pb(must_be_nonnull: false)pb"))));
}

TEST(InferAnnotationsTest,
     RequiresNonNullWhenAnnotatedWithClangAnnotationAtDefinition) {
  static constexpr llvm::StringRef Src = R"cc(
    namespace custom {
    template <class T>
    using NonNull [[clang::annotate("Nonnull")]] = T;
    }  // namespace custom

    void Target(int *unannotated, custom::NonNull<int *> non_null,
                custom::NonNull<int *> *only_inner_layer_nonnull) {
      unannotated;
      non_null;
      only_inner_layer_nonnull;
    }
  )cc";
  EXPECT_THAT(inferAnnotationsForTargetFunction(Src),
              UnorderedElementsAre(
                  Pair(Parameter("unannotated"),
                       EqualsProto(R"pb(must_be_nonnull: false)pb")),
                  Pair(Parameter("non_null"), EqualsProto(R"pb(must_be_nonnull:
                                                                   true)pb")),
                  Pair(Parameter("only_inner_layer_nonnull"),
                       EqualsProto(R"pb(must_be_nonnull: false)pb"))));
}

}  // namespace
}  // namespace clang::tidy::nullability
