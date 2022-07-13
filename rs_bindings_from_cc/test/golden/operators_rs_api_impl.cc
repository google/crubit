// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/operators.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN18AddableConstMemberC1Ev(
    class AddableConstMember* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN18AddableConstMemberC1ERKS_(
    class AddableConstMember* __this,
    const class AddableConstMember* __param_0) {
  crubit::construct_at(__this, *__param_0);
}
extern "C" void __rust_thunk___ZN18AddableConstMemberC1EOS_(
    class AddableConstMember* __this, class AddableConstMember* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN18AddableConstMemberD1Ev(
    class AddableConstMember* __this) {
  std::destroy_at(__this);
}
extern "C" class AddableConstMember*
__rust_thunk___ZN18AddableConstMemberaSERKS_(
    class AddableConstMember* __this,
    const class AddableConstMember* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" class AddableConstMember*
__rust_thunk___ZN18AddableConstMemberaSEOS_(
    class AddableConstMember* __this, class AddableConstMember* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN21AddableNonConstMemberC1Ev(
    class AddableNonConstMember* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN21AddableNonConstMemberC1ERKS_(
    class AddableNonConstMember* __this,
    const class AddableNonConstMember* __param_0) {
  crubit::construct_at(__this, *__param_0);
}
extern "C" void __rust_thunk___ZN21AddableNonConstMemberC1EOS_(
    class AddableNonConstMember* __this,
    class AddableNonConstMember* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN21AddableNonConstMemberD1Ev(
    class AddableNonConstMember* __this) {
  std::destroy_at(__this);
}
extern "C" class AddableNonConstMember*
__rust_thunk___ZN21AddableNonConstMemberaSERKS_(
    class AddableNonConstMember* __this,
    const class AddableNonConstMember* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" class AddableNonConstMember*
__rust_thunk___ZN21AddableNonConstMemberaSEOS_(
    class AddableNonConstMember* __this,
    class AddableNonConstMember* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN13AddableFriendC1Ev(
    class AddableFriend* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN13AddableFriendC1ERKS_(
    class AddableFriend* __this, const class AddableFriend* __param_0) {
  crubit::construct_at(__this, *__param_0);
}
extern "C" void __rust_thunk___ZN13AddableFriendC1EOS_(
    class AddableFriend* __this, class AddableFriend* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN13AddableFriendD1Ev(
    class AddableFriend* __this) {
  std::destroy_at(__this);
}
extern "C" class AddableFriend* __rust_thunk___ZN13AddableFriendaSERKS_(
    class AddableFriend* __this, const class AddableFriend* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" class AddableFriend* __rust_thunk___ZN13AddableFriendaSEOS_(
    class AddableFriend* __this, class AddableFriend* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN11AddableFreeC1Ev(class AddableFree* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN11AddableFreeC1ERKS_(
    class AddableFree* __this, const class AddableFree* __param_0) {
  crubit::construct_at(__this, *__param_0);
}
extern "C" void __rust_thunk___ZN11AddableFreeC1EOS_(
    class AddableFree* __this, class AddableFree* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN11AddableFreeD1Ev(class AddableFree* __this) {
  std::destroy_at(__this);
}
extern "C" class AddableFree* __rust_thunk___ZN11AddableFreeaSERKS_(
    class AddableFree* __this, const class AddableFree* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" class AddableFree* __rust_thunk___ZN11AddableFreeaSEOS_(
    class AddableFree* __this, class AddableFree* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(sizeof(class AddableConstMember) == 4);
static_assert(alignof(class AddableConstMember) == 4);

static_assert(sizeof(class AddableNonConstMember) == 4);
static_assert(alignof(class AddableNonConstMember) == 4);

static_assert(sizeof(class AddableFriend) == 4);
static_assert(alignof(class AddableFriend) == 4);

static_assert(sizeof(class AddableFree) == 1);
static_assert(alignof(class AddableFree) == 1);

#pragma clang diagnostic pop
