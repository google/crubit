// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability_test.h"

TEST void symbolicNullability(symbolic::X<int *> P, symbolic::Y<int *> Q) {
  type<symbolic::X<int *>>(P);
  type<symbolic::X<int *> *_Nonnull>(&P);
  type<symbolic::Y<int *>>(Q);
}
