// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/absl_flat_hash_map:isolated

#include "support/bridge.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/absl_flat_hash_map/isolated.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(class ::crubit::test::NoDestructor) == 1);
static_assert(alignof(class ::crubit::test::NoDestructor) == 1);

extern "C" class ::crubit::test::NoDestructor*
__rust_thunk___ZN6crubit4test12NoDestructoraSERKS1_(
    class ::crubit::test::NoDestructor* __this,
    class ::crubit::test::NoDestructor const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((class ::crubit::test::NoDestructor &
               (::crubit::test::NoDestructor::*)(
                   class ::crubit::test::NoDestructor const&)) &
              ::crubit::test::NoDestructor::operator=);

static_assert(sizeof(class ::crubit::test::NoDelete) == 1);
static_assert(alignof(class ::crubit::test::NoDelete) == 1);

extern "C" void __rust_thunk___ZN6crubit4test8NoDeleteC1Ev(
    class ::crubit::test::NoDelete* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class ::absl::flat_hash_map<int, unsigned long, 42>) == 1);
static_assert(alignof(class ::absl::flat_hash_map<int, unsigned long, 42>) ==
              1);

extern "C" void __rust_thunk__25d7606d__ZN4absl13flat_hash_mapIimLi42EEC1Ev(
    class ::absl::flat_hash_map<int, unsigned long, 42>* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
