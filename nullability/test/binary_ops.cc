// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for control flow involving binary operators.

#include "nullability_test.h"

TEST void testAnd(Nullable<int *> X, Nullable<int *> Y) {
  if (X && Y) {
    nonnull(X);
    nonnull(Y);
    // Type hasn't changed, even though we know X and Y are nonnull.
    type<Nullable<int *>>(X);
    type<Nullable<int *>>(Y);
  } else {
    nullable(X);
    nullable(Y);
  }
  nullable(X);
  nullable(Y);
}

TEST void testOr(Nullable<int *> X, Nullable<int *> Y) {
  if (X || Y) {
    nullable(X);
    nullable(Y);
  } else {
    nullable(X);
    nullable(Y);
  }
  nullable(X);
  nullable(Y);
}

TEST void testNeither(Nullable<int *> X, Nullable<int *> Y) {
  if (!X && !Y) {
    nullable(X);
    nullable(Y);
  } else {
    nullable(X);
    nullable(Y);
  }
  nullable(X);
  nullable(Y);
}

TEST void testNotBoth(Nullable<int *> X, Nullable<int *> Y) {
  if (!X || !Y) {
    nullable(X);
    nullable(Y);
  } else {
    nonnull(X);
    nonnull(Y);
  }
  nullable(X);
  nullable(Y);
}
