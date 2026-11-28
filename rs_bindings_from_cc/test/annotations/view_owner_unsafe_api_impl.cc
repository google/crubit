// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:view_owner_unsafe

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/annotations/view_owner_unsafe.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"

static_assert(CRUBIT_SIZEOF(class crubit::test::UnsafeView) == 8);
static_assert(alignof(class crubit::test::UnsafeView) == 8);

extern "C" void __rust_thunk___ZN6crubit4test10UnsafeViewC1Ev(
    class crubit::test::UnsafeView* __this) {
  crubit::construct_at(__this);
}

extern "C" int __rust_thunk___ZNK6crubit4test10UnsafeView3GetEv(
    class crubit::test::UnsafeView const* __this) {
  return __this->Get();
}

static_assert((int (::crubit::test::UnsafeView::*)() const) &
              ::crubit::test::UnsafeView::Get);

static_assert(CRUBIT_SIZEOF(class crubit::test::SafeOwner) == 8);
static_assert(alignof(class crubit::test::SafeOwner) == 8);

extern "C" void __rust_thunk___ZN6crubit4test9SafeOwnerC1Ev(
    class crubit::test::SafeOwner* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN6crubit4test9SafeOwnerD1Ev(
    class crubit::test::SafeOwner* __this) {
  std::destroy_at(__this);
}

extern "C" int __rust_thunk___ZN6crubit4test14ReadUnsafeViewENS0_10UnsafeViewE(
    class crubit::test::UnsafeView* v) {
  return crubit::test::ReadUnsafeView(crubit::UnsafeTakeValue(v));
}

static_assert((int (*)(class crubit::test::UnsafeView)) &
              ::crubit::test::ReadUnsafeView);

#pragma clang diagnostic pop
