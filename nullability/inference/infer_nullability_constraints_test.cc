// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/infer_nullability_constraints.h"

#include <string>
#include <utility>
#include <vector>

#include "absl/log/check.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/proto_matchers.h"
#include "clang/AST/Decl.h"
#include "clang/AST/Expr.h"
#include "clang/Analysis/CFG.h"
#include "clang/Basic/LLVM.h"
#include "clang/Testing/TestAST.h"
#include "third_party/llvm/llvm-project/clang/unittests/Analysis/FlowSensitive/TestingSupport.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

using ::testing::IsEmpty;
using ::testing::Pair;
using ::testing::UnorderedElementsAre;

using InferredNullabilityConstraints =
    std::vector<std::pair<std::string, NullabilityConstraint>>;

InferredNullabilityConstraints inferAnnotationsForTargetFunction(
    llvm::StringRef Source) {
  clang::TestAST AST(Source);
  auto DeclResults = inferNullabilityConstraints(
      cast<FunctionDecl>(
          *dataflow::test::findValueDecl(AST.context(), "target")),
      AST.context());
  CHECK(DeclResults) << toString(DeclResults.takeError());
  InferredNullabilityConstraints StringResults;
  for (const auto &Entry : *DeclResults)
    StringResults.emplace_back(Entry.first->getName(), Entry.second);
  return StringResults;
}

TEST(InferAnnotationsTest, NoParams) {
  static constexpr llvm::StringRef Src = R"cc(
    void target() {}
  )cc";
  EXPECT_THAT(inferAnnotationsForTargetFunction(Src), IsEmpty());
}

TEST(InferAnnotationsTest, OneParamUnused) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p0) {}
  )cc";
  EXPECT_THAT(inferAnnotationsForTargetFunction(Src), IsEmpty());
}

TEST(InferAnnotationsTest, OneParamUsedWithoutRestriction) {
  static constexpr llvm::StringRef Src = R"cc(
    void takesUnknown(int *unknown) {}

    void target(int *p0) { takesUnknown(p0); }
  )cc";
  EXPECT_THAT(inferAnnotationsForTargetFunction(Src),
              UnorderedElementsAre(
                  Pair("p0", EqualsProto(R"pb(must_be_nonnull: false)pb"))));
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
  EXPECT_THAT(inferAnnotationsForTargetFunction(Src),
              UnorderedElementsAre(
                  Pair("p0", EqualsProto(R"pb(must_be_nonnull: true)pb")),
                  Pair("p1", EqualsProto(R"pb(must_be_nonnull: false)pb"))));
}

TEST(InferAnnotationsTest, DereferenceBeforeAssignment) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p) {
      *p;
      int i = 1;
      p = &i;
    }
  )cc";
  EXPECT_THAT(inferAnnotationsForTargetFunction(Src),
              UnorderedElementsAre(Pair("p", EqualsProto(R"pb(must_be_nonnull:
                                                                  true)pb"))));
}

TEST(InferAnnotationsTest, DereferenceAfterAssignment) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p) {
      int i = 1;
      p = &i;
      *p;
    }
  )cc";
  EXPECT_THAT(inferAnnotationsForTargetFunction(Src),
              UnorderedElementsAre(
                  Pair("p", EqualsProto(R"pb(must_be_nonnull: false)pb"))));
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
  EXPECT_THAT(inferAnnotationsForTargetFunction(Src),
              UnorderedElementsAre(
                  Pair("p0", EqualsProto(R"pb(must_be_nonnull: true)pb")),
                  Pair("p1", EqualsProto(R"pb(must_be_nonnull: false)pb"))));
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
  EXPECT_THAT(inferAnnotationsForTargetFunction(Src),
              UnorderedElementsAre(
                  Pair("p0", EqualsProto(R"pb(must_be_nonnull: true)pb")),
                  Pair("p1", EqualsProto(R"pb(must_be_nonnull: true)pb")),
                  Pair("p2", EqualsProto(R"pb(must_be_nonnull: true)pb"))));
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
  EXPECT_THAT(inferAnnotationsForTargetFunction(Src),
              UnorderedElementsAre(
                  Pair("p0", EqualsProto(R"pb(must_be_nonnull: true)pb"))));
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
  EXPECT_THAT(inferAnnotationsForTargetFunction(Src),
              UnorderedElementsAre(
                  Pair("p0", EqualsProto(R"pb(must_be_nonnull: true)pb"))));
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
  EXPECT_THAT(inferAnnotationsForTargetFunction(Src),
              UnorderedElementsAre(
                  Pair("p0", EqualsProto(R"pb(must_be_nonnull: false)pb"))));
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
  EXPECT_THAT(inferAnnotationsForTargetFunction(Src),
              UnorderedElementsAre(
                  Pair("p0", EqualsProto(R"pb(must_be_nonnull: true)pb"))));
}

TEST(InferAnnotationsTest,
     RequiresNonNullWhenAnnotatedWithClangNullabilityAttributeAtDefinition) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *unannotated, int *_Nonnull non_null,
                int *_Nonnull *only_inner_layer_nonnull) {
      unannotated;
      non_null;
      only_inner_layer_nonnull;
    }
  )cc";
  EXPECT_THAT(
      inferAnnotationsForTargetFunction(Src),
      UnorderedElementsAre(
          Pair("unannotated", EqualsProto(R"pb(must_be_nonnull: false)pb")),
          Pair("non_null", EqualsProto(R"pb(must_be_nonnull: true)pb")),
          Pair("only_inner_layer_nonnull", EqualsProto(R"pb(must_be_nonnull:
                                                                false)pb"))));
}

TEST(InferAnnotationsTest,
     RequiresNonNullWhenAnnotatedWithClangAnnotationAtDefinition) {
  static constexpr llvm::StringRef Src = R"cc(
    namespace custom {
    template <class T>
    using NonNull [[clang::annotate("Nonnull")]] = T;
    }  // namespace custom

    void target(int *unannotated, custom::NonNull<int *> non_null,
                custom::NonNull<int *> *only_inner_layer_nonnull) {
      unannotated;
      non_null;
      only_inner_layer_nonnull;
    }
  )cc";
  EXPECT_THAT(
      inferAnnotationsForTargetFunction(Src),
      UnorderedElementsAre(
          Pair("unannotated", EqualsProto(R"pb(must_be_nonnull: false)pb")),
          Pair("non_null", EqualsProto(R"pb(must_be_nonnull: true)pb")),
          Pair("only_inner_layer_nonnull", EqualsProto(R"pb(must_be_nonnull:
                                                                false)pb"))));
}

}  // namespace
}  // namespace clang::tidy::nullability
