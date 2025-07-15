// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/templates/regression_401857961:repro
// Features: infer_operator_lifetimes, supported, unsafe_types

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/templates/regression_401857961/repro.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct repro::Interval) == 1);
static_assert(alignof(struct repro::Interval) == 1);
static_assert(CRUBIT_OFFSET_OF(nanos, struct repro::Interval) == 0);

extern "C" void __rust_thunk___ZN5repro8IntervalC1Ev(
    struct repro::Interval* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN5repro8IntervalC1EOS0_(
    struct repro::Interval* __this, struct repro::Interval* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct repro::Interval* __rust_thunk___ZN5repro8IntervalaSERKS0_(
    struct repro::Interval* __this, const struct repro::Interval* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct repro::Interval* __rust_thunk___ZN5repro8IntervalaSEOS0_(
    struct repro::Interval* __this, struct repro::Interval* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

#pragma clang diagnostic pop
