// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/templates/named_instantiation:named_instantiation

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/templates/named_instantiation/named_instantiation.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert((void (*)(NiIF const&)) & ::SomeApi);

static_assert(sizeof(struct Ni<int, float>) == 1);
static_assert(alignof(struct Ni<int, float>) == 1);

extern "C" void __rust_thunk__426ccee6__ZN2NiIifEC1Eif(
    struct Ni<int, float>* __this, int t, float s) {
  crubit::construct_at(__this, t, s);
}

#pragma clang diagnostic pop
