// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/test/golden/item_order.h"

extern "C" void __rust_thunk___ZN11FirstStructC1Ev(class FirstStruct* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN11FirstStructC1ERKS_(
    class FirstStruct* __this, const class FirstStruct& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN11FirstStructD1Ev(class FirstStruct* __this) {
  std ::destroy_at(__this);
}
extern "C" class FirstStruct& __rust_thunk___ZN11FirstStructaSERKS_(
    class FirstStruct* __this, const class FirstStruct& __param_0) {
  return __this->operator=(__param_0);
}
extern "C" int __rust_thunk___Z10first_funcv() { return first_func(); }
extern "C" void __rust_thunk___ZN12SecondStructC1Ev(
    class SecondStruct* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN12SecondStructC1ERKS_(
    class SecondStruct* __this, const class SecondStruct& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN12SecondStructD1Ev(
    class SecondStruct* __this) {
  std ::destroy_at(__this);
}
extern "C" class SecondStruct& __rust_thunk___ZN12SecondStructaSERKS_(
    class SecondStruct* __this, const class SecondStruct& __param_0) {
  return __this->operator=(__param_0);
}
extern "C" int __rust_thunk___Z11second_funcv() { return second_func(); }

static_assert(sizeof(class FirstStruct) == 4);
static_assert(alignof(class FirstStruct) == 4);
static_assert(offsetof(class FirstStruct, field) * 8 == 0);

static_assert(sizeof(class SecondStruct) == 4);
static_assert(alignof(class SecondStruct) == 4);
static_assert(offsetof(class SecondStruct, field) * 8 == 0);
