// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:trivial_type_cc

#include <cstddef>
#include <memory>

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/trivial_type.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN2ns7TrivialC1Ev(struct ns::Trivial* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN2ns7TrivialC1EOS0_(
    struct ns::Trivial* __this, struct ns::Trivial* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" struct ns::Trivial* __rust_thunk___ZN2ns7TrivialaSERKS0_(
    struct ns::Trivial* __this, const struct ns::Trivial* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" struct ns::Trivial* __rust_thunk___ZN2ns7TrivialaSEOS0_(
    struct ns::Trivial* __this, struct ns::Trivial* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN2ns20TrivialWithDefaultedC1Ev(
    struct ns::TrivialWithDefaulted* __this) {
  crubit::construct_at(__this);
}
extern "C" struct ns::TrivialWithDefaulted*
__rust_thunk___ZN2ns20TrivialWithDefaultedaSERKS0_(
    struct ns::TrivialWithDefaulted* __this,
    const struct ns::TrivialWithDefaulted* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" void __rust_thunk___ZN2ns20TrivialWithDefaultedC1EOS0_(
    struct ns::TrivialWithDefaulted* __this,
    struct ns::TrivialWithDefaulted* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" struct ns::TrivialWithDefaulted*
__rust_thunk___ZN2ns20TrivialWithDefaultedaSEOS0_(
    struct ns::TrivialWithDefaulted* __this,
    struct ns::TrivialWithDefaulted* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN2ns15TrivialNonfinalC1Ev(
    struct ns::TrivialNonfinal* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN2ns15TrivialNonfinalC1ERKS0_(
    struct ns::TrivialNonfinal* __this,
    const struct ns::TrivialNonfinal* __param_0) {
  crubit::construct_at(__this, *__param_0);
}
extern "C" void __rust_thunk___ZN2ns15TrivialNonfinalC1EOS0_(
    struct ns::TrivialNonfinal* __this, struct ns::TrivialNonfinal* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" struct ns::TrivialNonfinal*
__rust_thunk___ZN2ns15TrivialNonfinalaSERKS0_(
    struct ns::TrivialNonfinal* __this,
    const struct ns::TrivialNonfinal* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" struct ns::TrivialNonfinal*
__rust_thunk___ZN2ns15TrivialNonfinalaSEOS0_(
    struct ns::TrivialNonfinal* __this, struct ns::TrivialNonfinal* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}
extern "C" void
__rust_thunk___ZN2ns27TakesTrivialNonfinalByValueENS_15TrivialNonfinalE(
    struct ns::TrivialNonfinal* trivial) {
  ns::TakesTrivialNonfinalByValue(std::move(*trivial));
}

static_assert(sizeof(struct ns::Trivial) == 4);
static_assert(alignof(struct ns::Trivial) == 4);
static_assert(CRUBIT_OFFSET_OF(trivial_field, struct ns::Trivial) == 0);

static_assert(sizeof(struct ns::TrivialWithDefaulted) == 4);
static_assert(alignof(struct ns::TrivialWithDefaulted) == 4);
static_assert(CRUBIT_OFFSET_OF(trivial_field,
                               struct ns::TrivialWithDefaulted) == 0);

static_assert(sizeof(struct ns::TrivialNonfinal) == 4);
static_assert(alignof(struct ns::TrivialNonfinal) == 4);
static_assert(CRUBIT_OFFSET_OF(trivial_field, struct ns::TrivialNonfinal) == 0);

#pragma clang diagnostic pop
