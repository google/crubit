// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/manual_bridge_vocabulary_types:span_lib

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/manual_bridge_vocabulary_types/span_lib.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"

static_assert(CRUBIT_SIZEOF(struct NonTrivial) == 8);
static_assert(alignof(struct NonTrivial) == 8);
static_assert(CRUBIT_OFFSET_OF(num, struct NonTrivial) == 0);

extern "C" void __rust_thunk___ZN10NonTrivialC1Ev(struct NonTrivial* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN10NonTrivialC1ERKS_(
    struct NonTrivial* __this, struct NonTrivial const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" struct NonTrivial* __rust_thunk___ZN10NonTrivialaSERKS_(
    struct NonTrivial* __this, struct NonTrivial const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct NonTrivial &
               (::NonTrivial::*)(struct NonTrivial const&)) &
              ::NonTrivial::operator=);

extern "C" void __rust_thunk___ZN10NonTrivialD1Ev(struct NonTrivial* __this) {
  std::destroy_at(__this);
}

extern "C" void __rust_thunk___Z15TruncateSpanMutN4absl4SpanIiEEm(
    class absl::Span<int>* __return, class absl::Span<int>* span, size_t len) {
  new (__return) auto(TruncateSpanMut(crubit::UnsafeTakeValue(span), len));
}

static_assert((class absl::Span<int> (*)(class absl::Span<int>, size_t)) &
              ::TruncateSpanMut);

extern "C" void __rust_thunk___Z12TruncateSpanN4absl4SpanIKiEEm(
    class absl::Span<const int>* __return, class absl::Span<const int>* span,
    size_t len) {
  new (__return) auto(TruncateSpan(crubit::UnsafeTakeValue(span), len));
}

static_assert((class absl::Span<const int> (*)(class absl::Span<const int>,
                                               size_t)) &
              ::TruncateSpan);

extern "C" void __rust_thunk___Z14MakeNonTrivialN4absl4SpanIKiEE(
    struct NonTrivial* __return, class absl::Span<const int>* span) {
  new (__return) auto(MakeNonTrivial(crubit::UnsafeTakeValue(span)));
}

static_assert((struct NonTrivial (*)(class absl::Span<const int>)) &
              ::MakeNonTrivial);

extern "C" void
__rust_thunk___Z25TruncateSpanNonTrivialMutN4absl4SpanI10NonTrivialEEm(
    class absl::Span<struct NonTrivial>* __return,
    class absl::Span<struct NonTrivial>* span, size_t len) {
  new (__return) auto(
      TruncateSpanNonTrivialMut(crubit::UnsafeTakeValue(span), len));
}

static_assert((class absl::Span<struct NonTrivial> (*)(
                  class absl::Span<struct NonTrivial>, size_t)) &
              ::TruncateSpanNonTrivialMut);

extern "C" void
__rust_thunk___Z22TruncateSpanNonTrivialN4absl4SpanIK10NonTrivialEEm(
    class absl::Span<const struct NonTrivial>* __return,
    class absl::Span<const struct NonTrivial>* span, size_t len) {
  new (__return) auto(
      TruncateSpanNonTrivial(crubit::UnsafeTakeValue(span), len));
}

static_assert((class absl::Span<const struct NonTrivial> (*)(
                  class absl::Span<const struct NonTrivial>, size_t)) &
              ::TruncateSpanNonTrivial);

static_assert(CRUBIT_SIZEOF(struct StructWithSpanField) == 24);
static_assert(alignof(struct StructWithSpanField) == 8);
static_assert(CRUBIT_OFFSET_OF(span, struct StructWithSpanField) == 0);
static_assert(CRUBIT_OFFSET_OF(other, struct StructWithSpanField) == 16);

extern "C" void __rust_thunk___ZN19StructWithSpanFieldC1Ev(
    struct StructWithSpanField* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct StructWithLifetimeAnnotatedSpanField) == 24);
