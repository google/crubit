// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/struct/destructors:destructors

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/struct/destructors/destructors.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"

static_assert(CRUBIT_SIZEOF(class DestructionOrderRecorder) == 24);
static_assert(alignof(class DestructionOrderRecorder) == 8);

extern "C" void
__rust_thunk___ZN24DestructionOrderRecorderC1EiNSt3__u10shared_ptrIiEE(
    class DestructionOrderRecorder* __this, int int_field,
    class std::shared_ptr<int>* record) {
  crubit::construct_at(__this, int_field,
                       crubit::UnsafeTakeValueOnConversion(record));
}

extern "C" void __rust_thunk___ZN24DestructionOrderRecorderC1EOS_(
    class DestructionOrderRecorder* __this,
    class DestructionOrderRecorder* other) {
  crubit::construct_at(__this, std::move(*other));
}

extern "C" class DestructionOrderRecorder*
__rust_thunk___ZN24DestructionOrderRecorderaSEOS_(
    class DestructionOrderRecorder* __this,
    class DestructionOrderRecorder* other) {
  return std::addressof(__this->operator=(std::move(*other)));
}

static_assert(
    (class DestructionOrderRecorder &
     (::DestructionOrderRecorder::*)(class DestructionOrderRecorder&&)) &
    ::DestructionOrderRecorder::operator=);

extern "C" void __rust_thunk___ZN24DestructionOrderRecorderD1Ev(
    class DestructionOrderRecorder* __this) {
  std::destroy_at(__this);
}

static_assert(CRUBIT_SIZEOF(class FieldDestructionOrderTester) == 72);
static_assert(alignof(class FieldDestructionOrderTester) == 8);

extern "C" void __rust_thunk___ZN27FieldDestructionOrderTesterC1EOS_(
    class FieldDestructionOrderTester* __this,
    class FieldDestructionOrderTester* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" void __rust_thunk___ZN27FieldDestructionOrderTesterD1Ev(
    class FieldDestructionOrderTester* __this) {
  std::destroy_at(__this);
}

