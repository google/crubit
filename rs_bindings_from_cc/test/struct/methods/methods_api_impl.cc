// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/struct/methods:methods
// Features: assume_lifetimes, supported

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/struct/methods/methods.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(class SomeClass) == 4);
static_assert(alignof(class SomeClass) == 4);
static_assert(CRUBIT_OFFSET_OF(int_var, class SomeClass) == 0);

extern "C" void __rust_thunk___ZN9SomeClassC1Ev(class SomeClass* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN9SomeClass21static_factory_methodEi(
    class SomeClass* __return, int int_var_initial_value) {
  new (__return) auto(SomeClass::static_factory_method(int_var_initial_value));
}

static_assert((class SomeClass (*)(int)) & ::SomeClass::static_factory_method);

static_assert((int (*)(int, int)) &
              ::SomeClass::static_method_that_multiplies_its_args);

extern "C" int __rust_thunk___ZN9SomeClass20static_inline_methodEi(int arg) {
  return SomeClass::static_inline_method(arg);
}

static_assert((int (*)(int)) & ::SomeClass::static_inline_method);

static_assert(CRUBIT_SIZEOF(struct InstanceMethods) == 4);
static_assert(alignof(struct InstanceMethods) == 4);
static_assert(CRUBIT_OFFSET_OF(int_field, struct InstanceMethods) == 0);

extern "C" void __rust_thunk___ZN15InstanceMethodsC1Ev(
    struct InstanceMethods* __this) {
  crubit::construct_at(__this);
}

static_assert((int (InstanceMethods::*)() const) &
              ::InstanceMethods::get_int_field);

static_assert((void (InstanceMethods::*)(int)) &
              ::InstanceMethods::set_int_field);

extern "C" int __rust_thunk___ZNK15InstanceMethods20inline_get_int_fieldEv(
    struct InstanceMethods const* __this) {
  return __this->inline_get_int_field();
}

static_assert((int (InstanceMethods::*)() const) &
              ::InstanceMethods::inline_get_int_field);

extern "C" void __rust_thunk___ZN15InstanceMethods20inline_set_int_fieldEi(
    struct InstanceMethods* __this, int new_value) {
  __this->inline_set_int_field(new_value);
}

static_assert((void (InstanceMethods::*)(int)) &
              ::InstanceMethods::inline_set_int_field);

extern "C" int* __rust_thunk___ZN15InstanceMethods21takes_and_returns_refERi(
    struct InstanceMethods* __this, int* input_ref) {
  return std::addressof(__this->takes_and_returns_ref(*input_ref));
}

static_assert((int& (InstanceMethods::*)(int&)) &
              ::InstanceMethods::takes_and_returns_ref);

extern "C" void __rust_thunk___ZNR15InstanceMethods13ref_qualifiedEv(
    struct InstanceMethods* __this) {
  __this->ref_qualified();
}

static_assert((void (InstanceMethods::*)() &)&::InstanceMethods::ref_qualified);

extern "C" void __rust_thunk___ZNKR15InstanceMethods19const_ref_qualifiedEv(
    struct InstanceMethods const* __this) {
  __this->const_ref_qualified();
}

static_assert((void (InstanceMethods::*)()
                   const&)&::InstanceMethods::const_ref_qualified);

extern "C" void __rust_thunk___ZNO15InstanceMethods16rvalue_qualifiedEv(
    struct InstanceMethods* __this) {
  std::move(*__this).rvalue_qualified();
}

static_assert(
    (void (InstanceMethods::*)() &&)&::InstanceMethods::rvalue_qualified);

#pragma clang diagnostic pop
