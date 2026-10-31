// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:boxes_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/boxes.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___ZN4test7MakeBoxEi(rs::Box<int>* __return,
                                                 int v) {
  new (__return) auto(::test::MakeBox(v));
}

static_assert((rs::Box<int> (*)(int)) & ::test::MakeBox);

static_assert((int (*)(rs::Box<int> const&)) & ::test::ReadBox);

static_assert((void (*)(rs::Box<int>&)) & ::test::MutateBox);

extern "C" int __rust_thunk___ZN4test10ConsumeBoxEN2rs3BoxIiEE(
    rs::Box<int>* b) {
  return ::test::ConsumeBox(crubit::UnsafeTakeValue(b));
}

static_assert((int (*)(rs::Box<int>)) & ::test::ConsumeBox);

extern "C" void __rust_thunk___ZN4test12RoundtripBoxEN2rs3BoxIiEE(
    rs::Box<int>* __return, rs::Box<int>* b) {
  new (__return) auto(::test::RoundtripBox(crubit::UnsafeTakeValue(b)));
}

static_assert((rs::Box<int> (*)(rs::Box<int>)) & ::test::RoundtripBox);

static_assert((int (*)(rs::Box<int> const*)) & ::test::ReadBoxPtr);

static_assert(CRUBIT_SIZEOF(struct ::test::Point) == 8);
static_assert(alignof(struct ::test::Point) == 4);
static_assert(CRUBIT_OFFSET_OF(x, struct ::test::Point) == 0);
static_assert(CRUBIT_OFFSET_OF(y, struct ::test::Point) == 4);

extern "C" void __rust_thunk___ZN4test14MakeBoxedPointEv(
    rs::Box<test::Point>* __return) {
  new (__return) auto(::test::MakeBoxedPoint());
}

static_assert((rs::Box<test::Point> (*)()) & ::test::MakeBoxedPoint);

extern "C" int __rust_thunk___ZN4test6PointXEN2rs3BoxINS_5PointEEE(
    rs::Box<test::Point>* b) {
  return ::test::PointX(crubit::UnsafeTakeValue(b));
}

static_assert((int (*)(rs::Box<test::Point>)) & ::test::PointX);

static_assert(CRUBIT_SIZEOF(struct ::test::Holder) == 8);
static_assert(alignof(struct ::test::Holder) == 8);
static_assert(CRUBIT_OFFSET_OF(b, struct ::test::Holder) == 0);

extern "C" void __rust_thunk___ZN4test6HolderD1Ev(
    struct ::test::Holder* __this) {
  std::destroy_at(__this);
}

extern "C" void __rust_thunk___ZN4test10MakeHolderEi(
    struct ::test::Holder* __return, int v) {
  new (__return) auto(::test::MakeHolder(v));
}

static_assert((struct ::test::Holder (*)(int)) & ::test::MakeHolder);

static_assert((int (*)(struct ::test::Holder const&)) & ::test::HolderValue);

static_assert((void (*)(struct ::test::Holder&, int)) & ::test::SetHolderValue);

static_assert(CRUBIT_SIZEOF(struct ::test::Outer) == 16);
static_assert(alignof(struct ::test::Outer) == 8);
static_assert(CRUBIT_OFFSET_OF(h, struct ::test::Outer) == 0);
static_assert(CRUBIT_OFFSET_OF(tag, struct ::test::Outer) == 8);

extern "C" void __rust_thunk___ZN4test9MakeOuterEi(
    struct ::test::Outer* __return, int v) {
  new (__return) auto(::test::MakeOuter(v));
}

static_assert((struct ::test::Outer (*)(int)) & ::test::MakeOuter);

static_assert(CRUBIT_SIZEOF(struct ::test::ArrayHolder) == 16);
static_assert(alignof(struct ::test::ArrayHolder) == 8);
static_assert(CRUBIT_OFFSET_OF(boxes, struct ::test::ArrayHolder) == 0);

extern "C" void __rust_thunk___ZN4test11ArrayHolderD1Ev(
    struct ::test::ArrayHolder* __this) {
  std::destroy_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct ::test::NonTrivial) == 4);
static_assert(alignof(struct ::test::NonTrivial) == 4);
static_assert(CRUBIT_OFFSET_OF(val, struct ::test::NonTrivial) == 0);

extern "C" void __rust_thunk___ZN4test10NonTrivialC1Ei(
    struct ::test::NonTrivial* __this, int v) {
  crubit::construct_at(__this, v);
}

extern "C" void __rust_thunk___ZN4test10NonTrivialC1EOS0_(
    struct ::test::NonTrivial* __this, struct ::test::NonTrivial* other) {
  crubit::construct_at(__this, std::move(*other));
}

extern "C" struct ::test::NonTrivial* __rust_thunk___ZN4test10NonTrivialaSEOS0_(
    struct ::test::NonTrivial* __this, struct ::test::NonTrivial* other) {
  return std::addressof(__this->operator=(std::move(*other)));
}

static_assert((struct ::test::NonTrivial &
               (::test::NonTrivial::*)(struct ::test::NonTrivial&&)) &
              ::test::NonTrivial::operator=);

extern "C" void __rust_thunk___ZN4test10NonTrivialD1Ev(
    struct ::test::NonTrivial* __this) {
  std::destroy_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct ::test::PinHolder) == 8);
static_assert(alignof(struct ::test::PinHolder) == 8);
static_assert(CRUBIT_OFFSET_OF(p, struct ::test::PinHolder) == 0);

extern "C" void __rust_thunk___ZN4test9PinHolderD1Ev(
    struct ::test::PinHolder* __this) {
  std::destroy_at(__this);
}

