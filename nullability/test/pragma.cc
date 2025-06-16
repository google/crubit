// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for influence of pragma on effective nullability.

#include "nullability_test.h"
#include "pragma_none.h"
#include "pragma_nonnull.h"
#include "pragma_support.h"

TEST void testSimpleValues(pragma_none::IntPtr a, pragma_nonnull::IntPtr b) {
  unknown(a);
  nonnull(b);
}

TEST void testCompoundTypes(pragma_none::Pointer<pragma_nonnull::IntPtr> a,
                            pragma_nonnull::Pointer<pragma_none::IntPtr> b) {
  type<int* _Nonnull*>(a);
  type<int** _Nonnull>(b);
}

TEST void testMemberInstantiation(pragma_none::IntPtrVec& a,
                                  pragma_nonnull::IntPtrVec& b) {
  unknown(a.front());
  nonnull(b.front());
  unknown(*a.data);
  nonnull(*b.data);
}

TEST void testFunctionInstantiation(pragma_none::IntPtrVec& a,
                                    pragma_nonnull::IntPtrVec& b) {
  unknown(create<pragma_none::IntPtr>());
  nonnull(create<pragma_nonnull::IntPtr>());
}

TEST void testOutputParameters(int* _Nonnull x, int* _Nonnull y) {
  pragma_none::maybeMutatePointer(x);
  unknown(x);

  pragma_nonnull::maybeMutatePointer(y);
  // The ref-param is effectively annotated non-null, so the function is not
  // allowed to set it to null.
  nonnull(y);
}
