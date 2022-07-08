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
extern "C" void __rust_thunk___ZN16NontrivialInlineC1Ev(
    class NontrivialInline* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN16NontrivialInlineC1Ei(
    class NontrivialInline* __this, int field) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(field)>(field));
}
extern "C" void __rust_thunk___ZN16NontrivialInlineC1Eii(
    class NontrivialInline* __this, int field, int unused) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(field)>(field),
                       std::forward<decltype(unused)>(unused));
}
extern "C" void __rust_thunk___ZN16NontrivialInlineC1ERKS_(
    class NontrivialInline* __this, const class NontrivialInline& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN16NontrivialInlineC1EOS_(
    class NontrivialInline* __this, class NontrivialInline&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class NontrivialInline& __rust_thunk___ZN16NontrivialInlineaSERKS_(
    class NontrivialInline* __this, const class NontrivialInline& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class NontrivialInline& __rust_thunk___ZN16NontrivialInlineaSEOS_(
    class NontrivialInline* __this, class NontrivialInline&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class NontrivialInline& __rust_thunk___ZN16NontrivialInlineaSEi(
    class NontrivialInline* __this, int __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN16NontrivialInlineD1Ev(
    class NontrivialInline* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN16NontrivialInline14MemberFunctionEv(
    class NontrivialInline* __this) {
  __this->MemberFunction();
}
extern "C" void __rust_thunk___ZN17NontrivialMembersC1Ev(
    class NontrivialMembers* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN17NontrivialMembersC1ERKS_(
    class NontrivialMembers* __this, const class NontrivialMembers& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN17NontrivialMembersC1EOS_(
    class NontrivialMembers* __this, class NontrivialMembers&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN17NontrivialMembersD1Ev(
    class NontrivialMembers* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class NontrivialMembers& __rust_thunk___ZN17NontrivialMembersaSERKS_(
    class NontrivialMembers* __this, const class NontrivialMembers& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class NontrivialMembers& __rust_thunk___ZN17NontrivialMembersaSEOS_(
    class NontrivialMembers* __this, class NontrivialMembers&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN17NontrivialByValueC1Ev(
    class NontrivialByValue* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN17NontrivialByValueC1ERKS_(
    class NontrivialByValue* __this, const class NontrivialByValue& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN17NontrivialByValueD1Ev(
    class NontrivialByValue* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}

static_assert(sizeof(class Nontrivial) == 4);
static_assert(alignof(class Nontrivial) == 4);
static_assert(CRUBIT_OFFSET_OF(field, class Nontrivial) == 0);

static_assert(sizeof(class NontrivialInline) == 4);
static_assert(alignof(class NontrivialInline) == 4);
static_assert(CRUBIT_OFFSET_OF(field, class NontrivialInline) == 0);

static_assert(sizeof(class NontrivialMembers) == 4);
static_assert(alignof(class NontrivialMembers) == 4);
static_assert(CRUBIT_OFFSET_OF(nontrivial_member, class NontrivialMembers) ==
              0);

static_assert(sizeof(class NontrivialUnpin) == 4);
static_assert(alignof(class NontrivialUnpin) == 4);
static_assert(CRUBIT_OFFSET_OF(field, class NontrivialUnpin) == 0);

static_assert(sizeof(class NontrivialByValue) == 1);
static_assert(alignof(class NontrivialByValue) == 1);

#pragma clang diagnostic pop
