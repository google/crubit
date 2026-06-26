// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/function/special_linking:special_linking

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/function/special_linking/special_linking.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert((void (*)()) & ::min_android_version_9999);

static_assert(CRUBIT_SIZEOF(struct ::SimpleStruct) == 4);
static_assert(alignof(struct ::SimpleStruct) == 4);
static_assert(CRUBIT_OFFSET_OF(x, struct ::SimpleStruct) == 0);

extern "C" void __rust_thunk___ZN12SimpleStructC1Ev(
    struct ::SimpleStruct* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct ::OtherStruct) == 4);
static_assert(alignof(struct ::OtherStruct) == 4);
static_assert(CRUBIT_OFFSET_OF(y, struct ::OtherStruct) == 0);

extern "C" void __rust_thunk___ZN11OtherStructC1Ev(
    struct ::OtherStruct* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__my_asm_conflict_func(
    struct ::SimpleStruct* __return) {
  new (__return) auto(::my_asm_conflict_func1());
}

static_assert((struct ::SimpleStruct (*)()) & ::my_asm_conflict_func1);

#pragma clang diagnostic pop
