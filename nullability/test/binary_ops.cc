// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for control flow involving binary operators.

#include "nullability_test.h"

TEST void testAnd(int *_Nullable X, int *_Nullable Y) {
  if (X && Y) {
    nonnull(X);
    nonnull(Y);
    // Type hasn't changed, even though we know X and Y are nonnull.
    type<int *_Nullable>(X);
    type<int *_Nullable>(Y);
  } else {
    nullable(X);
    nullable(Y);
  }
  nullable(X);
  nullable(Y);
}

TEST void testOr(int *_Nullable X, int *_Nullable Y) {
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

TEST void testNeither(int *_Nullable X, int *_Nullable Y) {
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

TEST void testNotBoth(int *_Nullable X, int *_Nullable Y) {
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

TEST void testComma(int *_Nonnull X, int *_Nullable Y) {
  int *Z = (X, Y);
  nullable(Z);

  type<int *_Nullable>((X, Y));
}
