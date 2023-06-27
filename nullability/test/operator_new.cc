// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for operator new.

#include "nullability/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest, ThrowingNew) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {
      int *p = new int;
      *p;
      delete p;
    }
  )cc"));
}

TEST(PointerNullabilityTest, AssignFromNewMakesNullableNonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {
      int *_Nullable p = nullptr;
      p = new int;
      *p;
    }
  )cc"));
}

TEST(PointerNullabilityTest, NoThrowNew) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <new>
    void target() {
      int *p = new (std::nothrow) int;
      *p;  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, AssignFromNoThrowNewMakesNonnullNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <new>
    void target() {
      int i = 0;
      int *_Nonnull p = &i;
      p = new (std::nothrow) int;
      *p;  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, NewPreservesNullabilityOnAllocatedType) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <new>
    void target() {
      __assert_nullability<NK_nonnull, NK_nonnull>(new (int *_Nonnull));
      __assert_nullability<NK_nonnull, NK_nullable>(new (int *_Nullable));
      __assert_nullability<NK_nullable, NK_nonnull>(
          new (std::nothrow)(int *_Nonnull));
      __assert_nullability<NK_nullable, NK_nullable>(
          new (std::nothrow)(int *_Nullable));
    }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability
