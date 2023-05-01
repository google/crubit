// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests that check nullability is transferred correctly across initializers.

#include "nullability/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {
namespace {

TEST(PointerNullabilityTest, TransitiveNullCheck) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable x) {
      int *y = x;
      *x;  // [[unsafe]]
      if (y) {
        *x;
      } else {
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable x) {
      int *y = x;
      *y;  // [[unsafe]]
      if (x) {
        *y;
      } else {
        *y;  // [[unsafe]]
      }
      *y;  // [[unsafe]]
    }
  )cc"));
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
