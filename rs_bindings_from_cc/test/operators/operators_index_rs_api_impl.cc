// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/operators:operators_index
// Features: fmt, leading_colons_for_cpp_type, supported, types

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/operators/operators_index.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct ::crubit::test::ItemUnpin) == 4);
static_assert(alignof(struct ::crubit::test::ItemUnpin) == 4);
static_assert(CRUBIT_OFFSET_OF(value, struct ::crubit::test::ItemUnpin) == 0);

extern "C" void __rust_thunk___ZN6crubit4test9ItemUnpinC1Ev(
    struct ::crubit::test::ItemUnpin* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct ::crubit::test::ItemNonUnpin) == 4);
static_assert(alignof(struct ::crubit::test::ItemNonUnpin) == 4);
static_assert(CRUBIT_OFFSET_OF(value, struct ::crubit::test::ItemNonUnpin) ==
              0);

extern "C" void __rust_thunk___ZN6crubit4test12ItemNonUnpinC1Ev(
    struct ::crubit::test::ItemNonUnpin* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN6crubit4test12ItemNonUnpinC1ERKS1_(
    struct ::crubit::test::ItemNonUnpin* __this,
    struct ::crubit::test::ItemNonUnpin const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" struct ::crubit::test::ItemNonUnpin*
__rust_thunk___ZN6crubit4test12ItemNonUnpinaSERKS1_(
    struct ::crubit::test::ItemNonUnpin* __this,
    struct ::crubit::test::ItemNonUnpin const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

extern "C" void __rust_thunk___ZN6crubit4test12ItemNonUnpinD1Ev(
    struct ::crubit::test::ItemNonUnpin* __this) {
  std::destroy_at(__this);
}

static_assert(CRUBIT_SIZEOF(class ::crubit::test::ContainerUnpinItemUnpin) ==
              40);
static_assert(alignof(class ::crubit::test::ContainerUnpinItemUnpin) == 4);

extern "C" void __rust_thunk___ZN6crubit4test23ContainerUnpinItemUnpinC1Ev(
    class ::crubit::test::ContainerUnpinItemUnpin* __this) {
  crubit::construct_at(__this);
}

extern "C" struct ::crubit::test::ItemUnpin const*
__rust_thunk___ZNK6crubit4test23ContainerUnpinItemUnpinixEj(
    class ::crubit::test::ContainerUnpinItemUnpin const* __this,
    unsigned int index) {
  return std::addressof(__this->operator[](index));
}

extern "C" struct ::crubit::test::ItemUnpin*
__rust_thunk___ZN6crubit4test23ContainerUnpinItemUnpinixEj(
    class ::crubit::test::ContainerUnpinItemUnpin* __this, unsigned int index) {
  return std::addressof(__this->operator[](index));
}

static_assert(CRUBIT_SIZEOF(class ::crubit::test::ContainerUnpinItemNonUnpin) ==
              48);
static_assert(alignof(class ::crubit::test::ContainerUnpinItemNonUnpin) == 8);
static_assert(
    CRUBIT_OFFSET_OF(items_storage_,
                     class ::crubit::test::ContainerUnpinItemNonUnpin) == 0);
static_assert(
    CRUBIT_OFFSET_OF(items_,
                     class ::crubit::test::ContainerUnpinItemNonUnpin) == 40);

extern "C" void
__rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinC1ERKS1_(
    class ::crubit::test::ContainerUnpinItemNonUnpin* __this,
    class ::crubit::test::ContainerUnpinItemNonUnpin const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" void
__rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinC1EOS1_(
    class ::crubit::test::ContainerUnpinItemNonUnpin* __this,
    class ::crubit::test::ContainerUnpinItemNonUnpin* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" void __rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinD1Ev(
    class ::crubit::test::ContainerUnpinItemNonUnpin* __this) {
  std::destroy_at(__this);
}

extern "C" class ::crubit::test::ContainerUnpinItemNonUnpin*
__rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinaSERKS1_(
    class ::crubit::test::ContainerUnpinItemNonUnpin* __this,
    class ::crubit::test::ContainerUnpinItemNonUnpin const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

extern "C" class ::crubit::test::ContainerUnpinItemNonUnpin*
__rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinaSEOS1_(
    class ::crubit::test::ContainerUnpinItemNonUnpin* __this,
    class ::crubit::test::ContainerUnpinItemNonUnpin* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

extern "C" void __rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinC1Ev(
    class ::crubit::test::ContainerUnpinItemNonUnpin* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinC1EPNS0_12ItemNonUnpinE(
    class crubit::test::ContainerUnpinItemNonUnpin* __this,
    struct crubit::test::ItemNonUnpin* items) {
  crubit::construct_at(__this, items);
}

extern "C" struct crubit::test::ItemNonUnpin const*
__rust_thunk___ZNK6crubit4test26ContainerUnpinItemNonUnpinixEj(
    class ::crubit::test::ContainerUnpinItemNonUnpin const* __this,
    unsigned int index) {
  return std::addressof(__this->operator[](index));
}

extern "C" struct ::crubit::test::ItemNonUnpin*
__rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinixEj(
    class ::crubit::test::ContainerUnpinItemNonUnpin* __this,
    unsigned int index) {
  return std::addressof(__this->operator[](index));
}

static_assert(CRUBIT_SIZEOF(class ::crubit::test::ContainerNonUnpinItemUnpin) ==
              40);
static_assert(alignof(class ::crubit::test::ContainerNonUnpinItemUnpin) == 4);

extern "C" void
__rust_thunk___ZN6crubit4test26ContainerNonUnpinItemUnpinC1ERKS1_(
    class ::crubit::test::ContainerNonUnpinItemUnpin* __this,
    class ::crubit::test::ContainerNonUnpinItemUnpin const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" class ::crubit::test::ContainerNonUnpinItemUnpin*
__rust_thunk___ZN6crubit4test26ContainerNonUnpinItemUnpinaSERKS1_(
    class ::crubit::test::ContainerNonUnpinItemUnpin* __this,
    class ::crubit::test::ContainerNonUnpinItemUnpin const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

extern "C" void __rust_thunk___ZN6crubit4test26ContainerNonUnpinItemUnpinC1Ev(
    class ::crubit::test::ContainerNonUnpinItemUnpin* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN6crubit4test26ContainerNonUnpinItemUnpinD1Ev(
    class ::crubit::test::ContainerNonUnpinItemUnpin* __this) {
  std::destroy_at(__this);
}

extern "C" struct ::crubit::test::ItemUnpin const*
__rust_thunk___ZNK6crubit4test26ContainerNonUnpinItemUnpinixEj(
    class ::crubit::test::ContainerNonUnpinItemUnpin const* __this,
    unsigned int index) {
  return std::addressof(__this->operator[](index));
}

extern "C" struct ::crubit::test::ItemUnpin*
__rust_thunk___ZN6crubit4test26ContainerNonUnpinItemUnpinixEj(
    class ::crubit::test::ContainerNonUnpinItemUnpin* __this,
    unsigned int index) {
  return std::addressof(__this->operator[](index));
}

static_assert(
    CRUBIT_SIZEOF(class ::crubit::test::ContainerNonUnpinItemNonUnpin) == 40);
static_assert(alignof(class ::crubit::test::ContainerNonUnpinItemNonUnpin) ==
              4);

extern "C" void
__rust_thunk___ZN6crubit4test29ContainerNonUnpinItemNonUnpinC1ERKS1_(
    class ::crubit::test::ContainerNonUnpinItemNonUnpin* __this,
    class ::crubit::test::ContainerNonUnpinItemNonUnpin const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" class ::crubit::test::ContainerNonUnpinItemNonUnpin*
__rust_thunk___ZN6crubit4test29ContainerNonUnpinItemNonUnpinaSERKS1_(
    class ::crubit::test::ContainerNonUnpinItemNonUnpin* __this,
    class ::crubit::test::ContainerNonUnpinItemNonUnpin const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

extern "C" void
__rust_thunk___ZN6crubit4test29ContainerNonUnpinItemNonUnpinC1Ev(
    class ::crubit::test::ContainerNonUnpinItemNonUnpin* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN6crubit4test29ContainerNonUnpinItemNonUnpinD1Ev(
    class ::crubit::test::ContainerNonUnpinItemNonUnpin* __this) {
  std::destroy_at(__this);
}

extern "C" struct ::crubit::test::ItemNonUnpin const*
__rust_thunk___ZNK6crubit4test29ContainerNonUnpinItemNonUnpinixEj(
    class ::crubit::test::ContainerNonUnpinItemNonUnpin const* __this,
    unsigned int index) {
  return std::addressof(__this->operator[](index));
}

extern "C" struct ::crubit::test::ItemNonUnpin*
__rust_thunk___ZN6crubit4test29ContainerNonUnpinItemNonUnpinixEj(
    class ::crubit::test::ContainerNonUnpinItemNonUnpin* __this,
    unsigned int index) {
  return std::addressof(__this->operator[](index));
}

static_assert(CRUBIT_SIZEOF(struct ::crubit::test::ContainerValue) == 4);
static_assert(alignof(struct ::crubit::test::ContainerValue) == 4);
static_assert(CRUBIT_OFFSET_OF(value, struct ::crubit::test::ContainerValue) ==
              0);

extern "C" void __rust_thunk___ZN6crubit4test14ContainerValueC1Ev(
    struct ::crubit::test::ContainerValue* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct ::crubit::test::ContainerRvalue) == 4);
static_assert(alignof(struct ::crubit::test::ContainerRvalue) == 4);
static_assert(CRUBIT_OFFSET_OF(value, struct ::crubit::test::ContainerRvalue) ==
              0);

extern "C" void __rust_thunk___ZN6crubit4test15ContainerRvalueC1Ev(
    struct ::crubit::test::ContainerRvalue* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct ::crubit::test::ContainerMutRefFromConst) ==
              4);
static_assert(alignof(struct ::crubit::test::ContainerMutRefFromConst) == 4);
static_assert(CRUBIT_OFFSET_OF(
                  value, struct ::crubit::test::ContainerMutRefFromConst) == 0);

extern "C" void __rust_thunk___ZN6crubit4test24ContainerMutRefFromConstC1Ev(
    struct ::crubit::test::ContainerMutRefFromConst* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct ::crubit::test::ContainerConstRefFromMut) ==
              4);
static_assert(alignof(struct ::crubit::test::ContainerConstRefFromMut) == 4);
static_assert(CRUBIT_OFFSET_OF(
                  value, struct ::crubit::test::ContainerConstRefFromMut) == 0);

extern "C" void __rust_thunk___ZN6crubit4test24ContainerConstRefFromMutC1Ev(
    struct ::crubit::test::ContainerConstRefFromMut* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
