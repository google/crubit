// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/namespace/root_namespaces:root_namespaces_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/namespace/root_namespaces/root_namespaces.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct test_namespace::Foo) == 4);
static_assert(alignof(struct test_namespace::Foo) == 4);
static_assert(CRUBIT_OFFSET_OF(x, struct test_namespace::Foo) == 0);

extern "C" void __rust_thunk___ZN14test_namespace3FooC1Ev(
    struct test_namespace::Foo* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN14test_namespace3barEv() {
  test_namespace::bar();
}

static_assert((void (*)()) & ::test_namespace::bar);

#pragma clang diagnostic pop
