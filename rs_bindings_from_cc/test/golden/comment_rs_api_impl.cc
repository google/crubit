// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/comment.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN3FooC1Ev(struct Foo* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN3FooC1ERKS_(struct Foo* __this,
                                             const struct Foo* __param_0) {
  crubit::construct_at(__this, *__param_0);
}
extern "C" void __rust_thunk___ZN3FooC1EOS_(struct Foo* __this,
                                            struct Foo* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN3FooD1Ev(struct Foo* __this) {
  std::destroy_at(__this);
}
extern "C" struct Foo* __rust_thunk___ZN3FooaSERKS_(
    struct Foo* __this, const struct Foo* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" struct Foo* __rust_thunk___ZN3FooaSEOS_(struct Foo* __this,
                                                   struct Foo* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}
extern "C" void __rust_thunk___Z3foov() { foo(); }
extern "C" void __rust_thunk___ZN3BarC1Ev(struct Bar* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN3BarC1ERKS_(struct Bar* __this,
                                             const struct Bar* __param_0) {
  crubit::construct_at(__this, *__param_0);
}
extern "C" void __rust_thunk___ZN3BarC1EOS_(struct Bar* __this,
                                            struct Bar* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN3BarD1Ev(struct Bar* __this) {
  std::destroy_at(__this);
}
extern "C" struct Bar* __rust_thunk___ZN3BaraSERKS_(
    struct Bar* __this, const struct Bar* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" struct Bar* __rust_thunk___ZN3BaraSEOS_(struct Bar* __this,
                                                   struct Bar* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN13HasNoCommentsC1Ev(
    struct HasNoComments* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN13HasNoCommentsC1ERKS_(
    struct HasNoComments* __this, const struct HasNoComments* __param_0) {
  crubit::construct_at(__this, *__param_0);
}
extern "C" void __rust_thunk___ZN13HasNoCommentsC1EOS_(
    struct HasNoComments* __this, struct HasNoComments* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN13HasNoCommentsD1Ev(
    struct HasNoComments* __this) {
  std::destroy_at(__this);
}
extern "C" struct HasNoComments* __rust_thunk___ZN13HasNoCommentsaSERKS_(
    struct HasNoComments* __this, const struct HasNoComments* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" struct HasNoComments* __rust_thunk___ZN13HasNoCommentsaSEOS_(
    struct HasNoComments* __this, struct HasNoComments* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(sizeof(struct Foo) == 8);
static_assert(alignof(struct Foo) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct Foo) == 0);
static_assert(CRUBIT_OFFSET_OF(j, struct Foo) == 4);

static_assert(sizeof(struct Bar) == 4);
static_assert(alignof(struct Bar) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct Bar) == 0);

static_assert(sizeof(struct HasNoComments) == 4);
static_assert(alignof(struct HasNoComments) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct HasNoComments) == 0);

#pragma clang diagnostic pop
