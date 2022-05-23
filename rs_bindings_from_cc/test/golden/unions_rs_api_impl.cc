// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/unions.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN10EmptyUnionC1Ev(union EmptyUnion* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN10EmptyUnionC1ERKS_(
    union EmptyUnion* __this, const union EmptyUnion& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN10EmptyUnionD1Ev(union EmptyUnion* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" union EmptyUnion& __rust_thunk___ZN10EmptyUnionaSERKS_(
    union EmptyUnion* __this, const union EmptyUnion& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN13NonEmptyUnionC1Ev(
    union NonEmptyUnion* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN13NonEmptyUnionC1ERKS_(
    union NonEmptyUnion* __this, const union NonEmptyUnion& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN13NonEmptyUnionD1Ev(
    union NonEmptyUnion* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" union NonEmptyUnion& __rust_thunk___ZN13NonEmptyUnionaSERKS_(
    union NonEmptyUnion* __this, const union NonEmptyUnion& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}

static_assert(sizeof(union EmptyUnion) == 1);
static_assert(alignof(union EmptyUnion) == 1);

static_assert(sizeof(union NonEmptyUnion) == 8);
static_assert(alignof(union NonEmptyUnion) == 8);
static_assert(CRUBIT_OFFSET_OF(bool_field, union NonEmptyUnion) * 8 == 0);
static_assert(CRUBIT_OFFSET_OF(char_field, union NonEmptyUnion) * 8 == 0);
static_assert(CRUBIT_OFFSET_OF(int_field, union NonEmptyUnion) * 8 == 0);
static_assert(CRUBIT_OFFSET_OF(long_long_field, union NonEmptyUnion) * 8 == 0);

#pragma clang diagnostic pop
