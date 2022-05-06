// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/polymorphic.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN15PolymorphicBaseC1Ev(
    class PolymorphicBase* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN15PolymorphicBaseC1ERKS_(
    class PolymorphicBase* __this, const class PolymorphicBase& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class PolymorphicBase& __rust_thunk___ZN15PolymorphicBaseaSERKS_(
    class PolymorphicBase* __this, const class PolymorphicBase& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN15PolymorphicBaseD1Ev(
    class PolymorphicBase* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN16PolymorphicBase2C1Ev(
    class PolymorphicBase2* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN16PolymorphicBase2C1ERKS_(
    class PolymorphicBase2* __this, const class PolymorphicBase2& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class PolymorphicBase2& __rust_thunk___ZN16PolymorphicBase2aSERKS_(
    class PolymorphicBase2* __this, const class PolymorphicBase2& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN16PolymorphicBase23FooEv(
    class PolymorphicBase2* __this) {
  __this->Foo();
}
extern "C" void __rust_thunk___ZN16PolymorphicBase2D1Ev(
    class PolymorphicBase2* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN18PolymorphicDerivedC1Ev(
    class PolymorphicDerived* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN18PolymorphicDerivedC1ERKS_(
    class PolymorphicDerived* __this,
    const class PolymorphicDerived& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN18PolymorphicDerivedC1EOS_(
    class PolymorphicDerived* __this, class PolymorphicDerived&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN18PolymorphicDerivedD1Ev(
    class PolymorphicDerived* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class PolymorphicDerived&
__rust_thunk___ZN18PolymorphicDerivedaSERKS_(
    class PolymorphicDerived* __this,
    const class PolymorphicDerived& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class PolymorphicDerived&
__rust_thunk___ZN18PolymorphicDerivedaSEOS_(
    class PolymorphicDerived* __this, class PolymorphicDerived&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}

static_assert(sizeof(class PolymorphicBase) == 8);
static_assert(alignof(class PolymorphicBase) == 8);

static_assert(sizeof(class PolymorphicBase2) == 8);
static_assert(alignof(class PolymorphicBase2) == 8);

static_assert(sizeof(class PolymorphicDerived) == 16);
static_assert(alignof(class PolymorphicDerived) == 8);

#pragma clang diagnostic pop
