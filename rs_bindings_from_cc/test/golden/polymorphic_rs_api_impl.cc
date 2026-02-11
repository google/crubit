// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:polymorphic_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/polymorphic.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(class PolymorphicBase) == 8);
static_assert(alignof(class PolymorphicBase) == 8);

extern "C" void __rust_thunk___ZN15PolymorphicBaseC1Ev(
    class PolymorphicBase* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN15PolymorphicBaseD1Ev(
    class PolymorphicBase* __this) {
  std::destroy_at(__this);
}

extern "C" void
__crubit_operator_delete__15PolymorphicBase___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3apolymorphic_5fcc(
    class PolymorphicBase* ptr) {
  delete ptr;
}

static_assert(CRUBIT_SIZEOF(class PolymorphicBase2) == 8);
static_assert(alignof(class PolymorphicBase2) == 8);

extern "C" void __rust_thunk___ZN16PolymorphicBase2C1Ev(
    class PolymorphicBase2* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN16PolymorphicBase23FooEv(
    class PolymorphicBase2* __this) {
  __this->Foo();
}

static_assert((void (PolymorphicBase2::*)()) & ::PolymorphicBase2::Foo);

extern "C" void __rust_thunk___ZN16PolymorphicBase2D1Ev(
    class PolymorphicBase2* __this) {
  std::destroy_at(__this);
}

extern "C" void
__crubit_operator_delete__16PolymorphicBase2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3apolymorphic_5fcc(
    class PolymorphicBase2* ptr) {
  delete ptr;
}

static_assert(CRUBIT_SIZEOF(class PolymorphicDerived) == 16);
static_assert(alignof(class PolymorphicDerived) == 8);

extern "C" void __rust_thunk___ZN18PolymorphicDerivedC1Ev(
    class PolymorphicDerived* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN18PolymorphicDerivedD1Ev(
    class PolymorphicDerived* __this) {
  std::destroy_at(__this);
}

extern "C" void
__crubit_operator_delete__18PolymorphicDerived___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3apolymorphic_5fcc(
    class PolymorphicDerived* ptr) {
  delete ptr;
}

#pragma clang diagnostic pop
