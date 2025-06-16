// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for joining different nullability types.

#include "nullability_test.h"

int *_Nonnull makeNonnull();
int *_Nullable makeNullable();
int *makeUnknown();

bool cond();

TEST void conditionalOperator() {
  nonnull(cond() ? makeNonnull() : makeNonnull());
  nullable(cond() ? makeNonnull() : makeNullable());
  unknown(cond() ? makeNonnull() : makeUnknown());

  nullable(cond() ? makeNullable() : makeNonnull());
  nullable(cond() ? makeNullable() : makeNullable());
  nullable(cond() ? makeNullable() : makeUnknown());

  unknown(cond() ? makeUnknown() : makeNonnull());
  nullable(cond() ? makeUnknown() : makeNullable());
  unknown(cond() ? makeUnknown() : makeUnknown());
}
