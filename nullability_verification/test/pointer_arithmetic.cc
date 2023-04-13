// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for pointer arithmetic.

#include "nullability_verification/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {
namespace {

// TODO: fix false positives due to unsupported PointerValues in the framework.
TEST(PointerNullabilityTest, PointerArithmetic) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable p, int *_Nonnull q, int *r) {
      *++p;  // [[unsafe]]
      *p++;  // [[unsafe]]
      *--p;  // [[unsafe]]
      *p--;  // [[unsafe]]
      *+p;   // [[unsafe]]

      *++q;  // [[unsafe]] TODO: fix false positive
      *q++;  // [[unsafe]] TODO: fix false positive
      *--q;  // [[unsafe]] TODO: fix false positive
      *q--;  // [[unsafe]] TODO: fix false positive
      *+q;   // [[unsafe]] TODO: fix false positive

      *++r;  // [[unsafe]] TODO: fix false positive
      *r++;  // [[unsafe]] TODO: fix false positive
      *--r;  // [[unsafe]] TODO: fix false positive
      *r--;  // [[unsafe]] TODO: fix false positive
      *+r;   // [[unsafe]] TODO: fix false positive
    }
  )cc"));
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
