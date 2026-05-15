// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:struct_with_lifetimebound
// Features: assume_lifetimes, callables, fmt, supported, types

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/assume_lifetimes/struct_with_lifetimebound.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct PlainStruct) == 1);
static_assert(alignof(struct PlainStruct) == 1);

extern "C" void __rust_thunk___ZN11PlainStructC1Ev(struct PlainStruct* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct StructWithLifetimeboundMemberFunction) == 1);
static_assert(alignof(struct StructWithLifetimeboundMemberFunction) == 1);

extern "C" void __rust_thunk___ZN37StructWithLifetimeboundMemberFunctionC1Ev(
    struct StructWithLifetimeboundMemberFunction* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZNK37StructWithLifetimeboundMemberFunction1fEv(
    struct PlainStruct* __return,
    struct StructWithLifetimeboundMemberFunction const* __this) {
  new (__return) auto(__this->f());
}

static_assert((struct PlainStruct const (
                  ::StructWithLifetimeboundMemberFunction::*)() const) &
              ::StructWithLifetimeboundMemberFunction::f);

static_assert(sizeof(struct StructWithLifetimeboundRefMemberFunction) == 1);
static_assert(alignof(struct StructWithLifetimeboundRefMemberFunction) == 1);

extern "C" void __rust_thunk___ZN40StructWithLifetimeboundRefMemberFunctionC1Ev(
    struct StructWithLifetimeboundRefMemberFunction* __this) {
  crubit::construct_at(__this);
}

static_assert((struct PlainStruct const& (
                  ::StructWithLifetimeboundRefMemberFunction::*)() const) &
              ::StructWithLifetimeboundRefMemberFunction::f);

static_assert(sizeof(class DropClassWithLifetimeboundMemberFunction) == 1);
static_assert(alignof(class DropClassWithLifetimeboundMemberFunction) == 1);

extern "C" void __rust_thunk___ZN40DropClassWithLifetimeboundMemberFunctionC1Ev(
    class DropClassWithLifetimeboundMemberFunction* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN40DropClassWithLifetimeboundMemberFunctionC1ERKS_(
    class DropClassWithLifetimeboundMemberFunction* __this,
    class DropClassWithLifetimeboundMemberFunction const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" class DropClassWithLifetimeboundMemberFunction*
__rust_thunk___ZN40DropClassWithLifetimeboundMemberFunctionaSERKS_(
    class DropClassWithLifetimeboundMemberFunction* __this,
    class DropClassWithLifetimeboundMemberFunction const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

extern "C" void
__rust_thunk___ZNK40DropClassWithLifetimeboundMemberFunction1fEv(
    struct PlainStruct* __return,
    class DropClassWithLifetimeboundMemberFunction const* __this) {
  new (__return) auto(__this->f());
}

static_assert((struct PlainStruct const (
                  ::DropClassWithLifetimeboundMemberFunction::*)() const) &
              ::DropClassWithLifetimeboundMemberFunction::f);

static_assert(sizeof(class DropClassWithLifetimeboundRefMemberFunction) == 1);
static_assert(alignof(class DropClassWithLifetimeboundRefMemberFunction) == 1);

extern "C" void
__rust_thunk___ZN43DropClassWithLifetimeboundRefMemberFunctionC1Ev(
    class DropClassWithLifetimeboundRefMemberFunction* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN43DropClassWithLifetimeboundRefMemberFunctionC1ERKS_(
    class DropClassWithLifetimeboundRefMemberFunction* __this,
    class DropClassWithLifetimeboundRefMemberFunction const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" class DropClassWithLifetimeboundRefMemberFunction*
__rust_thunk___ZN43DropClassWithLifetimeboundRefMemberFunctionaSERKS_(
    class DropClassWithLifetimeboundRefMemberFunction* __this,
    class DropClassWithLifetimeboundRefMemberFunction const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct PlainStruct const& (
                  ::DropClassWithLifetimeboundRefMemberFunction::*)() const) &
              ::DropClassWithLifetimeboundRefMemberFunction::f);

static_assert(sizeof(struct StructWithLifetimeboundCtor) == 1);
static_assert(alignof(struct StructWithLifetimeboundCtor) == 1);

extern "C" void __rust_thunk___ZN27StructWithLifetimeboundCtorC1E11PlainStruct(
    struct StructWithLifetimeboundCtor* __this, struct PlainStruct* s) {
  crubit::construct_at(__this, std::move(*s));
}

static_assert(sizeof(struct StructWithLifetimeboundRefCtor) == 1);
static_assert(alignof(struct StructWithLifetimeboundRefCtor) == 1);

extern "C" void
__rust_thunk___ZN30StructWithLifetimeboundRefCtorC1ERK11PlainStruct(
    struct StructWithLifetimeboundRefCtor* __this,
    struct PlainStruct const* s) {
  crubit::construct_at(__this, *s);
}

static_assert(sizeof(struct DropStructWithLifetimeboundCtor) == 1);
static_assert(alignof(struct DropStructWithLifetimeboundCtor) == 1);

extern "C" void __rust_thunk___ZN31DropStructWithLifetimeboundCtorC1ERKS_(
    struct DropStructWithLifetimeboundCtor* __this,
    struct DropStructWithLifetimeboundCtor const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" struct DropStructWithLifetimeboundCtor*
__rust_thunk___ZN31DropStructWithLifetimeboundCtoraSERKS_(
    struct DropStructWithLifetimeboundCtor* __this,
    struct DropStructWithLifetimeboundCtor const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

extern "C" void
__rust_thunk___ZN31DropStructWithLifetimeboundCtorC1E11PlainStruct(
    struct DropStructWithLifetimeboundCtor* __this, struct PlainStruct* s) {
  crubit::construct_at(__this, std::move(*s));
}

static_assert(sizeof(struct DropStructWithLifetimeboundRefCtor) == 1);
static_assert(alignof(struct DropStructWithLifetimeboundRefCtor) == 1);

extern "C" void __rust_thunk___ZN34DropStructWithLifetimeboundRefCtorC1ERKS_(
    struct DropStructWithLifetimeboundRefCtor* __this,
    struct DropStructWithLifetimeboundRefCtor const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" struct DropStructWithLifetimeboundRefCtor*
__rust_thunk___ZN34DropStructWithLifetimeboundRefCtoraSERKS_(
    struct DropStructWithLifetimeboundRefCtor* __this,
    struct DropStructWithLifetimeboundRefCtor const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

extern "C" void
__rust_thunk___ZN34DropStructWithLifetimeboundRefCtorC1ERK11PlainStruct(
    struct DropStructWithLifetimeboundRefCtor* __this,
    struct PlainStruct const* s) {
  crubit::construct_at(__this, *s);
}

static_assert(sizeof(struct DropStructWithRefCtorAndRefMemberFunction) == 1);
static_assert(alignof(struct DropStructWithRefCtorAndRefMemberFunction) == 1);

extern "C" void
__rust_thunk___ZN41DropStructWithRefCtorAndRefMemberFunctionC1ERKS_(
    struct DropStructWithRefCtorAndRefMemberFunction* __this,
    struct DropStructWithRefCtorAndRefMemberFunction const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" struct DropStructWithRefCtorAndRefMemberFunction*
__rust_thunk___ZN41DropStructWithRefCtorAndRefMemberFunctionaSERKS_(
    struct DropStructWithRefCtorAndRefMemberFunction* __this,
    struct DropStructWithRefCtorAndRefMemberFunction const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

extern "C" void
__rust_thunk___ZN41DropStructWithRefCtorAndRefMemberFunctionC1ERK11PlainStruct(
    struct DropStructWithRefCtorAndRefMemberFunction* __this,
    struct PlainStruct const* s) {
  crubit::construct_at(__this, *s);
}

static_assert((struct PlainStruct const& (
                  ::DropStructWithRefCtorAndRefMemberFunction::*)() const) &
              ::DropStructWithRefCtorAndRefMemberFunction::f);

static_assert(sizeof(struct DropStructWithCtorAndMemberFunction) == 1);
static_assert(alignof(struct DropStructWithCtorAndMemberFunction) == 1);

extern "C" void __rust_thunk___ZN35DropStructWithCtorAndMemberFunctionC1ERKS_(
    struct DropStructWithCtorAndMemberFunction* __this,
    struct DropStructWithCtorAndMemberFunction const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" struct DropStructWithCtorAndMemberFunction*
__rust_thunk___ZN35DropStructWithCtorAndMemberFunctionaSERKS_(
    struct DropStructWithCtorAndMemberFunction* __this,
    struct DropStructWithCtorAndMemberFunction const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

extern "C" void
__rust_thunk___ZN35DropStructWithCtorAndMemberFunctionC1E11PlainStruct(
    struct DropStructWithCtorAndMemberFunction* __this, struct PlainStruct* s) {
  crubit::construct_at(__this, std::move(*s));
}

extern "C" void __rust_thunk___ZNK35DropStructWithCtorAndMemberFunction1fEv(
    struct PlainStruct* __return,
    struct DropStructWithCtorAndMemberFunction const* __this) {
  new (__return) auto(__this->f());
}

static_assert((struct PlainStruct const (
                  ::DropStructWithCtorAndMemberFunction::*)() const) &
              ::DropStructWithCtorAndMemberFunction::f);

static_assert(sizeof(struct DropStructWithCtorAndRefMemberFunction) == 1);
static_assert(alignof(struct DropStructWithCtorAndRefMemberFunction) == 1);

extern "C" void
__rust_thunk___ZN38DropStructWithCtorAndRefMemberFunctionC1ERKS_(
    struct DropStructWithCtorAndRefMemberFunction* __this,
    struct DropStructWithCtorAndRefMemberFunction const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" struct DropStructWithCtorAndRefMemberFunction*
__rust_thunk___ZN38DropStructWithCtorAndRefMemberFunctionaSERKS_(
    struct DropStructWithCtorAndRefMemberFunction* __this,
    struct DropStructWithCtorAndRefMemberFunction const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

extern "C" void
__rust_thunk___ZN38DropStructWithCtorAndRefMemberFunctionC1E11PlainStruct(
    struct DropStructWithCtorAndRefMemberFunction* __this,
    struct PlainStruct* s) {
  crubit::construct_at(__this, std::move(*s));
}

static_assert((struct PlainStruct const& (
                  ::DropStructWithCtorAndRefMemberFunction::*)() const) &
              ::DropStructWithCtorAndRefMemberFunction::f);

static_assert(sizeof(struct DropStructWithRefCtorAndMemberFunction) == 1);
static_assert(alignof(struct DropStructWithRefCtorAndMemberFunction) == 1);

extern "C" void
__rust_thunk___ZN38DropStructWithRefCtorAndMemberFunctionC1ERKS_(
    struct DropStructWithRefCtorAndMemberFunction* __this,
    struct DropStructWithRefCtorAndMemberFunction const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" struct DropStructWithRefCtorAndMemberFunction*
__rust_thunk___ZN38DropStructWithRefCtorAndMemberFunctionaSERKS_(
    struct DropStructWithRefCtorAndMemberFunction* __this,
    struct DropStructWithRefCtorAndMemberFunction const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

extern "C" void
__rust_thunk___ZN38DropStructWithRefCtorAndMemberFunctionC1ERK11PlainStruct(
    struct DropStructWithRefCtorAndMemberFunction* __this,
    struct PlainStruct const* s) {
  crubit::construct_at(__this, *s);
}

extern "C" void __rust_thunk___ZNK38DropStructWithRefCtorAndMemberFunction1fEv(
    struct PlainStruct* __return,
    struct DropStructWithRefCtorAndMemberFunction const* __this) {
  new (__return) auto(__this->f());
}

static_assert((struct PlainStruct const (
                  ::DropStructWithRefCtorAndMemberFunction::*)() const) &
              ::DropStructWithRefCtorAndMemberFunction::f);

#pragma clang diagnostic pop
