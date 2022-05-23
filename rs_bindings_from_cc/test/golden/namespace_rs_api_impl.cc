// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/namespace.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN23test_namespace_bindings1SC1Ev(
    class test_namespace_bindings::S* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN23test_namespace_bindings1SC1ERKS0_(
    class test_namespace_bindings::S* __this,
    const class test_namespace_bindings::S& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN23test_namespace_bindings1SD1Ev(
    class test_namespace_bindings::S* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class test_namespace_bindings::S&
__rust_thunk___ZN23test_namespace_bindings1SaSERKS0_(
    class test_namespace_bindings::S* __this,
    const class test_namespace_bindings::S& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN32test_namespace_bindings_reopened5inner1SC1Ev(
    class test_namespace_bindings_reopened::inner::S* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN32test_namespace_bindings_reopened5inner1SC1ERKS1_(
    class test_namespace_bindings_reopened::inner::S* __this,
    const class test_namespace_bindings_reopened::inner::S& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN32test_namespace_bindings_reopened5inner1SD1Ev(
    class test_namespace_bindings_reopened::inner::S* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class test_namespace_bindings_reopened::inner::S&
__rust_thunk___ZN32test_namespace_bindings_reopened5inner1SaSERKS1_(
    class test_namespace_bindings_reopened::inner::S* __this,
    const class test_namespace_bindings_reopened::inner::S& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}

static_assert(sizeof(class test_namespace_bindings::S) == 4);
static_assert(alignof(class test_namespace_bindings::S) == 4);
static_assert(CRUBIT_OFFSET_OF(i, class test_namespace_bindings::S) == 0);

static_assert(sizeof(class test_namespace_bindings_reopened::inner::S) == 1);
static_assert(alignof(class test_namespace_bindings_reopened::inner::S) == 1);

#pragma clang diagnostic pop
