// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:operators_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/operators.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(class AddableConstMember) == 4);
static_assert(alignof(class AddableConstMember) == 4);

extern "C" void __rust_thunk___ZN18AddableConstMemberC1Ev(
    class AddableConstMember* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZNK18AddableConstMemberplERKS_(
    class AddableConstMember* __return, class AddableConstMember const* __this,
    class AddableConstMember const* rhs) {
  new (__return) auto(__this->operator+(*rhs));
}

static_assert((class AddableConstMember (::AddableConstMember::*)(
                  class AddableConstMember const&) const) &
              ::AddableConstMember::operator+);

static_assert(CRUBIT_SIZEOF(class AddableNonConstMember) == 4);
static_assert(alignof(class AddableNonConstMember) == 4);

extern "C" void __rust_thunk___ZN21AddableNonConstMemberC1Ev(
    class AddableNonConstMember* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN21AddableNonConstMemberplERKS_(
    class AddableNonConstMember* __return, class AddableNonConstMember* __this,
    class AddableNonConstMember const* rhs) {
  new (__return) auto(__this->operator+(*rhs));
}

static_assert((class AddableNonConstMember (::AddableNonConstMember::*)(
                  class AddableNonConstMember const&)) &
              ::AddableNonConstMember::operator+);

static_assert(CRUBIT_SIZEOF(class AddableFriend) == 4);
static_assert(alignof(class AddableFriend) == 4);

extern "C" void __rust_thunk___ZN13AddableFriendC1Ev(
    class AddableFriend* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZplRK13AddableFriendS1_(
    class AddableFriend* __return, class AddableFriend const* lhs,
    class AddableFriend const* rhs) {
  new (__return) auto(operator+(*lhs, *rhs));
}

static_assert(sizeof(class AddableFreeByConstRef) == 1);
static_assert(alignof(class AddableFreeByConstRef) == 1);

extern "C" void __rust_thunk___ZN21AddableFreeByConstRefC1Ev(
    class AddableFreeByConstRef* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class AddableFreeByMutRef) == 1);
static_assert(alignof(class AddableFreeByMutRef) == 1);

extern "C" void __rust_thunk___ZN19AddableFreeByMutRefC1Ev(
    class AddableFreeByMutRef* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class AddableFreeByValue) == 1);
static_assert(alignof(class AddableFreeByValue) == 1);

extern "C" void __rust_thunk___ZN18AddableFreeByValueC1Ev(
    class AddableFreeByValue* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class AddableFreeByRValueRef) == 1);
static_assert(alignof(class AddableFreeByRValueRef) == 1);

extern "C" void __rust_thunk___ZN22AddableFreeByRValueRefC1Ev(
    class AddableFreeByRValueRef* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZplRK21AddableFreeByConstRefS1_(
    class AddableFreeByConstRef* __return,
    class AddableFreeByConstRef const* lhs,
    class AddableFreeByConstRef const* rhs) {
  new (__return) auto(operator+(*lhs, *rhs));
}

static_assert(
    (class AddableFreeByConstRef (*)(class AddableFreeByConstRef const&,
                                     class AddableFreeByConstRef const&)) &
    ::operator+);

extern "C" void __rust_thunk___ZplR19AddableFreeByMutRefS0_(
    class AddableFreeByMutRef* __return, class AddableFreeByMutRef* lhs,
    class AddableFreeByMutRef* rhs) {
  new (__return) auto(operator+(*lhs, *rhs));
}

static_assert((class AddableFreeByMutRef (*)(class AddableFreeByMutRef&,
                                             class AddableFreeByMutRef&)) &
              ::operator+);

extern "C" void __rust_thunk___Zpl18AddableFreeByValueS_(
    class AddableFreeByValue* __return, class AddableFreeByValue* lhs,
    class AddableFreeByValue* rhs) {
  new (__return) auto(operator+(std::move(*lhs), std::move(*rhs)));
}

static_assert((class AddableFreeByValue (*)(class AddableFreeByValue,
                                            class AddableFreeByValue)) &
              ::operator+);

static_assert(sizeof(class Overloaded) == 1);
static_assert(alignof(class Overloaded) == 1);

extern "C" void __rust_thunk___ZN10OverloadedC1Ev(class Overloaded* __this) {
  crubit::construct_at(__this);
}

static_assert((int (*)(class Overloaded const&, int)) & ::operator+);

static_assert((int (*)(class Overloaded const&, unsigned int)) & ::operator+);

static_assert(sizeof(class IncompatibleLHS) == 1);
static_assert(alignof(class IncompatibleLHS) == 1);

extern "C" void __rust_thunk___ZN15IncompatibleLHSC1Ev(
    class IncompatibleLHS* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class AddableReturnsVoid) == 4);
