// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest, ArrayArgumentSyntax) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    // Though declared as arrays, these parameters are actually pointers.
    // Their nullability syntax is unusual.
    void target(int unknown[], int nonnull[_Nonnull], int nullable[_Nullable]) {
      *unknown;
      *nonnull;
      *nullable;  // [[unsafe]]
    }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability
