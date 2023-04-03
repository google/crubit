// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for binary operators.

#include "nullability_verification/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {
namespace {

TEST(PointerNullabilityTest, BinaryExpressions) {
  // x && y
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nullable y) {
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
      if (x && y) {
        *x;
        *y;
      } else {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
    }
  )cc"));

  // x || y
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nullable y) {
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
      if (x || y) {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      } else {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
    }
  )cc"));

  // !x && !y
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nullable y) {
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
      if (!x && !y) {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      } else {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
    }
  )cc"));

  // !x || !y
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nullable y) {
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
      if (!x || !y) {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      } else {
        *x;
        *y;
      }
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
    }
  )cc"));
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
