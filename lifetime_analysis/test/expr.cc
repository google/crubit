// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for various types of expressions.

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_analysis/test/lifetime_analysis_test.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

TEST_F(LifetimeAnalysisTest, IncrementAndDecrement) {
  EXPECT_THAT(GetLifetimes(R"(
    int* prefix_inc(int* p) {
      return ++p;
    }
    int* prefix_dec(int* p) {
      return --p;
    }
    int* postfix_inc(int* p) {
      return p++;
    }
    int* postfix_dec(int* p) {
      return p--;
    }
  )"),
              LifetimesAre({{"prefix_inc", "a -> a"},
                            {"prefix_dec", "a -> a"},
                            {"postfix_inc", "a -> a"},
                            {"postfix_dec", "a -> a"}}));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
