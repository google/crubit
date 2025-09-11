// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:do_not_bind
// Features: supported, unsafe_types

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

static_assert(sizeof(struct crubit::test::ArgumentToUnboundOverload) == 1);
static_assert(alignof(struct crubit::test::ArgumentToUnboundOverload) == 1);

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

static_assert(sizeof(struct crubit::test::StructWithDoNotBindMethod) == 1);
static_assert(alignof(struct crubit::test::StructWithDoNotBindMethod) == 1);

extern "C" void
__rust_thunk___ZN6crubit4test25StructWithDoNotBindMethod15DoNotBindMethodENS0_23ArgumentToBoundOverloadE(
    struct crubit::test::StructWithDoNotBindMethod* __this,
    struct crubit::test::ArgumentToBoundOverload* __param_0) {
  __this->DoNotBindMethod(std::move(*__param_0));
}

static_assert((void (::crubit::test::StructWithDoNotBindMethod::*)(
    struct crubit::test::ArgumentToBoundOverload))&crubit::test::
                  StructWithDoNotBindMethod::DoNotBindMethod);

#pragma clang diagnostic pop
