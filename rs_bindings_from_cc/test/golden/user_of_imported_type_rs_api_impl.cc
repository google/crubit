// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:user_of_imported_type_cc

#include <cstddef>
#include <memory>

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/user_of_imported_type.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN18UserOfImportedTypeC1Ev(
    struct UserOfImportedType* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN18UserOfImportedTypeC1EOS_(
    struct UserOfImportedType* __this, struct UserOfImportedType* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

static_assert(sizeof(struct UserOfImportedType) == 8);
static_assert(alignof(struct UserOfImportedType) == 8);
static_assert(CRUBIT_OFFSET_OF(trivial, struct UserOfImportedType) == 0);

#pragma clang diagnostic pop
