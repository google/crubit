// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/test/golden/user_of_base_class.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN8Derived2C1Ev(class Derived2* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN8Derived2C1ERKS_(
    class Derived2* __this, const class Derived2& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN8Derived2D1Ev(class Derived2* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class Derived2& __rust_thunk___ZN8Derived2aSERKS_(
    class Derived2* __this, const class Derived2& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}

static_assert(sizeof(class Derived2) == 16);
static_assert(alignof(class Derived2) == 8);
static_assert(offsetof(class Derived2, derived_1) * 8 == 96);

#pragma clang diagnostic pop
