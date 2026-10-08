// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:using_function_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/using_function.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk__d6cf3509__ZN1A3FooEv() { A::Foo(); }

static_assert((void (*)()) & ::A::Foo);

extern "C" int __rust_thunk__e4bd4550__ZN1A3AddEii(int a, int b) {
  return A::Add(a, b);
}

static_assert((int (*)(int, int)) & ::A::Add);

extern "C" void __rust_thunk__2e00d1b3__ZN1A3FooEv() { B::Foo(); }

static_assert((void (*)()) & ::B::Foo);

extern "C" int __rust_thunk__beecbe97__ZN1A3AddEii(int a, int b) {
  return B::Add(a, b);
}

static_assert((int (*)(int, int)) & ::B::Add);

#pragma clang diagnostic pop
