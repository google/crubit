// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests involving lifetime parameters.

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_analysis/test/lifetime_analysis_test.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

TEST_F(LifetimeAnalysisTest, SimpleLifetimeParams) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* x;
    };

    S target(S s) {
      return s;
    }
  )"),
              LifetimesContain({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, LifetimeParamsMultiplePointers) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a", "b")]] S {
      [[clang::annotate("member_lifetimes", "a", "b")]]
      int** x;
    };

    S target(S s) {
      return s;
    }
  )"),
              LifetimesContain({{"target", "([a, b]) -> ([a, b])"}}));
}

TEST_F(LifetimeAnalysisTest, LifetimeParamsMultiplePointersMultipleMembers) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a", "b")]] S {
      [[clang::annotate("member_lifetimes", "a", "b")]]
      int** x;
      [[clang::annotate("member_lifetimes", "b", "a")]]
      int** y;
    };

    int** ret_x(S s) {
      return s.x;
    }

    int** ret_y(S s) {
      return s.y;
    }
  )"),
              LifetimesAre({{"ret_y", "([a, b]) -> (b, a)"},
                            {"ret_x", "([a, b]) -> (a, b)"}}));
}

TEST_F(LifetimeAnalysisTest, LifetimeParamsNested) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a", "b")]] T {
      [[clang::annotate("member_lifetimes", "a", "b")]]
      int** x;
    };

    struct [[clang::annotate("lifetime_params", "a", "b")]] S {
      [[clang::annotate("member_lifetimes", "b", "a")]]
      T t;
    };

    int** target(S s) {
      return s.t.x;
    }
  )"),
              LifetimesContain({{"target", "([a, b]) -> (b, a)"}}));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
