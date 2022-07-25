// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/private_members.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN23test_namespace_bindings9SomeClassC1Ev(
    class test_namespace_bindings::SomeClass* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN23test_namespace_bindings9SomeClassC1EOS0_(
    class test_namespace_bindings::SomeClass* __this,
    class test_namespace_bindings::SomeClass* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

static_assert(sizeof(class test_namespace_bindings::SomeClass) == 8);
static_assert(alignof(class test_namespace_bindings::SomeClass) == 4);
static_assert(CRUBIT_OFFSET_OF(public_member_variable_,
                               class test_namespace_bindings::SomeClass) == 0);

#pragma clang diagnostic pop
