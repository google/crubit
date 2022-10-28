// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:operators_cc

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
extern "C" void __rust_thunk___ZN18AddableConstMemberC1EOS_(
    class AddableConstMember* __this, class AddableConstMember* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN21AddableNonConstMemberC1Ev(
    class AddableNonConstMember* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN21AddableNonConstMemberC1EOS_(
    class AddableNonConstMember* __this,
    class AddableNonConstMember* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN13AddableFriendC1Ev(
    class AddableFriend* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN13AddableFriendC1EOS_(
    class AddableFriend* __this, class AddableFriend* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN11AddableFreeC1Ev(class AddableFree* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN11AddableFreeC1EOS_(
    class AddableFree* __this, class AddableFree* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN10OverloadedC1Ev(class Overloaded* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN10OverloadedC1EOS_(
    class Overloaded* __this, class Overloaded* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN15IncompatibleLHSC1Ev(
    class IncompatibleLHS* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN15IncompatibleLHSC1EOS_(
    class IncompatibleLHS* __this, class IncompatibleLHS* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN18AddableReturnsVoidC1Ev(
    class AddableReturnsVoid* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN18AddableReturnsVoidC1EOS_(
    class AddableReturnsVoid* __this, class AddableReturnsVoid* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN26AddableConstMemberNonunpinC1Ev(
    class AddableConstMemberNonunpin* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN26AddableConstMemberNonunpinC1ERKS_(
    class AddableConstMemberNonunpin* __this,
    const class AddableConstMemberNonunpin* __param_0) {
  crubit::construct_at(__this, *__param_0);
}
extern "C" class AddableConstMemberNonunpin*
__rust_thunk___ZN26AddableConstMemberNonunpinaSERKS_(
    class AddableConstMemberNonunpin* __this,
    const class AddableConstMemberNonunpin* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" void __rust_thunk___ZNK26AddableConstMemberNonunpinplERKS_(
    class AddableConstMemberNonunpin* __return,
    const class AddableConstMemberNonunpin* __this,
    const class AddableConstMemberNonunpin* rhs) {
  new (__return) auto(__this->operator+(*rhs));
}
extern "C" void __rust_thunk___ZN26AddableConstMemberNonunpinD1Ev(
    class AddableConstMemberNonunpin* __this) {
  std::destroy_at(__this);
}
extern "C" void __rust_thunk___ZN18AddAssignMemberIntC1Ev(
    struct AddAssignMemberInt* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN18AddAssignMemberIntC1EOS_(
    struct AddAssignMemberInt* __this, struct AddAssignMemberInt* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN25AddAssignMemberByConstRefC1Ev(
    struct AddAssignMemberByConstRef* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN25AddAssignMemberByConstRefC1EOS_(
    struct AddAssignMemberByConstRef* __this,
    struct AddAssignMemberByConstRef* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN23AddAssignFreeByConstRefC1Ev(
    struct AddAssignFreeByConstRef* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN23AddAssignFreeByConstRefC1EOS_(
    struct AddAssignFreeByConstRef* __this,
    struct AddAssignFreeByConstRef* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN20AddAssignFreeByValueC1Ev(
    struct AddAssignFreeByValue* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN20AddAssignFreeByValueC1EOS_(
    struct AddAssignFreeByValue* __this,
    struct AddAssignFreeByValue* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN25AddAssignFriendByConstRefC1Ev(
    struct AddAssignFriendByConstRef* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN25AddAssignFriendByConstRefC1EOS_(
    struct AddAssignFriendByConstRef* __this,
    struct AddAssignFriendByConstRef* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN22AddAssignFriendByValueC1Ev(
    struct AddAssignFriendByValue* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN22AddAssignFriendByValueC1EOS_(
    struct AddAssignFriendByValue* __this,
    struct AddAssignFriendByValue* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN30AddAssignProhibitedConstMemberC1Ev(
    struct AddAssignProhibitedConstMember* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN30AddAssignProhibitedConstMemberC1EOS_(
    struct AddAssignProhibitedConstMember* __this,
    struct AddAssignProhibitedConstMember* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN33AddAssignProhibitedFriendConstLhsC1Ev(
    struct AddAssignProhibitedFriendConstLhs* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN33AddAssignProhibitedFriendConstLhsC1EOS_(
    struct AddAssignProhibitedFriendConstLhs* __this,
    struct AddAssignProhibitedFriendConstLhs* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN13ManyOperatorsC1Ev(
    struct ManyOperators* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN13ManyOperatorsC1EOS_(
    struct ManyOperators* __this, struct ManyOperators* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

static_assert(sizeof(class AddableConstMember) == 4);
static_assert(alignof(class AddableConstMember) == 4);

static_assert(sizeof(class AddableNonConstMember) == 4);
static_assert(alignof(class AddableNonConstMember) == 4);

static_assert(sizeof(class AddableFriend) == 4);
static_assert(alignof(class AddableFriend) == 4);

static_assert(sizeof(class AddableFree) == 1);
static_assert(alignof(class AddableFree) == 1);

static_assert(sizeof(class Overloaded) == 1);
static_assert(alignof(class Overloaded) == 1);

static_assert(sizeof(class IncompatibleLHS) == 1);
static_assert(alignof(class IncompatibleLHS) == 1);

static_assert(sizeof(class AddableReturnsVoid) == 4);
static_assert(alignof(class AddableReturnsVoid) == 4);

static_assert(sizeof(class AddableConstMemberNonunpin) == 4);
static_assert(alignof(class AddableConstMemberNonunpin) == 4);

static_assert(sizeof(struct AddAssignMemberInt) == 1);
static_assert(alignof(struct AddAssignMemberInt) == 1);

static_assert(sizeof(struct AddAssignMemberByConstRef) == 1);
static_assert(alignof(struct AddAssignMemberByConstRef) == 1);

static_assert(sizeof(struct AddAssignFreeByConstRef) == 1);
static_assert(alignof(struct AddAssignFreeByConstRef) == 1);

static_assert(sizeof(struct AddAssignFreeByValue) == 1);
static_assert(alignof(struct AddAssignFreeByValue) == 1);

static_assert(sizeof(struct AddAssignFriendByConstRef) == 1);
static_assert(alignof(struct AddAssignFriendByConstRef) == 1);

static_assert(sizeof(struct AddAssignFriendByValue) == 1);
static_assert(alignof(struct AddAssignFriendByValue) == 1);

static_assert(sizeof(struct AddAssignProhibitedConstMember) == 1);
static_assert(alignof(struct AddAssignProhibitedConstMember) == 1);

static_assert(sizeof(struct AddAssignProhibitedFriendConstLhs) == 1);
static_assert(alignof(struct AddAssignProhibitedFriendConstLhs) == 1);

static_assert(sizeof(struct ManyOperators) == 1);
static_assert(alignof(struct ManyOperators) == 1);

#pragma clang diagnostic pop
