// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/unnamed_fields.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN17WithUnnamedFieldsC1Ev(
    class WithUnnamedFields* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN17WithUnnamedFieldsC1ERKS_(
    class WithUnnamedFields* __this, const class WithUnnamedFields& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN17WithUnnamedFieldsD1Ev(
    class WithUnnamedFields* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class WithUnnamedFields& __rust_thunk___ZN17WithUnnamedFieldsaSERKS_(
    class WithUnnamedFields* __this, const class WithUnnamedFields& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}

static_assert(sizeof(class WithUnnamedFields) == 20);
static_assert(alignof(class WithUnnamedFields) == 4);
static_assert(CRUBIT_OFFSET_OF(foo, class WithUnnamedFields) * 8 == 0);
static_assert(CRUBIT_OFFSET_OF(bar, class WithUnnamedFields) * 8 == 64);
static_assert(CRUBIT_OFFSET_OF(baz, class WithUnnamedFields) * 8 == 128);

#pragma clang diagnostic pop
