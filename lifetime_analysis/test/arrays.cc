// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests involving arrays.

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_analysis/test/lifetime_analysis_test.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

TEST_F(LifetimeAnalysisTest, ArrayOfInts) {
  EXPECT_THAT(GetLifetimes(R"(
    void target() {
      int x[] = {0};
    }
  )"),
              LifetimesAre({{"target", ""}}));
}

TEST_F(LifetimeAnalysisTest, ArrayMergesLifetimes) {
  EXPECT_THAT(GetLifetimes(R"(
    void target(int** array, int* p, int* q) {
      array[0] = p;
      array[1] = q;
    }
  )"),
              LifetimesAre({{"target", "(a, b), a, a"}}));
}

TEST_F(LifetimeAnalysisTest, ArrayOfStructsMergesLifetimes) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* i;
    };
    void target(S** array, S* p, S* q) {
      array[0] = p;
      array[1] = q;
    }
  )"),
              LifetimesAre({{"target", "(a, b, c), (a, b), (a, b)"}}));
}

TEST_F(LifetimeAnalysisTest, SimpleArray) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* a, int* b, unsigned x) {
      int* v[2];
      v[0] = a;
      v[1] = b;
      return v[x & 1];
    }
  )"),
              LifetimesAre({{"target", "a, a, () -> a"}}));
}

TEST_F(LifetimeAnalysisTest, SimpleArrayInit) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* a, int* b, unsigned x) {
      int* v[2] = {a, b};
      return v[x & 1];
    }
  )"),
              LifetimesAre({{"target", "a, a, () -> a"}}));
}

TEST_F(LifetimeAnalysisTest, SimpleArrayInitConstExprSubscriptIndex) {
  // There is a potential to track the lifetime of each array element
  // separately, when the array's size and subscript indices are known
  // statically. But is hard-to-impossible to do for all arrays. We treat an
  // array as a single object as a result, and merge the points-to sets of all
  // its elements.
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* a, int* b) {
      int* v[2] = {a, b};
      return v[0];
    }
  )"),
              LifetimesAre({{"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, SimpleArrayPtr) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* a, int* b, unsigned x) {
      int* v[2];
      *v = a;
      *(v + 1) = b;
      return *(v + (x & 1));
    }
  )"),
              LifetimesAre({{"target", "a, a, () -> a"}}));
}

TEST_F(LifetimeAnalysisTest, SimpleArrayFnCall) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* a, int* b, int** c, unsigned x) {
      *c = a;
      *(c + 1) = b;
      return *(c + (x & 1));
    }
  )"),
              LifetimesAre({{"target", "a, a, (a, b), () -> a"}}));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
