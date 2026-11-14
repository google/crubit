// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:thread_safe

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/annotations/thread_safe.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"

static_assert(CRUBIT_SIZEOF(class crubit::test::ThreadSafeStruct) == 4);
static_assert(alignof(class crubit::test::ThreadSafeStruct) == 4);

extern "C" void __rust_thunk___ZN6crubit4test16ThreadSafeStructC1Ev(
    class crubit::test::ThreadSafeStruct* __this) {
  crubit::construct_at(__this);
}

extern "C" int __rust_thunk___ZNK6crubit4test16ThreadSafeStruct8ConstGetEv(
    class crubit::test::ThreadSafeStruct const* __this) {
  return __this->ConstGet();
}

static_assert((int (::crubit::test::ThreadSafeStruct::*)() const) &
              ::crubit::test::ThreadSafeStruct::ConstGet);

extern "C" int __rust_thunk___ZN6crubit4test16ThreadSafeStruct11NonConstGetEv(
    class crubit::test::ThreadSafeStruct* __this) {
  return __this->NonConstGet();
}

static_assert((int (::crubit::test::ThreadSafeStruct::*)()) &
              ::crubit::test::ThreadSafeStruct::NonConstGet);

static_assert(CRUBIT_SIZEOF(class crubit::test::RegularStruct) == 4);
static_assert(alignof(class crubit::test::RegularStruct) == 4);

extern "C" void __rust_thunk___ZN6crubit4test13RegularStructC1Ev(
    class crubit::test::RegularStruct* __this) {
  crubit::construct_at(__this);
}

extern "C" int __rust_thunk___ZNK6crubit4test13RegularStruct8ConstGetEv(
    class crubit::test::RegularStruct const* __this) {
  return __this->ConstGet();
}

static_assert((int (::crubit::test::RegularStruct::*)() const) &
              ::crubit::test::RegularStruct::ConstGet);

extern "C" int __rust_thunk___ZN6crubit4test13RegularStruct11NonConstGetEv(
    class crubit::test::RegularStruct* __this) {
  return __this->NonConstGet();
}

static_assert((int (::crubit::test::RegularStruct::*)()) &
              ::crubit::test::RegularStruct::NonConstGet);

static_assert(sizeof(struct crubit::test::ThreadSafeUnpin) == 1);
static_assert(alignof(struct crubit::test::ThreadSafeUnpin) == 1);

extern "C" void __rust_thunk___ZN6crubit4test15ThreadSafeUnpinC1Ev(
    struct crubit::test::ThreadSafeUnpin* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN6crubit4test15ThreadSafeUnpinC1ERKS1_(
    struct crubit::test::ThreadSafeUnpin* __this,
    struct crubit::test::ThreadSafeUnpin const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" void __rust_thunk___ZN6crubit4test15ThreadSafeUnpinC1EOS1_(
    struct crubit::test::ThreadSafeUnpin* __this,
    struct crubit::test::ThreadSafeUnpin* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct crubit::test::ThreadSafeUnpin*
__rust_thunk___ZN6crubit4test15ThreadSafeUnpinaSERKS1_(
    struct crubit::test::ThreadSafeUnpin* __this,
    struct crubit::test::ThreadSafeUnpin const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct crubit::test::ThreadSafeUnpin &
               (::crubit::test::ThreadSafeUnpin::*)(
                   struct crubit::test::ThreadSafeUnpin const&)) &
              ::crubit::test::ThreadSafeUnpin::operator=);

extern "C" struct crubit::test::ThreadSafeUnpin*
__rust_thunk___ZN6crubit4test15ThreadSafeUnpinaSEOS1_(
    struct crubit::test::ThreadSafeUnpin* __this,
    struct crubit::test::ThreadSafeUnpin* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct crubit::test::ThreadSafeUnpin &
               (::crubit::test::ThreadSafeUnpin::*)(
                   struct crubit::test::ThreadSafeUnpin&&)) &
              ::crubit::test::ThreadSafeUnpin::operator=);

static_assert(sizeof(struct crubit::test::ThreadSafePinned) == 1);
static_assert(alignof(struct crubit::test::ThreadSafePinned) == 1);

