// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:rust_name
// Features: do_not_hardcode_status_bridge, experimental, infer_operator_lifetimes, supported, unsafe_types, wrapper

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/annotations/rust_name.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___ZN6crubit4test13FreeFnOldNameEv() {
  crubit::test::FreeFnOldName();
}

static_assert(sizeof(struct crubit::test::StructOldName) == 1);
static_assert(alignof(struct crubit::test::StructOldName) == 1);

extern "C" void __rust_thunk___ZN6crubit4test13StructOldNameC1Ev(
    struct crubit::test::StructOldName* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct crubit::test::SomeStruct) == 4);
static_assert(alignof(struct crubit::test::SomeStruct) == 4);
static_assert(CRUBIT_OFFSET_OF(field_old_name,
                               struct crubit::test::SomeStruct) == 0);

extern "C" void __rust_thunk___ZN6crubit4test10SomeStructC1Ev(
    struct crubit::test::SomeStruct* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN6crubit4test10SomeStructC1Eiii(
    struct crubit::test::SomeStruct* __this, int a, int b, int c) {
  crubit::construct_at(__this, a, b, c);
}

extern "C" void __rust_thunk___ZNK6crubit4test10SomeStruct13MethodOldNameEv(
    const struct crubit::test::SomeStruct* __this) {
  __this->MethodOldName();
}

static_assert(sizeof(struct std::integral_constant<bool, false>) == 1);
static_assert(alignof(struct std::integral_constant<bool, false>) == 1);

extern "C" void
__rust_thunk___ZNSt3__u17integral_constantIbLb0EEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fannotations_3arust_5fname(
    struct std::integral_constant<bool, false>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::integral_constant<bool, true>) == 1);
static_assert(alignof(struct std::integral_constant<bool, true>) == 1);

extern "C" void
__rust_thunk___ZNSt3__u17integral_constantIbLb1EEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fannotations_3arust_5fname(
    struct std::integral_constant<bool, true>* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
