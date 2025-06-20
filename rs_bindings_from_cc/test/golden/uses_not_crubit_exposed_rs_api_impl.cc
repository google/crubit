// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:uses_not_crubit_exposed_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/uses_not_crubit_exposed.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct CannotUpcastInCrubit) == 4);
static_assert(alignof(struct CannotUpcastInCrubit) == 4);

extern "C" void __rust_thunk___ZN20CannotUpcastInCrubitC1Ev(
    struct CannotUpcastInCrubit* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN20CannotUpcastInCrubitC1EOS_(
    struct CannotUpcastInCrubit* __this,
    struct CannotUpcastInCrubit* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct CannotUpcastInCrubit*
__rust_thunk___ZN20CannotUpcastInCrubitaSERKS_(
    struct CannotUpcastInCrubit* __this,
    const struct CannotUpcastInCrubit* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct CannotUpcastInCrubit*
__rust_thunk___ZN20CannotUpcastInCrubitaSEOS_(
    struct CannotUpcastInCrubit* __this,
    struct CannotUpcastInCrubit* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

#pragma clang diagnostic pop
