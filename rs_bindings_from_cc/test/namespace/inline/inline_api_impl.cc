// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/namespace/inline:inline
// Features: infer_operator_lifetimes, non_unpin_ctor, std_unique_ptr, std_vector, supported

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/namespace/inline/inline.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct foo::inline1::MyStruct) == 4);
static_assert(alignof(struct foo::inline1::MyStruct) == 4);
static_assert(CRUBIT_OFFSET_OF(value, struct foo::inline1::MyStruct) == 0);

extern "C" void __rust_thunk___ZN3foo7inline18MyStructC1Ev(
    struct foo::inline1::MyStruct* __this) {
  crubit::construct_at(__this);
}

extern "C" int __rust_thunk___ZN3foo7inline115GetStructValue1EPKNS0_8MyStructE(
    struct foo::inline1::MyStruct const* s) {
  return foo::inline1::GetStructValue1(s);
}

static_assert((int (*)(
    struct foo::inline1::MyStruct const*))&foo::inline1::GetStructValue1);

extern "C" int __rust_thunk___ZN3foo7inline115GetStructValue2EPKNS0_8MyStructE(
    struct foo::inline1::MyStruct const* s) {
  return foo::inline1::GetStructValue2(s);
}

static_assert((int (*)(
    struct foo::inline1::MyStruct const*))&foo::inline1::GetStructValue2);

extern "C" int __rust_thunk___ZN3foo7inline115GetStructValue3EPKNS0_8MyStructE(
    struct foo::inline1::MyStruct const* s) {
  return foo::inline1::GetStructValue3(s);
}

static_assert((int (*)(
    struct foo::inline1::MyStruct const*))&foo::inline1::GetStructValue3);

extern "C" int __rust_thunk___ZN3foo7inline115GetStructValue4EPKNS0_8MyStructE(
    struct foo::inline1::MyStruct const* s) {
  return foo::inline1::GetStructValue4(s);
}

static_assert((int (*)(
    struct foo::inline1::MyStruct const*))&foo::inline1::GetStructValue4);

#pragma clang diagnostic pop
