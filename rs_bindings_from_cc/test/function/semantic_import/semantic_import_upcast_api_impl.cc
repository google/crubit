// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/function/semantic_import:semantic_import_upcast

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/function/semantic_import/semantic_import.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(class ::S) == 4);
static_assert(alignof(class ::S) == 4);

extern "C" void __rust_thunk___ZN1SC1Ei(class ::S* __this, int x) {
  crubit::construct_at(__this, x);
}

static_assert(CRUBIT_SIZEOF(class ::T) == 8);
static_assert(alignof(class ::T) == 4);

extern "C" void __rust_thunk___ZN1TC1Eif(class ::T* __this, int x, float y) {
  crubit::construct_at(__this, x, y);
}

static_assert(sizeof(class ::Chars) == 3);
static_assert(alignof(class ::Chars) == 1);

extern "C" void __rust_thunk___ZN5CharsC1Ev(class ::Chars* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class ::Bools) == 1);
static_assert(alignof(class ::Bools) == 1);

extern "C" void __rust_thunk___ZN5BoolsC1Ev(class ::Bools* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
