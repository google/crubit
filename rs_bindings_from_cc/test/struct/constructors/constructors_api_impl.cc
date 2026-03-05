// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/struct/constructors:constructors
// Features: assume_lifetimes, callables, check_default_initialized, experimental, supported, unsafe_view, wrapper

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/struct/constructors/constructors.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct StructWithUserProvidedConstructors) == 4);
static_assert(alignof(struct StructWithUserProvidedConstructors) == 4);
static_assert(CRUBIT_OFFSET_OF(int_field,
                               struct StructWithUserProvidedConstructors) == 0);

extern "C" struct StructWithUserProvidedConstructors*
__rust_thunk___ZN34StructWithUserProvidedConstructorsaSERKS_(
    struct StructWithUserProvidedConstructors* __this,
    struct StructWithUserProvidedConstructors const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert(CRUBIT_SIZEOF(struct StructWithExplicitConversionConstructor) ==
              4);
static_assert(alignof(struct StructWithExplicitConversionConstructor) == 4);
static_assert(
    CRUBIT_OFFSET_OF(int_field,
                     struct StructWithExplicitConversionConstructor) == 0);

extern "C" void __rust_thunk___ZN39StructWithExplicitConversionConstructorC1Ei(
    struct StructWithExplicitConversionConstructor* __this, int i) {
  crubit::construct_at(__this, i);
}

static_assert(CRUBIT_SIZEOF(struct StructWithMultipleConstructors) == 4);
static_assert(alignof(struct StructWithMultipleConstructors) == 4);
static_assert(CRUBIT_OFFSET_OF(int_field,
                               struct StructWithMultipleConstructors) == 0);

extern "C" void __rust_thunk___ZN30StructWithMultipleConstructorsC1Ei(
    struct StructWithMultipleConstructors* __this, int i) {
  crubit::construct_at(__this, i);
}

extern "C" void __rust_thunk___ZN30StructWithMultipleConstructorsC1Eii(
    struct StructWithMultipleConstructors* __this, int i, int j) {
  crubit::construct_at(__this, i, j);
}

extern "C" void __rust_thunk___ZN30StructWithMultipleConstructorsC1Eiii(
    struct StructWithMultipleConstructors* __this, int i, int j, int k) {
  crubit::construct_at(__this, i, j, k);
}

static_assert(CRUBIT_SIZEOF(struct StructWithImplicitConversionConstructor) ==
              4);
static_assert(alignof(struct StructWithImplicitConversionConstructor) == 4);
static_assert(
    CRUBIT_OFFSET_OF(int_field,
                     struct StructWithImplicitConversionConstructor) == 0);

extern "C" void __rust_thunk___ZN39StructWithImplicitConversionConstructorC1Ei(
    struct StructWithImplicitConversionConstructor* __this, int i) {
  crubit::construct_at(__this, i);
}

static_assert(CRUBIT_SIZEOF(struct OtherSimpleStruct) == 4);
static_assert(alignof(struct OtherSimpleStruct) == 4);
static_assert(CRUBIT_OFFSET_OF(int_field, struct OtherSimpleStruct) == 0);

extern "C" void __rust_thunk___ZN17OtherSimpleStructC1Ev(
    struct OtherSimpleStruct* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct StructWithImplicitConversionFromReference) ==
              4);
static_assert(alignof(struct StructWithImplicitConversionFromReference) == 4);
static_assert(
    CRUBIT_OFFSET_OF(int_field,
                     struct StructWithImplicitConversionFromReference) == 0);

extern "C" void
__rust_thunk___ZN41StructWithImplicitConversionFromReferenceC1ERK17OtherSimpleStruct(
    struct StructWithImplicitConversionFromReference* __this,
    struct OtherSimpleStruct const* other) {
  crubit::construct_at(__this, *other);
}

static_assert(CRUBIT_SIZEOF(struct StructWithInlineConstructors) == 4);
static_assert(alignof(struct StructWithInlineConstructors) == 4);
static_assert(CRUBIT_OFFSET_OF(int_field,
                               struct StructWithInlineConstructors) == 0);

extern "C" struct StructWithInlineConstructors*
__rust_thunk___ZN28StructWithInlineConstructorsaSERKS_(
    struct StructWithInlineConstructors* __this,
    struct StructWithInlineConstructors const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

extern "C" void __rust_thunk___ZN28StructWithInlineConstructorsC1Ev(
    struct StructWithInlineConstructors* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN28StructWithInlineConstructorsC1ERKS_(
    struct StructWithInlineConstructors* __this,
    struct StructWithInlineConstructors const* other) {
  crubit::construct_at(__this, *other);
}

extern "C" void __rust_thunk___ZN28StructWithInlineConstructorsC1Ei(
    struct StructWithInlineConstructors* __this, int i) {
  crubit::construct_at(__this, i);
}

static_assert(CRUBIT_SIZEOF(struct StructWithDeletedConstructors) == 4);
static_assert(alignof(struct StructWithDeletedConstructors) == 4);
static_assert(CRUBIT_OFFSET_OF(int_field,
                               struct StructWithDeletedConstructors) == 0);

extern "C" struct StructWithDeletedConstructors*
__rust_thunk___ZN29StructWithDeletedConstructorsaSERKS_(
    struct StructWithDeletedConstructors* __this,
    struct StructWithDeletedConstructors const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert(CRUBIT_SIZEOF(struct StructWithPrivateConstructors) == 4);
static_assert(alignof(struct StructWithPrivateConstructors) == 4);

extern "C" struct StructWithPrivateConstructors*
__rust_thunk___ZN29StructWithPrivateConstructorsaSERKS_(
    struct StructWithPrivateConstructors* __this,
    struct StructWithPrivateConstructors const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert(CRUBIT_SIZEOF(struct StructWithExplicitlyDefaultedConstructors) ==
              8);
static_assert(alignof(struct StructWithExplicitlyDefaultedConstructors) == 4);
static_assert(
    CRUBIT_OFFSET_OF(field_with_explicit_initializer,
                     struct StructWithExplicitlyDefaultedConstructors) == 0);
static_assert(
    CRUBIT_OFFSET_OF(field_with_no_initializer,
                     struct StructWithExplicitlyDefaultedConstructors) == 4);

extern "C" void
__rust_thunk___ZN41StructWithExplicitlyDefaultedConstructorsC1Ev(
    struct StructWithExplicitlyDefaultedConstructors* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct NonTrivialStructWithConstructors) == 4);
static_assert(alignof(struct NonTrivialStructWithConstructors) == 4);
static_assert(CRUBIT_OFFSET_OF(int_field,
                               struct NonTrivialStructWithConstructors) == 0);

extern "C" void __rust_thunk___ZN32NonTrivialStructWithConstructorsC1ERKS_(
    struct NonTrivialStructWithConstructors* __this,
    struct NonTrivialStructWithConstructors const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" struct NonTrivialStructWithConstructors*
__rust_thunk___ZN32NonTrivialStructWithConstructorsaSERKS_(
    struct NonTrivialStructWithConstructors* __this,
    struct NonTrivialStructWithConstructors const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

#pragma clang diagnostic pop
