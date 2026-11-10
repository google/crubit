// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/send_sync_inference:send_sync_inference

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/send_sync_inference/send_sync_inference.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"

static_assert(CRUBIT_SIZEOF(struct crubit::test::Plain) == 8);
static_assert(alignof(struct crubit::test::Plain) == 4);
static_assert(CRUBIT_OFFSET_OF(x, struct crubit::test::Plain) == 0);
static_assert(CRUBIT_OFFSET_OF(y, struct crubit::test::Plain) == 4);

extern "C" void __rust_thunk___ZN6crubit4test5PlainC1Ev(
    struct crubit::test::Plain* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct crubit::test::Empty) == 1);
static_assert(alignof(struct crubit::test::Empty) == 1);

extern "C" void __rust_thunk___ZN6crubit4test5EmptyC1Ev(
    struct crubit::test::Empty* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct crubit::test::ContainsPlain) == 8);
static_assert(alignof(struct crubit::test::ContainsPlain) == 4);
static_assert(CRUBIT_OFFSET_OF(plain, struct crubit::test::ContainsPlain) == 0);

extern "C" void __rust_thunk___ZN6crubit4test13ContainsPlainC1Ev(
    struct crubit::test::ContainsPlain* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class crubit::test::NonAggregatePlain) == 4);
static_assert(alignof(class crubit::test::NonAggregatePlain) == 4);
static_assert(CRUBIT_OFFSET_OF(x_, class crubit::test::NonAggregatePlain) == 0);

extern "C" void __rust_thunk___ZN6crubit4test17NonAggregatePlainC1Ev(
    class crubit::test::NonAggregatePlain* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct crubit::test::WithZeroWidthBitfield) == 8);
static_assert(alignof(struct crubit::test::WithZeroWidthBitfield) == 4);

extern "C" void __rust_thunk___ZN6crubit4test21WithZeroWidthBitfieldC1Ev(
    struct crubit::test::WithZeroWidthBitfield* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct crubit::test::WithPointer) == 8);
static_assert(alignof(struct crubit::test::WithPointer) == 8);
static_assert(CRUBIT_OFFSET_OF(p, struct crubit::test::WithPointer) == 0);

extern "C" void __rust_thunk___ZN6crubit4test11WithPointerC1Ev(
    struct crubit::test::WithPointer* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct crubit::test::ContainsWithPointer) == 8);
static_assert(alignof(struct crubit::test::ContainsWithPointer) == 8);
static_assert(CRUBIT_OFFSET_OF(with_pointer,
                               struct crubit::test::ContainsWithPointer) == 0);

extern "C" void __rust_thunk___ZN6crubit4test19ContainsWithPointerC1Ev(
    struct crubit::test::ContainsWithPointer* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class crubit::test::WithPrivateField) == 8);
static_assert(alignof(class crubit::test::WithPrivateField) == 4);
static_assert(CRUBIT_OFFSET_OF(public_field_,
                               class crubit::test::WithPrivateField) == 0);

extern "C" void __rust_thunk___ZN6crubit4test16WithPrivateFieldC1Ev(
    class crubit::test::WithPrivateField* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct crubit::test::WithUnsupportedField) == 4);
static_assert(alignof(struct crubit::test::WithUnsupportedField) == 4);
static_assert(CRUBIT_OFFSET_OF(unsupported,
                               struct crubit::test::WithUnsupportedField) == 0);

extern "C" void __rust_thunk___ZN6crubit4test20WithUnsupportedFieldC1Ev(
    struct crubit::test::WithUnsupportedField* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct crubit::test::Base) == 4);
static_assert(alignof(struct crubit::test::Base) == 4);
static_assert(CRUBIT_OFFSET_OF(base_field, struct crubit::test::Base) == 0);

extern "C" void __rust_thunk___ZN6crubit4test4BaseC1Ev(
    struct crubit::test::Base* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct crubit::test::Derived) == 8);
static_assert(alignof(struct crubit::test::Derived) == 4);
static_assert(CRUBIT_OFFSET_OF(derived_field, struct crubit::test::Derived) ==
              4);

extern "C" void __rust_thunk___ZN6crubit4test7DerivedC1Ev(
    struct crubit::test::Derived* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct crubit::test::NotSend) == 4);
static_assert(alignof(struct crubit::test::NotSend) == 4);
static_assert(CRUBIT_OFFSET_OF(x, struct crubit::test::NotSend) == 0);

extern "C" void __rust_thunk___ZN6crubit4test7NotSendC1Ev(
    struct crubit::test::NotSend* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct crubit::test::NotSync) == 4);
static_assert(alignof(struct crubit::test::NotSync) == 4);
static_assert(CRUBIT_OFFSET_OF(x, struct crubit::test::NotSync) == 0);