static_assert(alignof(class AddableReturnsVoid) == 4);

extern "C" void __rust_thunk___ZN18AddableReturnsVoidC1Ev(
    class AddableReturnsVoid* __this) {
  crubit::construct_at(__this);
}

static_assert((void (::AddableReturnsVoid::*)(class AddableReturnsVoid const&)
                   const) &
              ::AddableReturnsVoid::operator+);

static_assert(CRUBIT_SIZEOF(class AddableConstMemberNonunpin) == 4);
static_assert(alignof(class AddableConstMemberNonunpin) == 4);

extern "C" void __rust_thunk___ZN26AddableConstMemberNonunpinC1Ev(
    class AddableConstMemberNonunpin* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN26AddableConstMemberNonunpinC1ERKS_(
    class AddableConstMemberNonunpin* __this,
    class AddableConstMemberNonunpin const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" class AddableConstMemberNonunpin*
__rust_thunk___ZN26AddableConstMemberNonunpinaSERKS_(
    class AddableConstMemberNonunpin* __this,
    class AddableConstMemberNonunpin const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((class AddableConstMemberNonunpin &
               (::AddableConstMemberNonunpin::*)(
                   class AddableConstMemberNonunpin const&)) &
              ::AddableConstMemberNonunpin::operator=);

extern "C" void __rust_thunk___ZNK26AddableConstMemberNonunpinplERKS_(
    class AddableConstMemberNonunpin* __return,
    class AddableConstMemberNonunpin const* __this,
    class AddableConstMemberNonunpin const* rhs) {
  new (__return) auto(__this->operator+(*rhs));
}

static_assert(
    (class AddableConstMemberNonunpin (::AddableConstMemberNonunpin::*)(
        class AddableConstMemberNonunpin const&) const) &
    ::AddableConstMemberNonunpin::operator+);

extern "C" void __rust_thunk___ZN26AddableConstMemberNonunpinD1Ev(
    class AddableConstMemberNonunpin* __this) {
  std::destroy_at(__this);
}

static_assert(sizeof(struct AddAssignMemberInt) == 1);
static_assert(alignof(struct AddAssignMemberInt) == 1);

extern "C" void __rust_thunk___ZN18AddAssignMemberIntC1Ev(
    struct AddAssignMemberInt* __this) {
  crubit::construct_at(__this);
}

static_assert((int (::AddAssignMemberInt::*)(int)) &
              ::AddAssignMemberInt::operator+=);

static_assert(sizeof(struct AddAssignMemberByConstRef) == 1);
static_assert(alignof(struct AddAssignMemberByConstRef) == 1);

extern "C" void __rust_thunk___ZN25AddAssignMemberByConstRefC1Ev(
    struct AddAssignMemberByConstRef* __this) {
  crubit::construct_at(__this);
}

static_assert((struct AddAssignMemberByConstRef &
               (::AddAssignMemberByConstRef::*)(
                   struct AddAssignMemberByConstRef const&)) &
              ::AddAssignMemberByConstRef::operator+=);

static_assert(sizeof(struct AddAssignFreeByConstRef) == 1);
static_assert(alignof(struct AddAssignFreeByConstRef) == 1);

extern "C" void __rust_thunk___ZN23AddAssignFreeByConstRefC1Ev(
    struct AddAssignFreeByConstRef* __this) {
  crubit::construct_at(__this);
}

static_assert((struct AddAssignFreeByConstRef &
               (*)(struct AddAssignFreeByConstRef&,
                   struct AddAssignFreeByConstRef const&)) &
              ::operator+=);

static_assert(sizeof(struct AddAssignFreeByValue) == 1);
static_assert(alignof(struct AddAssignFreeByValue) == 1);

extern "C" void __rust_thunk___ZN20AddAssignFreeByValueC1Ev(
    struct AddAssignFreeByValue* __this) {
  crubit::construct_at(__this);
}

extern "C" struct AddAssignFreeByValue*
__rust_thunk___ZpLR20AddAssignFreeByValueS_(struct AddAssignFreeByValue* lhs,
                                            struct AddAssignFreeByValue* rhs) {
  return std::addressof(operator+=(*lhs, std::move(*rhs)));
}

static_assert((struct AddAssignFreeByValue &
               (*)(struct AddAssignFreeByValue&, struct AddAssignFreeByValue)) &
              ::operator+=);

static_assert(sizeof(struct AddAssignFriendByConstRef) == 1);
static_assert(alignof(struct AddAssignFriendByConstRef) == 1);

extern "C" void __rust_thunk___ZN25AddAssignFriendByConstRefC1Ev(
    struct AddAssignFriendByConstRef* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct AddAssignFriendByValue) == 1);
static_assert(alignof(struct AddAssignFriendByValue) == 1);

extern "C" void __rust_thunk___ZN22AddAssignFriendByValueC1Ev(
    struct AddAssignFriendByValue* __this) {
  crubit::construct_at(__this);
}

extern "C" struct AddAssignFriendByValue*
__rust_thunk___ZpLR22AddAssignFriendByValueS_(
    struct AddAssignFriendByValue* lhs, struct AddAssignFriendByValue* rhs) {
  return std::addressof(operator+=(*lhs, std::move(*rhs)));
}

static_assert(sizeof(struct AddAssignProhibitedConstMember) == 1);
static_assert(alignof(struct AddAssignProhibitedConstMember) == 1);

extern "C" void __rust_thunk___ZN30AddAssignProhibitedConstMemberC1Ev(
    struct AddAssignProhibitedConstMember* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct AddAssignProhibitedFriendConstLhs) == 1);
static_assert(alignof(struct AddAssignProhibitedFriendConstLhs) == 1);

extern "C" void __rust_thunk___ZN33AddAssignProhibitedFriendConstLhsC1Ev(
    struct AddAssignProhibitedFriendConstLhs* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct ManyOperators) == 1);
