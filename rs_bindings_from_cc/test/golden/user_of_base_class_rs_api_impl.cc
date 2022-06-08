// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/user_of_base_class.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN8Derived2C1Ev(class Derived2* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN8Derived2C1ERKS_(
    class Derived2* __this, const class Derived2& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN8Derived2C1EOS_(class Derived2* __this,
                                                 class Derived2&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN8Derived2D1Ev(class Derived2* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class Derived2& __rust_thunk___ZN8Derived2aSERKS_(
    class Derived2* __this, const class Derived2& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class Derived2& __rust_thunk___ZN8Derived2aSEOS_(
    class Derived2* __this, class Derived2&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN15VirtualDerived2C1Ev(
    class VirtualDerived2* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN15VirtualDerived2C1ERKS_(
    class VirtualDerived2* __this, const class VirtualDerived2& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN15VirtualDerived2C1EOS_(
    class VirtualDerived2* __this, class VirtualDerived2&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN15VirtualDerived2D1Ev(
    class VirtualDerived2* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class VirtualDerived2& __rust_thunk___ZN15VirtualDerived2aSERKS_(
    class VirtualDerived2* __this, const class VirtualDerived2& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class VirtualDerived2& __rust_thunk___ZN15VirtualDerived2aSEOS_(
    class VirtualDerived2* __this, class VirtualDerived2&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}

static_assert(sizeof(class Derived2) == 24);
static_assert(alignof(class Derived2) == 8);
static_assert(CRUBIT_OFFSET_OF(derived_1, class Derived2) == 20);

static_assert(sizeof(class VirtualDerived2) == 32);
static_assert(alignof(class VirtualDerived2) == 8);

#pragma clang diagnostic pop

extern "C" const Base0& __crubit_dynamic_upcast__Derived2__to__Base0(
    const Derived2& from) {
  return from;
}

extern "C" const VirtualBase1&
__crubit_dynamic_upcast__VirtualDerived2__to__VirtualBase1(
    const VirtualDerived2& from) {
  return from;
}
extern "C" const Base1& __crubit_dynamic_upcast__VirtualDerived2__to__Base1(
    const VirtualDerived2& from) {
  return from;
}
extern "C" const VirtualBase2&
__crubit_dynamic_upcast__VirtualDerived2__to__VirtualBase2(
    const VirtualDerived2& from) {
  return from;
}
