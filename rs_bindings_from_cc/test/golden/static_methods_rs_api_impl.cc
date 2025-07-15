// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:static_methods_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/static_methods.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(class SomeClass) == 4);
static_assert(alignof(class SomeClass) == 4);

extern "C" void __rust_thunk___ZN9SomeClassC1Ev(class SomeClass* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN9SomeClass21static_factory_methodEi(
    class SomeClass* __return, int initial_value_of_field) {
  new (__return) auto(SomeClass::static_factory_method(initial_value_of_field));
}

#pragma clang diagnostic pop
