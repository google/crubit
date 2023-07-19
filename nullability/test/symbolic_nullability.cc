// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability_test.h"

TEST void symbolicNullability(symbolic::X<int *> p, symbolic::Y<int *> q) {
  type<symbolic::X<int *>>(p);
  type<Nonnull<symbolic::X<int *> *>>(&p);
  type<symbolic::Y<int *>>(q);
}