static_assert(alignof(struct ManyOperators) == 1);

extern "C" void __rust_thunk___ZN13ManyOperatorsC1Ev(
    struct ManyOperators* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZNK13ManyOperatorspsEv(
    struct ManyOperators* __return, struct ManyOperators const* __this) {
  new (__return) auto(__this->operator+());
}

static_assert((struct ManyOperators (::ManyOperators::*)() const) &
              ::ManyOperators::operator+);

extern "C" void __rust_thunk___ZNK13ManyOperatorsngEv(
    struct ManyOperators* __return, struct ManyOperators const* __this) {
  new (__return) auto(__this->operator-());
}

static_assert((struct ManyOperators (::ManyOperators::*)() const) &
              ::ManyOperators::operator-);

extern "C" void __rust_thunk___ZNK13ManyOperatorsntEv(
    struct ManyOperators* __return, struct ManyOperators const* __this) {
  new (__return) auto(__this->operator!());
}

static_assert((struct ManyOperators (::ManyOperators::*)() const) &
              ::ManyOperators::operator!);

extern "C" void __rust_thunk___ZNK13ManyOperatorsplERKS_(
    struct ManyOperators* __return, struct ManyOperators const* __this,
    struct ManyOperators const* rhs) {
  new (__return) auto(__this->operator+(*rhs));
}

static_assert((struct ManyOperators (::ManyOperators::*)(
                  struct ManyOperators const&) const) &
              ::ManyOperators::operator+);

extern "C" void __rust_thunk___ZNK13ManyOperatorsmiERKS_(
    struct ManyOperators* __return, struct ManyOperators const* __this,
    struct ManyOperators const* rhs) {
  new (__return) auto(__this->operator-(*rhs));
}

static_assert((struct ManyOperators (::ManyOperators::*)(
                  struct ManyOperators const&) const) &
              ::ManyOperators::operator-);

extern "C" void __rust_thunk___ZNK13ManyOperatorsmlERKS_(
    struct ManyOperators* __return, struct ManyOperators const* __this,
    struct ManyOperators const* rhs) {
  new (__return) auto(__this->operator*(*rhs));
}

static_assert((struct ManyOperators (::ManyOperators::*)(
                  struct ManyOperators const&) const) &
              ::ManyOperators::operator*);

extern "C" void __rust_thunk___ZNK13ManyOperatorsdvERKS_(
    struct ManyOperators* __return, struct ManyOperators const* __this,
    struct ManyOperators const* rhs) {
  new (__return) auto(__this->operator/(*rhs));
}

static_assert((struct ManyOperators (::ManyOperators::*)(
                  struct ManyOperators const&) const) &
              ::ManyOperators::operator/);

extern "C" void __rust_thunk___ZNK13ManyOperatorsrmERKS_(
    struct ManyOperators* __return, struct ManyOperators const* __this,
    struct ManyOperators const* rhs) {
  new (__return) auto(__this->operator%(*rhs));
}

static_assert((struct ManyOperators (::ManyOperators::*)(
                  struct ManyOperators const&) const) &
              ::ManyOperators::operator%);

extern "C" void __rust_thunk___ZNK13ManyOperatorsanERKS_(
    struct ManyOperators* __return, struct ManyOperators const* __this,
    struct ManyOperators const* rhs) {
  new (__return) auto(__this->operator&(*rhs));
}

static_assert((struct ManyOperators (::ManyOperators::*)(
                  struct ManyOperators const&) const) &
              ::ManyOperators::operator&);

extern "C" void __rust_thunk___ZNK13ManyOperatorsorERKS_(
    struct ManyOperators* __return, struct ManyOperators const* __this,
    struct ManyOperators const* rhs) {
  new (__return) auto(__this->operator|(*rhs));
}

static_assert((struct ManyOperators (::ManyOperators::*)(
                  struct ManyOperators const&) const) &
              ::ManyOperators::operator|);

extern "C" void __rust_thunk___ZNK13ManyOperatorseoERKS_(
    struct ManyOperators* __return, struct ManyOperators const* __this,
    struct ManyOperators const* rhs) {
  new (__return) auto(__this->operator^(*rhs));
}

static_assert((struct ManyOperators (::ManyOperators::*)(
                  struct ManyOperators const&) const) &
              ::ManyOperators::operator^);

extern "C" void __rust_thunk___ZNK13ManyOperatorslsERKS_(
    struct ManyOperators* __return, struct ManyOperators const* __this,
    struct ManyOperators const* rhs) {
  new (__return) auto(__this->operator<<(*rhs));
}

static_assert((struct ManyOperators (::ManyOperators::*)(
                  struct ManyOperators const&) const) &
              ::ManyOperators::operator<<);

extern "C" void __rust_thunk___ZNK13ManyOperatorsrsERKS_(
    struct ManyOperators* __return, struct ManyOperators const* __this,
    struct ManyOperators const* rhs) {
  new (__return) auto(__this->operator>>(*rhs));
}

static_assert((struct ManyOperators (::ManyOperators::*)(
                  struct ManyOperators const&) const) &
              ::ManyOperators::operator>>);

static_assert((struct ManyOperators &
               (::ManyOperators::*)(struct ManyOperators const&)) &
              ::ManyOperators::operator+=);

static_assert((struct ManyOperators &
               (::ManyOperators::*)(struct ManyOperators const&)) &
              ::ManyOperators::operator-=);

static_assert((struct ManyOperators &
               (::ManyOperators::*)(struct ManyOperators const&)) &
              ::ManyOperators::operator*=);

static_assert((struct ManyOperators &
               (::ManyOperators::*)(struct ManyOperators const&)) &
              ::ManyOperators::operator/=);

static_assert((struct ManyOperators &
               (::ManyOperators::*)(struct ManyOperators const&)) &
              ::ManyOperators::operator%=);

static_assert((struct ManyOperators &
               (::ManyOperators::*)(struct ManyOperators const&)) &
              ::ManyOperators::operator&=);

static_assert((struct ManyOperators &
               (::ManyOperators::*)(struct ManyOperators const&)) &
              ::ManyOperators::operator|=);

static_assert((struct ManyOperators &
               (::ManyOperators::*)(struct ManyOperators const&)) &
              ::ManyOperators::operator^=);

static_assert((struct ManyOperators &
               (::ManyOperators::*)(struct ManyOperators const&)) &
              ::ManyOperators::operator<<=);

static_assert((struct ManyOperators &
               (::ManyOperators::*)(struct ManyOperators const&)) &
              ::ManyOperators::operator>>=);

#pragma clang diagnostic pop