extern "C" class FieldDestructionOrderTester*
__rust_thunk___ZN27FieldDestructionOrderTesteraSEOS_(
    class FieldDestructionOrderTester* __this,
    class FieldDestructionOrderTester* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert(
    (class FieldDestructionOrderTester &
     (::FieldDestructionOrderTester::*)(class FieldDestructionOrderTester&&)) &
    ::FieldDestructionOrderTester::operator=);

extern "C" void
__rust_thunk___ZN27FieldDestructionOrderTesterC1E24DestructionOrderRecorderS0_S0_(
    class FieldDestructionOrderTester* __this,
    class DestructionOrderRecorder* field1,
    class DestructionOrderRecorder* field2,
    class DestructionOrderRecorder* field3) {
  crubit::construct_at(__this, crubit::UnsafeTakeValueOnConversion(field1),
                       crubit::UnsafeTakeValueOnConversion(field2),
                       crubit::UnsafeTakeValueOnConversion(field3));
}

extern "C" void
__rust_thunk___ZN27FieldDestructionOrderTester15DestructFromCppEiiiNSt3__u10shared_ptrIiEE(
    int field1, int field2, int field3, class std::shared_ptr<int>* record) {
  FieldDestructionOrderTester::DestructFromCpp(field1, field2, field3,
                                               crubit::UnsafeTakeValue(record));
}

static_assert((void (*)(int, int, int, class std::shared_ptr<int>)) &
              ::FieldDestructionOrderTester::DestructFromCpp);

static_assert(CRUBIT_SIZEOF(class PrivateOperatorDelete) == 16);
static_assert(alignof(class PrivateOperatorDelete) == 8);

extern "C" void __rust_thunk___ZN21PrivateOperatorDeleteC1ERKS_(
    class PrivateOperatorDelete* __this,
    class PrivateOperatorDelete const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" void __rust_thunk___ZN21PrivateOperatorDeleteC1EOS_(
    class PrivateOperatorDelete* __this,
    class PrivateOperatorDelete* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" void __rust_thunk___ZN21PrivateOperatorDeleteD1Ev(
    class PrivateOperatorDelete* __this) {
  std::destroy_at(__this);
}

extern "C" class PrivateOperatorDelete*
__rust_thunk___ZN21PrivateOperatorDeleteaSERKS_(
    class PrivateOperatorDelete* __this,
    class PrivateOperatorDelete const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert(
    (class PrivateOperatorDelete &
     (::PrivateOperatorDelete::*)(class PrivateOperatorDelete const&)) &
    ::PrivateOperatorDelete::operator=);

extern "C" class PrivateOperatorDelete*
__rust_thunk___ZN21PrivateOperatorDeleteaSEOS_(
    class PrivateOperatorDelete* __this,
    class PrivateOperatorDelete* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((class PrivateOperatorDelete &
               (::PrivateOperatorDelete::*)(class PrivateOperatorDelete&&)) &
              ::PrivateOperatorDelete::operator=);

extern "C" void
__rust_thunk___ZN21PrivateOperatorDeleteC1ENSt3__u10shared_ptrIiEE(
    class PrivateOperatorDelete* __this, class std::shared_ptr<int>* token) {
  crubit::construct_at(__this, crubit::UnsafeTakeValueOnConversion(token));
}

static_assert(CRUBIT_SIZEOF(class std::weak_ptr<int>) == 16);
static_assert(alignof(class std::weak_ptr<int>) == 8);

extern "C" void __rust_thunk__aee881e6__ZNSt3__u8weak_ptrIiEC1Ev(
    class std::weak_ptr<int>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__603d2e7b__ZNSt3__u8weak_ptrIiEC1ERKS1_(
    class std::weak_ptr<int>* __this, class std::weak_ptr<int> const* __r) {
  crubit::construct_at(__this, *__r);
}

extern "C" void __rust_thunk__c4d4a469__ZNSt3__u8weak_ptrIiEC1EOS1_(
    class std::weak_ptr<int>* __this, class std::weak_ptr<int>* __r) {
  crubit::construct_at(__this, std::move(*__r));
}

extern "C" void __rust_thunk__0fa7c98c__ZNSt3__u8weak_ptrIiED1Ev(
    class std::weak_ptr<int>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::weak_ptr<int>*
__rust_thunk__3056dfe4__ZNSt3__u8weak_ptrIiEaSERKS1_(
    class std::weak_ptr<int>* __this, class std::weak_ptr<int> const* __r) {
  return std::addressof(__this->operator=(*__r));
}

static_assert((class std::weak_ptr<int> &
               (::std::weak_ptr<int>::*)(class std::weak_ptr<int> const&)) &
              ::std::weak_ptr<int>::operator=);

extern "C" class std::weak_ptr<int>*
__rust_thunk__95fed9fb__ZNSt3__u8weak_ptrIiEaSEOS1_(
    class std::weak_ptr<int>* __this, class std::weak_ptr<int>* __r) {
  return std::addressof(__this->operator=(std::move(*__r)));
}

static_assert((class std::weak_ptr<int> &
               (::std::weak_ptr<int>::*)(class std::weak_ptr<int>&&)) &
              ::std::weak_ptr<int>::operator=);

extern "C" void __rust_thunk__362c7055__ZNSt3__u8weak_ptrIiE4swapERS1_(
    class std::weak_ptr<int>* __this, class std::weak_ptr<int>* __r) {
  __this->swap(*__r);
}

static_assert((void (::std::weak_ptr<int>::*)(class std::weak_ptr<int>&)) &
              ::std::weak_ptr<int>::swap);

extern "C" void __rust_thunk__d20e9826__ZNSt3__u8weak_ptrIiE5resetEv(
    class std::weak_ptr<int>* __this) {
  __this->reset();
}

static_assert((void (::std::weak_ptr<int>::*)()) & ::std::weak_ptr<int>::reset);

extern "C" long __rust_thunk__12ba3f91__ZNKSt3__u8weak_ptrIiE9use_countEv(
    class std::weak_ptr<int> const* __this) {
  return __this->use_count();
}

static_assert((long (::std::weak_ptr<int>::*)() const) &
              ::std::weak_ptr<int>::use_count);

extern "C" bool __rust_thunk__e00487c3__ZNKSt3__u8weak_ptrIiE7expiredEv(
    class std::weak_ptr<int> const* __this) {
  return __this->expired();
}

static_assert((bool (::std::weak_ptr<int>::*)() const) &
              ::std::weak_ptr<int>::expired);

extern "C" void __rust_thunk__710c15b7__ZNKSt3__u8weak_ptrIiE4lockEv(
    class std::shared_ptr<int>* __return,
    class std::weak_ptr<int> const* __this) {
  new (__return) auto(__this->lock());
}

static_assert((class std::shared_ptr<int> (::std::weak_ptr<int>::*)() const) &
              ::std::weak_ptr<int>::lock);

#pragma clang diagnostic pop
