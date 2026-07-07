// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:typedef_incomplete_types_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/typedef_incomplete_types.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct HasPointerToIncompleteTypedefs) == 16);
static_assert(alignof(struct HasPointerToIncompleteTypedefs) == 8);
static_assert(CRUBIT_OFFSET_OF(incomplete_extern_c,
                               struct HasPointerToIncompleteTypedefs) == 0);
static_assert(CRUBIT_OFFSET_OF(incomplete,
                               struct HasPointerToIncompleteTypedefs) == 8);

extern "C" void __rust_thunk___ZN30HasPointerToIncompleteTypedefsC1Ev(
    struct HasPointerToIncompleteTypedefs* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
