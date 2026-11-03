// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:rust_layout_equivalent

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/slot.h"

#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/annotations/rust_layout_equivalent.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void
__rust_thunk___ZN6crubit4test17AddOneToSameCrateENS0_16SameCrateCppTypeE(
    crubit::test::SameCrateCppType* __return,
    crubit::test::SameCrateCppType* v) {
  new (__return) auto(
      crubit::test::AddOneToSameCrate(crubit::UnsafeTakeValue(v)));
}

static_assert(
    (crubit::test::SameCrateCppType (*)(crubit::test::SameCrateCppType)) &
    ::crubit::test::AddOneToSameCrate);

extern "C" int64_t
__rust_thunk___ZN6crubit4test6AddTagENS0_11TaggedValueIiLl10EEE(
    crubit::test::TaggedValue<int, 10>* v) {
  return crubit::test::AddTag(crubit::UnsafeTakeValue(v));
}

static_assert((int64_t (*)(crubit::test::TaggedValue<int, 10>)) &
              ::crubit::test::AddTag);

extern "C" void
__rust_thunk___ZN6crubit4test14DoubleExternalENS0_15ExternalCppTypeE(
    crubit::test::ExternalCppType* __return, crubit::test::ExternalCppType* v) {
  new (__return) auto(crubit::test::DoubleExternal(crubit::UnsafeTakeValue(v)));
}

static_assert(
    (crubit::test::ExternalCppType (*)(crubit::test::ExternalCppType)) &
    ::crubit::test::DoubleExternal);

#pragma clang diagnostic pop
