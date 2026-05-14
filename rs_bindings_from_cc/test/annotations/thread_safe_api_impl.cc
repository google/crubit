// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:thread_safe
// Features: fmt, supported, types

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

#pragma clang diagnostic pop
