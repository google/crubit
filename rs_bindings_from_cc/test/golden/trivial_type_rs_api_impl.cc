// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/test/golden/trivial_type.h"

extern "C" void __rust_thunk___ZN7TrivialC1Ev(class Trivial* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN7TrivialC1ERKS_(
    class Trivial* __this, const class Trivial& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN7TrivialD1Ev(class Trivial* __this) {
  std ::destroy_at(__this);
}
extern "C" void __rust_thunk___ZN20TrivialWithDefaultedC1Ev(
    class TrivialWithDefaulted* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN20TrivialWithDefaultedC1ERKS_(
    class TrivialWithDefaulted* __this,
    const class TrivialWithDefaulted& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN20TrivialWithDefaultedD1Ev(
    class TrivialWithDefaulted* __this) {
  std ::destroy_at(__this);
}
extern "C" void __rust_thunk___ZN15TrivialNonfinalC1Ev(
    class TrivialNonfinal* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN15TrivialNonfinalC1ERKS_(
    class TrivialNonfinal* __this, const class TrivialNonfinal& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN15TrivialNonfinalD1Ev(
    class TrivialNonfinal* __this) {
  std ::destroy_at(__this);
}

static_assert(sizeof(class Trivial) == 4);
static_assert(alignof(class Trivial) == 4);
static_assert(offsetof(class Trivial, trivial_field) * 8 == 0);

static_assert(sizeof(class TrivialWithDefaulted) == 4);
static_assert(alignof(class TrivialWithDefaulted) == 4);
static_assert(offsetof(class TrivialWithDefaulted, trivial_field) * 8 == 0);

static_assert(sizeof(class TrivialNonfinal) == 4);
static_assert(alignof(class TrivialNonfinal) == 4);
static_assert(offsetof(class TrivialNonfinal, trivial_field) * 8 == 0);
