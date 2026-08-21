// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:free_function

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/assume_lifetimes/free_function.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(class C) == 1);
static_assert(alignof(class C) == 1);

extern "C" void __rust_thunk___ZN1CC1Ev(class C* __this) {
  crubit::construct_at(__this);
}

static_assert((int& (::C::*)()) & ::C::f);

#pragma clang diagnostic pop
