// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unsupported_cc
// Features: experimental, supported

#include <cstddef>
#include <memory>

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

// Public headers of the C++ library being wrapped.
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
extern "C" struct TrivialCustomType*
__rust_thunk___ZN17TrivialCustomTypeaSERKS_(
    struct TrivialCustomType* __this,
    const struct TrivialCustomType* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" struct TrivialCustomType* __rust_thunk___ZN17TrivialCustomTypeaSEOS_(
    struct TrivialCustomType* __this, struct TrivialCustomType* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN16ContainingStructC1Ev(
    struct ContainingStruct* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN16ContainingStructC1EOS_(
    struct ContainingStruct* __this, struct ContainingStruct* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" struct ContainingStruct* __rust_thunk___ZN16ContainingStructaSERKS_(
    struct ContainingStruct* __this, const struct ContainingStruct* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" struct ContainingStruct* __rust_thunk___ZN16ContainingStructaSEOS_(
    struct ContainingStruct* __this, struct ContainingStruct* __param_0) {
  return &__this->operator=(std::move(*__param_0));
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
