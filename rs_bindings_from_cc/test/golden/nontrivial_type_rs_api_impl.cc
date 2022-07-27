// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
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
    struct Nontrivial* nontrivial) {
  TakesByValue(std::move(*nontrivial));
}
extern "C" void __rust_thunk___Z18TakesByValueInline16NontrivialInline(
    struct NontrivialInline* nontrivial) {
  TakesByValueInline(std::move(*nontrivial));
}
extern "C" void __rust_thunk___Z14ReturnsByValuev(struct Nontrivial* __return) {
  new (__return) auto(ReturnsByValue());
}
extern "C" void __rust_thunk___ZN17NontrivialByValueC1Ev(
    struct NontrivialByValue* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN17NontrivialByValueC1ERKS_(
    struct NontrivialByValue* __this,
    const struct NontrivialByValue* __param_0) {
  crubit::construct_at(__this, *__param_0);
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
