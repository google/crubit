// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unsupported_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/unsupported.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct TrivialCustomType) == 4);
static_assert(alignof(struct TrivialCustomType) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct TrivialCustomType) == 0);

extern "C" void __rust_thunk___ZN17TrivialCustomTypeC1Ev(
    struct TrivialCustomType* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct NontrivialCustomType) == 4);
static_assert(alignof(struct NontrivialCustomType) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct NontrivialCustomType) == 0);

#pragma clang diagnostic pop
