// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/namespace/nested_items:nested_items
// Features: callables, supported, types

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/namespace/nested_items/nested_items.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" int __rust_thunk___ZN4same9AFunctionEv() {
  return same::AFunction();
}

static_assert((int (*)()) & ::same::AFunction);

static_assert(sizeof(class Same) == 1);
static_assert(alignof(class Same) == 1);

extern "C" void __rust_thunk___ZN4SameC1Ev(class Same* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct Same::NestedItem) == 1);
static_assert(alignof(struct Same::NestedItem) == 1);

extern "C" void __rust_thunk___ZN4Same10NestedItemC1Ev(
    struct Same::NestedItem* __this) {
  crubit::construct_at(__this);
}

extern "C" int __rust_thunk___ZN4Same10NestedItem18NestedItemFunctionEv(
    struct Same::NestedItem* __this) {
  return __this->NestedItemFunction();
}

static_assert((int (::Same::NestedItem::*)()) &
              ::Same::NestedItem::NestedItemFunction);

extern "C" int __rust_thunk___ZN4Same6MethodEv(class Same* __this) {
  return __this->Method();
}

static_assert((int (::Same::*)()) & ::Same::Method);

static_assert(sizeof(struct foo::Foo) == 1);
static_assert(alignof(struct foo::Foo) == 1);

extern "C" void __rust_thunk___ZN3foo3FooC1Ev(struct foo::Foo* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct foo::Foo::foo) == 1);
static_assert(alignof(struct foo::Foo::foo) == 1);

extern "C" void __rust_thunk___ZN3foo3Foo3fooC1Ev(
    struct foo::Foo::foo* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct foo::Foo::foo::Item) == 1);
static_assert(alignof(struct foo::Foo::foo::Item) == 1);

extern "C" void __rust_thunk___ZN3foo3Foo3foo4ItemC1Ev(
    struct foo::Foo::foo::Item* __this) {
  crubit::construct_at(__this);
}

extern "C" int __rust_thunk___ZN3foo3Foo3foo9BFunctionEv() {
  return foo::Foo::foo::BFunction();
}

static_assert((int (*)()) & ::foo::Foo::foo::BFunction);

static_assert(sizeof(struct OuterCpp) == 1);
static_assert(alignof(struct OuterCpp) == 1);

extern "C" void __rust_thunk___ZN8OuterCppC1Ev(struct OuterCpp* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct OuterCpp::Inner) == 1);
static_assert(alignof(struct OuterCpp::Inner) == 1);

extern "C" void __rust_thunk___ZN8OuterCpp5InnerC1Ev(
    struct OuterCpp::Inner* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
