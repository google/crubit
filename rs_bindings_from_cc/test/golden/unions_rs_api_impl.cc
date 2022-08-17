// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/unions.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN10EmptyUnionC1Ev(union EmptyUnion* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN10EmptyUnionC1EOS_(
    union EmptyUnion* __this, union EmptyUnion* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
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
extern "C" void __rust_thunk___ZN13NonEmptyUnionC1Ev(
    union NonEmptyUnion* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN13NonEmptyUnionC1EOS_(
    union NonEmptyUnion* __this, union NonEmptyUnion* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" union NonCopyUnion2* __rust_thunk___ZN13NonCopyUnion2aSERKS_(
    union NonCopyUnion2* __this, const union NonCopyUnion2* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" union NonCopyUnion2* __rust_thunk___ZN13NonCopyUnion2aSEOS_(
    union NonCopyUnion2* __this, union NonCopyUnion2* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN20UnionWithOpaqueFieldC1Ev(
    union UnionWithOpaqueField* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN20UnionWithOpaqueFieldC1EOS_(
    union UnionWithOpaqueField* __this, union UnionWithOpaqueField* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN21TrivialButInheritableC1Ev(
    struct TrivialButInheritable* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN21TrivialButInheritableC1ERKS_(
    struct TrivialButInheritable* __this,
    const struct TrivialButInheritable* __param_0) {
  crubit::construct_at(__this, *__param_0);
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
extern "C" void __rust_thunk___ZN20UnionWithInheritableC1Ev(
    union UnionWithInheritable* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN20UnionWithInheritableC1ERKS_(
    union UnionWithInheritable* __this,
    const union UnionWithInheritable* __param_0) {
  crubit::construct_at(__this, *__param_0);
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

static_assert(sizeof(union EmptyUnion) == 1);
static_assert(alignof(union EmptyUnion) == 1);

static_assert(sizeof(struct Nontrivial) == 4);
static_assert(alignof(struct Nontrivial) == 4);
static_assert(CRUBIT_OFFSET_OF(field, struct Nontrivial) == 0);

static_assert(sizeof(struct TriviallyCopyableButNontriviallyDestructible) == 1);
static_assert(alignof(struct TriviallyCopyableButNontriviallyDestructible) ==
              1);

static_assert(sizeof(union NonEmptyUnion) == 8);
static_assert(alignof(union NonEmptyUnion) == 8);
static_assert(CRUBIT_OFFSET_OF(bool_field, union NonEmptyUnion) == 0);
static_assert(CRUBIT_OFFSET_OF(char_field, union NonEmptyUnion) == 0);
static_assert(CRUBIT_OFFSET_OF(int_field, union NonEmptyUnion) == 0);
static_assert(CRUBIT_OFFSET_OF(long_long_field, union NonEmptyUnion) == 0);

static_assert(sizeof(union NonCopyUnion) == 4);
static_assert(alignof(union NonCopyUnion) == 4);
static_assert(CRUBIT_OFFSET_OF(trivial_member, union NonCopyUnion) == 0);
static_assert(CRUBIT_OFFSET_OF(nontrivial_member, union NonCopyUnion) == 0);

static_assert(sizeof(union NonCopyUnion2) == 1);
static_assert(alignof(union NonCopyUnion2) == 1);
static_assert(CRUBIT_OFFSET_OF(trivial_member, union NonCopyUnion2) == 0);
static_assert(CRUBIT_OFFSET_OF(nontrivial_member, union NonCopyUnion2) == 0);

static_assert(sizeof(union UnionWithOpaqueField) == 42);
static_assert(alignof(union UnionWithOpaqueField) == 1);
static_assert(CRUBIT_OFFSET_OF(constant_array_field_not_yet_supported,
                               union UnionWithOpaqueField) == 0);

static_assert(sizeof(struct TrivialButInheritable) == 4);
static_assert(alignof(struct TrivialButInheritable) == 4);
static_assert(CRUBIT_OFFSET_OF(x, struct TrivialButInheritable) == 0);

static_assert(sizeof(union UnionWithInheritable) == 4);
static_assert(alignof(union UnionWithInheritable) == 4);
static_assert(CRUBIT_OFFSET_OF(t, union UnionWithInheritable) == 0);

#pragma clang diagnostic pop
