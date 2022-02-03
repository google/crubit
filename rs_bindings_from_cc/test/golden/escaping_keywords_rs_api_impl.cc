// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/test/golden/escaping_keywords.h"

extern "C" void __rust_thunk___ZN4typeC1Ev(class type* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN4typeC1ERKS_(class type* __this,
                                              const class type& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN4typeD1Ev(class type* __this) {
  std ::destroy_at(__this);
}
extern "C" class type& __rust_thunk___ZN4typeaSERKS_(
    class type* __this, const class type& __param_0) {
  return __this->operator=(__param_0);
}

static_assert(sizeof(class type) == 4);
static_assert(alignof(class type) == 4);
static_assert(offsetof(class type, dyn) * 8 == 0);
