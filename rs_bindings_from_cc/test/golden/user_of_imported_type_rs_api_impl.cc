// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:user_of_imported_type_cc
// Features: experimental, supported

#include <cstddef>
#include <memory>

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/user_of_imported_type.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___Z16UsesImportedTypeN2ns7TrivialE(
    struct ns::Trivial* __return, struct ns::Trivial* t) {
  new (__return) auto(UsesImportedType(std::move(*t)));
}

static_assert(sizeof(struct UserOfImportedType) == 8);
static_assert(alignof(struct UserOfImportedType) == 8);
static_assert(CRUBIT_OFFSET_OF(trivial, struct UserOfImportedType) == 0);

extern "C" void __rust_thunk___ZN18UserOfImportedTypeC1Ev(
    struct UserOfImportedType* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN18UserOfImportedTypeC1EOS_(
    struct UserOfImportedType* __this, struct UserOfImportedType* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct UserOfImportedType*
__rust_thunk___ZN18UserOfImportedTypeaSERKS_(
    struct UserOfImportedType* __this,
    const struct UserOfImportedType* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct UserOfImportedType*
__rust_thunk___ZN18UserOfImportedTypeaSEOS_(
    struct UserOfImportedType* __this, struct UserOfImportedType* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

#pragma clang diagnostic pop
