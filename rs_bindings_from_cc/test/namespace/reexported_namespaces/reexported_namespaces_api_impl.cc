// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/namespace/reexported_namespaces:reexported_namespaces

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/namespace/reexported_namespaces/reexported_namespaces.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___ZN4absl14MyAbslFunctionEv() {
  absl::MyAbslFunction();
}

static_assert((void (*)()) & ::absl::MyAbslFunction);

extern "C" void __rust_thunk___ZN4base14MyBaseFunctionEv() {
  base::MyBaseFunction();
}

static_assert((void (*)()) & ::base::MyBaseFunction);

#pragma clang diagnostic pop
