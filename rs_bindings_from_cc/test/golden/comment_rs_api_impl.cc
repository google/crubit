// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:comment_cc

#include <cstddef>
#include <memory>

#include "support/cxx20_backports.h"
#include "support/offsetof.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/comment.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN3FooC1Ev(struct Foo* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN3FooC1EOS_(struct Foo* __this,
                                            struct Foo* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___Z3foov() { foo(); }
extern "C" void __rust_thunk___ZN3BarC1Ev(struct Bar* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN3BarC1EOS_(struct Bar* __this,
                                            struct Bar* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN13HasNoCommentsC1Ev(
    struct HasNoComments* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN13HasNoCommentsC1EOS_(
    struct HasNoComments* __this, struct HasNoComments* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
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
