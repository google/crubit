// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unions_cc
// Features: experimental, non_extern_c_functions, supported

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/unions.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(union EmptyUnion) == 1);
static_assert(alignof(union EmptyUnion) == 1);

extern "C" void __rust_thunk___ZN10EmptyUnionC1Ev(union EmptyUnion* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN10EmptyUnionC1EOS_(
    union EmptyUnion* __this, union EmptyUnion* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" union EmptyUnion* __rust_thunk___ZN10EmptyUnionaSERKS_(
    union EmptyUnion* __this, const union EmptyUnion* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" union EmptyUnion* __rust_thunk___ZN10EmptyUnionaSEOS_(
    union EmptyUnion* __this, union EmptyUnion* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(struct Nontrivial) == 4);
static_assert(alignof(struct Nontrivial) == 4);
static_assert(CRUBIT_OFFSET_OF(field, struct Nontrivial) == 0);

static_assert(sizeof(struct TriviallyCopyableButNontriviallyDestructible) == 1);
static_assert(alignof(struct TriviallyCopyableButNontriviallyDestructible) ==
              1);

extern "C" struct TriviallyCopyableButNontriviallyDestructible*
__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleaSERKS_(
    struct TriviallyCopyableButNontriviallyDestructible* __this,
    const struct TriviallyCopyableButNontriviallyDestructible* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" void
__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleC1ERKS_(
    struct TriviallyCopyableButNontriviallyDestructible* __this,
    const struct TriviallyCopyableButNontriviallyDestructible* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" void
__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev(
    struct TriviallyCopyableButNontriviallyDestructible* __this) {
  std::destroy_at(__this);
}

static_assert(CRUBIT_SIZEOF(union NonEmptyUnion) == 8);
static_assert(alignof(union NonEmptyUnion) == 8);
static_assert(CRUBIT_OFFSET_OF(bool_field, union NonEmptyUnion) == 0);
static_assert(CRUBIT_OFFSET_OF(char_field, union NonEmptyUnion) == 0);
static_assert(CRUBIT_OFFSET_OF(int_field, union NonEmptyUnion) == 0);
static_assert(CRUBIT_OFFSET_OF(long_long_field, union NonEmptyUnion) == 0);

extern "C" void __rust_thunk___ZN13NonEmptyUnionC1Ev(
    union NonEmptyUnion* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN13NonEmptyUnionC1EOS_(
    union NonEmptyUnion* __this, union NonEmptyUnion* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" union NonEmptyUnion* __rust_thunk___ZN13NonEmptyUnionaSERKS_(
    union NonEmptyUnion* __this, const union NonEmptyUnion* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" union NonEmptyUnion* __rust_thunk___ZN13NonEmptyUnionaSEOS_(
    union NonEmptyUnion* __this, union NonEmptyUnion* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(union NonCopyUnion) == 4);
static_assert(alignof(union NonCopyUnion) == 4);
static_assert(CRUBIT_OFFSET_OF(trivial_member, union NonCopyUnion) == 0);
static_assert(CRUBIT_OFFSET_OF(nontrivial_member, union NonCopyUnion) == 0);

static_assert(sizeof(union NonCopyUnion2) == 1);
static_assert(alignof(union NonCopyUnion2) == 1);
static_assert(CRUBIT_OFFSET_OF(trivial_member, union NonCopyUnion2) == 0);
static_assert(CRUBIT_OFFSET_OF(nontrivial_member, union NonCopyUnion2) == 0);

extern "C" union NonCopyUnion2* __rust_thunk___ZN13NonCopyUnion2aSERKS_(
    union NonCopyUnion2* __this, const union NonCopyUnion2* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" union NonCopyUnion2* __rust_thunk___ZN13NonCopyUnion2aSEOS_(
    union NonCopyUnion2* __this, union NonCopyUnion2* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(sizeof(union UnionWithOpaqueField) == 42);
static_assert(alignof(union UnionWithOpaqueField) == 1);
static_assert(CRUBIT_OFFSET_OF(constant_array_field_not_yet_supported,
                               union UnionWithOpaqueField) == 0);

extern "C" void __rust_thunk___ZN20UnionWithOpaqueFieldC1Ev(
    union UnionWithOpaqueField* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN20UnionWithOpaqueFieldC1EOS_(
    union UnionWithOpaqueField* __this, union UnionWithOpaqueField* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" union UnionWithOpaqueField*
__rust_thunk___ZN20UnionWithOpaqueFieldaSERKS_(
    union UnionWithOpaqueField* __this,
    const union UnionWithOpaqueField* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" union UnionWithOpaqueField*
__rust_thunk___ZN20UnionWithOpaqueFieldaSEOS_(
    union UnionWithOpaqueField* __this, union UnionWithOpaqueField* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(struct TrivialButInheritable) == 4);
static_assert(alignof(struct TrivialButInheritable) == 4);
static_assert(CRUBIT_OFFSET_OF(x, struct TrivialButInheritable) == 0);

extern "C" void __rust_thunk___ZN21TrivialButInheritableC1Ev(
    struct TrivialButInheritable* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN21TrivialButInheritableC1EOS_(
    struct TrivialButInheritable* __this,
    struct TrivialButInheritable* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct TrivialButInheritable*
__rust_thunk___ZN21TrivialButInheritableaSERKS_(
    struct TrivialButInheritable* __this,
    const struct TrivialButInheritable* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct TrivialButInheritable*
__rust_thunk___ZN21TrivialButInheritableaSEOS_(
    struct TrivialButInheritable* __this,
    struct TrivialButInheritable* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(union UnionWithInheritable) == 4);
static_assert(alignof(union UnionWithInheritable) == 4);
static_assert(CRUBIT_OFFSET_OF(t, union UnionWithInheritable) == 0);

extern "C" void __rust_thunk___ZN20UnionWithInheritableC1Ev(
    union UnionWithInheritable* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN20UnionWithInheritableC1EOS_(
    union UnionWithInheritable* __this, union UnionWithInheritable* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" union UnionWithInheritable*
__rust_thunk___ZN20UnionWithInheritableaSERKS_(
    union UnionWithInheritable* __this,
    const union UnionWithInheritable* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" union UnionWithInheritable*
__rust_thunk___ZN20UnionWithInheritableaSEOS_(
    union UnionWithInheritable* __this, union UnionWithInheritable* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(sizeof(TypedefUnion) == 1);
static_assert(alignof(TypedefUnion) == 1);
static_assert(CRUBIT_OFFSET_OF(trivial_member, TypedefUnion) == 0);

extern "C" void __rust_thunk___ZN12TypedefUnionC1Ev(TypedefUnion* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN12TypedefUnionC1EOS_(TypedefUnion* __this,
                                                      TypedefUnion* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" TypedefUnion* __rust_thunk___ZN12TypedefUnionaSERKS_(
    TypedefUnion* __this, const TypedefUnion* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" TypedefUnion* __rust_thunk___ZN12TypedefUnionaSEOS_(
    TypedefUnion* __this, TypedefUnion* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(TypedefUnionWithInheritable) == 4);
static_assert(alignof(TypedefUnionWithInheritable) == 4);
static_assert(CRUBIT_OFFSET_OF(t, TypedefUnionWithInheritable) == 0);

extern "C" void __rust_thunk___ZN27TypedefUnionWithInheritableC1Ev(
    TypedefUnionWithInheritable* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN27TypedefUnionWithInheritableC1EOS_(
    TypedefUnionWithInheritable* __this,
    TypedefUnionWithInheritable* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" TypedefUnionWithInheritable*
__rust_thunk___ZN27TypedefUnionWithInheritableaSERKS_(
    TypedefUnionWithInheritable* __this,
    const TypedefUnionWithInheritable* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" TypedefUnionWithInheritable*
__rust_thunk___ZN27TypedefUnionWithInheritableaSEOS_(
    TypedefUnionWithInheritable* __this,
    TypedefUnionWithInheritable* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

#pragma clang diagnostic pop
