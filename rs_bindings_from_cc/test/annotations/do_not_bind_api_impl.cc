// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:do_not_bind
// Features: do_not_hardcode_status_bridge, experimental, infer_operator_lifetimes, supported, unsafe_types, wrapper

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/annotations/do_not_bind.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct crubit::test::ArgumentToBoundOverload) == 1);
static_assert(alignof(struct crubit::test::ArgumentToBoundOverload) == 1);

extern "C" void __rust_thunk___ZN6crubit4test23ArgumentToBoundOverloadC1Ev(
    struct crubit::test::ArgumentToBoundOverload* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct crubit::test::ArgumentToUnboundOverload) == 1);
static_assert(alignof(struct crubit::test::ArgumentToUnboundOverload) == 1);

extern "C" void __rust_thunk___ZN6crubit4test25ArgumentToUnboundOverloadC1Ev(
    struct crubit::test::ArgumentToUnboundOverload* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN6crubit4test11DoNotBindFnENS0_23ArgumentToBoundOverloadE(
    struct crubit::test::ArgumentToBoundOverload* __param_0) {
  crubit::test::DoNotBindFn(std::move(*__param_0));
}

static_assert((void (*)(
    struct crubit::test::ArgumentToBoundOverload))&crubit::test::DoNotBindFn);

static_assert(sizeof(struct crubit::test::StructWithDoNotBindConstructor) == 1);
static_assert(alignof(struct crubit::test::StructWithDoNotBindConstructor) ==
              1);

extern "C" void
__rust_thunk___ZN6crubit4test30StructWithDoNotBindConstructorC1ENS0_23ArgumentToBoundOverloadE(
    struct crubit::test::StructWithDoNotBindConstructor* __this,
    struct crubit::test::ArgumentToBoundOverload* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

static_assert(sizeof(struct crubit::test::StructWithDoNotBindMethod) == 1);
static_assert(alignof(struct crubit::test::StructWithDoNotBindMethod) == 1);

extern "C" void __rust_thunk___ZN6crubit4test25StructWithDoNotBindMethodC1Ev(
    struct crubit::test::StructWithDoNotBindMethod* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN6crubit4test25StructWithDoNotBindMethod15DoNotBindMethodENS0_23ArgumentToBoundOverloadE(
    struct crubit::test::StructWithDoNotBindMethod* __this,
    struct crubit::test::ArgumentToBoundOverload* __param_0) {
  __this->DoNotBindMethod(std::move(*__param_0));
}

static_assert((void (::crubit::test::StructWithDoNotBindMethod::*)(
    struct crubit::test::ArgumentToBoundOverload))&crubit::test::
                  StructWithDoNotBindMethod::DoNotBindMethod);

static_assert(sizeof(struct std::integral_constant<bool, false>) == 1);
static_assert(alignof(struct std::integral_constant<bool, false>) == 1);

extern "C" void
__rust_thunk___ZNSt3__u17integral_constantIbLb0EEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fannotations_3ado_5fnot_5fbind(
    struct std::integral_constant<bool, false>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::integral_constant<bool, true>) == 1);
static_assert(alignof(struct std::integral_constant<bool, true>) == 1);

extern "C" void
__rust_thunk___ZNSt3__u17integral_constantIbLb1EEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fannotations_3ado_5fnot_5fbind(
    struct std::integral_constant<bool, true>* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
