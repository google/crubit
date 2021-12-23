// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>
#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/test/golden/item_order.h"

extern "C" void __rust_thunk___ZN11FirstStructC1Ev(FirstStruct* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN11FirstStructD1Ev(FirstStruct* __this) {
  std ::destroy_at(__this);
}
extern "C" int __rust_thunk___Z10first_funcv() { return first_func(); }
extern "C" void __rust_thunk___ZN12SecondStructC1Ev(SecondStruct* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN12SecondStructD1Ev(SecondStruct* __this) {
  std ::destroy_at(__this);
}
extern "C" int __rust_thunk___Z11second_funcv() { return second_func(); }

static_assert(sizeof(FirstStruct) == 4);
static_assert(alignof(FirstStruct) == 4);
static_assert(offsetof(FirstStruct, field) * 8 == 0);

static_assert(sizeof(SecondStruct) == 4);
static_assert(alignof(SecondStruct) == 4);
static_assert(offsetof(SecondStruct, field) * 8 == 0);
