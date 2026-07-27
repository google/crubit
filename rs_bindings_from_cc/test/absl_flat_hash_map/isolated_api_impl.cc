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

extern "C" size_t
__crubit_flat_hash_map_len___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const absl::flat_hash_map<int, unsigned long, 42>* __this) {
  return __this->size();
}

extern "C" size_t
__crubit_flat_hash_map_capacity___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const absl::flat_hash_map<int, unsigned long, 42>* __this) {
  return __this->capacity();
}

extern "C" bool
__crubit_flat_hash_map_is_empty___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const absl::flat_hash_map<int, unsigned long, 42>* __this) {
  return __this->empty();
}

extern "C" bool
__crubit_flat_hash_map_try_insert___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    absl::flat_hash_map<int, unsigned long, 42>* __this, int* key,
    unsigned long* value, int const** result_key,
    unsigned long** result_value) {
  auto it = __this->try_emplace(std::move(*key), std::move(*value));
  *result_key = &it.first->first;
  *result_value = &it.first->second;
  return it.second;
}

#pragma clang diagnostic pop
