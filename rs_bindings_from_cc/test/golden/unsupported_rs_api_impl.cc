// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/unsupported.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN20NontrivialCustomTypeD1Ev(
    class NontrivialCustomType* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN16ContainingStructC1Ev(
    class ContainingStruct* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN16ContainingStructC1ERKS_(
    class ContainingStruct* __this, const class ContainingStruct& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN16ContainingStructC1EOS_(
    class ContainingStruct* __this, class ContainingStruct&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN16ContainingStructD1Ev(
    class ContainingStruct* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class ContainingStruct& __rust_thunk___ZN16ContainingStructaSERKS_(
    class ContainingStruct* __this, const class ContainingStruct& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class ContainingStruct& __rust_thunk___ZN16ContainingStructaSEOS_(
    class ContainingStruct* __this, class ContainingStruct&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}

static_assert(sizeof(class NontrivialCustomType) == 4);
static_assert(alignof(class NontrivialCustomType) == 4);
static_assert(CRUBIT_OFFSET_OF(i, class NontrivialCustomType) == 0);

static_assert(sizeof(class ContainingStruct) == 1);
static_assert(alignof(class ContainingStruct) == 1);
static_assert(CRUBIT_OFFSET_OF(nested_struct, class ContainingStruct) == 0);

#pragma clang diagnostic pop
