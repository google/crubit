// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:callables_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/callables.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(class NotCABICompatible) == 4);
static_assert(alignof(class NotCABICompatible) == 4);

extern "C" void __rust_thunk___ZN17NotCABICompatibleC1Ei(
    class NotCABICompatible* __this, int x) {
  crubit::construct_at(__this, x);
}

extern "C" int __rust_thunk___ZNK17NotCABICompatible3getEv(
    class NotCABICompatible const* __this) {
  return __this->get();
}

static_assert((int (NotCABICompatible::*)() const) & ::NotCABICompatible::get);

#pragma clang diagnostic pop
