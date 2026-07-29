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

extern "C" struct crubit::test::ThreadSafeUnpin*
__rust_thunk___ZN6crubit4test15ThreadSafeUnpinaSEOS1_(
    struct crubit::test::ThreadSafeUnpin* __this,
    struct crubit::test::ThreadSafeUnpin* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

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

extern "C" struct crubit::test::ThreadSafePinned*
__rust_thunk___ZN6crubit4test16ThreadSafePinnedaSEOS1_(
    struct crubit::test::ThreadSafePinned* __this,
    struct crubit::test::ThreadSafePinned* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

extern "C" void __rust_thunk___ZN6crubit4test16ThreadSafePinnedD1Ev(
    struct crubit::test::ThreadSafePinned* __this) {
  std::destroy_at(__this);
}

#pragma clang diagnostic pop
