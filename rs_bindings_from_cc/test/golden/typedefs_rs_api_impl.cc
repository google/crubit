// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
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
extern "C" void __rust_thunk___ZN10SomeStructD1Ev(struct SomeStruct* __this) {
  std::destroy_at(__this);
}
extern "C" struct SomeStruct* __rust_thunk___ZN10SomeStructaSERKS_(
    struct SomeStruct* __this, const struct SomeStruct* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" struct SomeStruct* __rust_thunk___ZN10SomeStructaSEOS_(
    struct SomeStruct* __this, struct SomeStruct* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN9SomeUnionC1Ev(union SomeUnion* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN9SomeUnionC1ERKS_(
    union SomeUnion* __this, const union SomeUnion* __param_0) {
  crubit::construct_at(__this, *__param_0);
}
extern "C" void __rust_thunk___ZN9SomeUnionC1EOS_(union SomeUnion* __this,
                                                  union SomeUnion* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN9SomeUnionD1Ev(union SomeUnion* __this) {
  std::destroy_at(__this);
}
extern "C" union SomeUnion* __rust_thunk___ZN9SomeUnionaSERKS_(
    union SomeUnion* __this, const union SomeUnion* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" union SomeUnion* __rust_thunk___ZN9SomeUnionaSEOS_(
    union SomeUnion* __this, union SomeUnion* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(sizeof(struct SomeStruct) == 1);
static_assert(alignof(struct SomeStruct) == 1);

static_assert(sizeof(union SomeUnion) == 1);
static_assert(alignof(union SomeUnion) == 1);

#pragma clang diagnostic pop
