// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for static types of declarations.

#include "nullability_test.h"

bool cond();

// Define a version of the `Nullable` annotation that does not add the
// `_Nullable` attribute to `T`.
template <typename T>
using NullableWithoutAttribute [[clang::annotate("Nullable")]] = T;

// Clang performs merging of nullability attributes on function parameter types.
// This isn't necessarily desirable: it only works with _Nullable, and only on
// pointer parameters, not return types, nested types, etc.
int *merged(int *, int *);
int *_Nullable merged(int *_Nullable, int *);
int *merged(int *, NullableWithoutAttribute<int *>);
TEST int *merged(int *A, int *B) {
  type<Nullable<int *>>(A);  // _Nullable attributes are merged
  type<int *>(B);            // clang::annotate-based attributes are not merged
  // Put a condition in front of recursive call to prevent error message about
  // infinite recursion.
  if (cond()) type<int *>(merged(A, B));  // return types are not merged

  return nullptr;
}
