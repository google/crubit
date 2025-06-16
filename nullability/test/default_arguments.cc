// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for nullability correctness of default arguments.

#include "nullability/test/check_diagnostics.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest, DefaultArgNonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull = nullptr /* [[unsafe]] */);
  )cc"));
}

TEST(PointerNullabilityTest, DefaultArgNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable = nullptr);
  )cc"));
}

TEST(PointerNullabilityTest, DefaultArgUnannotated) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* = nullptr);
  )cc"));
}

TEST(PointerNullabilityTest, DefaultArgNonnullValueNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nullable p;
    void target(int *_Nonnull = p /* [[unsafe]] */);
  )cc"));
}

TEST(PointerNullabilityTest, DefaultArgNonnullValueNonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull p;
    void target(int *_Nonnull = p);
  )cc"));
}

TEST(PointerNullabilityTest, DefaultArgNonnullValueUnannotated) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* p;
    void target(int* _Nonnull = p);
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability
