// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:ignore_attr

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/annotations/ignore_attr.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct crubit::test::MyStruct) == 1);
static_assert(alignof(struct crubit::test::MyStruct) == 1);

extern "C" void __rust_thunk___ZN6crubit4test8MyStructB3fooC1Ev(
    struct crubit::test::MyStruct* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct crubit::test::PackedStruct) == 2);
static_assert(alignof(struct crubit::test::PackedStruct) == 1);
static_assert(CRUBIT_OFFSET_OF(x, struct crubit::test::PackedStruct) == 0);
static_assert(CRUBIT_OFFSET_OF(y, struct crubit::test::PackedStruct) == 1);

extern "C" void __rust_thunk___ZN6crubit4test12PackedStructC1Ev(
    struct crubit::test::PackedStruct* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct crubit::test::PointerStruct) == 4);
static_assert(alignof(struct crubit::test::PointerStruct) == 4);
static_assert(CRUBIT_OFFSET_OF(x, struct crubit::test::PointerStruct) == 0);

extern "C" void __rust_thunk___ZN6crubit4test13PointerStructC1Ev(
    struct crubit::test::PointerStruct* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
