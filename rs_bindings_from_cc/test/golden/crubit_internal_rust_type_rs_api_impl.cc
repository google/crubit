// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/crubit_internal_rust_type.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct ExistingRustTypeFieldTypes) == 5);
static_assert(alignof(struct ExistingRustTypeFieldTypes) == 1);
static_assert(CRUBIT_OFFSET_OF(my_i8_struct,
                               struct ExistingRustTypeFieldTypes) == 0);
static_assert(CRUBIT_OFFSET_OF(my_i8_class,
                               struct ExistingRustTypeFieldTypes) == 1);
static_assert(CRUBIT_OFFSET_OF(my_i8_enum, struct ExistingRustTypeFieldTypes) ==
              2);
static_assert(CRUBIT_OFFSET_OF(my_i8_alias,
                               struct ExistingRustTypeFieldTypes) == 3);
static_assert(CRUBIT_OFFSET_OF(error, struct ExistingRustTypeFieldTypes) == 4);

extern "C" void __rust_thunk___ZN26ExistingRustTypeFieldTypesC1Ev(
    struct ExistingRustTypeFieldTypes* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
