// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/global:global
// Features: callables, supported, types

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/global/global.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" int* __crubit_get_tls_c_x0000003a_x00000040thread_ulocal_uint() {
  return std::addressof(thread_local_int);
}

extern "C" int* __crubit_get_tls_c_x0000003a_x00000040thread_ulocal_uref() {
  return std::addressof(thread_local_ref);
}

extern "C" int const*
__crubit_get_tls_c_x0000003a_x00000040thread_ulocal_uconst_uint() {
  return std::addressof(thread_local_const_int);
}

extern "C" void __rust_thunk___Z6Unusedi(int arg) { Unused(arg); }

static_assert((void (*)(int)) & ::Unused);

static_assert((int (*)()) & ::GetIntVal);

static_assert((int (*)()) & ::GetNamespacedIntVal);

static_assert((int (*)()) & ::GetCNamespacedIntVal);

static_assert((int (*)()) & ::GetInlineIntVal);

static_assert(sizeof(struct StructWithAnonEnum) == 1);
static_assert(alignof(struct StructWithAnonEnum) == 1);

extern "C" void __rust_thunk___ZN18StructWithAnonEnumC1Ev(
    struct StructWithAnonEnum* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