extern "C" void __rust_thunk___ZN4test20MakePinBoxNonTrivialEi(
    rs::PinBox<test::NonTrivial>* __return, int val) {
  new (__return) auto(::test::MakePinBoxNonTrivial(val));
}

static_assert((rs::PinBox<test::NonTrivial> (*)(int)) &
              ::test::MakePinBoxNonTrivial);

extern "C" void
__rust_thunk___ZN4test21SharePinBoxNonTrivialERKN2rs6PinBoxINS_10NonTrivialEEE(
    rs::PinBox<test::NonTrivial>* __return,
    rs::PinBox<test::NonTrivial> const* b) {
  new (__return) auto(::test::SharePinBoxNonTrivial(*b));
}

static_assert(
    (rs::PinBox<test::NonTrivial> (*)(rs::PinBox<test::NonTrivial> const&)) &
    ::test::SharePinBoxNonTrivial);

static_assert((long (*)(rs::PinBox<test::NonTrivial> const&)) &
              ::test::NonTrivialUseCount);

extern "C" void
__rust_thunk___ZN4test13ConsumePinBoxEN2rs6PinBoxINS_10NonTrivialEEE(
    rs::PinBox<test::NonTrivial>* b) {
  ::test::ConsumePinBox(crubit::UnsafeTakeValue(b));
}

static_assert((void (*)(rs::PinBox<test::NonTrivial>)) & ::test::ConsumePinBox);

static_assert((int (*)(rs::PinBox<test::NonTrivial> const&)) &
              ::test::ReadPinBoxNonTrivial);

extern "C" void __rust_thunk___ZN4test15MakePinBoxUnpinEv(
    rs::PinBox<int>* __return) {
  new (__return) auto(::test::MakePinBoxUnpin());
}

static_assert((rs::PinBox<int> (*)()) & ::test::MakePinBoxUnpin);

extern "C" void __rust_thunk___ZN4test12MakeSliceBoxEm(
    rs::SliceBox<int>* __return, unsigned long n) {
  new (__return) auto(::test::MakeSliceBox(n));
}

static_assert((rs::SliceBox<int> (*)(unsigned long)) & ::test::MakeSliceBox);

static_assert((int (*)(rs::SliceBox<int> const&)) & ::test::SumSliceBox);

extern "C" unsigned long
__rust_thunk___ZN4test15ConsumeSliceBoxEN2rs8SliceBoxIiEE(
    rs::SliceBox<int>* b) {
  return ::test::ConsumeSliceBox(crubit::UnsafeTakeValue(b));
}

static_assert((unsigned long (*)(rs::SliceBox<int>)) & ::test::ConsumeSliceBox);

extern "C" void __rust_thunk___ZN4test10MakeStrBoxEv(rs::StrBox* __return) {
  new (__return) auto(::test::MakeStrBox());
}

static_assert((rs::StrBox (*)()) & ::test::MakeStrBox);

static_assert((unsigned long (*)(rs::StrBox const&)) & ::test::StrBoxLen);

extern "C" unsigned long __rust_thunk___ZN4test13ConsumeStrBoxEN2rs6StrBoxE(
    rs::StrBox* b) {
  return ::test::ConsumeStrBox(crubit::UnsafeTakeValue(b));
}

static_assert((unsigned long (*)(rs::StrBox)) & ::test::ConsumeStrBox);

static_assert(CRUBIT_SIZEOF(class ::std::initializer_list<int>) == 16);
static_assert(alignof(class ::std::initializer_list<int>) == 8);

extern "C" void __rust_thunk__d3f8dea6__ZNSt16initializer_listIiEC1Ev(
    class ::std::initializer_list<int>* __this) {
  crubit::construct_at(__this);
}

extern "C" bool __rust_thunk__c77c867f__ZNKSt16initializer_listIiE5emptyEv(
    class ::std::initializer_list<int> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::initializer_list<int>::*)() const) &
              ::std::initializer_list<int>::empty);

extern "C" int const* __rust_thunk__2f3b1219__ZNKSt16initializer_listIiE3endEv(
    class ::std::initializer_list<int> const* __this) {
  return __this->end();
}

static_assert((int const* (::std::initializer_list<int>::*)() const) &
              ::std::initializer_list<int>::end);

static_assert(
    CRUBIT_SIZEOF(class ::std::reverse_iterator<
                  class std::__wrap_iter<const struct test::NonTrivial*>>) ==
    8);
static_assert(
    alignof(class ::std::reverse_iterator<
            class std::__wrap_iter<const struct test::NonTrivial*>>) == 8);

