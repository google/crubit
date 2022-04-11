// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/test/golden/escaping_keywords.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN4typeC1Ev(class type* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN4typeC1ERKS_(class type* __this,
                                              const class type& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN4typeC1EOS_(class type* __this,
                                             class type&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN4typeD1Ev(class type* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class type& __rust_thunk___ZN4typeaSERKS_(
    class type* __this, const class type& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class type& __rust_thunk___ZN4typeaSEOS_(class type* __this,
                                                    class type&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}

static_assert(sizeof(class type) == 4);
static_assert(alignof(class type) == 4);
static_assert(offsetof(class type, dyn) * 8 == 0);

#pragma clang diagnostic pop
