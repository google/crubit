// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/references:references
// Features: fmt, leading_colons_for_cpp_type, supported, types

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/references/references.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(class TypeWithPtrConstructor) == 1);
static_assert(alignof(class TypeWithPtrConstructor) == 1);

extern "C" void __rust_thunk___ZN22TypeWithPtrConstructorC1EPi(
    class TypeWithPtrConstructor* __this, int* ptr) {
  crubit::construct_at(__this, ptr);
}

static_assert(sizeof(class TypeWithNonNullPtrConstructor) == 1);
static_assert(alignof(class TypeWithNonNullPtrConstructor) == 1);

extern "C" void __rust_thunk___ZN29TypeWithNonNullPtrConstructorC1EPi(
    class TypeWithNonNullPtrConstructor* __this, int* ptr) {
  crubit::construct_at(__this, ptr);
}

static_assert(sizeof(class ::TypeWithReferenceConstructor) == 1);
static_assert(alignof(class ::TypeWithReferenceConstructor) == 1);

extern "C" void __rust_thunk___ZN28TypeWithReferenceConstructorC1ERi(
    class ::TypeWithReferenceConstructor* __this, int* ref) {
  crubit::construct_at(__this, *ref);
}

#pragma clang diagnostic pop
