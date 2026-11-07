// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for the shared, analyzer-only `ABSL_ANALYZER_ASSUME` primitive and its
// use by error-checking macros. `ABSL_ANALYZER_ASSUME(cond)` feeds `cond`
// directly to the dataflow analysis; error-checking macros emit it on their
// success path so a later use of the checked pointer is not flagged as a
// possible null dereference.

#include <memory>

#include "check.h"
#include "nullability_test.h"

// The primitive itself: assuming `p != nullptr` makes `p` non-null.
TEST void assumeNotNullMakesNonnull(int* _Nullable P) {
  ABSL_ANALYZER_ASSUME(P != nullptr);
  nonnull(P);
}

// The comparison may be written in either order.
TEST void assumeNotNullRightForm(int* _Nullable P) {
  ABSL_ANALYZER_ASSUME(nullptr != P);
  nonnull(P);
}

// Regression guard: WITHOUT the assume, the pointer stays nullable. If the
// primitive were (incorrectly) modeled as unconditionally establishing
// non-nullness, this assertion would fail.
TEST void withoutAssumeStaysNullable(int* _Nullable P) { nullable(P); }

// Works for smart pointers too: the comparison is modeled as the smart
// pointer's null state, which the assume then forces.
TEST void assumeNotNullSmartPointer(std::unique_ptr<int> P) {
  ABSL_ANALYZER_ASSUME(P != nullptr);
  nonnull(P);
}

// End-to-end through a check macro: `CAR_CHECK_NE` expands (under the analyzer)
// to the shared primitive on its success path, so `P` is non-null afterwards.
TEST void carCheckNeMakesNonnull(int* _Nullable P) {
  CAR_CHECK_NE(P, nullptr);
  nonnull(P);
}

TEST void carCheckNeRightForm(int* _Nullable P) {
  CAR_CHECK_NE(nullptr, P);
  nonnull(P);
}
