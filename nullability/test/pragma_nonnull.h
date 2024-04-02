// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "pragma_support.h"

#pragma nullability file_default nonnull

namespace pragma_nonnull {
using IntPtr = int*;

template <class T>
using Pointer = T*;

using IntPtrVec = Vec<int*>;

void maybeMutatePointer(int*&);
}  // namespace pragma_nonnull
