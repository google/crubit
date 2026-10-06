// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:inherited_friend_operators_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/inherited_friend_operators.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct Equatable) == 4);
static_assert(alignof(struct Equatable) == 4);
static_assert(CRUBIT_OFFSET_OF(value, struct Equatable) == 0);

extern "C" void __rust_thunk___ZN9EquatableC1Ev(struct Equatable* __this) {
  crubit::construct_at(__this);
}

extern "C" bool __rust_thunk___ZN6mixinseqERK9EquatableS2_(
    struct Equatable const* lhs, struct Equatable const* rhs) {
  return operator==(*lhs, *rhs);
}

static_assert(CRUBIT_SIZEOF(struct Comparable) == 4);
static_assert(alignof(struct Comparable) == 4);
static_assert(CRUBIT_OFFSET_OF(value, struct Comparable) == 0);

extern "C" void __rust_thunk___ZN10ComparableC1Ev(struct Comparable* __this) {
  crubit::construct_at(__this);
}

extern "C" bool __rust_thunk___ZN6mixinseqERK10ComparableS2_(
    struct Comparable const* lhs, struct Comparable const* rhs) {
  return operator==(*lhs, *rhs);
}

extern "C" bool __rust_thunk___ZN6mixinsltERK10ComparableS2_(
    struct Comparable const* lhs, struct Comparable const* rhs) {
  return operator<(*lhs, *rhs);
}

static_assert(CRUBIT_SIZEOF(struct ns::NamespacedEquatable) == 4);
static_assert(alignof(struct ns::NamespacedEquatable) == 4);
static_assert(CRUBIT_OFFSET_OF(value, struct ns::NamespacedEquatable) == 0);

extern "C" void __rust_thunk___ZN2ns19NamespacedEquatableC1Ev(
    struct ns::NamespacedEquatable* __this) {
  crubit::construct_at(__this);
}

extern "C" bool __rust_thunk___ZN6mixinseqERKN2ns19NamespacedEquatableES3_(
    struct ns::NamespacedEquatable const* lhs,
    struct ns::NamespacedEquatable const* rhs) {
  return operator==(*lhs, *rhs);
}

static_assert(sizeof(struct Outer) == 1);
static_assert(alignof(struct Outer) == 1);

extern "C" void __rust_thunk___ZN5OuterC1Ev(struct Outer* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct Outer::NestedEquatable) == 4);
static_assert(alignof(struct Outer::NestedEquatable) == 4);
static_assert(CRUBIT_OFFSET_OF(value, struct Outer::NestedEquatable) == 0);

extern "C" void __rust_thunk___ZN5Outer15NestedEquatableC1Ev(
    struct Outer::NestedEquatable* __this) {
  crubit::construct_at(__this);
}

extern "C" bool __rust_thunk___ZN6mixinseqERKN5Outer15NestedEquatableES3_(
    struct Outer::NestedEquatable const* lhs,
    struct Outer::NestedEquatable const* rhs) {
  return operator==(*lhs, *rhs);
}

static_assert(CRUBIT_SIZEOF(struct NonTrivialEquatable) == 4);
static_assert(alignof(struct NonTrivialEquatable) == 4);
static_assert(CRUBIT_OFFSET_OF(value, struct NonTrivialEquatable) == 0);

extern "C" void __rust_thunk___ZN19NonTrivialEquatableC1Ev(
    struct NonTrivialEquatable* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN19NonTrivialEquatableC1ERKS_(
    struct NonTrivialEquatable* __this,
    struct NonTrivialEquatable const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" struct NonTrivialEquatable*
__rust_thunk___ZN19NonTrivialEquatableaSERKS_(
    struct NonTrivialEquatable* __this,
    struct NonTrivialEquatable const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct NonTrivialEquatable &
               (::NonTrivialEquatable::*)(struct NonTrivialEquatable const&)) &
              ::NonTrivialEquatable::operator=);

extern "C" void __rust_thunk___ZN19NonTrivialEquatableD1Ev(
    struct NonTrivialEquatable* __this) {
  std::destroy_at(__this);
}

extern "C" bool __rust_thunk___ZN6mixinseqERK19NonTrivialEquatableS2_(
    struct NonTrivialEquatable const* lhs,
    struct NonTrivialEquatable const* rhs) {
  return operator==(*lhs, *rhs);
}

static_assert(CRUBIT_SIZEOF(struct NotEquatable) == 4);
static_assert(alignof(struct NotEquatable) == 4);
static_assert(CRUBIT_OFFSET_OF(value, struct NotEquatable) == 0);

extern "C" void __rust_thunk___ZN12NotEquatableC1Ev(
    struct NotEquatable* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct IntEquatable) == 4);
static_assert(alignof(struct IntEquatable) == 4);
static_assert(CRUBIT_OFFSET_OF(value, struct IntEquatable) == 0);

extern "C" void __rust_thunk___ZN12IntEquatableC1Ev(
    struct IntEquatable* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
