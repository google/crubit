// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>
#include "rs_bindings_from_cc/test/golden/trivial_type.h"

namespace {
template <class T, class... Args>
constexpr T* construct_at(T* p, Args&&... args) {
  return ::new (const_cast<void*>(static_cast<const volatile void*>(p)))
      T(std ::forward<Args>(args)...);
}
}  // namespace
extern "C" void __rust_thunk___ZN7TrivialC1Ev(Trivial* __this) {
  construct_at(__this);
}
extern "C" void __rust_thunk___ZN7TrivialD1Ev(Trivial* __this) {
  std ::destroy_at(__this);
}
extern "C" void __rust_thunk___ZN20TrivialWithDefaultedC1Ev(
    TrivialWithDefaulted* __this) {
  construct_at(__this);
}
extern "C" void __rust_thunk___ZN20TrivialWithDefaultedD1Ev(
    TrivialWithDefaulted* __this) {
  std ::destroy_at(__this);
}
extern "C" void __rust_thunk___ZN15TrivialNonfinalC1Ev(
    TrivialNonfinal* __this) {
  construct_at(__this);
}
extern "C" void __rust_thunk___ZN15TrivialNonfinalD1Ev(
    TrivialNonfinal* __this) {
  std ::destroy_at(__this);
}

static_assert(sizeof(Trivial) == 4);
static_assert(alignof(Trivial) == 4);
static_assert(offsetof(Trivial, trivial_field) * 8 == 0);

static_assert(sizeof(TrivialWithDefaulted) == 4);
static_assert(alignof(TrivialWithDefaulted) == 4);
static_assert(offsetof(TrivialWithDefaulted, trivial_field) * 8 == 0);

static_assert(sizeof(TrivialNonfinal) == 4);
static_assert(alignof(TrivialNonfinal) == 4);
static_assert(offsetof(TrivialNonfinal, trivial_field) * 8 == 0);