extern "C" void __rust_thunk___ZN6crubit4test16ThreadSafePinnedC1Ev(
    struct crubit::test::ThreadSafePinned* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN6crubit4test16ThreadSafePinnedC1ERKS1_(
    struct crubit::test::ThreadSafePinned* __this,
    struct crubit::test::ThreadSafePinned const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" void __rust_thunk___ZN6crubit4test16ThreadSafePinnedC1EOS1_(
    struct crubit::test::ThreadSafePinned* __this,
    struct crubit::test::ThreadSafePinned* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct crubit::test::ThreadSafePinned*
__rust_thunk___ZN6crubit4test16ThreadSafePinnedaSERKS1_(
    struct crubit::test::ThreadSafePinned* __this,
    struct crubit::test::ThreadSafePinned const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct crubit::test::ThreadSafePinned &
               (::crubit::test::ThreadSafePinned::*)(
                   struct crubit::test::ThreadSafePinned const&)) &
              ::crubit::test::ThreadSafePinned::operator=);

extern "C" struct crubit::test::ThreadSafePinned*
__rust_thunk___ZN6crubit4test16ThreadSafePinnedaSEOS1_(
    struct crubit::test::ThreadSafePinned* __this,
    struct crubit::test::ThreadSafePinned* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct crubit::test::ThreadSafePinned &
               (::crubit::test::ThreadSafePinned::*)(
                   struct crubit::test::ThreadSafePinned&&)) &
              ::crubit::test::ThreadSafePinned::operator=);

extern "C" void __rust_thunk___ZN6crubit4test16ThreadSafePinnedD1Ev(
    struct crubit::test::ThreadSafePinned* __this) {
  std::destroy_at(__this);
}

static_assert(CRUBIT_SIZEOF(class crubit::test::ThreadSafeInterface) == 8);
static_assert(alignof(class crubit::test::ThreadSafeInterface) == 8);

