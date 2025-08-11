// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:noexcept_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/noexcept.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(class SomeClass) == 1);
static_assert(alignof(class SomeClass) == 1);

extern "C" void __rust_thunk___ZN9SomeClassC1Ev(class SomeClass* __this) {
  crubit::construct_at(__this);
}

static_assert((void (*)(int, char))&SomeClass::create);

static_assert((void (::SomeClass::*)())&SomeClass::no_except_member);

static_assert((void (::SomeClass::*)())&SomeClass::no_except_true_member);

static_assert((void (::SomeClass::*)())&SomeClass::no_except_false_member);

static_assert((void (*)())&no_except);

static_assert((void (*)())&no_except_true);

static_assert((void (*)())&no_except_false);

#pragma clang diagnostic pop
