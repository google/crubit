// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:escaping_keywords_cc

#include <cstddef>
#include <memory>

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/escaping_keywords.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN4typeC1Ev(struct type* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN4typeC1EOS_(struct type* __this,
                                             struct type* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

static_assert(sizeof(struct type) == 4);
static_assert(alignof(struct type) == 4);
static_assert(CRUBIT_OFFSET_OF(dyn, struct type) == 0);

#pragma clang diagnostic pop
