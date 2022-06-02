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
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN10EmptyUnionC1ERKS_(
    union EmptyUnion* __this, const union EmptyUnion& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN10EmptyUnionC1EOS_(
    union EmptyUnion* __this, union EmptyUnion&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN10EmptyUnionD1Ev(union EmptyUnion* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" union EmptyUnion& __rust_thunk___ZN10EmptyUnionaSERKS_(
    union EmptyUnion* __this, const union EmptyUnion& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" union EmptyUnion& __rust_thunk___ZN10EmptyUnionaSEOS_(
    union EmptyUnion* __this, union EmptyUnion&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN10NontrivialD1Ev(class Nontrivial* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class TriviallyCopyableButNontriviallyDestructible&
__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleaSERKS_(
    class TriviallyCopyableButNontriviallyDestructible* __this,
    const class TriviallyCopyableButNontriviallyDestructible& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleC1ERKS_(
    class TriviallyCopyableButNontriviallyDestructible* __this,
    const class TriviallyCopyableButNontriviallyDestructible& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev(
    class TriviallyCopyableButNontriviallyDestructible* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN13NonEmptyUnionC1Ev(
    union NonEmptyUnion* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN13NonEmptyUnionC1ERKS_(
    union NonEmptyUnion* __this, const union NonEmptyUnion& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN13NonEmptyUnionC1EOS_(
    union NonEmptyUnion* __this, union NonEmptyUnion&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN13NonEmptyUnionD1Ev(
    union NonEmptyUnion* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" union NonEmptyUnion& __rust_thunk___ZN13NonEmptyUnionaSERKS_(
    union NonEmptyUnion* __this, const union NonEmptyUnion& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" union NonEmptyUnion& __rust_thunk___ZN13NonEmptyUnionaSEOS_(
    union NonEmptyUnion* __this, union NonEmptyUnion&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN12NonCopyUnionD1Ev(
    union NonCopyUnion* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN13NonCopyUnion2C1ERKS_(
    union NonCopyUnion2* __this, const union NonCopyUnion2& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN13NonCopyUnion2C1EOS_(
    union NonCopyUnion2* __this, union NonCopyUnion2&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" union NonCopyUnion2& __rust_thunk___ZN13NonCopyUnion2aSERKS_(
    union NonCopyUnion2* __this, const union NonCopyUnion2& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" union NonCopyUnion2& __rust_thunk___ZN13NonCopyUnion2aSEOS_(
    union NonCopyUnion2* __this, union NonCopyUnion2&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN20UnionWithOpaqueFieldC1Ev(
    union UnionWithOpaqueField* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN20UnionWithOpaqueFieldC1ERKS_(
    union UnionWithOpaqueField* __this,
    const union UnionWithOpaqueField& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN20UnionWithOpaqueFieldC1EOS_(
    union UnionWithOpaqueField* __this,
    union UnionWithOpaqueField&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN20UnionWithOpaqueFieldD1Ev(
    union UnionWithOpaqueField* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" union UnionWithOpaqueField&
__rust_thunk___ZN20UnionWithOpaqueFieldaSERKS_(
    union UnionWithOpaqueField* __this,
    const union UnionWithOpaqueField& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" union UnionWithOpaqueField&
__rust_thunk___ZN20UnionWithOpaqueFieldaSEOS_(
    union UnionWithOpaqueField* __this,
    union UnionWithOpaqueField&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}

static_assert(sizeof(union EmptyUnion) == 1);
static_assert(alignof(union EmptyUnion) == 1);

static_assert(sizeof(class Nontrivial) == 4);
static_assert(alignof(class Nontrivial) == 4);
static_assert(CRUBIT_OFFSET_OF(field, class Nontrivial) == 0);

static_assert(sizeof(class TriviallyCopyableButNontriviallyDestructible) == 1);
static_assert(alignof(class TriviallyCopyableButNontriviallyDestructible) == 1);

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

#pragma clang diagnostic pop
