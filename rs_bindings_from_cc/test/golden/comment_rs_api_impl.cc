// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>
#include "rs_bindings_from_cc/test/golden/comment.h"

namespace {
template <class T, class... Args>
constexpr T* construct_at(T* p, Args&&... args) {
  return ::new (const_cast<void*>(static_cast<const volatile void*>(p)))
      T(std ::forward<Args>(args)...);
}
}  // namespace
extern "C" void __rust_thunk___ZN3FooC1Ev(Foo* __this) { construct_at(__this); }
extern "C" void __rust_thunk___ZN3FooD1Ev(Foo* __this) {
  std ::destroy_at(__this);
}
extern "C" void __rust_thunk___Z3foov() { foo(); }
extern "C" void __rust_thunk___ZN3BarC1Ev(Bar* __this) { construct_at(__this); }
extern "C" void __rust_thunk___ZN3BarD1Ev(Bar* __this) {
  std ::destroy_at(__this);
}
extern "C" void __rust_thunk___ZN13HasNoCommentsC1Ev(HasNoComments* __this) {
  construct_at(__this);
}
extern "C" void __rust_thunk___ZN13HasNoCommentsD1Ev(HasNoComments* __this) {
  std ::destroy_at(__this);
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
