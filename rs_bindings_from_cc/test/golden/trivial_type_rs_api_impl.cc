// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/trivial_type.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN7TrivialC1Ev(class Trivial* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN7TrivialC1ERKS_(
    class Trivial* __this, const class Trivial& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN7TrivialC1EOS_(class Trivial* __this,
                                                class Trivial&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN7TrivialD1Ev(class Trivial* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class Trivial& __rust_thunk___ZN7TrivialaSERKS_(
    class Trivial* __this, const class Trivial& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class Trivial& __rust_thunk___ZN7TrivialaSEOS_(
    class Trivial* __this, class Trivial&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN20TrivialWithDefaultedC1Ev(
    class TrivialWithDefaulted* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN20TrivialWithDefaultedC1ERKS_(
    class TrivialWithDefaulted* __this,
    const class TrivialWithDefaulted& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class TrivialWithDefaulted&
__rust_thunk___ZN20TrivialWithDefaultedaSERKS_(
    class TrivialWithDefaulted* __this,
    const class TrivialWithDefaulted& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN20TrivialWithDefaultedC1EOS_(
    class TrivialWithDefaulted* __this,
    class TrivialWithDefaulted&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class TrivialWithDefaulted&
__rust_thunk___ZN20TrivialWithDefaultedaSEOS_(
    class TrivialWithDefaulted* __this,
    class TrivialWithDefaulted&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN20TrivialWithDefaultedD1Ev(
    class TrivialWithDefaulted* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN15TrivialNonfinalC1Ev(
    class TrivialNonfinal* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN15TrivialNonfinalC1ERKS_(
    class TrivialNonfinal* __this, const class TrivialNonfinal& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN15TrivialNonfinalC1EOS_(
    class TrivialNonfinal* __this, class TrivialNonfinal&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN15TrivialNonfinalD1Ev(
    class TrivialNonfinal* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class TrivialNonfinal& __rust_thunk___ZN15TrivialNonfinalaSERKS_(
    class TrivialNonfinal* __this, const class TrivialNonfinal& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class TrivialNonfinal& __rust_thunk___ZN15TrivialNonfinalaSEOS_(
    class TrivialNonfinal* __this, class TrivialNonfinal&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}

static_assert(sizeof(class Trivial) == 4);
static_assert(alignof(class Trivial) == 4);
static_assert(CRUBIT_OFFSET_OF(trivial_field, class Trivial) == 0);

static_assert(sizeof(class TrivialWithDefaulted) == 4);
static_assert(alignof(class TrivialWithDefaulted) == 4);
static_assert(CRUBIT_OFFSET_OF(trivial_field, class TrivialWithDefaulted) == 0);

static_assert(sizeof(class TrivialNonfinal) == 4);
static_assert(alignof(class TrivialNonfinal) == 4);
static_assert(CRUBIT_OFFSET_OF(trivial_field, class TrivialNonfinal) == 0);

#pragma clang diagnostic pop
