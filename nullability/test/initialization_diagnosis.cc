// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests that check nullability is transferred correctly across initializers.

#include "nullability/test/check_diagnostics.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
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

TEST(PointerNullabilityTest, InitializerList) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      int* _Nonnull p;
    };

    void target() {
      S{nullptr};       // [[unsafe]]
      S{.p = nullptr};  // [[unsafe]]

      S{new int};
      S{.p = new int};
    }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability
