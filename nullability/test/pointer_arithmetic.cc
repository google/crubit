// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for pointer arithmetic.

#include "nullability/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest, PointerArithmetic) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable p, int *_Nonnull q, int *r) {
      *++p;  // [[unsafe]]
      *p++;  // [[unsafe]]
      *--p;  // [[unsafe]]
      *p--;  // [[unsafe]]
      *+p;   // [[unsafe]]

      *++q;
      *q++;
      *--q;
      *q--;
      *+q;

      *++r;
      *r++;
      *--r;
      *r--;
      *+r;
    }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability
