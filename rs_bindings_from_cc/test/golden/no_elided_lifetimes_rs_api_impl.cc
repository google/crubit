// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:no_elided_lifetimes_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/no_elided_lifetimes.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert((int& (*)(int&)) & ::free_function);

static_assert(sizeof(struct S) == 1);
static_assert(alignof(struct S) == 1);

extern "C" void __rust_thunk___ZN1SC1Ev(struct S* __this) {
  crubit::construct_at(__this);
}

static_assert((int& (S::*)(int&, int&) const) & ::S::const_method);

static_assert((int& (S::*)(int&, int&)) & ::S::method);

static_assert(sizeof(struct TriviallyCopyableButNontriviallyDestructible) == 1);
static_assert(alignof(struct TriviallyCopyableButNontriviallyDestructible) ==
              1);

extern "C" struct TriviallyCopyableButNontriviallyDestructible*
__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleaSERKS_(
    struct TriviallyCopyableButNontriviallyDestructible* __this,
    struct TriviallyCopyableButNontriviallyDestructible const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

extern "C" void
__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleC1ERKS_(
    struct TriviallyCopyableButNontriviallyDestructible* __this,
    struct TriviallyCopyableButNontriviallyDestructible const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

static_assert((void (*)(int*)) & ::take_pointer);

static_assert(CRUBIT_SIZEOF(class WrappedValue) == 4);
static_assert(alignof(class WrappedValue) == 4);

extern "C" void __rust_thunk___ZN12WrappedValueC1Ei(class WrappedValue* __this,
                                                    int value) {
  crubit::construct_at(__this, value);
}

extern "C" void __rust_thunk___ZNK12WrappedValueplERKS_(
    class WrappedValue* __return, class WrappedValue const* __this,
    class WrappedValue const* rhs) {
  new (__return) auto(__this->operator+(*rhs));
}

#pragma clang diagnostic pop
