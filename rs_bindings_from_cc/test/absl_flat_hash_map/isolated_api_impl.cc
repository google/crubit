// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/absl_flat_hash_map:isolated

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/absl_flat_hash_map/isolated.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(class ::absl::flat_hash_map<int, unsigned long, 42>) == 1);
static_assert(alignof(class ::absl::flat_hash_map<int, unsigned long, 42>) ==
              1);

extern "C" void __rust_thunk__15b85ee3__ZN4absl13flat_hash_mapIimLi42EEC1Ev(
    class ::absl::flat_hash_map<int, unsigned long, 42>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__25d7606d__ZNK4absl13flat_hash_mapIimLi42EE25FunctionRemovedByOverrideEv(
    class ::absl::flat_hash_map<int, unsigned long, 42> const* __this) {
  __this->FunctionRemovedByOverride();
}

static_assert(
    (void (::absl::flat_hash_map<int, unsigned long, 42>::*)() const) &
    ::absl::flat_hash_map<int, unsigned long, 42>::FunctionRemovedByOverride);

#pragma clang diagnostic pop
