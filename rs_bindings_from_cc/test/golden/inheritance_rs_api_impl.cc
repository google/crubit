// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/test/golden/inheritance.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN5Base0C1Ev(class Base0* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN5Base0C1ERKS_(class Base0* __this,
                                               const class Base0& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN5Base0D1Ev(class Base0* __this) {
  std ::destroy_at(__this);
}
extern "C" class Base0& __rust_thunk___ZN5Base0aSERKS_(
    class Base0* __this, const class Base0& __param_0) {
  return __this->operator=(__param_0);
}
extern "C" void __rust_thunk___ZN5Base1C1Ev(class Base1* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN5Base1C1ERKS_(class Base1* __this,
                                               const class Base1& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN5Base1D1Ev(class Base1* __this) {
  std ::destroy_at(__this);
}
extern "C" class Base1& __rust_thunk___ZN5Base1aSERKS_(
    class Base1* __this, const class Base1& __param_0) {
  return __this->operator=(__param_0);
}
extern "C" void __rust_thunk___ZN5Base2C1Ev(class Base2* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN5Base2C1ERKS_(class Base2* __this,
                                               const class Base2& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN5Base2D1Ev(class Base2* __this) {
  std ::destroy_at(__this);
}
extern "C" class Base2& __rust_thunk___ZN5Base2aSERKS_(
    class Base2* __this, const class Base2& __param_0) {
  return __this->operator=(__param_0);
}
extern "C" void __rust_thunk___ZN7DerivedC1Ev(class Derived* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN7DerivedC1ERKS_(
    class Derived* __this, const class Derived& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN7DerivedD1Ev(class Derived* __this) {
  std ::destroy_at(__this);
}
extern "C" class Derived& __rust_thunk___ZN7DerivedaSERKS_(
    class Derived* __this, const class Derived& __param_0) {
  return __this->operator=(__param_0);
}

static_assert(sizeof(class Base0) == 1);
static_assert(alignof(class Base0) == 1);

static_assert(sizeof(class Base1) == 16);
static_assert(alignof(class Base1) == 8);

static_assert(sizeof(class Base2) == 2);
static_assert(alignof(class Base2) == 2);

static_assert(sizeof(class Derived) == 16);
static_assert(alignof(class Derived) == 8);
static_assert(offsetof(class Derived, derived_1) * 8 == 96);

#pragma clang diagnostic pop
