// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>
#include "rs_bindings_from_cc/test/golden/user_of_imported_type.h"

extern "C" void __rust_thunk___ZN18UserOfImportedTypeD1Ev(
    UserOfImportedType* __this) {
  return std ::destroy_at(__this);
}

static_assert(sizeof(UserOfImportedType) == 8);
static_assert(alignof(UserOfImportedType) == 8);
static_assert(offsetof(UserOfImportedType, trivial) * 8 == 0);
