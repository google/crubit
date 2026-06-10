// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:type_alias

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/assume_lifetimes/type_alias.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct TypeAliasCtor) == 1);
static_assert(alignof(struct TypeAliasCtor) == 1);

extern "C" void __rust_thunk___ZN13TypeAliasCtorC1ERKS_(
    struct TypeAliasCtor* __this, struct TypeAliasCtor const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" struct TypeAliasCtor* __rust_thunk___ZN13TypeAliasCtoraSERKS_(
    struct TypeAliasCtor* __this, struct TypeAliasCtor const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

extern "C" void
__rust_thunk___ZN13TypeAliasCtorC1ENSt3__u17basic_string_viewIcNS0_11char_traitsIcEEEE(
    struct TypeAliasCtor* __this, ::std::__u::string_view* a) {
  crubit::construct_at(__this, std::move(*a));
}

#pragma clang diagnostic pop
