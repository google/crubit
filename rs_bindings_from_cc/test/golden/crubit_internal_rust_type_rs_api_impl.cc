// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc
// Features: experimental, supported

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/crubit_internal_rust_type.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct TypeMapOverrideFieldTypes) == 5);
static_assert(alignof(struct TypeMapOverrideFieldTypes) == 1);
static_assert(CRUBIT_OFFSET_OF(my_i8_struct,
                               struct TypeMapOverrideFieldTypes) == 0);
static_assert(CRUBIT_OFFSET_OF(my_i8_class, struct TypeMapOverrideFieldTypes) ==
              1);
static_assert(CRUBIT_OFFSET_OF(my_i8_enum, struct TypeMapOverrideFieldTypes) ==
              2);
static_assert(CRUBIT_OFFSET_OF(my_i8_alias, struct TypeMapOverrideFieldTypes) ==
              3);
static_assert(CRUBIT_OFFSET_OF(error, struct TypeMapOverrideFieldTypes) == 4);

extern "C" void __rust_thunk___ZN25TypeMapOverrideFieldTypesC1Ev(
    struct TypeMapOverrideFieldTypes* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN25TypeMapOverrideFieldTypesC1EOS_(
    struct TypeMapOverrideFieldTypes* __this,
    struct TypeMapOverrideFieldTypes* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct TypeMapOverrideFieldTypes*
__rust_thunk___ZN25TypeMapOverrideFieldTypesaSERKS_(
    struct TypeMapOverrideFieldTypes* __this,
    const struct TypeMapOverrideFieldTypes* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct TypeMapOverrideFieldTypes*
__rust_thunk___ZN25TypeMapOverrideFieldTypesaSEOS_(
    struct TypeMapOverrideFieldTypes* __this,
    struct TypeMapOverrideFieldTypes* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

#pragma clang diagnostic pop