extern "C" void __rust_thunk___ZN6crubit4test7NotSyncC1Ev(
    struct crubit::test::NotSync* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct crubit::test::NotSendNotSync) == 4);
static_assert(alignof(struct crubit::test::NotSendNotSync) == 4);
static_assert(CRUBIT_OFFSET_OF(x, struct crubit::test::NotSendNotSync) == 0);

extern "C" void __rust_thunk___ZN6crubit4test14NotSendNotSyncC1Ev(
    struct crubit::test::NotSendNotSync* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct crubit::test::ExplicitSendSync) == 8);
static_assert(alignof(struct crubit::test::ExplicitSendSync) == 8);
static_assert(CRUBIT_OFFSET_OF(p, struct crubit::test::ExplicitSendSync) == 0);

extern "C" void __rust_thunk___ZN6crubit4test16ExplicitSendSyncC1Ev(
    struct crubit::test::ExplicitSendSync* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct crubit::test::ThreadAffineTag) == 1);
static_assert(alignof(struct crubit::test::ThreadAffineTag) == 1);

extern "C" void __rust_thunk___ZN6crubit4test15ThreadAffineTagC1Ev(
    struct crubit::test::ThreadAffineTag* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct crubit::test::DerivedFromThreadAffineTag) ==
              4);
static_assert(alignof(struct crubit::test::DerivedFromThreadAffineTag) == 4);
static_assert(
    CRUBIT_OFFSET_OF(x, struct crubit::test::DerivedFromThreadAffineTag) == 0);

extern "C" void __rust_thunk___ZN6crubit4test26DerivedFromThreadAffineTagC1Ev(
    struct crubit::test::DerivedFromThreadAffineTag* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct crubit::test::EmptyBase) == 1);
static_assert(alignof(struct crubit::test::EmptyBase) == 1);

extern "C" void __rust_thunk___ZN6crubit4test9EmptyBaseC1Ev(
    struct crubit::test::EmptyBase* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class crubit::test::DerivedFromPrivateEmpty) == 4);
static_assert(alignof(class crubit::test::DerivedFromPrivateEmpty) == 4);
static_assert(CRUBIT_OFFSET_OF(x_,
                               class crubit::test::DerivedFromPrivateEmpty) ==
              0);

extern "C" void __rust_thunk___ZN6crubit4test23DerivedFromPrivateEmptyC1Ev(
    class crubit::test::DerivedFromPrivateEmpty* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(struct crubit::test::NoUniqueAddressThreadAffineTag) == 4);
static_assert(alignof(struct crubit::test::NoUniqueAddressThreadAffineTag) ==
              4);
static_assert(
    CRUBIT_OFFSET_OF(tag,
                     struct crubit::test::NoUniqueAddressThreadAffineTag) == 0);
static_assert(CRUBIT_OFFSET_OF(
                  x, struct crubit::test::NoUniqueAddressThreadAffineTag) == 0);

extern "C" void
__rust_thunk___ZN6crubit4test30NoUniqueAddressThreadAffineTagC1Ev(
    struct crubit::test::NoUniqueAddressThreadAffineTag* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct crubit::test::WithZeroLengthPointerArray) ==
              8);
static_assert(alignof(struct crubit::test::WithZeroLengthPointerArray) == 8);
static_assert(
    CRUBIT_OFFSET_OF(x, struct crubit::test::WithZeroLengthPointerArray) == 0);
static_assert(CRUBIT_OFFSET_OF(
                  items, struct crubit::test::WithZeroLengthPointerArray) == 8);

extern "C" void __rust_thunk___ZN6crubit4test26WithZeroLengthPointerArrayC1Ev(
    struct crubit::test::WithZeroLengthPointerArray* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct crubit::test::MemberThreadAffineTag) == 8);
static_assert(alignof(struct crubit::test::MemberThreadAffineTag) == 4);
static_assert(CRUBIT_OFFSET_OF(tag,
                               struct crubit::test::MemberThreadAffineTag) ==
              0);
static_assert(CRUBIT_OFFSET_OF(x, struct crubit::test::MemberThreadAffineTag) ==
              4);

extern "C" void __rust_thunk___ZN6crubit4test21MemberThreadAffineTagC1Ev(
    struct crubit::test::MemberThreadAffineTag* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct crubit::test::ThreadAffineTemplate<int>) ==
              4);
static_assert(alignof(struct crubit::test::ThreadAffineTemplate<int>) == 4);
static_assert(
    CRUBIT_OFFSET_OF(x, struct crubit::test::ThreadAffineTemplate<int>) == 0);

extern "C" void
__rust_thunk__b813127c__ZN6crubit4test20ThreadAffineTemplateIiEC1Ev(
    struct crubit::test::ThreadAffineTemplate<int>* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
