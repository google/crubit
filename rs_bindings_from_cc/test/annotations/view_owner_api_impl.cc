// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:view_owner

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/annotations/view_owner.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"

static_assert(CRUBIT_SIZEOF(class crubit::test::View) == 8);
static_assert(alignof(class crubit::test::View) == 8);

extern "C" void __rust_thunk___ZN6crubit4test4ViewC1Ev(
    class crubit::test::View* __this) {
  crubit::construct_at(__this);
}

extern "C" int __rust_thunk___ZNK6crubit4test4View3GetEv(
    class crubit::test::View const* __this) {
  return __this->Get();
}

static_assert((int (::crubit::test::View::*)() const) &
              ::crubit::test::View::Get);

static_assert(CRUBIT_SIZEOF(class crubit::test::Owner) == 8);
static_assert(alignof(class crubit::test::Owner) == 8);

extern "C" void __rust_thunk___ZN6crubit4test5OwnerC1Ev(
    class crubit::test::Owner* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN6crubit4test5OwnerD1Ev(
    class crubit::test::Owner* __this) {
  std::destroy_at(__this);
}

static_assert(CRUBIT_SIZEOF(class crubit::test::Plain) == 8);
static_assert(alignof(class crubit::test::Plain) == 8);

extern "C" void __rust_thunk___ZN6crubit4test5PlainC1Ev(
    class crubit::test::Plain* __this) {
  crubit::construct_at(__this);
}

extern "C" int __rust_thunk___ZNK6crubit4test5Plain3GetEv(
    class crubit::test::Plain const* __this) {
  return __this->Get();
}

static_assert((int (::crubit::test::Plain::*)() const) &
              ::crubit::test::Plain::Get);

extern "C" int __rust_thunk___ZN6crubit4test8ReadViewENS0_4ViewE(
    class crubit::test::View* v) {
  return crubit::test::ReadView(crubit::UnsafeTakeValue(v));
}

static_assert((int (*)(class crubit::test::View)) & ::crubit::test::ReadView);

extern "C" int __rust_thunk___ZN6crubit4test9ReadPlainENS0_5PlainE(
    class crubit::test::Plain* p) {
  return crubit::test::ReadPlain(crubit::UnsafeTakeValue(p));
}

static_assert((int (*)(class crubit::test::Plain)) & ::crubit::test::ReadPlain);

#pragma clang diagnostic pop
