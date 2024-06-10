// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for pointer arithmetic.

#include "nullability/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest, PointerArithmeticNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable nullable, int i) {
      int *orig = nullable;

      // Operations without side-effects.
      *(nullable + i);  // [[unsafe]]
      *(nullable - i);  // [[unsafe]]
      *+nullable;       // [[unsafe]]

      // Operations with side-effects; these need to restore the original value
      // of `nullable` every time.
      *nullable++;  // [[unsafe]]
      nullable = orig;

      *++nullable;  // [[unsafe]]
      nullable = orig;

      *nullable--;  // [[unsafe]]
      nullable = orig;

      *--nullable;  // [[unsafe]]
      nullable = orig;

      // On a nullable pointer, the pointer arithmetic itself should already be
      // considered unsafe, unless we know that the offset is zero.
      nullable + i;  // TODO(b/321265696): False negative.
      nullable - i;  // TODO(b/321265696): False negative.
      nullable + 0;
      nullable - 0;

      // Unary `+` is safe on a nullable pointer.
      +nullable;

      ++nullable;  // TODO(b/321265696): False negative.
      nullable = orig;

      nullable++;  // TODO(b/321265696): False negative.
      nullable = orig;

      --nullable;  // TODO(b/321265696): False negative.
      nullable = orig;

      nullable--;  // TODO(b/321265696): False negative.
      nullable = orig;
    }
  )cc"));
}

TEST(PointerNullabilityTest, PointerArithmeticNonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull nonnull, int i) {
      int *orig = nonnull;

      *(nonnull + i);
      *(nonnull - i);
      *+nonnull;

      *++nonnull;
      nonnull = orig;

      *nonnull++;
      nonnull = orig;

      *--nonnull;
      nonnull = orig;

      *nonnull--;
      nonnull = orig;
    }
  )cc"));
}

TEST(PointerNullabilityTest, PointerArithmeticUnknown) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *unknown, int i) {
      int *orig = unknown;

      *(unknown + i);
      *(unknown - i);
      *+unknown;

      *++unknown;
      unknown = orig;

      *unknown++;
      unknown = orig;

      *--unknown;
      unknown = orig;

      *unknown--;
      unknown = orig;
    }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability
