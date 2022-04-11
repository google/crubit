// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/test/golden/static_methods.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN9SomeClassC1Ev(class SomeClass* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN9SomeClassC1ERKS_(
    class SomeClass* __this, const class SomeClass& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN9SomeClassC1EOS_(class SomeClass* __this,
                                                  class SomeClass&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN9SomeClassD1Ev(class SomeClass* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class SomeClass& __rust_thunk___ZN9SomeClassaSERKS_(
    class SomeClass* __this, const class SomeClass& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class SomeClass& __rust_thunk___ZN9SomeClassaSEOS_(
    class SomeClass* __this, class SomeClass&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}

static_assert(sizeof(class SomeClass) == 4);
static_assert(alignof(class SomeClass) == 4);

#pragma clang diagnostic pop
