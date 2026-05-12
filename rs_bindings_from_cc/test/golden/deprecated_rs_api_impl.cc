// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:deprecated_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/deprecated.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___Z19deprecated_functionv() {
  ::deprecated_function();
}

static_assert((void (*)()) & ::deprecated_function);

extern "C" void __rust_thunk___Z32deprecated_function_with_messagev() {
  ::deprecated_function_with_message();
}

static_assert((void (*)()) & ::deprecated_function_with_message);

static_assert(sizeof(struct ::DeprecatedStruct) == 1);
static_assert(alignof(struct ::DeprecatedStruct) == 1);

extern "C" void __rust_thunk___ZN16DeprecatedStructC1Ev(
    struct ::DeprecatedStruct* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct ::DeprecatedStructWithMessage) == 1);
static_assert(alignof(struct ::DeprecatedStructWithMessage) == 1);

extern "C" void __rust_thunk___ZN27DeprecatedStructWithMessageC1Ev(
    struct ::DeprecatedStructWithMessage* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN19DeprecatedNamespace1fEv() {
  ::DeprecatedNamespace::f();
}

static_assert((void (*)()) & ::DeprecatedNamespace::f);

extern "C" void __rust_thunk___ZN30DeprecatedNamespaceWithMessage1fEv() {
  ::DeprecatedNamespaceWithMessage::f();
}

static_assert((void (*)()) & ::DeprecatedNamespaceWithMessage::f);

static_assert(CRUBIT_SIZEOF(struct ::DeprecatedFields) == 8);
static_assert(alignof(struct ::DeprecatedFields) == 4);
static_assert(CRUBIT_OFFSET_OF(no_message, struct ::DeprecatedFields) == 0);
static_assert(CRUBIT_OFFSET_OF(message, struct ::DeprecatedFields) == 4);

extern "C" void __rust_thunk___ZN16DeprecatedFieldsC1Ev(
    struct ::DeprecatedFields* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
