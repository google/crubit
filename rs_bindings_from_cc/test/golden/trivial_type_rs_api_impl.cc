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
extern "C" void __rust_thunk___ZN23test_namespace_bindings7TrivialC1Ev(
    struct test_namespace_bindings::Trivial* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN23test_namespace_bindings7TrivialC1ERKS0_(
    struct test_namespace_bindings::Trivial* __this,
    const struct test_namespace_bindings::Trivial& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN23test_namespace_bindings7TrivialC1EOS0_(
    struct test_namespace_bindings::Trivial* __this,
    struct test_namespace_bindings::Trivial&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN23test_namespace_bindings7TrivialD1Ev(
    struct test_namespace_bindings::Trivial* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" struct test_namespace_bindings::Trivial&
__rust_thunk___ZN23test_namespace_bindings7TrivialaSERKS0_(
    struct test_namespace_bindings::Trivial* __this,
    const struct test_namespace_bindings::Trivial& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" struct test_namespace_bindings::Trivial&
__rust_thunk___ZN23test_namespace_bindings7TrivialaSEOS0_(
    struct test_namespace_bindings::Trivial* __this,
    struct test_namespace_bindings::Trivial&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings20TrivialWithDefaultedC1Ev(
    struct test_namespace_bindings::TrivialWithDefaulted* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings20TrivialWithDefaultedC1ERKS0_(
    struct test_namespace_bindings::TrivialWithDefaulted* __this,
    const struct test_namespace_bindings::TrivialWithDefaulted& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" struct test_namespace_bindings::TrivialWithDefaulted&
__rust_thunk___ZN23test_namespace_bindings20TrivialWithDefaultedaSERKS0_(
    struct test_namespace_bindings::TrivialWithDefaulted* __this,
    const struct test_namespace_bindings::TrivialWithDefaulted& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings20TrivialWithDefaultedC1EOS0_(
    struct test_namespace_bindings::TrivialWithDefaulted* __this,
    struct test_namespace_bindings::TrivialWithDefaulted&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" struct test_namespace_bindings::TrivialWithDefaulted&
__rust_thunk___ZN23test_namespace_bindings20TrivialWithDefaultedaSEOS0_(
    struct test_namespace_bindings::TrivialWithDefaulted* __this,
    struct test_namespace_bindings::TrivialWithDefaulted&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings20TrivialWithDefaultedD1Ev(
    struct test_namespace_bindings::TrivialWithDefaulted* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN23test_namespace_bindings15TrivialNonfinalC1Ev(
    struct test_namespace_bindings::TrivialNonfinal* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings15TrivialNonfinalC1ERKS0_(
    struct test_namespace_bindings::TrivialNonfinal* __this,
    const struct test_namespace_bindings::TrivialNonfinal& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings15TrivialNonfinalC1EOS0_(
    struct test_namespace_bindings::TrivialNonfinal* __this,
    struct test_namespace_bindings::TrivialNonfinal&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN23test_namespace_bindings15TrivialNonfinalD1Ev(
    struct test_namespace_bindings::TrivialNonfinal* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" struct test_namespace_bindings::TrivialNonfinal&
__rust_thunk___ZN23test_namespace_bindings15TrivialNonfinalaSERKS0_(
    struct test_namespace_bindings::TrivialNonfinal* __this,
    const struct test_namespace_bindings::TrivialNonfinal& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" struct test_namespace_bindings::TrivialNonfinal&
__rust_thunk___ZN23test_namespace_bindings15TrivialNonfinalaSEOS0_(
    struct test_namespace_bindings::TrivialNonfinal* __this,
    struct test_namespace_bindings::TrivialNonfinal&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}

static_assert(sizeof(struct test_namespace_bindings::Trivial) == 4);
static_assert(alignof(struct test_namespace_bindings::Trivial) == 4);
static_assert(CRUBIT_OFFSET_OF(trivial_field,
                               struct test_namespace_bindings::Trivial) == 0);

static_assert(sizeof(struct test_namespace_bindings::TrivialWithDefaulted) ==
              4);
static_assert(alignof(struct test_namespace_bindings::TrivialWithDefaulted) ==
              4);
static_assert(CRUBIT_OFFSET_OF(
                  trivial_field,
                  struct test_namespace_bindings::TrivialWithDefaulted) == 0);

static_assert(sizeof(struct test_namespace_bindings::TrivialNonfinal) == 4);
static_assert(alignof(struct test_namespace_bindings::TrivialNonfinal) == 4);
static_assert(
    CRUBIT_OFFSET_OF(trivial_field,
                     struct test_namespace_bindings::TrivialNonfinal) == 0);

#pragma clang diagnostic pop
