// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:item_order_cc

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/item_order.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN11FirstStructC1Ev(struct FirstStruct* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN11FirstStructC1EOS_(
    struct FirstStruct* __this, struct FirstStruct* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" int __rust_thunk___Z10first_funcv() { return first_func(); }
extern "C" void __rust_thunk___ZN12SecondStructC1Ev(
    struct SecondStruct* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN12SecondStructC1EOS_(
    struct SecondStruct* __this, struct SecondStruct* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" int __rust_thunk___Z11second_funcv() { return second_func(); }

static_assert(sizeof(struct FirstStruct) == 4);
static_assert(alignof(struct FirstStruct) == 4);
static_assert(CRUBIT_OFFSET_OF(field, struct FirstStruct) == 0);

static_assert(sizeof(struct SecondStruct) == 4);
static_assert(alignof(struct SecondStruct) == 4);
static_assert(CRUBIT_OFFSET_OF(field, struct SecondStruct) == 0);

#pragma clang diagnostic pop
