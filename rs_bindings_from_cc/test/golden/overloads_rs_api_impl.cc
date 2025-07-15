// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:overloads_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/overloads.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___Z20AlsoTemplateOverloadv() {
  AlsoTemplateOverload();
}

static_assert(sizeof(class Foo) == 1);
static_assert(alignof(class Foo) == 1);

extern "C" void __rust_thunk___ZN3FooC1Ev(class Foo* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN3Foo3BarE6SizeofIiE(
    class Foo* __this, struct Sizeof<int>* __param_0) {
  __this->Bar(std::move(*__param_0));
}

extern "C" void __rust_thunk___ZN3Foo3BarE6SizeofIfE(
    class Foo* __this, struct Sizeof<float>* __param_0) {
  __this->Bar(std::move(*__param_0));
}

static_assert(sizeof(struct Sizeof<float>) == 1);
static_assert(alignof(struct Sizeof<float>) == 1);

extern "C" void
__rust_thunk___ZN6SizeofIfEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aoverloads_5fcc(
    struct Sizeof<float>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct Sizeof<int>) == 1);
static_assert(alignof(struct Sizeof<int>) == 1);

extern "C" void
__rust_thunk___ZN6SizeofIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aoverloads_5fcc(
    struct Sizeof<int>* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
