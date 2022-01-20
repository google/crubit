// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>
#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/test/golden/no_elided_lifetimes.h"

extern "C" void __rust_thunk___ZN1SC1Ev(class S* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN1SC1ERKS_(class S* __this,
                                           const class S& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN1SD1Ev(class S* __this) {
  std ::destroy_at(__this);
}

static_assert(sizeof(class S) == 1);
static_assert(alignof(class S) == 1);
