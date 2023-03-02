// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc
// Features: experimental, supported

#include <cstddef>
#include <memory>

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/nontrivial_type.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN10NontrivialaSEf(struct Nontrivial* __return,
                                                  struct Nontrivial* __this,
                                                  float __param_0) {
  new (__return) auto(__this->operator=(__param_0));
}
extern "C" void __rust_thunk___ZN16NontrivialInlineC1Ev(
    struct NontrivialInline* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN16NontrivialInlineC1Ei(
    struct NontrivialInline* __this, int field) {
  crubit::construct_at(__this, field);
}
extern "C" void __rust_thunk___ZN16NontrivialInlineC1Eii(
    struct NontrivialInline* __this, int field, int unused) {
  crubit::construct_at(__this, field, unused);
}
extern "C" void __rust_thunk___ZN16NontrivialInlineC1ERKS_(
    struct NontrivialInline* __this, const struct NontrivialInline* __param_0) {
  crubit::construct_at(__this, *__param_0);
}
extern "C" void __rust_thunk___ZN16NontrivialInlineC1EOS_(
    struct NontrivialInline* __this, struct NontrivialInline* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" struct NontrivialInline* __rust_thunk___ZN16NontrivialInlineaSERKS_(
    struct NontrivialInline* __this, const struct NontrivialInline* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" struct NontrivialInline* __rust_thunk___ZN16NontrivialInlineaSEOS_(
    struct NontrivialInline* __this, struct NontrivialInline* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}
extern "C" struct NontrivialInline* __rust_thunk___ZN16NontrivialInlineaSEi(
    struct NontrivialInline* __this, int __param_0) {
  return &__this->operator=(__param_0);
}
extern "C" void __rust_thunk___ZN16NontrivialInlineD1Ev(
    struct NontrivialInline* __this) {
  std::destroy_at(__this);
}
extern "C" void __rust_thunk___ZN16NontrivialInline14MemberFunctionEv(
    struct NontrivialInline* __this) {
  __this->MemberFunction();
}
extern "C" void __rust_thunk___ZN17NontrivialMembersC1Ev(
    struct NontrivialMembers* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN17NontrivialMembersC1ERKS_(
    struct NontrivialMembers* __this,
    const struct NontrivialMembers* __param_0) {
  crubit::construct_at(__this, *__param_0);
}
extern "C" void __rust_thunk___ZN17NontrivialMembersC1EOS_(
    struct NontrivialMembers* __this, struct NontrivialMembers* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN17NontrivialMembersD1Ev(
    struct NontrivialMembers* __this) {
  std::destroy_at(__this);
}
extern "C" struct NontrivialMembers*
__rust_thunk___ZN17NontrivialMembersaSERKS_(
    struct NontrivialMembers* __this,
    const struct NontrivialMembers* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" struct NontrivialMembers* __rust_thunk___ZN17NontrivialMembersaSEOS_(
    struct NontrivialMembers* __this, struct NontrivialMembers* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}
extern "C" void __rust_thunk___Z12TakesByValue10Nontrivial(
    struct Nontrivial* __return, struct Nontrivial* nontrivial) {
  new (__return) auto(TakesByValue(std::move(*nontrivial)));
}
extern "C" void __rust_thunk___Z18TakesByValueInline16NontrivialInline(
    struct NontrivialInline* __return, struct NontrivialInline* nontrivial) {
  new (__return) auto(TakesByValueInline(std::move(*nontrivial)));
}
extern "C" void __rust_thunk___ZN17NontrivialByValueC1ERKS_(
    struct NontrivialByValue* __this, const struct NontrivialByValue* other) {
  crubit::construct_at(__this, *other);
}
extern "C" void __rust_thunk___ZN17NontrivialByValueC1EOS_(
    struct NontrivialByValue* __this, struct NontrivialByValue* other) {
  crubit::construct_at(__this, std::move(*other));
}
extern "C" struct NontrivialByValue*
__rust_thunk___ZN17NontrivialByValueaSERKS_(
    struct NontrivialByValue* __this, const struct NontrivialByValue* other) {
  return &__this->operator=(*other);
}
extern "C" struct NontrivialByValue* __rust_thunk___ZN17NontrivialByValueaSEOS_(
    struct NontrivialByValue* __this, struct NontrivialByValue* other) {
  return &__this->operator=(std::move(*other));
}
extern "C" void __rust_thunk___ZN17NontrivialByValueaSE10Nontrivial(
    struct NontrivialByValue* __return, struct NontrivialByValue* __this,
    struct Nontrivial* other) {
  new (__return) auto(__this->operator=(std::move(*other)));
}
extern "C" void __rust_thunk___Z24ReturnsNonmovableByValuev(
    struct Nonmovable* __return) {
  new (__return) auto(ReturnsNonmovableByValue());
}

static_assert(sizeof(struct Nontrivial) == 4);
static_assert(alignof(struct Nontrivial) == 4);
static_assert(CRUBIT_OFFSET_OF(field, struct Nontrivial) == 0);

static_assert(sizeof(struct NontrivialInline) == 4);
static_assert(alignof(struct NontrivialInline) == 4);
static_assert(CRUBIT_OFFSET_OF(field, struct NontrivialInline) == 0);

static_assert(sizeof(struct NontrivialMembers) == 4);
static_assert(alignof(struct NontrivialMembers) == 4);
static_assert(CRUBIT_OFFSET_OF(nontrivial_member, struct NontrivialMembers) ==
              0);

static_assert(sizeof(struct NontrivialUnpin) == 4);
static_assert(alignof(struct NontrivialUnpin) == 4);
static_assert(CRUBIT_OFFSET_OF(field, struct NontrivialUnpin) == 0);

static_assert(sizeof(struct NontrivialByValue) == 1);
static_assert(alignof(struct NontrivialByValue) == 1);

static_assert(sizeof(struct Nonmovable) == 1);
static_assert(alignof(struct Nonmovable) == 1);

#pragma clang diagnostic pop
