// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_UNSAFE_ATTRIBUTES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_UNSAFE_ATTRIBUTES_H_

#include "support/annotations.h"

struct SafeStructUnannotated {};

struct CRUBIT_UNSAFE SafeStructAnnotatedUnsafe {};

struct UnsafeStructUnannotated {
 public:
  void* ptr = nullptr;
};

struct CRUBIT_UNSAFE_MARK_SAFE UnsafeStructAnnotatedSafe {
 public:
  void* ptr = nullptr;
};

void UseSafeStructUnannotated(SafeStructUnannotated s);
void UseSafeStructAnnotatedUnsafe(SafeStructAnnotatedUnsafe s);

void UseUnsafeStructUnannotated(UnsafeStructUnannotated s);
void UseUnsafeStructAnnotatedSafe(UnsafeStructAnnotatedSafe s);

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_UNSAFE_ATTRIBUTES_H_
