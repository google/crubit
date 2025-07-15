// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nodiscard_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/nodiscard.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct NoDiscard) == 1);
static_assert(alignof(struct NoDiscard) == 1);

extern "C" void __rust_thunk___ZN9NoDiscardC1Ev(struct NoDiscard* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct NoDiscardWithMessage) == 1);
static_assert(alignof(struct NoDiscardWithMessage) == 1);

extern "C" void __rust_thunk___ZN20NoDiscardWithMessageC1Ev(
    struct NoDiscardWithMessage* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
