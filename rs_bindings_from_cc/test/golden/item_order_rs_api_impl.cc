// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include "rs_bindings_from_cc/test/golden/item_order.h"
extern "C" int __rust_thunk__first_func() { return first_func(); }
extern "C" int __rust_thunk__second_func() { return second_func(); }
static_assert(sizeof(FirstStruct) == 4);
static_assert(alignof(FirstStruct) == 4);
static_assert(offsetof(FirstStruct, field) * 8 == 0);
static_assert(sizeof(SecondStruct) == 4);
static_assert(alignof(SecondStruct) == 4);
static_assert(offsetof(SecondStruct, field) * 8 == 0);