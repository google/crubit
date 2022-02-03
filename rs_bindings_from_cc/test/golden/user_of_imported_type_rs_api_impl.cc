// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/test/golden/user_of_imported_type.h"

extern "C" void __rust_thunk___ZN18UserOfImportedTypeC1Ev(
    class UserOfImportedType* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN18UserOfImportedTypeC1ERKS_(
    class UserOfImportedType* __this,
    const class UserOfImportedType& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN18UserOfImportedTypeD1Ev(
    class UserOfImportedType* __this) {
  std ::destroy_at(__this);
}
extern "C" class UserOfImportedType&
__rust_thunk___ZN18UserOfImportedTypeaSERKS_(
    class UserOfImportedType* __this,
    const class UserOfImportedType& __param_0) {
  return __this->operator=(__param_0);
}

static_assert(sizeof(class UserOfImportedType) == 8);
static_assert(alignof(class UserOfImportedType) == 8);
static_assert(offsetof(class UserOfImportedType, trivial) * 8 == 0);
