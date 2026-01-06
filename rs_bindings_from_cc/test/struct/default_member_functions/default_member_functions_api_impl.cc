// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/struct/default_member_functions:default_member_functions
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/struct/default_member_functions/default_member_functions.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(class Uncopyable) == 1);
static_assert(alignof(class Uncopyable) == 1);

extern "C" void __rust_thunk___ZN10UncopyableC1Ev(class Uncopyable* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class UncopyableDespiteDecl) == 24);
static_assert(alignof(class UncopyableDespiteDecl) == 8);

extern "C" void __rust_thunk___ZN21UncopyableDespiteDeclD1Ev(
    class UncopyableDespiteDecl* __this) {
  std::destroy_at(__this);
}

extern "C" void __rust_thunk___ZN21UncopyableDespiteDeclC1Ev(
    class UncopyableDespiteDecl* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
