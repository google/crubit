// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:static_methods_cc

#include <cstddef>
#include <memory>

#include "support/cxx20_backports.h"
#include "support/offsetof.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/static_methods.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN9SomeClassC1Ev(class SomeClass* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN9SomeClassC1EOS_(class SomeClass* __this,
                                                  class SomeClass* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

static_assert(sizeof(class SomeClass) == 4);
static_assert(alignof(class SomeClass) == 4);

#pragma clang diagnostic pop
