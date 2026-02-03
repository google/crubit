// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/struct/unsafe_attributes/unsafe_attributes.h"

void UseSafeStructUnannotated(SafeStructUnannotated s) {}
void UseSafeStructAnnotatedUnsafe(SafeStructAnnotatedUnsafe s) {}
void UseUnsafeStructUnannotated(UnsafeStructUnannotated s) {}
void UseUnsafeStructAnnotatedSafe(UnsafeStructAnnotatedSafe s) {}
