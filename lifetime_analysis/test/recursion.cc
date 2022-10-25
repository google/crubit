// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests involving recursion.

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_analysis/test/lifetime_analysis_test.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

TEST_F(LifetimeAnalysisTest, InfiniteDirectRecursion) {
  // TODO(danakj): Infinite recursion is UB, so we would like to avoid that we
  // call an opaque function that is able to break the recursion (by exiting the
  // program, theoretically).
  EXPECT_THAT(GetLifetimes(R"(
    void opaque();
    int* f(int* a) {
      // TODO(danakj): opaque();
      return f(a);
    }
  )"),
              LifetimesAre({{"f", "a -> static"}}));
}

TEST_F(LifetimeAnalysisTest, FiniteDirectRecursion_1Pointee) {
  EXPECT_THAT(GetLifetimes(R"(
    int* f(int n, int* a) {
      if (n <= 0) return a;
      return f(n - 1, a);
    }
  )"),
              LifetimesAre({{"f", "(), a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, FiniteDirectRecursion_2Pointees) {
  EXPECT_THAT(GetLifetimes(R"(
    int* f(int n, int* a, int* b) {
      if (n <= 0) return a;
      return f(n - 1, b, a);
    }
  )"),
              LifetimesAre({{"f", "(), a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, FiniteDirectRecursion_3Pointees) {
  EXPECT_THAT(GetLifetimes(R"(
    int* f(int n, int* a, int* b, int *c) {
      if (n <= 0) return a;
      return f(n - 1, b, c, a);
    }
  )"),
              LifetimesAre({{"f", "(), a, a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, MutualFiniteRecursion) {
  EXPECT_THAT(GetLifetimes(R"(
    int* g(int n, int* a);
    int* f(int n, int* a) {
      if (n == 0) return a;
      return g(n - 1, a);
    }
    int* g(int n, int* a) {
      if (n == 0) return a;
      return f(n - 1, a);
    }
  )"),
              LifetimesAre({{"f", "(), a -> a"}, {"g", "(), a -> a"}}));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
