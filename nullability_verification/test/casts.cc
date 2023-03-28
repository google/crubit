// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for casts of types containing nullability annotations.

#include "nullability_verification/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {
namespace {

// TODO(b/233582219): Implement diagnosis of unreachable program points
TEST(PointerNullabilityTest, NonNullPtrImplicitCastToBool) {
  // x
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull x) {
      *x;
      if (x) {
        *x;
      } else {
        *x;  // unreachable
      }
      *x;
    }
  )cc"));

  // !x
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull x) {
      *x;
      if (!x) {
        *x;  // unreachable
      } else {
        *x;
      }
      *x;
    }
  )cc"));
}

TEST(PointerNullabilityTest, NullablePtrImplicitCastToBool) {
  // x
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x) {
      *x;  // [[unsafe]]
      if (x) {
        *x;
      } else {
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
    }
  )cc"));

  // !x
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x) {
      *x;  // [[unsafe]]
      if (!x) {
        *x;  // [[unsafe]]
      } else {
        *x;
      }
      *x;  // [[unsafe]]
    }
  )cc"));
}

// TODO(b/233582219): Fix false negatives. Casting the pointer to boolean is
// evidence of the author considering null a possibility, hence the unnannotated
// pointer should be considered nullable and emit warnings where it fails or is
// not null checked.
TEST(PointerNullabilityTest, UnknownPtrImplicitCastToBool) {
  // x
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x) {
      *x;  // false-negative
      if (x) {
        *x;
      } else {
        *x;  // false-negative
      }
      *x;  // false-negative
    }
  )cc"));

  // !x
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x) {
      *x;  // false-negative
      if (!x) {
        *x;  // false-negative
      } else {
        *x;
      }
      *x;  // false-negative
    }
  )cc"));
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