extern "C" class crubit::test::ThreadSafeInterface*
__rust_thunk___ZN6crubit4test19ThreadSafeInterfaceaSERKS1_(
    class crubit::test::ThreadSafeInterface* __this,
    class crubit::test::ThreadSafeInterface const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((class crubit::test::ThreadSafeInterface &
               (::crubit::test::ThreadSafeInterface::*)(
                   class crubit::test::ThreadSafeInterface const&)) &
              ::crubit::test::ThreadSafeInterface::operator=);

extern "C" void __rust_thunk___ZN6crubit4test19ThreadSafeInterfaceD1Ev(
    class crubit::test::ThreadSafeInterface* __this) {
  std::destroy_at(__this);
}

extern "C" int
__rust_thunk___ZN6crubit4test19ThreadSafeInterface15IncrementAndGetEv(
    class crubit::test::ThreadSafeInterface* __this) {
  return __this->IncrementAndGet();
}

static_assert((int (::crubit::test::ThreadSafeInterface::*)()) &
              ::crubit::test::ThreadSafeInterface::IncrementAndGet);

extern "C" int __rust_thunk___ZNK6crubit4test19ThreadSafeInterface3GetEv(
    class crubit::test::ThreadSafeInterface const* __this) {
  return __this->Get();
}

static_assert((int (::crubit::test::ThreadSafeInterface::*)() const) &
              ::crubit::test::ThreadSafeInterface::Get);

extern "C" void
__crubit_operator_delete__N6crubit4test19ThreadSafeInterfaceE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fannotations_3athread_5fsafe(
    class crubit::test::ThreadSafeInterface* ptr) {
  delete ptr;
}

static_assert(CRUBIT_SIZEOF(class crubit::test::ThreadSafeInterfaceImpl) == 16);
static_assert(alignof(class crubit::test::ThreadSafeInterfaceImpl) == 8);

extern "C" void __rust_thunk___ZN6crubit4test23ThreadSafeInterfaceImplC1Ev(
    class crubit::test::ThreadSafeInterfaceImpl* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN6crubit4test23ThreadSafeInterfaceImplD1Ev(
    class crubit::test::ThreadSafeInterfaceImpl* __this) {
  std::destroy_at(__this);
}

extern "C" int
__rust_thunk___ZN6crubit4test23ThreadSafeInterfaceImpl15IncrementAndGetEv(
    class crubit::test::ThreadSafeInterfaceImpl* __this) {
  return __this->IncrementAndGet();
}

static_assert((int (::crubit::test::ThreadSafeInterfaceImpl::*)()) &
              ::crubit::test::ThreadSafeInterfaceImpl::IncrementAndGet);

extern "C" int __rust_thunk___ZNK6crubit4test23ThreadSafeInterfaceImpl3GetEv(
    class crubit::test::ThreadSafeInterfaceImpl const* __this) {
  return __this->Get();
}

static_assert((int (::crubit::test::ThreadSafeInterfaceImpl::*)() const) &
              ::crubit::test::ThreadSafeInterfaceImpl::Get);

extern "C" void __rust_thunk___ZN6crubit4test29MakeThreadSafeInterfaceSharedEv(
    class std::shared_ptr<class crubit::test::ThreadSafeInterface>* __return) {
  new (__return) auto(crubit::test::MakeThreadSafeInterfaceShared());
}

static_assert(
    (class std::shared_ptr<class crubit::test::ThreadSafeInterface> (*)()) &
    ::crubit::test::MakeThreadSafeInterfaceShared);

extern "C" void __rust_thunk___ZN6crubit4test29MakeThreadSafeInterfaceUniqueEv(
    class std::unique_ptr<
        class crubit::test::ThreadSafeInterface,
        struct std::default_delete<class crubit::test::ThreadSafeInterface>>*
        __return) {
  new (__return) auto(crubit::test::MakeThreadSafeInterfaceUnique());
}

static_assert((class std::unique_ptr<
                  class crubit::test::ThreadSafeInterface,
                  struct std::default_delete<
                      class crubit::test::ThreadSafeInterface>> (*)()) &
              ::crubit::test::MakeThreadSafeInterfaceUnique);

static_assert(
    sizeof(
        struct std::default_delete<class crubit::test::ThreadSafeInterface>) ==
    1);
static_assert(
    alignof(
        struct std::default_delete<class crubit::test::ThreadSafeInterface>) ==
    1);

extern "C" void
__rust_thunk__2d1083d5__ZNSt3__u14default_deleteIN6crubit4test19ThreadSafeInterfaceEEC1Ev(
    struct std::default_delete<class crubit::test::ThreadSafeInterface>*
        __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::weak_ptr<class crubit::test::ThreadSafeInterface>) == 16);
static_assert(
    alignof(class std::weak_ptr<class crubit::test::ThreadSafeInterface>) == 8);

extern "C" void
__rust_thunk__a22320d3__ZNSt3__u8weak_ptrIN6crubit4test19ThreadSafeInterfaceEEC1Ev(
    class std::weak_ptr<class crubit::test::ThreadSafeInterface>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__dc4e1abb__ZNSt3__u8weak_ptrIN6crubit4test19ThreadSafeInterfaceEEC1ERKS4_(
    class std::weak_ptr<class crubit::test::ThreadSafeInterface>* __this,
    class std::weak_ptr<class crubit::test::ThreadSafeInterface> const* __r) {
  crubit::construct_at(__this, *__r);
}

extern "C" void
__rust_thunk__e8a99535__ZNSt3__u8weak_ptrIN6crubit4test19ThreadSafeInterfaceEEC1EOS4_(
    class std::weak_ptr<class crubit::test::ThreadSafeInterface>* __this,
    class std::weak_ptr<class crubit::test::ThreadSafeInterface>* __r) {
  crubit::construct_at(__this, std::move(*__r));
}

extern "C" void
__rust_thunk__155cad2c__ZNSt3__u8weak_ptrIN6crubit4test19ThreadSafeInterfaceEED1Ev(
    class std::weak_ptr<class crubit::test::ThreadSafeInterface>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::weak_ptr<class crubit::test::ThreadSafeInterface>*
__rust_thunk__8f8902d1__ZNSt3__u8weak_ptrIN6crubit4test19ThreadSafeInterfaceEEaSERKS4_(
    class std::weak_ptr<class crubit::test::ThreadSafeInterface>* __this,
    class std::weak_ptr<class crubit::test::ThreadSafeInterface> const* __r) {
  return std::addressof(__this->operator=(*__r));
}

static_assert(
    (class std::weak_ptr<class crubit::test::ThreadSafeInterface> &
     (::std::weak_ptr<class crubit::test::ThreadSafeInterface>::*)(
         class std::weak_ptr<class crubit::test::ThreadSafeInterface> const&)) &
    ::std::weak_ptr<class crubit::test::ThreadSafeInterface>::operator=);

extern "C" class std::weak_ptr<class crubit::test::ThreadSafeInterface>*
__rust_thunk__0fb39bf8__ZNSt3__u8weak_ptrIN6crubit4test19ThreadSafeInterfaceEEaSEOS4_(
    class std::weak_ptr<class crubit::test::ThreadSafeInterface>* __this,
    class std::weak_ptr<class crubit::test::ThreadSafeInterface>* __r) {
  return std::addressof(__this->operator=(std::move(*__r)));
}

static_assert(
    (class std::weak_ptr<class crubit::test::ThreadSafeInterface> &
     (::std::weak_ptr<class crubit::test::ThreadSafeInterface>::*)(
         class std::weak_ptr<class crubit::test::ThreadSafeInterface>&&)) &
    ::std::weak_ptr<class crubit::test::ThreadSafeInterface>::operator=);

extern "C" void
__rust_thunk__aa8052cf__ZNSt3__u8weak_ptrIN6crubit4test19ThreadSafeInterfaceEE4swapERS4_(
    class std::weak_ptr<class crubit::test::ThreadSafeInterface>* __this,
    class std::weak_ptr<class crubit::test::ThreadSafeInterface>* __r) {
  __this->swap(*__r);
}

static_assert(
    (void (::std::weak_ptr<class crubit::test::ThreadSafeInterface>::*)(
        class std::weak_ptr<class crubit::test::ThreadSafeInterface>&)) &
    ::std::weak_ptr<class crubit::test::ThreadSafeInterface>::swap);

extern "C" void
__rust_thunk__65d8e410__ZNSt3__u8weak_ptrIN6crubit4test19ThreadSafeInterfaceEE5resetEv(
    class std::weak_ptr<class crubit::test::ThreadSafeInterface>* __this) {
  __this->reset();
}

static_assert(
    (void (::std::weak_ptr<class crubit::test::ThreadSafeInterface>::*)()) &
    ::std::weak_ptr<class crubit::test::ThreadSafeInterface>::reset);

extern "C" long
__rust_thunk__f0c2a540__ZNKSt3__u8weak_ptrIN6crubit4test19ThreadSafeInterfaceEE9use_countEv(
    class std::weak_ptr<class crubit::test::ThreadSafeInterface> const*
        __this) {
  return __this->use_count();
}

static_assert(
    (long (::std::weak_ptr<class crubit::test::ThreadSafeInterface>::*)()
         const) &
    ::std::weak_ptr<class crubit::test::ThreadSafeInterface>::use_count);

extern "C" bool
__rust_thunk__3aedcb03__ZNKSt3__u8weak_ptrIN6crubit4test19ThreadSafeInterfaceEE7expiredEv(
    class std::weak_ptr<class crubit::test::ThreadSafeInterface> const*
        __this) {
  return __this->expired();
}

static_assert(
    (bool (::std::weak_ptr<class crubit::test::ThreadSafeInterface>::*)()
         const) &
    ::std::weak_ptr<class crubit::test::ThreadSafeInterface>::expired);

extern "C" void
__rust_thunk__716bacab__ZNKSt3__u8weak_ptrIN6crubit4test19ThreadSafeInterfaceEE4lockEv(
    class std::shared_ptr<class crubit::test::ThreadSafeInterface>* __return,
    class std::weak_ptr<class crubit::test::ThreadSafeInterface> const*
        __this) {
  new (__return) auto(__this->lock());
}

static_assert((class std::shared_ptr<class crubit::test::ThreadSafeInterface> (
                  ::std::weak_ptr<class crubit::test::ThreadSafeInterface>::*)()
                   const) &
              ::std::weak_ptr<class crubit::test::ThreadSafeInterface>::lock);

#pragma clang diagnostic pop
