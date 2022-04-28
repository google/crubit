// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/inheritance.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN5Base0C1Ev(class Base0* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN5Base0C1ERKS_(class Base0* __this,
                                               const class Base0& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN5Base0D1Ev(class Base0* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class Base0& __rust_thunk___ZN5Base0aSERKS_(
    class Base0* __this, const class Base0& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN5Base1C1Ev(class Base1* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN5Base1C1ERKS_(class Base1* __this,
                                               const class Base1& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN5Base1D1Ev(class Base1* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class Base1& __rust_thunk___ZN5Base1aSERKS_(
    class Base1* __this, const class Base1& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN5Base2C1Ev(class Base2* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN5Base2C1ERKS_(class Base2* __this,
                                               const class Base2& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN5Base2D1Ev(class Base2* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class Base2& __rust_thunk___ZN5Base2aSERKS_(
    class Base2* __this, const class Base2& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN7DerivedC1Ev(class Derived* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN7DerivedC1ERKS_(
    class Derived* __this, const class Derived& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN7DerivedD1Ev(class Derived* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class Derived& __rust_thunk___ZN7DerivedaSERKS_(
    class Derived* __this, const class Derived& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN12VirtualBase1C1Ev(
    class VirtualBase1* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN12VirtualBase1C1ERKS_(
    class VirtualBase1* __this, const class VirtualBase1& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN12VirtualBase1D1Ev(
    class VirtualBase1* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class VirtualBase1& __rust_thunk___ZN12VirtualBase1aSERKS_(
    class VirtualBase1* __this, const class VirtualBase1& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN12VirtualBase2C1Ev(
    class VirtualBase2* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN12VirtualBase2C1ERKS_(
    class VirtualBase2* __this, const class VirtualBase2& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN12VirtualBase2D1Ev(
    class VirtualBase2* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class VirtualBase2& __rust_thunk___ZN12VirtualBase2aSERKS_(
    class VirtualBase2* __this, const class VirtualBase2& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN14VirtualDerivedC1Ev(
    class VirtualDerived* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN14VirtualDerivedC1ERKS_(
    class VirtualDerived* __this, const class VirtualDerived& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN14VirtualDerivedD1Ev(
    class VirtualDerived* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class VirtualDerived& __rust_thunk___ZN14VirtualDerivedaSERKS_(
    class VirtualDerived* __this, const class VirtualDerived& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}

static_assert(sizeof(class Base0) == 1);
static_assert(alignof(class Base0) == 1);

static_assert(sizeof(class Base1) == 16);
static_assert(alignof(class Base1) == 8);

static_assert(sizeof(class Base2) == 2);
static_assert(alignof(class Base2) == 2);

static_assert(sizeof(class Derived) == 16);
static_assert(alignof(class Derived) == 8);
static_assert(CRUBIT_OFFSET_OF(derived_1, class Derived) * 8 == 96);

static_assert(sizeof(class VirtualBase1) == 24);
static_assert(alignof(class VirtualBase1) == 8);

static_assert(sizeof(class VirtualBase2) == 24);
static_assert(alignof(class VirtualBase2) == 8);

static_assert(sizeof(class VirtualDerived) == 32);
static_assert(alignof(class VirtualDerived) == 8);

#pragma clang diagnostic pop

extern "C" const Base1& __crubit_dynamic_upcast__VirtualBase1__to__Base1(
    const VirtualBase1& from) {
  return from;
}

extern "C" const Base1& __crubit_dynamic_upcast__VirtualBase2__to__Base1(
    const VirtualBase2& from) {
  return from;
}

extern "C" const VirtualBase1&
__crubit_dynamic_upcast__VirtualDerived__to__VirtualBase1(
    const VirtualDerived& from) {
  return from;
}
extern "C" const Base1& __crubit_dynamic_upcast__VirtualDerived__to__Base1(
    const VirtualDerived& from) {
  return from;
}
extern "C" const VirtualBase2&
__crubit_dynamic_upcast__VirtualDerived__to__VirtualBase2(
    const VirtualDerived& from) {
  return from;
}
