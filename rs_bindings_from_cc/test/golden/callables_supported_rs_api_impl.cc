// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:callables_supported_cc

#include "support/bridge.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/callables_supported.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct ABICompatible) == 4);
static_assert(alignof(struct ABICompatible) == 4);
static_assert(CRUBIT_OFFSET_OF(x, struct ABICompatible) == 0);

extern "C" void __rust_thunk___ZN13ABICompatibleC1Ev(
    struct ABICompatible* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class LayoutCompatible) == 4);
static_assert(alignof(class LayoutCompatible) == 4);

extern "C" void __rust_thunk___ZN16LayoutCompatible6CreateEi(
    class LayoutCompatible* __return, int x) {
  new (__return) auto(LayoutCompatible::Create(x));
}

static_assert((class LayoutCompatible (*)(int)) & ::LayoutCompatible::Create);

extern "C" int __rust_thunk___ZNK16LayoutCompatible3getEv(
    class LayoutCompatible const* __this) {
  return __this->get();
}

static_assert((int (LayoutCompatible::*)() const) & ::LayoutCompatible::get);

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_string_view<wchar_t, std::char_traits<wchar_t>>) ==
    16);
static_assert(
    alignof(class std::basic_string_view<wchar_t, std::char_traits<wchar_t>>) ==
    8);

#pragma clang diagnostic pop
