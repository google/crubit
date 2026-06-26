// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:conversion_operators_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/conversion_operators.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct DstLocalMovable) == 4);
static_assert(alignof(struct DstLocalMovable) == 4);
static_assert(CRUBIT_OFFSET_OF(val, struct DstLocalMovable) == 0);

extern "C" void __rust_thunk___ZN15DstLocalMovableC1Ev(
    struct DstLocalMovable* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct DstLocalNonMovable) == 4);
static_assert(alignof(struct DstLocalNonMovable) == 4);
static_assert(CRUBIT_OFFSET_OF(val, struct DstLocalNonMovable) == 0);

static_assert(CRUBIT_SIZEOF(struct Src) == 4);
static_assert(alignof(struct Src) == 4);
static_assert(CRUBIT_OFFSET_OF(value, struct Src) == 0);

extern "C" void __rust_thunk___ZN3SrcC1Ev(struct Src* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZNK3Srccv15DstLocalMovableEv(
    struct DstLocalMovable* __return, struct Src const* __this) {
  new (__return) auto(__this->operator struct DstLocalMovable());
}

extern "C" void __rust_thunk___ZNK3Srccv18DstLocalNonMovableEv(
    struct DstLocalNonMovable* __return, struct Src const* __this) {
  new (__return) auto(__this->operator struct DstLocalNonMovable());
}

#pragma clang diagnostic pop
