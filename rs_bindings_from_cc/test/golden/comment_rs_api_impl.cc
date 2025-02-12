// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:comment_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/comment.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct Foo) == 8);
static_assert(alignof(struct Foo) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct Foo) == 0);
static_assert(CRUBIT_OFFSET_OF(j, struct Foo) == 4);

extern "C" void __rust_thunk___ZN3FooC1Ev(struct Foo* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN3FooC1EOS_(struct Foo* __this,
                                            struct Foo* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
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

static_assert(CRUBIT_SIZEOF(struct Bar) == 4);
static_assert(alignof(struct Bar) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct Bar) == 0);

extern "C" void __rust_thunk___ZN3BarC1Ev(struct Bar* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN3BarC1EOS_(struct Bar* __this,
                                            struct Bar* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct Bar* __rust_thunk___ZN3BaraSERKS_(
    struct Bar* __this, const struct Bar* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct Bar* __rust_thunk___ZN3BaraSEOS_(struct Bar* __this,
                                                   struct Bar* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(struct HasNoComments) == 4);
static_assert(alignof(struct HasNoComments) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct HasNoComments) == 0);

extern "C" void __rust_thunk___ZN13HasNoCommentsC1Ev(
    struct HasNoComments* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN13HasNoCommentsC1EOS_(
    struct HasNoComments* __this, struct HasNoComments* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct HasNoComments* __rust_thunk___ZN13HasNoCommentsaSERKS_(
    struct HasNoComments* __this, const struct HasNoComments* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct HasNoComments* __rust_thunk___ZN13HasNoCommentsaSEOS_(
    struct HasNoComments* __this, struct HasNoComments* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

#pragma clang diagnostic pop
