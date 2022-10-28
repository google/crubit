// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unsupported_cc

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/unsupported.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN17TrivialCustomTypeC1Ev(
    struct TrivialCustomType* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN17TrivialCustomTypeC1EOS_(
    struct TrivialCustomType* __this, struct TrivialCustomType* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN16ContainingStructC1Ev(
    struct ContainingStruct* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN16ContainingStructC1EOS_(
    struct ContainingStruct* __this, struct ContainingStruct* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

static_assert(sizeof(struct TrivialCustomType) == 4);
static_assert(alignof(struct TrivialCustomType) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct TrivialCustomType) == 0);

static_assert(sizeof(struct NontrivialCustomType) == 4);
static_assert(alignof(struct NontrivialCustomType) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct NontrivialCustomType) == 0);

static_assert(sizeof(struct ContainingStruct) == 1);
static_assert(alignof(struct ContainingStruct) == 1);
static_assert(CRUBIT_OFFSET_OF(nested_struct, struct ContainingStruct) == 0);

#pragma clang diagnostic pop
