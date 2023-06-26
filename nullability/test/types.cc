// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for static types of declarations.

#include "nullability_test.h"

// Clang performs merging of nullability attributes on function parameter types.
// This isn't necessarily desirable: it only works with _Nullable, and only on
// pointer parameters, not return types, nested types, etc.
int *merged(int *, int *);
int *_Nullable merged(int *_Nullable, int *);
int *merged(int *, Nullable<int *>);
TEST int *merged(int *a, int *b) {
  type<Nullable<int *>>(a);   // _Nullable attributes are merged
  type<int *>(b);             // clang::annotate-based attributes are not merged
  type<int *>(merged(a, b));  // return types are not merged
}
