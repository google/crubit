// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:inheritance_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/inheritance.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(class Base0) == 1);
static_assert(alignof(class Base0) == 1);

extern "C" void __rust_thunk___ZN5Base0C1Ev(class Base0* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN5Base0C1EOS_(class Base0* __this,
                                              class Base0* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class Base0* __rust_thunk___ZN5Base0aSERKS_(
    class Base0* __this, const class Base0* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" class Base0* __rust_thunk___ZN5Base0aSEOS_(class Base0* __this,
                                                      class Base0* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(class Base1) == 16);
static_assert(alignof(class Base1) == 8);

extern "C" void __rust_thunk___ZN5Base1C1Ev(class Base1* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN5Base1C1EOS_(class Base1* __this,
                                              class Base1* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class Base1* __rust_thunk___ZN5Base1aSERKS_(
    class Base1* __this, const class Base1* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" class Base1* __rust_thunk___ZN5Base1aSEOS_(class Base1* __this,
                                                      class Base1* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(class Base2) == 2);
static_assert(alignof(class Base2) == 2);

extern "C" void __rust_thunk___ZN5Base2C1Ev(class Base2* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN5Base2C1EOS_(class Base2* __this,
                                              class Base2* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class Base2* __rust_thunk___ZN5Base2aSERKS_(
    class Base2* __this, const class Base2* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" class Base2* __rust_thunk___ZN5Base2aSEOS_(class Base2* __this,
                                                      class Base2* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(struct Derived) == 16);
static_assert(alignof(struct Derived) == 8);
static_assert(CRUBIT_OFFSET_OF(derived_1, struct Derived) == 12);

extern "C" void __rust_thunk___ZN7DerivedC1Ev(struct Derived* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN7DerivedC1EOS_(struct Derived* __this,
                                                struct Derived* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct Derived* __rust_thunk___ZN7DerivedaSERKS_(
    struct Derived* __this, const struct Derived* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct Derived* __rust_thunk___ZN7DerivedaSEOS_(
    struct Derived* __this, struct Derived* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(class VirtualBase1) == 24);
static_assert(alignof(class VirtualBase1) == 8);

extern "C" void __rust_thunk___ZN12VirtualBase1C1Ev(
    class VirtualBase1* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN12VirtualBase1C1ERKS_(
    class VirtualBase1* __this, const class VirtualBase1* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" void __rust_thunk___ZN12VirtualBase1C1EOS_(
    class VirtualBase1* __this, class VirtualBase1* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class VirtualBase1* __rust_thunk___ZN12VirtualBase1aSERKS_(
    class VirtualBase1* __this, const class VirtualBase1* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" class VirtualBase1* __rust_thunk___ZN12VirtualBase1aSEOS_(
    class VirtualBase1* __this, class VirtualBase1* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" const class Base1&
__crubit_dynamic_upcast__12VirtualBase1__to__5Base1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3ainheritance_5fcc(
    const class VirtualBase1& from) {
  return from;
}

static_assert(CRUBIT_SIZEOF(class VirtualBase2) == 24);
static_assert(alignof(class VirtualBase2) == 8);

extern "C" void __rust_thunk___ZN12VirtualBase2C1Ev(
    class VirtualBase2* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN12VirtualBase2C1ERKS_(
    class VirtualBase2* __this, const class VirtualBase2* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" void __rust_thunk___ZN12VirtualBase2C1EOS_(
    class VirtualBase2* __this, class VirtualBase2* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class VirtualBase2* __rust_thunk___ZN12VirtualBase2aSERKS_(
    class VirtualBase2* __this, const class VirtualBase2* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" class VirtualBase2* __rust_thunk___ZN12VirtualBase2aSEOS_(
    class VirtualBase2* __this, class VirtualBase2* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" const class Base1&
__crubit_dynamic_upcast__12VirtualBase2__to__5Base1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3ainheritance_5fcc(
    const class VirtualBase2& from) {
  return from;
}

static_assert(CRUBIT_SIZEOF(class VirtualDerived) == 32);
static_assert(alignof(class VirtualDerived) == 8);

extern "C" void __rust_thunk___ZN14VirtualDerivedC1Ev(
    class VirtualDerived* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN14VirtualDerivedC1ERKS_(
    class VirtualDerived* __this, const class VirtualDerived* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" void __rust_thunk___ZN14VirtualDerivedC1EOS_(
    class VirtualDerived* __this, class VirtualDerived* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class VirtualDerived* __rust_thunk___ZN14VirtualDerivedaSERKS_(
    class VirtualDerived* __this, const class VirtualDerived* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" class VirtualDerived* __rust_thunk___ZN14VirtualDerivedaSEOS_(
    class VirtualDerived* __this, class VirtualDerived* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" const class VirtualBase1&
__crubit_dynamic_upcast__14VirtualDerived__to__12VirtualBase1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3ainheritance_5fcc(
    const class VirtualDerived& from) {
  return from;
}

extern "C" const class Base1&
__crubit_dynamic_upcast__14VirtualDerived__to__5Base1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3ainheritance_5fcc(
    const class VirtualDerived& from) {
  return from;
}

extern "C" const class VirtualBase2&
__crubit_dynamic_upcast__14VirtualDerived__to__12VirtualBase2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3ainheritance_5fcc(
    const class VirtualDerived& from) {
  return from;
}

static_assert(CRUBIT_SIZEOF(class MyAbstractClass) == 8);
static_assert(alignof(class MyAbstractClass) == 8);

extern "C" class MyAbstractClass* __rust_thunk___ZN15MyAbstractClassaSERKS_(
    class MyAbstractClass* __this, const class MyAbstractClass* __param_0) {
  return &__this->operator=(*__param_0);
}

static_assert(sizeof(class MethodBase1) == 1);
static_assert(alignof(class MethodBase1) == 1);

extern "C" void __rust_thunk___ZN11MethodBase1C1Ev(class MethodBase1* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN11MethodBase1C1EOS_(
    class MethodBase1* __this, class MethodBase1* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class MethodBase1* __rust_thunk___ZN11MethodBase1aSERKS_(
    class MethodBase1* __this, const class MethodBase1* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" class MethodBase1* __rust_thunk___ZN11MethodBase1aSEOS_(
    class MethodBase1* __this, class MethodBase1* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(sizeof(class MethodBase2) == 1);
static_assert(alignof(class MethodBase2) == 1);

extern "C" void __rust_thunk___ZN11MethodBase2C1Ev(class MethodBase2* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN11MethodBase2C1EOS_(
    class MethodBase2* __this, class MethodBase2* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class MethodBase2* __rust_thunk___ZN11MethodBase2aSERKS_(
    class MethodBase2* __this, const class MethodBase2* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" class MethodBase2* __rust_thunk___ZN11MethodBase2aSEOS_(
    class MethodBase2* __this, class MethodBase2* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(sizeof(class MethodDerived) == 1);
static_assert(alignof(class MethodDerived) == 1);

extern "C" void __rust_thunk___ZN13MethodDerivedC1Ev(
    class MethodDerived* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN13MethodDerivedC1EOS_(
    class MethodDerived* __this, class MethodDerived* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class MethodDerived* __rust_thunk___ZN13MethodDerivedaSERKS_(
    class MethodDerived* __this, const class MethodDerived* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" class MethodDerived* __rust_thunk___ZN13MethodDerivedaSEOS_(
    class MethodDerived* __this, class MethodDerived* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

#pragma clang diagnostic pop
