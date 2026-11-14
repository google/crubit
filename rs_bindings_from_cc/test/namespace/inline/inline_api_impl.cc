// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/namespace/inline:inline

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/namespace/inline/inline.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"

static_assert(CRUBIT_SIZEOF(struct foo::MyStruct) == 4);
static_assert(alignof(struct foo::MyStruct) == 4);
static_assert(CRUBIT_OFFSET_OF(value, struct foo::MyStruct) == 0);

extern "C" void __rust_thunk___ZN3foo7inline18MyStructC1Ev(
    struct foo::MyStruct* __this) {
  crubit::construct_at(__this);
}

extern "C" int __rust_thunk___ZN3foo7inline115GetStructValue1EPKNS0_8MyStructE(
    struct foo::MyStruct const* s) {
  return foo::GetStructValue1(s);
}

static_assert((int (*)(struct foo::MyStruct const*)) & ::foo::GetStructValue1);

extern "C" int __rust_thunk___ZN3foo7inline115GetStructValue2EPKNS0_8MyStructE(
    struct foo::MyStruct const* s) {
  return foo::GetStructValue2(s);
}

static_assert((int (*)(struct foo::MyStruct const*)) & ::foo::GetStructValue2);

extern "C" int __rust_thunk___ZN3foo7inline115GetStructValue3EPKNS0_8MyStructE(
    struct foo::MyStruct const* s) {
  return foo::GetStructValue3(s);
}

static_assert((int (*)(struct foo::MyStruct const*)) & ::foo::GetStructValue3);

extern "C" int __rust_thunk___ZN3foo7inline115GetStructValue4EPKNS0_8MyStructE(
    struct foo::MyStruct const* s) {
  return foo::GetStructValue4(s);
}

static_assert((int (*)(struct foo::MyStruct const*)) & ::foo::GetStructValue4);

#pragma clang diagnostic pop
