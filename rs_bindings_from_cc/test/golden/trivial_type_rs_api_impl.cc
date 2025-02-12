// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:trivial_type_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/trivial_type.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct ns::Trivial) == 4);
static_assert(alignof(struct ns::Trivial) == 4);
static_assert(CRUBIT_OFFSET_OF(trivial_field, struct ns::Trivial) == 0);

extern "C" void __rust_thunk___ZN2ns7TrivialC1Ev(struct ns::Trivial* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN2ns7TrivialC1EOS0_(
    struct ns::Trivial* __this, struct ns::Trivial* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct ns::Trivial* __rust_thunk___ZN2ns7TrivialaSERKS0_(
    struct ns::Trivial* __this, const struct ns::Trivial* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct ns::Trivial* __rust_thunk___ZN2ns7TrivialaSEOS0_(
    struct ns::Trivial* __this, struct ns::Trivial* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" void __rust_thunk___ZN2ns12TakesByValueENS_7TrivialE(
    struct ns::Trivial* __return, struct ns::Trivial* trivial) {
  new (__return) auto(ns::TakesByValue(std::move(*trivial)));
}

#pragma clang diagnostic pop
