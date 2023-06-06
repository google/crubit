// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for control flow involving binary operators.

#include "nullability_test.h"

TEST void testAnd(Nullable<int*> x, Nullable<int*> y) {
  if (x && y) {
    nonnull(x);
    nonnull(y);
    // Type hasn't changed, even though we know x and y are nonnull.
    type<Nullable<int*>>(x);
    type<Nullable<int*>>(y);
  } else {
    nullable(x);
    nullable(y);
  }
  nullable(x);
  nullable(y);
}

TEST void testOr(Nullable<int*> x, Nullable<int*> y) {
  if (x || y) {
    nullable(x);
    nullable(y);
  } else {
    nullable(x);
    nullable(y);
  }
  nullable(x);
  nullable(y);
}

TEST void testNeither(Nullable<int*> x, Nullable<int*> y) {
  if (!x && !y) {
    nullable(x);
    nullable(y);
  } else {
    nullable(x);
    nullable(y);
  }
  nullable(x);
  nullable(y);
}

TEST void testNotBoth(Nullable<int*> x, Nullable<int*> y) {
  if (!x || !y) {
    nullable(x);
    nullable(y);
  } else {
    nonnull(x);
    nonnull(y);
  }
  nullable(x);
  nullable(y);
}
