// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/nontrivial_type.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct Nontrivial) == 4);
static_assert(alignof(struct Nontrivial) == 4);
static_assert(CRUBIT_OFFSET_OF(field, struct Nontrivial) == 0);

static_assert((struct Nontrivial &
               (::Nontrivial::*)(struct Nontrivial const&)) &
              ::Nontrivial::operator=);

static_assert((struct Nontrivial & (::Nontrivial::*)(struct Nontrivial&&)) &
              ::Nontrivial::operator=);

static_assert((struct Nontrivial & (::Nontrivial::*)(int)) &
              ::Nontrivial::operator=);

extern "C" void __rust_thunk___ZN10NontrivialaSEf(struct Nontrivial* __return,
                                                  struct Nontrivial* __this,
                                                  float __param_0) {
  new (__return) auto(__this->operator=(__param_0));
}

static_assert((struct Nontrivial (::Nontrivial::*)(float)) &
              ::Nontrivial::operator=);

static_assert((void (::Nontrivial::*)()) & ::Nontrivial::Unqualified);

static_assert((void (::Nontrivial::*)() const) & ::Nontrivial::ConstQualified);

static_assert((void (::Nontrivial::*)() &)&::Nontrivial::LvalueRefQualified);

static_assert((void (::Nontrivial::*)()
                   const&)&::Nontrivial::ConstLvalueRefQualified);

static_assert((void (::Nontrivial::*)() &&)&::Nontrivial::RvalueRefQualified);

static_assert((void (::Nontrivial::*)()
                   const&&)&::Nontrivial::ConstRvalueRefQualified);

static_assert((bool (::Nontrivial::*)(struct Nontrivial const&) const) &
              ::Nontrivial::operator==);

static_assert((bool (::Nontrivial::*)(struct Nontrivial const&) const) &
              ::Nontrivial::operator<);

extern "C" void __rust_thunk___ZNK10NontrivialplERKS_(
    struct Nontrivial* __return, struct Nontrivial const* __this,
    struct Nontrivial const* rhs) {
  new (__return) auto(__this->operator+(*rhs));
}

static_assert((struct Nontrivial (::Nontrivial::*)(struct Nontrivial const&)
                   const) &
              ::Nontrivial::operator+);

static_assert(CRUBIT_SIZEOF(struct NontrivialInline) == 4);
static_assert(alignof(struct NontrivialInline) == 4);
static_assert(CRUBIT_OFFSET_OF(field, struct NontrivialInline) == 0);

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
    struct NontrivialInline* __this, struct NontrivialInline const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" void __rust_thunk___ZN16NontrivialInlineC1EOS_(
    struct NontrivialInline* __this, struct NontrivialInline* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct NontrivialInline* __rust_thunk___ZN16NontrivialInlineaSERKS_(
    struct NontrivialInline* __this, struct NontrivialInline const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct NontrivialInline &
               (::NontrivialInline::*)(struct NontrivialInline const&)) &
              ::NontrivialInline::operator=);

extern "C" struct NontrivialInline* __rust_thunk___ZN16NontrivialInlineaSEOS_(
    struct NontrivialInline* __this, struct NontrivialInline* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct NontrivialInline &
               (::NontrivialInline::*)(struct NontrivialInline&&)) &
              ::NontrivialInline::operator=);

extern "C" struct NontrivialInline* __rust_thunk___ZN16NontrivialInlineaSEi(
    struct NontrivialInline* __this, int __param_0) {
  return std::addressof(__this->operator=(__param_0));
}

static_assert((struct NontrivialInline & (::NontrivialInline::*)(int)) &
              ::NontrivialInline::operator=);

extern "C" void __rust_thunk___ZN16NontrivialInlineD1Ev(
    struct NontrivialInline* __this) {
  std::destroy_at(__this);
}

extern "C" void __rust_thunk___ZN16NontrivialInline14MemberFunctionEv(
    struct NontrivialInline* __this) {
  __this->MemberFunction();
}

static_assert((void (::NontrivialInline::*)()) &
              ::NontrivialInline::MemberFunction);

static_assert(CRUBIT_SIZEOF(struct NontrivialMembers) == 4);
static_assert(alignof(struct NontrivialMembers) == 4);
static_assert(CRUBIT_OFFSET_OF(nontrivial_member, struct NontrivialMembers) ==
              0);

