// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:type_alias

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"

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

static_assert((struct TypeAliasCtor &
               (::TypeAliasCtor::*)(struct TypeAliasCtor const&)) &
              ::TypeAliasCtor::operator=);

extern "C" void
__rust_thunk___ZN13TypeAliasCtorC1ENSt3__u17basic_string_viewIcNS0_11char_traitsIcEEEE(
    struct TypeAliasCtor* __this, ::std::__u::string_view* a) {
  crubit::construct_at(__this, crubit::UnsafeTakeValueOnConversion(a));
}

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const char*>) == 8);
static_assert(alignof(class std::reverse_iterator<const char*>) == 8);

extern "C" void __rust_thunk__63d556e1__ZNSt3__u16reverse_iteratorIPKcEC1Ev(
    class std::reverse_iterator<const char*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__9cfa002e__ZNSt3__u16reverse_iteratorIPKcEC1ES2_(
    class std::reverse_iterator<const char*>* __this, char const* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" char const*
__rust_thunk__1b6af95a__ZNKSt3__u16reverse_iteratorIPKcE4baseEv(
    class std::reverse_iterator<const char*> const* __this) {
  return __this->base();
}

static_assert((char const* (::std::reverse_iterator<const char*>::*)() const) &
              ::std::reverse_iterator<const char*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const wchar_t*>) == 8);
static_assert(alignof(class std::reverse_iterator<const wchar_t*>) == 8);

extern "C" void __rust_thunk__63d556e1__ZNSt3__u16reverse_iteratorIPKwEC1Ev(
    class std::reverse_iterator<const wchar_t*>* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
