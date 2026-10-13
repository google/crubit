// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/function/semantic_import:semantic_import

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/function/semantic_import/semantic_import.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(class S) == 4);
static_assert(alignof(class S) == 4);

extern "C" void __rust_thunk___ZN1SC1Ei(class S* __this, int x) {
  crubit::construct_at(__this, x);
}

static_assert(CRUBIT_SIZEOF(class T) == 8);
static_assert(alignof(class T) == 4);

extern "C" void __rust_thunk___ZN1TC1Eif(class T* __this, int x, float y) {
  crubit::construct_at(__this, x, y);
}

static_assert(sizeof(class Chars) == 3);
static_assert(alignof(class Chars) == 1);

extern "C" void __rust_thunk___ZN5CharsC1Ev(class Chars* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class Bools) == 1);
static_assert(alignof(class Bools) == 1);

extern "C" void __rust_thunk___ZN5BoolsC1Ev(class Bools* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class Pointers) == 16);
static_assert(alignof(class Pointers) == 8);

extern "C" void __rust_thunk___ZN8PointersC1Ev(class Pointers* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class NonTrivial) == 16);
static_assert(alignof(class NonTrivial) == 8);

extern "C" void __rust_thunk___ZN10NonTrivialC1ERKS_(
    class NonTrivial* __this, class NonTrivial const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" class NonTrivial* __rust_thunk___ZN10NonTrivialaSERKS_(
    class NonTrivial* __this, class NonTrivial const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((class NonTrivial & (::NonTrivial::*)(class NonTrivial const&)) &
              ::NonTrivial::operator=);

extern "C" void __rust_thunk___ZN10NonTrivialC1Ev(class NonTrivial* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN10NonTrivialD1Ev(class NonTrivial* __this) {
  std::destroy_at(__this);
}

static_assert(CRUBIT_SIZEOF(class MorePointers) == 16);
static_assert(alignof(class MorePointers) == 8);

extern "C" void __rust_thunk___ZN12MorePointersC1Ev(
    class MorePointers* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class Slices) == 16);
static_assert(alignof(class Slices) == 8);

extern "C" void __rust_thunk___ZN6SlicesC1Ev(class Slices* __this) {
  crubit::construct_at(__this);
}

extern "C" ::rs_std::SliceRef<const int> __rust_thunk___ZNK6Slices1sEv(
    class Slices const* __this) {
  return __this->s();
}

static_assert((::rs_std::SliceRef<const int> (::Slices::*)() const) &
              ::Slices::s);

extern "C" void __rust_thunk___ZN6Slices5set_sEN6rs_std8SliceRefIKiEE(
    class Slices* __this, ::rs_std::SliceRef<const int> s) {
  __this->set_s(s);
}

static_assert((void (::Slices::*)(::rs_std::SliceRef<const int>)) &
              ::Slices::set_s);

#pragma clang diagnostic pop
