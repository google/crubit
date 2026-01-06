// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/struct/operator_and:operator_and
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/struct/operator_and/operator_and.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(class MyBadClass) == 1);
static_assert(alignof(class MyBadClass) == 1);

extern "C" void __rust_thunk___ZN10MyBadClassC1Ev(class MyBadClass* __this) {
  crubit::construct_at(__this);
}

extern "C" class MyBadClass* __rust_thunk___ZN10MyBadClass7ReturnsEv() {
  return std::addressof(MyBadClass::Returns());
}

static_assert((class MyBadClass & (*)()) & ::MyBadClass::Returns);

extern "C" void __rust_thunk___ZN10MyBadClass7AcceptsERS_(
    class MyBadClass* __param_0) {
  MyBadClass::Accepts(*__param_0);
}

static_assert((void (*)(class MyBadClass&)) & ::MyBadClass::Accepts);

#pragma clang diagnostic pop