static_assert(alignof(struct StructWithLifetimeAnnotatedSpanField) == 8);
static_assert(CRUBIT_OFFSET_OF(span,
                               struct StructWithLifetimeAnnotatedSpanField) ==
              0);
static_assert(CRUBIT_OFFSET_OF(other,
                               struct StructWithLifetimeAnnotatedSpanField) ==
              16);

extern "C" void __rust_thunk___ZN36StructWithLifetimeAnnotatedSpanFieldC1Ev(
    struct StructWithLifetimeAnnotatedSpanField* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<struct NonTrivial*>) ==
              8);
static_assert(alignof(class std::reverse_iterator<struct NonTrivial*>) == 8);

extern "C" void
__rust_thunk__cbab09f8__ZNSt3__u16reverse_iteratorIP10NonTrivialEC1Ev(
    class std::reverse_iterator<struct NonTrivial*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__466dfd43__ZNSt3__u16reverse_iteratorIP10NonTrivialEC1ES2_(
    class std::reverse_iterator<struct NonTrivial*>* __this,
    struct NonTrivial* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" struct NonTrivial*
__rust_thunk__3ef42fe9__ZNKSt3__u16reverse_iteratorIP10NonTrivialE4baseEv(
    class std::reverse_iterator<struct NonTrivial*> const* __this) {
  return __this->base();
}

static_assert((struct NonTrivial *
               (::std::reverse_iterator<struct NonTrivial*>::*)() const) &
              ::std::reverse_iterator<struct NonTrivial*>::base);

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<const struct NonTrivial*>) == 8);
static_assert(alignof(class std::reverse_iterator<const struct NonTrivial*>) ==
              8);

extern "C" void
__rust_thunk__cbab09f8__ZNSt3__u16reverse_iteratorIPK10NonTrivialEC1Ev(
    class std::reverse_iterator<const struct NonTrivial*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__466dfd43__ZNSt3__u16reverse_iteratorIPK10NonTrivialEC1ES3_(
    class std::reverse_iterator<const struct NonTrivial*>* __this,
    struct NonTrivial const* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" struct NonTrivial const*
__rust_thunk__3ef42fe9__ZNKSt3__u16reverse_iteratorIPK10NonTrivialE4baseEv(
    class std::reverse_iterator<const struct NonTrivial*> const* __this) {
  return __this->base();
}

static_assert((struct NonTrivial const* (
                  ::std::reverse_iterator<const struct NonTrivial*>::*)()
                   const) &
              ::std::reverse_iterator<const struct NonTrivial*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const int*>) == 8);
static_assert(alignof(class std::reverse_iterator<const int*>) == 8);

extern "C" void __rust_thunk__cbab09f8__ZNSt3__u16reverse_iteratorIPKiEC1Ev(
    class std::reverse_iterator<const int*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__466dfd43__ZNSt3__u16reverse_iteratorIPKiEC1ES2_(
    class std::reverse_iterator<const int*>* __this, int const* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" int const*
__rust_thunk__3ef42fe9__ZNKSt3__u16reverse_iteratorIPKiE4baseEv(
    class std::reverse_iterator<const int*> const* __this) {
  return __this->base();
}

static_assert((int const* (::std::reverse_iterator<const int*>::*)() const) &
              ::std::reverse_iterator<const int*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<int*>) == 8);
static_assert(alignof(class std::reverse_iterator<int*>) == 8);

extern "C" void __rust_thunk__cbab09f8__ZNSt3__u16reverse_iteratorIPiEC1Ev(
    class std::reverse_iterator<int*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__466dfd43__ZNSt3__u16reverse_iteratorIPiEC1ES1_(
    class std::reverse_iterator<int*>* __this, int* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" int* __rust_thunk__3ef42fe9__ZNKSt3__u16reverse_iteratorIPiE4baseEv(
    class std::reverse_iterator<int*> const* __this) {
  return __this->base();
}

static_assert((int* (::std::reverse_iterator<int*>::*)() const) &
              ::std::reverse_iterator<int*>::base);

#pragma clang diagnostic pop
