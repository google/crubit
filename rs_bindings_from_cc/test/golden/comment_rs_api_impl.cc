// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>
#include "rs_bindings_from_cc/test/golden/comment.h"

extern "C" void __rust_destructor_thunk__Foo(Foo* __this) {
  return std ::destroy_at(__this);
}
extern "C" void __rust_thunk__foo() { return foo(); }
extern "C" void __rust_destructor_thunk__Bar(Bar* __this) {
  return std ::destroy_at(__this);
}
extern "C" void __rust_destructor_thunk__HasNoComments(HasNoComments* __this) {
  return std ::destroy_at(__this);
}

static_assert(sizeof(Foo) == 8);
static_assert(alignof(Foo) == 4);
static_assert(offsetof(Foo, i) * 8 == 0);
static_assert(offsetof(Foo, j) * 8 == 32);

static_assert(sizeof(Bar) == 4);
static_assert(alignof(Bar) == 4);
static_assert(offsetof(Bar, i) * 8 == 0);

static_assert(sizeof(HasNoComments) == 4);
static_assert(alignof(HasNoComments) == 4);
static_assert(offsetof(HasNoComments, i) * 8 == 0);