extern "C" void
__rust_thunk__34c90dec__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEC1Ev(
    class ::std::reverse_iterator<
        class std::__wrap_iter<const struct test::NonTrivial*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__466217a1__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEC1ES6_(
    class ::std::reverse_iterator<
        class std::__wrap_iter<const struct test::NonTrivial*>>* __this,
    class ::std::__wrap_iter<const struct test::NonTrivial*>* __x) {
  crubit::construct_at(__this, crubit::UnsafeTakeValueOnConversion(__x));
}

static_assert(
    CRUBIT_SIZEOF(
        class ::std::reverse_iterator<class std::__wrap_iter<const int*>>) ==
    8);
static_assert(
    alignof(
        class ::std::reverse_iterator<class std::__wrap_iter<const int*>>) ==
    8);

extern "C" void
__rust_thunk__34c90dec__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1Ev(
    class ::std::reverse_iterator<class std::__wrap_iter<const int*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__466217a1__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1ES4_(
    class ::std::reverse_iterator<class std::__wrap_iter<const int*>>* __this,
    class ::std::__wrap_iter<const int*>* __x) {
  crubit::construct_at(__this, crubit::UnsafeTakeValueOnConversion(__x));
}

static_assert(
    CRUBIT_SIZEOF(class ::std::reverse_iterator<
                  class std::__wrap_iter<struct test::NonTrivial*>>) == 8);
static_assert(alignof(class ::std::reverse_iterator<
                      class std::__wrap_iter<struct test::NonTrivial*>>) == 8);

extern "C" void
__rust_thunk__34c90dec__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEC1Ev(
    class ::std::reverse_iterator<
        class std::__wrap_iter<struct test::NonTrivial*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__466217a1__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEC1ES5_(
    class ::std::reverse_iterator<
        class std::__wrap_iter<struct test::NonTrivial*>>* __this,
    class ::std::__wrap_iter<struct test::NonTrivial*>* __x) {
  crubit::construct_at(__this, crubit::UnsafeTakeValueOnConversion(__x));
}

static_assert(
    CRUBIT_SIZEOF(
        class ::std::reverse_iterator<class std::__wrap_iter<int*>>) == 8);
static_assert(
    alignof(class ::std::reverse_iterator<class std::__wrap_iter<int*>>) == 8);

extern "C" void
__rust_thunk__34c90dec__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1Ev(
    class ::std::reverse_iterator<class std::__wrap_iter<int*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__466217a1__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1ES3_(
    class ::std::reverse_iterator<class std::__wrap_iter<int*>>* __this,
    class ::std::__wrap_iter<int*>* __x) {
  crubit::construct_at(__this, crubit::UnsafeTakeValueOnConversion(__x));
}

static_assert(
    CRUBIT_SIZEOF(class ::std::__wrap_iter<const struct test::NonTrivial*>) ==
    8);
static_assert(
    alignof(class ::std::__wrap_iter<const struct test::NonTrivial*>) == 8);

extern "C" void
__rust_thunk__27ad6cd0__ZNSt3__u11__wrap_iterIPKN4test10NonTrivialEEC1Ev(
    class ::std::__wrap_iter<const struct test::NonTrivial*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__d9068c5b__ZNKSt3__u11__wrap_iterIPKN4test10NonTrivialEEplEl(
    class ::std::__wrap_iter<const struct test::NonTrivial*>* __return,
    class ::std::__wrap_iter<const struct test::NonTrivial*> const* __this,
    ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class ::std::__wrap_iter<const struct test::NonTrivial*> (
                  ::std::__wrap_iter<const struct test::NonTrivial*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<const struct test::NonTrivial*>::operator+);

extern "C" class ::std::__wrap_iter<const struct test::NonTrivial*>*
__rust_thunk__ed7b06d0__ZNSt3__u11__wrap_iterIPKN4test10NonTrivialEEpLEl(
    class ::std::__wrap_iter<const struct test::NonTrivial*>* __this,
    ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert(
    (class ::std::__wrap_iter<const struct test::NonTrivial*> &
     (::std::__wrap_iter<const struct test::NonTrivial*>::*)(ptrdiff_t)) &
    ::std::__wrap_iter<const struct test::NonTrivial*>::operator+=);

extern "C" void
__rust_thunk__58f1a91b__ZNKSt3__u11__wrap_iterIPKN4test10NonTrivialEEmiEl(
    class ::std::__wrap_iter<const struct test::NonTrivial*>* __return,
    class ::std::__wrap_iter<const struct test::NonTrivial*> const* __this,
    ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class ::std::__wrap_iter<const struct test::NonTrivial*> (
                  ::std::__wrap_iter<const struct test::NonTrivial*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<const struct test::NonTrivial*>::operator-);

extern "C" class ::std::__wrap_iter<const struct test::NonTrivial*>*
__rust_thunk__ebbbe4fe__ZNSt3__u11__wrap_iterIPKN4test10NonTrivialEEmIEl(
    class ::std::__wrap_iter<const struct test::NonTrivial*>* __this,
    ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert(
    (class ::std::__wrap_iter<const struct test::NonTrivial*> &
     (::std::__wrap_iter<const struct test::NonTrivial*>::*)(ptrdiff_t)) &
    ::std::__wrap_iter<const struct test::NonTrivial*>::operator-=);

extern "C" struct ::test::NonTrivial const*
__rust_thunk__3dcfade5__ZNKSt3__u11__wrap_iterIPKN4test10NonTrivialEEixEl(
    class ::std::__wrap_iter<const struct test::NonTrivial*> const* __this,
    ptrdiff_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert((struct ::test::NonTrivial const& (
                  ::std::__wrap_iter<const struct test::NonTrivial*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<const struct test::NonTrivial*>::operator[]);

static_assert(CRUBIT_SIZEOF(class ::std::__wrap_iter<const int*>) == 8);
static_assert(alignof(class ::std::__wrap_iter<const int*>) == 8);

extern "C" void __rust_thunk__27ad6cd0__ZNSt3__u11__wrap_iterIPKiEC1Ev(
    class ::std::__wrap_iter<const int*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__d9068c5b__ZNKSt3__u11__wrap_iterIPKiEplEl(
    class ::std::__wrap_iter<const int*>* __return,
    class ::std::__wrap_iter<const int*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class ::std::__wrap_iter<const int*> (
                  ::std::__wrap_iter<const int*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const int*>::operator+);

extern "C" class ::std::__wrap_iter<const int*>*
__rust_thunk__ed7b06d0__ZNSt3__u11__wrap_iterIPKiEpLEl(
    class ::std::__wrap_iter<const int*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class ::std::__wrap_iter<const int*> &
               (::std::__wrap_iter<const int*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const int*>::operator+=);

extern "C" void __rust_thunk__58f1a91b__ZNKSt3__u11__wrap_iterIPKiEmiEl(
    class ::std::__wrap_iter<const int*>* __return,
    class ::std::__wrap_iter<const int*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class ::std::__wrap_iter<const int*> (
                  ::std::__wrap_iter<const int*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const int*>::operator-);

extern "C" class ::std::__wrap_iter<const int*>*
__rust_thunk__ebbbe4fe__ZNSt3__u11__wrap_iterIPKiEmIEl(
    class ::std::__wrap_iter<const int*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class ::std::__wrap_iter<const int*> &
               (::std::__wrap_iter<const int*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const int*>::operator-=);

extern "C" int const* __rust_thunk__3dcfade5__ZNKSt3__u11__wrap_iterIPKiEixEl(
    class ::std::__wrap_iter<const int*> const* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert((int const& (::std::__wrap_iter<const int*>::*)(ptrdiff_t)
                   const) &
              ::std::__wrap_iter<const int*>::operator[]);

static_assert(
    CRUBIT_SIZEOF(class ::std::__wrap_iter<struct test::NonTrivial*>) == 8);
static_assert(alignof(class ::std::__wrap_iter<struct test::NonTrivial*>) == 8);

extern "C" void
__rust_thunk__27ad6cd0__ZNSt3__u11__wrap_iterIPN4test10NonTrivialEEC1Ev(
    class ::std::__wrap_iter<struct test::NonTrivial*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__d9068c5b__ZNKSt3__u11__wrap_iterIPN4test10NonTrivialEEplEl(
    class ::std::__wrap_iter<struct test::NonTrivial*>* __return,
    class ::std::__wrap_iter<struct test::NonTrivial*> const* __this,
    ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class ::std::__wrap_iter<struct test::NonTrivial*> (
                  ::std::__wrap_iter<struct test::NonTrivial*>::*)(ptrdiff_t)
                   const) &
              ::std::__wrap_iter<struct test::NonTrivial*>::operator+);

extern "C" class ::std::__wrap_iter<struct test::NonTrivial*>*
__rust_thunk__ed7b06d0__ZNSt3__u11__wrap_iterIPN4test10NonTrivialEEpLEl(
    class ::std::__wrap_iter<struct test::NonTrivial*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class ::std::__wrap_iter<struct test::NonTrivial*> &
               (::std::__wrap_iter<struct test::NonTrivial*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<struct test::NonTrivial*>::operator+=);

extern "C" void
__rust_thunk__58f1a91b__ZNKSt3__u11__wrap_iterIPN4test10NonTrivialEEmiEl(
    class ::std::__wrap_iter<struct test::NonTrivial*>* __return,
    class ::std::__wrap_iter<struct test::NonTrivial*> const* __this,
    ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class ::std::__wrap_iter<struct test::NonTrivial*> (
                  ::std::__wrap_iter<struct test::NonTrivial*>::*)(ptrdiff_t)
                   const) &
              ::std::__wrap_iter<struct test::NonTrivial*>::operator-);

extern "C" class ::std::__wrap_iter<struct test::NonTrivial*>*
__rust_thunk__ebbbe4fe__ZNSt3__u11__wrap_iterIPN4test10NonTrivialEEmIEl(
    class ::std::__wrap_iter<struct test::NonTrivial*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class ::std::__wrap_iter<struct test::NonTrivial*> &
               (::std::__wrap_iter<struct test::NonTrivial*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<struct test::NonTrivial*>::operator-=);

static_assert(CRUBIT_SIZEOF(class ::std::__wrap_iter<int*>) == 8);
static_assert(alignof(class ::std::__wrap_iter<int*>) == 8);

extern "C" void __rust_thunk__27ad6cd0__ZNSt3__u11__wrap_iterIPiEC1Ev(
    class ::std::__wrap_iter<int*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__d9068c5b__ZNKSt3__u11__wrap_iterIPiEplEl(
    class ::std::__wrap_iter<int*>* __return,
    class ::std::__wrap_iter<int*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class ::std::__wrap_iter<int*> (::std::__wrap_iter<int*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<int*>::operator+);

extern "C" class ::std::__wrap_iter<int*>*
__rust_thunk__ed7b06d0__ZNSt3__u11__wrap_iterIPiEpLEl(
    class ::std::__wrap_iter<int*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class ::std::__wrap_iter<int*> &
               (::std::__wrap_iter<int*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<int*>::operator+=);

extern "C" void __rust_thunk__58f1a91b__ZNKSt3__u11__wrap_iterIPiEmiEl(
    class ::std::__wrap_iter<int*>* __return,
    class ::std::__wrap_iter<int*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class ::std::__wrap_iter<int*> (::std::__wrap_iter<int*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<int*>::operator-);

extern "C" class ::std::__wrap_iter<int*>*
__rust_thunk__ebbbe4fe__ZNSt3__u11__wrap_iterIPiEmIEl(
    class ::std::__wrap_iter<int*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class ::std::__wrap_iter<int*> &
               (::std::__wrap_iter<int*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<int*>::operator-=);

static_assert(CRUBIT_SIZEOF(class ::std::span<const struct test::NonTrivial,
                                              18446744073709551615UL>) == 16);
static_assert(alignof(class ::std::span<const struct test::NonTrivial,
                                        18446744073709551615UL>) == 8);

extern "C" void
__rust_thunk__a2602314__ZNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEC1Ev(
    class ::std::span<const struct test::NonTrivial, 18446744073709551615UL>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__6ddb9499__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE5firstEm(
    class ::std::span<const struct test::NonTrivial, 18446744073709551615UL>*
        __return,
    class ::std::span<const struct test::NonTrivial,
                      18446744073709551615UL> const* __this,
    size_t __count) {
  new (__return) auto(__this->first(__count));
}

static_assert(
    (class ::std::span<const struct test::NonTrivial, 18446744073709551615UL> (
        ::std::span<const struct test::NonTrivial, 18446744073709551615UL>::*)(
        size_t) const) &
    ::std::span<const struct test::NonTrivial, 18446744073709551615UL>::first);

extern "C" void
__rust_thunk__70f2e79a__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE4lastEm(
    class ::std::span<const struct test::NonTrivial, 18446744073709551615UL>*
        __return,
    class ::std::span<const struct test::NonTrivial,
                      18446744073709551615UL> const* __this,
    size_t __count) {
  new (__return) auto(__this->last(__count));
}

static_assert(
    (class ::std::span<const struct test::NonTrivial, 18446744073709551615UL> (
        ::std::span<const struct test::NonTrivial, 18446744073709551615UL>::*)(
        size_t) const) &
    ::std::span<const struct test::NonTrivial, 18446744073709551615UL>::last);

extern "C" void
__rust_thunk__0b402a5e__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE7subspanEmm(
    class ::std::span<const struct test::NonTrivial, 18446744073709551615UL>*
        __return,
    class ::std::span<const struct test::NonTrivial,
                      18446744073709551615UL> const* __this,
    size_t __offset, size_t __count) {
  new (__return) auto(__this->subspan(__offset, __count));
}

static_assert(
    (class ::std::span<const struct test::NonTrivial, 18446744073709551615UL> (
        ::std::span<const struct test::NonTrivial, 18446744073709551615UL>::*)(
        size_t, size_t) const) &
    ::std::span<const struct test::NonTrivial,
                18446744073709551615UL>::subspan);

extern "C" size_t
__rust_thunk__db8aea99__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE10size_bytesEv(
    class ::std::span<const struct test::NonTrivial,
                      18446744073709551615UL> const* __this) {
  return __this->size_bytes();
}

static_assert((size_t (::std::span<const struct test::NonTrivial,
                                   18446744073709551615UL>::*)() const) &
              ::std::span<const struct test::NonTrivial,
                          18446744073709551615UL>::size_bytes);

extern "C" bool
__rust_thunk__adeb3748__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE5emptyEv(
    class ::std::span<const struct test::NonTrivial,
                      18446744073709551615UL> const* __this) {
  return __this->empty();
}

static_assert(
    (bool (
        ::std::span<const struct test::NonTrivial, 18446744073709551615UL>::*)()
         const) &
    ::std::span<const struct test::NonTrivial, 18446744073709551615UL>::empty);

extern "C" struct ::test::NonTrivial const*
__rust_thunk__863bfd93__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEixEm(
    class ::std::span<const struct test::NonTrivial,
                      18446744073709551615UL> const* __this,
    size_t __idx) {
  return std::addressof(__this->operator[](__idx));
}

static_assert((struct ::test::NonTrivial const& (
                  ::std::span<const struct test::NonTrivial,
                              18446744073709551615UL>::*)(size_t) const) &
              ::std::span<const struct test::NonTrivial,
                          18446744073709551615UL>::operator[]);

extern "C" struct ::test::NonTrivial const*
__rust_thunk__0eff2f4e__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE5frontEv(
    class ::std::span<const struct test::NonTrivial,
                      18446744073709551615UL> const* __this) {
  return std::addressof(__this->front());
}

static_assert(
    (struct ::test::NonTrivial const& (
        ::std::span<const struct test::NonTrivial, 18446744073709551615UL>::*)()
         const) &
    ::std::span<const struct test::NonTrivial, 18446744073709551615UL>::front);

extern "C" struct ::test::NonTrivial const*
__rust_thunk__0b709465__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE4backEv(
    class ::std::span<const struct test::NonTrivial,
                      18446744073709551615UL> const* __this) {
  return std::addressof(__this->back());
}

static_assert(
    (struct ::test::NonTrivial const& (
        ::std::span<const struct test::NonTrivial, 18446744073709551615UL>::*)()
         const) &
    ::std::span<const struct test::NonTrivial, 18446744073709551615UL>::back);

extern "C" void
__rust_thunk__c3b9de9e__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE5beginEv(
    class ::std::__wrap_iter<const struct test::NonTrivial*>* __return,
    class ::std::span<const struct test::NonTrivial,
                      18446744073709551615UL> const* __this) {
  new (__return) auto(__this->begin());
}

static_assert(
    (class ::std::__wrap_iter<const struct test::NonTrivial*> (
        ::std::span<const struct test::NonTrivial, 18446744073709551615UL>::*)()
         const) &
    ::std::span<const struct test::NonTrivial, 18446744073709551615UL>::begin);

extern "C" void
__rust_thunk__c0096ae4__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE3endEv(
    class ::std::__wrap_iter<const struct test::NonTrivial*>* __return,
    class ::std::span<const struct test::NonTrivial,
                      18446744073709551615UL> const* __this) {
  new (__return) auto(__this->end());
}

static_assert(
    (class ::std::__wrap_iter<const struct test::NonTrivial*> (
        ::std::span<const struct test::NonTrivial, 18446744073709551615UL>::*)()
         const) &
    ::std::span<const struct test::NonTrivial, 18446744073709551615UL>::end);

extern "C" void
__rust_thunk__30ed2d1f__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE6rbeginEv(
    class ::std::reverse_iterator<
        class std::__wrap_iter<const struct test::NonTrivial*>>* __return,
    class ::std::span<const struct test::NonTrivial,
                      18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rbegin());
}

static_assert(
    (class ::std::reverse_iterator<
        class std::__wrap_iter<const struct test::NonTrivial*>> (
        ::std::span<const struct test::NonTrivial, 18446744073709551615UL>::*)()
         const) &
    ::std::span<const struct test::NonTrivial, 18446744073709551615UL>::rbegin);

extern "C" void
__rust_thunk__bd3e0276__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE4rendEv(
    class ::std::reverse_iterator<
        class std::__wrap_iter<const struct test::NonTrivial*>>* __return,
    class ::std::span<const struct test::NonTrivial,
                      18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rend());
}

static_assert(
    (class ::std::reverse_iterator<
        class std::__wrap_iter<const struct test::NonTrivial*>> (
        ::std::span<const struct test::NonTrivial, 18446744073709551615UL>::*)()
         const) &
    ::std::span<const struct test::NonTrivial, 18446744073709551615UL>::rend);

static_assert(
    CRUBIT_SIZEOF(class ::std::span<const int, 18446744073709551615UL>) == 16);
static_assert(alignof(class ::std::span<const int, 18446744073709551615UL>) ==
              8);

extern "C" void
__rust_thunk__a2602314__ZNSt3__u4spanIKiLm18446744073709551615EEC1Ev(
    class ::std::span<const int, 18446744073709551615UL>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__6ddb9499__ZNKSt3__u4spanIKiLm18446744073709551615EE5firstEm(
    class ::std::span<const int, 18446744073709551615UL>* __return,
    class ::std::span<const int, 18446744073709551615UL> const* __this,
    size_t __count) {
  new (__return) auto(__this->first(__count));
}

static_assert((class ::std::span<const int, 18446744073709551615UL> (
                  ::std::span<const int, 18446744073709551615UL>::*)(size_t)
                   const) &
              ::std::span<const int, 18446744073709551615UL>::first);

extern "C" void
__rust_thunk__70f2e79a__ZNKSt3__u4spanIKiLm18446744073709551615EE4lastEm(
    class ::std::span<const int, 18446744073709551615UL>* __return,
    class ::std::span<const int, 18446744073709551615UL> const* __this,
    size_t __count) {
  new (__return) auto(__this->last(__count));
}

static_assert((class ::std::span<const int, 18446744073709551615UL> (
                  ::std::span<const int, 18446744073709551615UL>::*)(size_t)
                   const) &
              ::std::span<const int, 18446744073709551615UL>::last);

extern "C" void
__rust_thunk__0b402a5e__ZNKSt3__u4spanIKiLm18446744073709551615EE7subspanEmm(
    class ::std::span<const int, 18446744073709551615UL>* __return,
    class ::std::span<const int, 18446744073709551615UL> const* __this,
    size_t __offset, size_t __count) {
  new (__return) auto(__this->subspan(__offset, __count));
}

static_assert((class ::std::span<const int, 18446744073709551615UL> (
                  ::std::span<const int, 18446744073709551615UL>::*)(size_t,
                                                                     size_t)
                   const) &
              ::std::span<const int, 18446744073709551615UL>::subspan);

extern "C" size_t
__rust_thunk__db8aea99__ZNKSt3__u4spanIKiLm18446744073709551615EE10size_bytesEv(
    class ::std::span<const int, 18446744073709551615UL> const* __this) {
  return __this->size_bytes();
}

static_assert((size_t (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::size_bytes);

extern "C" bool
__rust_thunk__adeb3748__ZNKSt3__u4spanIKiLm18446744073709551615EE5emptyEv(
    class ::std::span<const int, 18446744073709551615UL> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::empty);

extern "C" int const*
__rust_thunk__863bfd93__ZNKSt3__u4spanIKiLm18446744073709551615EEixEm(
    class ::std::span<const int, 18446744073709551615UL> const* __this,
    size_t __idx) {
  return std::addressof(__this->operator[](__idx));
}

static_assert((int const& (::std::span<const int, 18446744073709551615UL>::*)(
                  size_t) const) &
              ::std::span<const int, 18446744073709551615UL>::operator[]);

extern "C" int const*
__rust_thunk__0eff2f4e__ZNKSt3__u4spanIKiLm18446744073709551615EE5frontEv(
    class ::std::span<const int, 18446744073709551615UL> const* __this) {
  return std::addressof(__this->front());
}

static_assert((int const& (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::front);

extern "C" int const*
__rust_thunk__0b709465__ZNKSt3__u4spanIKiLm18446744073709551615EE4backEv(
    class ::std::span<const int, 18446744073709551615UL> const* __this) {
  return std::addressof(__this->back());
}

static_assert((int const& (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::back);

extern "C" void
__rust_thunk__c3b9de9e__ZNKSt3__u4spanIKiLm18446744073709551615EE5beginEv(
    class ::std::__wrap_iter<const int*>* __return,
    class ::std::span<const int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->begin());
}

static_assert((class ::std::__wrap_iter<const int*> (
                  ::std::span<const int, 18446744073709551615UL>::*)() const) &
              ::std::span<const int, 18446744073709551615UL>::begin);

extern "C" void
__rust_thunk__c0096ae4__ZNKSt3__u4spanIKiLm18446744073709551615EE3endEv(
    class ::std::__wrap_iter<const int*>* __return,
    class ::std::span<const int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->end());
}

static_assert((class ::std::__wrap_iter<const int*> (
                  ::std::span<const int, 18446744073709551615UL>::*)() const) &
              ::std::span<const int, 18446744073709551615UL>::end);

extern "C" void
__rust_thunk__30ed2d1f__ZNKSt3__u4spanIKiLm18446744073709551615EE6rbeginEv(
    class ::std::reverse_iterator<class std::__wrap_iter<const int*>>* __return,
    class ::std::span<const int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rbegin());
}

static_assert(
    (class ::std::reverse_iterator<class std::__wrap_iter<const int*>> (
        ::std::span<const int, 18446744073709551615UL>::*)() const) &
    ::std::span<const int, 18446744073709551615UL>::rbegin);

extern "C" void
__rust_thunk__bd3e0276__ZNKSt3__u4spanIKiLm18446744073709551615EE4rendEv(
    class ::std::reverse_iterator<class std::__wrap_iter<const int*>>* __return,
    class ::std::span<const int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rend());
}

static_assert(
    (class ::std::reverse_iterator<class std::__wrap_iter<const int*>> (
        ::std::span<const int, 18446744073709551615UL>::*)() const) &
    ::std::span<const int, 18446744073709551615UL>::rend);

static_assert(CRUBIT_SIZEOF(class ::std::span<struct test::NonTrivial,
                                              18446744073709551615UL>) == 16);
static_assert(alignof(class ::std::span<struct test::NonTrivial,
                                        18446744073709551615UL>) == 8);

extern "C" void
__rust_thunk__a2602314__ZNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEC1Ev(
    class ::std::span<struct test::NonTrivial, 18446744073709551615UL>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__6ddb9499__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE5firstEm(
    class ::std::span<struct test::NonTrivial, 18446744073709551615UL>*
        __return,
    class ::std::span<struct test::NonTrivial, 18446744073709551615UL> const*
        __this,
    size_t __count) {
  new (__return) auto(__this->first(__count));
}

static_assert(
    (class ::std::span<struct test::NonTrivial, 18446744073709551615UL> (
        ::std::span<struct test::NonTrivial, 18446744073709551615UL>::*)(size_t)
         const) &
    ::std::span<struct test::NonTrivial, 18446744073709551615UL>::first);

extern "C" void
__rust_thunk__70f2e79a__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE4lastEm(
    class ::std::span<struct test::NonTrivial, 18446744073709551615UL>*
        __return,
    class ::std::span<struct test::NonTrivial, 18446744073709551615UL> const*
        __this,
    size_t __count) {
  new (__return) auto(__this->last(__count));
}

static_assert(
    (class ::std::span<struct test::NonTrivial, 18446744073709551615UL> (
        ::std::span<struct test::NonTrivial, 18446744073709551615UL>::*)(size_t)
         const) &
    ::std::span<struct test::NonTrivial, 18446744073709551615UL>::last);

extern "C" void
__rust_thunk__0b402a5e__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE7subspanEmm(
    class ::std::span<struct test::NonTrivial, 18446744073709551615UL>*
        __return,
    class ::std::span<struct test::NonTrivial, 18446744073709551615UL> const*
        __this,
    size_t __offset, size_t __count) {
  new (__return) auto(__this->subspan(__offset, __count));
}

static_assert(
    (class ::std::span<struct test::NonTrivial, 18446744073709551615UL> (
        ::std::span<struct test::NonTrivial, 18446744073709551615UL>::*)(size_t,
                                                                         size_t)
         const) &
    ::std::span<struct test::NonTrivial, 18446744073709551615UL>::subspan);

extern "C" size_t
__rust_thunk__db8aea99__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE10size_bytesEv(
    class ::std::span<struct test::NonTrivial, 18446744073709551615UL> const*
        __this) {
  return __this->size_bytes();
}

static_assert(
    (size_t (::std::span<struct test::NonTrivial, 18446744073709551615UL>::*)()
         const) &
    ::std::span<struct test::NonTrivial, 18446744073709551615UL>::size_bytes);

extern "C" bool
__rust_thunk__adeb3748__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE5emptyEv(
    class ::std::span<struct test::NonTrivial, 18446744073709551615UL> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::span<struct test::NonTrivial, 18446744073709551615UL>::*)()
         const) &
    ::std::span<struct test::NonTrivial, 18446744073709551615UL>::empty);

extern "C" struct ::test::NonTrivial*
__rust_thunk__0eff2f4e__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE5frontEv(
    class ::std::span<struct test::NonTrivial, 18446744073709551615UL> const*
        __this) {
  return std::addressof(__this->front());
}

static_assert(
    (struct ::test::NonTrivial &
     (::std::span<struct test::NonTrivial, 18446744073709551615UL>::*)()
         const) &
    ::std::span<struct test::NonTrivial, 18446744073709551615UL>::front);

extern "C" struct ::test::NonTrivial*
__rust_thunk__0b709465__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE4backEv(
    class ::std::span<struct test::NonTrivial, 18446744073709551615UL> const*
        __this) {
  return std::addressof(__this->back());
}

static_assert(
    (struct ::test::NonTrivial &
     (::std::span<struct test::NonTrivial, 18446744073709551615UL>::*)()
         const) &
    ::std::span<struct test::NonTrivial, 18446744073709551615UL>::back);

extern "C" void
__rust_thunk__c3b9de9e__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE5beginEv(
    class ::std::__wrap_iter<struct test::NonTrivial*>* __return,
    class ::std::span<struct test::NonTrivial, 18446744073709551615UL> const*
        __this) {
  new (__return) auto(__this->begin());
}

static_assert(
    (class ::std::__wrap_iter<struct test::NonTrivial*> (
        ::std::span<struct test::NonTrivial, 18446744073709551615UL>::*)()
         const) &
    ::std::span<struct test::NonTrivial, 18446744073709551615UL>::begin);

extern "C" void
__rust_thunk__c0096ae4__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE3endEv(
    class ::std::__wrap_iter<struct test::NonTrivial*>* __return,
    class ::std::span<struct test::NonTrivial, 18446744073709551615UL> const*
        __this) {
  new (__return) auto(__this->end());
}

static_assert(
    (class ::std::__wrap_iter<struct test::NonTrivial*> (
        ::std::span<struct test::NonTrivial, 18446744073709551615UL>::*)()
         const) &
    ::std::span<struct test::NonTrivial, 18446744073709551615UL>::end);

extern "C" void
__rust_thunk__30ed2d1f__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE6rbeginEv(
    class ::std::reverse_iterator<
        class std::__wrap_iter<struct test::NonTrivial*>>* __return,
    class ::std::span<struct test::NonTrivial, 18446744073709551615UL> const*
        __this) {
  new (__return) auto(__this->rbegin());
}

static_assert(
    (class ::std::reverse_iterator<
        class std::__wrap_iter<struct test::NonTrivial*>> (
        ::std::span<struct test::NonTrivial, 18446744073709551615UL>::*)()
         const) &
    ::std::span<struct test::NonTrivial, 18446744073709551615UL>::rbegin);

extern "C" void
__rust_thunk__bd3e0276__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE4rendEv(
    class ::std::reverse_iterator<
        class std::__wrap_iter<struct test::NonTrivial*>>* __return,
    class ::std::span<struct test::NonTrivial, 18446744073709551615UL> const*
        __this) {
  new (__return) auto(__this->rend());
}

static_assert(
    (class ::std::reverse_iterator<
        class std::__wrap_iter<struct test::NonTrivial*>> (
        ::std::span<struct test::NonTrivial, 18446744073709551615UL>::*)()
         const) &
    ::std::span<struct test::NonTrivial, 18446744073709551615UL>::rend);

static_assert(CRUBIT_SIZEOF(class ::std::span<int, 18446744073709551615UL>) ==
              16);
static_assert(alignof(class ::std::span<int, 18446744073709551615UL>) == 8);

extern "C" void
__rust_thunk__a2602314__ZNSt3__u4spanIiLm18446744073709551615EEC1Ev(
    class ::std::span<int, 18446744073709551615UL>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__6ddb9499__ZNKSt3__u4spanIiLm18446744073709551615EE5firstEm(
    class ::std::span<int, 18446744073709551615UL>* __return,
    class ::std::span<int, 18446744073709551615UL> const* __this,
    size_t __count) {
  new (__return) auto(__this->first(__count));
}

static_assert((class ::std::span<int, 18446744073709551615UL> (
                  ::std::span<int, 18446744073709551615UL>::*)(size_t) const) &
              ::std::span<int, 18446744073709551615UL>::first);

extern "C" void
__rust_thunk__70f2e79a__ZNKSt3__u4spanIiLm18446744073709551615EE4lastEm(
    class ::std::span<int, 18446744073709551615UL>* __return,
    class ::std::span<int, 18446744073709551615UL> const* __this,
    size_t __count) {
  new (__return) auto(__this->last(__count));
}

static_assert((class ::std::span<int, 18446744073709551615UL> (
                  ::std::span<int, 18446744073709551615UL>::*)(size_t) const) &
              ::std::span<int, 18446744073709551615UL>::last);

extern "C" void
__rust_thunk__0b402a5e__ZNKSt3__u4spanIiLm18446744073709551615EE7subspanEmm(
    class ::std::span<int, 18446744073709551615UL>* __return,
    class ::std::span<int, 18446744073709551615UL> const* __this,
    size_t __offset, size_t __count) {
  new (__return) auto(__this->subspan(__offset, __count));
}

static_assert((class ::std::span<int, 18446744073709551615UL> (
                  ::std::span<int, 18446744073709551615UL>::*)(size_t, size_t)
                   const) &
              ::std::span<int, 18446744073709551615UL>::subspan);

extern "C" size_t
__rust_thunk__db8aea99__ZNKSt3__u4spanIiLm18446744073709551615EE10size_bytesEv(
    class ::std::span<int, 18446744073709551615UL> const* __this) {
  return __this->size_bytes();
}

static_assert((size_t (::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::size_bytes);

extern "C" bool
__rust_thunk__adeb3748__ZNKSt3__u4spanIiLm18446744073709551615EE5emptyEv(
    class ::std::span<int, 18446744073709551615UL> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::empty);

extern "C" int*
__rust_thunk__0eff2f4e__ZNKSt3__u4spanIiLm18446744073709551615EE5frontEv(
    class ::std::span<int, 18446744073709551615UL> const* __this) {
  return std::addressof(__this->front());
}

static_assert((int& (::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::front);

extern "C" int*
__rust_thunk__0b709465__ZNKSt3__u4spanIiLm18446744073709551615EE4backEv(
    class ::std::span<int, 18446744073709551615UL> const* __this) {
  return std::addressof(__this->back());
}

static_assert((int& (::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::back);

extern "C" void
__rust_thunk__c3b9de9e__ZNKSt3__u4spanIiLm18446744073709551615EE5beginEv(
    class ::std::__wrap_iter<int*>* __return,
    class ::std::span<int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->begin());
}

static_assert((class ::std::__wrap_iter<int*> (
                  ::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::begin);

extern "C" void
__rust_thunk__c0096ae4__ZNKSt3__u4spanIiLm18446744073709551615EE3endEv(
    class ::std::__wrap_iter<int*>* __return,
    class ::std::span<int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->end());
}

static_assert((class ::std::__wrap_iter<int*> (
                  ::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::end);

extern "C" void
__rust_thunk__30ed2d1f__ZNKSt3__u4spanIiLm18446744073709551615EE6rbeginEv(
    class ::std::reverse_iterator<class std::__wrap_iter<int*>>* __return,
    class ::std::span<int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rbegin());
}

static_assert((class ::std::reverse_iterator<class std::__wrap_iter<int*>> (
                  ::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::rbegin);

extern "C" void
__rust_thunk__bd3e0276__ZNKSt3__u4spanIiLm18446744073709551615EE4rendEv(
    class ::std::reverse_iterator<class std::__wrap_iter<int*>>* __return,
    class ::std::span<int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rend());
}

static_assert((class ::std::reverse_iterator<class std::__wrap_iter<int*>> (
                  ::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::rend);

#pragma clang diagnostic pop
