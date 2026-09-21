// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:templates_source_order_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/templates_source_order.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct TopLevel) == 1);
static_assert(alignof(struct TopLevel) == 1);

extern "C" void __rust_thunk___ZN8TopLevelC1Ev(struct TopLevel* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct test_namespace_bindings::Inner) == 1);
static_assert(alignof(struct test_namespace_bindings::Inner) == 1);

extern "C" void __rust_thunk___ZN23test_namespace_bindings5InnerC1Ev(
    struct test_namespace_bindings::Inner* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class MyTemplate<TopLevel>) == 1);
static_assert(alignof(class MyTemplate<TopLevel>) == 1);

extern "C" void __rust_thunk__3c5bb598__ZN10MyTemplateI8TopLevelEC1Ev(
    class MyTemplate<TopLevel>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class MyTemplate<test_namespace_bindings::Inner>) == 1);
static_assert(alignof(class MyTemplate<test_namespace_bindings::Inner>) == 1);

extern "C" void
__rust_thunk__3c5bb598__ZN10MyTemplateIN23test_namespace_bindings5InnerEEC1Ev(
    class MyTemplate<test_namespace_bindings::Inner>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class MyTemplate<MyTemplate<TopLevel>>) == 1);
static_assert(alignof(class MyTemplate<MyTemplate<TopLevel>>) == 1);

extern "C" void __rust_thunk__3c5bb598__ZN10MyTemplateIS_I8TopLevelEEC1Ev(
    class MyTemplate<MyTemplate<TopLevel>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    sizeof(class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>) == 1);
static_assert(
    alignof(class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>) == 1);

extern "C" void
__rust_thunk__3c5bb598__ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEEC1Ev(
    class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class MyTemplate<bool>) == 1);
static_assert(alignof(class MyTemplate<bool>) == 1);

extern "C" void __rust_thunk__3c5bb598__ZN10MyTemplateIbEC1Ev(
    class MyTemplate<bool>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class MyTemplate<char>) == 1);
static_assert(alignof(class MyTemplate<char>) == 1);

extern "C" void __rust_thunk__3c5bb598__ZN10MyTemplateIcEC1Ev(
    class MyTemplate<char>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class MyTemplate<double>) == 8);
static_assert(alignof(class MyTemplate<double>) == 8);

extern "C" void __rust_thunk__3c5bb598__ZN10MyTemplateIdEC1Ev(
    class MyTemplate<double>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class MyTemplate<float>) == 4);
static_assert(alignof(class MyTemplate<float>) == 4);

extern "C" void __rust_thunk__3c5bb598__ZN10MyTemplateIfEC1Ev(
    class MyTemplate<float>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class MyTemplate<int>) == 4);
static_assert(alignof(class MyTemplate<int>) == 4);

extern "C" void __rust_thunk__3c5bb598__ZN10MyTemplateIiEC1Ev(
    class MyTemplate<int>* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
