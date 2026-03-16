// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:memcpy_movable
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/annotations/memcpy_movable.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(class crubit::test::MemcpyMovableClass) == 4);
static_assert(alignof(class crubit::test::MemcpyMovableClass) == 4);

extern "C" void __rust_thunk___ZN6crubit4test18MemcpyMovableClassC1Ev(
    class crubit::test::MemcpyMovableClass* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN6crubit4test18MemcpyMovableClassC1ERKS1_(
    class crubit::test::MemcpyMovableClass* __this,
    class crubit::test::MemcpyMovableClass const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" void __rust_thunk___ZN6crubit4test18MemcpyMovableClassC1EOS1_(
    class crubit::test::MemcpyMovableClass* __this,
    class crubit::test::MemcpyMovableClass* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class crubit::test::MemcpyMovableClass*
__rust_thunk___ZN6crubit4test18MemcpyMovableClassaSERKS1_(
    class crubit::test::MemcpyMovableClass* __this,
    class crubit::test::MemcpyMovableClass const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

extern "C" class crubit::test::MemcpyMovableClass*
__rust_thunk___ZN6crubit4test18MemcpyMovableClassaSEOS1_(
    class crubit::test::MemcpyMovableClass* __this,
    class crubit::test::MemcpyMovableClass* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert(CRUBIT_SIZEOF(class crubit::test::NonMemcpyMovableClass) == 4);
static_assert(alignof(class crubit::test::NonMemcpyMovableClass) == 4);

extern "C" void __rust_thunk___ZN6crubit4test21NonMemcpyMovableClassC1Ev(
    class crubit::test::NonMemcpyMovableClass* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN6crubit4test21NonMemcpyMovableClassC1ERKS1_(
    class crubit::test::NonMemcpyMovableClass* __this,
    class crubit::test::NonMemcpyMovableClass const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" void __rust_thunk___ZN6crubit4test21NonMemcpyMovableClassC1EOS1_(
    class crubit::test::NonMemcpyMovableClass* __this,
    class crubit::test::NonMemcpyMovableClass* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class crubit::test::NonMemcpyMovableClass*
__rust_thunk___ZN6crubit4test21NonMemcpyMovableClassaSERKS1_(
    class crubit::test::NonMemcpyMovableClass* __this,
    class crubit::test::NonMemcpyMovableClass const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

extern "C" class crubit::test::NonMemcpyMovableClass*
__rust_thunk___ZN6crubit4test21NonMemcpyMovableClassaSEOS1_(
    class crubit::test::NonMemcpyMovableClass* __this,
    class crubit::test::NonMemcpyMovableClass* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

extern "C" void __rust_thunk___ZN6crubit4test20ReturnsMemcpyMovableEv(
    class crubit::test::MemcpyMovableClass* __return) {
  new (__return) auto(crubit::test::ReturnsMemcpyMovable());
}

static_assert((class crubit::test::MemcpyMovableClass (*)()) &
              ::crubit::test::ReturnsMemcpyMovable);

extern "C" void __rust_thunk___ZN6crubit4test23ReturnsNonMemcpyMovableEv(
    class crubit::test::NonMemcpyMovableClass* __return) {
  new (__return) auto(crubit::test::ReturnsNonMemcpyMovable());
}

static_assert((class crubit::test::NonMemcpyMovableClass (*)()) &
              ::crubit::test::ReturnsNonMemcpyMovable);

extern "C" void
__rust_thunk___ZN6crubit4test20AcceptsMemcpyMovableENS0_18MemcpyMovableClassE(
    class crubit::test::MemcpyMovableClass* __param_0) {
  crubit::test::AcceptsMemcpyMovable(std::move(*__param_0));
}

static_assert((void (*)(class crubit::test::MemcpyMovableClass)) &
              ::crubit::test::AcceptsMemcpyMovable);

extern "C" void
__rust_thunk___ZN6crubit4test23AcceptsNonMemcpyMovableENS0_21NonMemcpyMovableClassE(
    class crubit::test::NonMemcpyMovableClass* __param_0) {
  crubit::test::AcceptsNonMemcpyMovable(std::move(*__param_0));
}

static_assert((void (*)(class crubit::test::NonMemcpyMovableClass)) &
              ::crubit::test::AcceptsNonMemcpyMovable);

#pragma clang diagnostic pop
