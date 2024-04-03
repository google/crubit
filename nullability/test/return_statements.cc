// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for return statements.

#include "nullability/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest, ReturnStatements) {
  // nonnull return type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull target() {
      return nullptr;  // [[unsafe]]
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull target(int *_Nonnull ptr_nonnull) {
      return ptr_nonnull;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull target(int *_Nullable ptr_nullable) {
      return ptr_nullable;  // [[unsafe]]
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull target(int *ptr_unannotated) {
      return ptr_unannotated;
    }
  )cc"));

  // nullable return type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nullable target() { return nullptr; }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nullable target(int *_Nonnull ptr_nonnull) {
      return ptr_nonnull;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nullable target(int *_Nullable ptr_nullable) {
      return ptr_nullable;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nullable target(int *ptr_unannotated) {
      return ptr_unannotated;
    }
  )cc"));

  // unannotated return type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *target() { return nullptr; }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *target(int *_Nonnull ptr_nonnull) {
      return ptr_nonnull;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *target(int *_Nullable ptr_nullable) {
      return ptr_nullable;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *target(int *ptr_unannotated) {
      return ptr_unannotated;
    }
  )cc"));

  // multiple return statements
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull target(bool b, int *_Nonnull ptr_nonnull) {
      if (b) {
        return nullptr;  // [[unsafe]]
      }
      return ptr_nonnull;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull target(int *_Nullable ptr_nullable,
                         int *_Nonnull ptr_nonnull) {
      if (ptr_nullable) {
        return ptr_nullable;
      }
      return ptr_nonnull;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull target(int *_Nullable ptr_nullable_1,
                         int *_Nullable ptr_nullable_2) {
      if (ptr_nullable_1) {
        return ptr_nullable_2;  // [[unsafe]]
      }
      return ptr_nullable_1;  // [[unsafe]]
    }
  )cc"));

  // return result of merging 2 pointer values
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull target(bool b, int i) {
      int *ptr;
      if (b) {
        ptr = &i;
      } else {
        ptr = nullptr;
      }
      return ptr;  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, NonPointerReturnType) {
  checkDiagnostics(R"cc(
    struct S {
      int *p;
      int *&target() { return p; }
    };
  )cc");

  checkDiagnostics(R"cc(
    struct S {
      int *_Nullable p;
      int *_Nonnull &target() {
        return p;  // [[unsafe]]
      }
    };
  )cc");
}

}  // namespace
}  // namespace clang::tidy::nullability
