// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:typedefs_cc

#include <cstddef>
#include <memory>

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/typedefs.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN10SomeStructC1Ev(struct SomeStruct* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN10SomeStructC1ERKS_(
    struct SomeStruct* __this, const struct SomeStruct* __param_0) {
  crubit::construct_at(__this, *__param_0);
}
extern "C" void __rust_thunk___ZN10SomeStructC1EOS_(
    struct SomeStruct* __this, struct SomeStruct* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" struct SomeStruct* __rust_thunk___ZN10SomeStructaSERKS_(
    struct SomeStruct* __this, const struct SomeStruct* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" struct SomeStruct* __rust_thunk___ZN10SomeStructaSEOS_(
    struct SomeStruct* __this, struct SomeStruct* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN15SomeOtherStructC1Ev(
    SomeOtherStruct* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN15SomeOtherStructC1ERKS_(
    SomeOtherStruct* __this, const SomeOtherStruct* __param_0) {
  crubit::construct_at(__this, *__param_0);
}
extern "C" void __rust_thunk___ZN15SomeOtherStructC1EOS_(
    SomeOtherStruct* __this, SomeOtherStruct* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" SomeOtherStruct* __rust_thunk___ZN15SomeOtherStructaSERKS_(
    SomeOtherStruct* __this, const SomeOtherStruct* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" SomeOtherStruct* __rust_thunk___ZN15SomeOtherStructaSEOS_(
    SomeOtherStruct* __this, SomeOtherStruct* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN9SomeUnionC1Ev(union SomeUnion* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN9SomeUnionC1EOS_(union SomeUnion* __this,
                                                  union SomeUnion* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN14SomeOtherUnionC1Ev(SomeOtherUnion* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN14SomeOtherUnionC1EOS_(
    SomeOtherUnion* __this, SomeOtherUnion* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

static_assert(sizeof(struct SomeStruct) == 1);
static_assert(alignof(struct SomeStruct) == 1);

static_assert(sizeof(SomeOtherStruct) == 1);
static_assert(alignof(SomeOtherStruct) == 1);

static_assert(sizeof(union SomeUnion) == 1);
static_assert(alignof(union SomeUnion) == 1);

static_assert(sizeof(SomeOtherUnion) == 1);
static_assert(alignof(SomeOtherUnion) == 1);

#pragma clang diagnostic pop
