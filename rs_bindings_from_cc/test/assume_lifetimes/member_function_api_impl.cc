// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:member_function
// Features: assume_lifetimes, custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/assume_lifetimes/member_function.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct S) == 4);
static_assert(alignof(struct S) == 4);
static_assert(CRUBIT_OFFSET_OF(int_field, struct S) == 0);

extern "C" void __rust_thunk___ZN1SC1Ev(struct S* __this) {
  crubit::construct_at(__this);
}

extern "C" int const* __rust_thunk___ZNK1S12int_accessorEv(
    struct S const* __this) {
  return std::addressof(__this->int_accessor());
}

static_assert((int const& (S::*)() const) & ::S::int_accessor);

#pragma clang diagnostic pop