extern "C" void __rust_thunk___ZN17NontrivialMembersC1Ev(
    struct NontrivialMembers* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN17NontrivialMembersC1ERKS_(
    struct NontrivialMembers* __this,
    struct NontrivialMembers const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" void __rust_thunk___ZN17NontrivialMembersC1EOS_(
    struct NontrivialMembers* __this, struct NontrivialMembers* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct NontrivialMembers*
__rust_thunk___ZN17NontrivialMembersaSERKS_(
    struct NontrivialMembers* __this,
    struct NontrivialMembers const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct NontrivialMembers &
               (::NontrivialMembers::*)(struct NontrivialMembers const&)) &
              ::NontrivialMembers::operator=);

extern "C" struct NontrivialMembers* __rust_thunk___ZN17NontrivialMembersaSEOS_(
    struct NontrivialMembers* __this, struct NontrivialMembers* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct NontrivialMembers &
               (::NontrivialMembers::*)(struct NontrivialMembers&&)) &
              ::NontrivialMembers::operator=);

static_assert(CRUBIT_SIZEOF(struct NontrivialUnpin) == 4);
static_assert(alignof(struct NontrivialUnpin) == 4);
static_assert(CRUBIT_OFFSET_OF(field, struct NontrivialUnpin) == 0);

static_assert((struct NontrivialUnpin &
               (::NontrivialUnpin::*)(struct NontrivialUnpin const&)) &
              ::NontrivialUnpin::operator=);

static_assert((struct NontrivialUnpin &
               (::NontrivialUnpin::*)(struct NontrivialUnpin&&)) &
              ::NontrivialUnpin::operator=);

static_assert((struct NontrivialUnpin & (::NontrivialUnpin::*)(int)) &
              ::NontrivialUnpin::operator=);

static_assert((void (::NontrivialUnpin::*)()) &
              ::NontrivialUnpin::MemberFunction);

extern "C" void __rust_thunk___Z12TakesByValue10Nontrivial(
    struct Nontrivial* __return, struct Nontrivial* nontrivial) {
  new (__return) auto(TakesByValue(std::move(*nontrivial)));
}

static_assert((struct Nontrivial (*)(struct Nontrivial)) & ::TakesByValue);

extern "C" void __rust_thunk___Z18TakesByValueInline16NontrivialInline(
    struct NontrivialInline* __return, struct NontrivialInline* nontrivial) {
  new (__return) auto(TakesByValueInline(std::move(*nontrivial)));
}

static_assert((struct NontrivialInline (*)(struct NontrivialInline)) &
              ::TakesByValueInline);

extern "C" void __rust_thunk___Z17TakesByValueUnpin15NontrivialUnpin(
    struct NontrivialUnpin* __return, struct NontrivialUnpin* nontrivial) {
  new (__return) auto(TakesByValueUnpin(std::move(*nontrivial)));
}

static_assert((struct NontrivialUnpin (*)(struct NontrivialUnpin)) &
              ::TakesByValueUnpin);

static_assert((struct Nontrivial & (*)(struct Nontrivial&)) &
              ::TakesByReference);

static_assert((struct NontrivialUnpin & (*)(struct NontrivialUnpin&)) &
              ::TakesUnpinByReference);

static_assert((struct Nontrivial const& (*)(struct Nontrivial const&)) &
              ::TakesByConstReference);

static_assert(
    (struct NontrivialUnpin const& (*)(struct NontrivialUnpin const&)) &
    ::TakesUnpinByConstReference);

static_assert((struct Nontrivial && (*)(struct Nontrivial&&)) &
              ::TakesByRvalueReference);

static_assert((struct NontrivialUnpin && (*)(struct NontrivialUnpin&&)) &
              ::TakesUnpinByRvalueReference);

static_assert((struct Nontrivial const && (*)(struct Nontrivial const&&)) &
              ::TakesByConstRvalueReference);

static_assert((struct NontrivialUnpin const &&
               (*)(struct NontrivialUnpin const&&)) &
              ::TakesUnpinByConstRvalueReference);

static_assert(sizeof(struct NontrivialByValue) == 1);
static_assert(alignof(struct NontrivialByValue) == 1);

extern "C" void __rust_thunk___ZN17NontrivialByValueaSE10Nontrivial(
    struct NontrivialByValue* __return, struct NontrivialByValue* __this,
    struct Nontrivial* other) {
  new (__return) auto(__this->operator=(std::move(*other)));
}

static_assert(
    (struct NontrivialByValue (::NontrivialByValue::*)(struct Nontrivial)) &
    ::NontrivialByValue::operator=);

static_assert(sizeof(struct Nonmovable) == 1);
static_assert(alignof(struct Nonmovable) == 1);

static_assert((void (::Nonmovable::*)()) & ::Nonmovable::MemberFunction);

extern "C" void __rust_thunk___Z24ReturnsNonmovableByValuev(
    struct Nonmovable* __return) {
  new (__return) auto(ReturnsNonmovableByValue());
}

static_assert((struct Nonmovable (*)()) & ::ReturnsNonmovableByValue);

#pragma clang diagnostic pop
