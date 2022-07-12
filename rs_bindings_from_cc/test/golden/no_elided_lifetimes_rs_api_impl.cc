// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/no_elided_lifetimes.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN1SC1Ev(struct S* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN1SC1ERKS_(struct S* __this,
                                           const struct S& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN1SD1Ev(struct S* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" struct S& __rust_thunk___ZN1SaSERKS_(struct S* __this,
                                                const struct S& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" struct TriviallyCopyableButNontriviallyDestructible&
__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleaSERKS_(
    struct TriviallyCopyableButNontriviallyDestructible* __this,
    const struct TriviallyCopyableButNontriviallyDestructible& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleC1ERKS_(
    struct TriviallyCopyableButNontriviallyDestructible* __this,
    const struct TriviallyCopyableButNontriviallyDestructible& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}

static_assert(sizeof(struct S) == 1);
static_assert(alignof(struct S) == 1);

static_assert(sizeof(struct TriviallyCopyableButNontriviallyDestructible) == 1);
static_assert(alignof(struct TriviallyCopyableButNontriviallyDestructible) ==
              1);

#pragma clang diagnostic pop
