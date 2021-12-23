// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>
#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/test/golden/unsupported.h"

extern "C" void __rust_thunk___ZN20NontrivialCustomTypeD1Ev(
    NontrivialCustomType* __this) {
  std ::destroy_at(__this);
}
extern "C" void __rust_thunk___ZN16ContainingStructC1Ev(
    ContainingStruct* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN16ContainingStructD1Ev(
    ContainingStruct* __this) {
  std ::destroy_at(__this);
}

static_assert(sizeof(NontrivialCustomType) == 4);
static_assert(alignof(NontrivialCustomType) == 4);
static_assert(offsetof(NontrivialCustomType, i) * 8 == 0);

static_assert(sizeof(ContainingStruct) == 1);
static_assert(alignof(ContainingStruct) == 1);
