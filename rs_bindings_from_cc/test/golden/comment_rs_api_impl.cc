// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/test/golden/comment.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN3FooC1Ev(class Foo* __this) {
  rs_api_impl_support ::construct_at(std ::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN3FooC1ERKS_(class Foo* __this,
                                             const class Foo& __param_0) {
  rs_api_impl_support ::construct_at(
      std ::forward<decltype(__this)>(__this),
      std ::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN3FooC1EOS_(class Foo* __this,
                                            class Foo&& __param_0) {
  rs_api_impl_support ::construct_at(
      std ::forward<decltype(__this)>(__this),
      std ::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN3FooD1Ev(class Foo* __this) {
  std ::destroy_at(std ::forward<decltype(__this)>(__this));
}
extern "C" class Foo& __rust_thunk___ZN3FooaSERKS_(class Foo* __this,
                                                   const class Foo& __param_0) {
  return __this->operator=(std ::forward<decltype(__param_0)>(__param_0));
}
extern "C" class Foo& __rust_thunk___ZN3FooaSEOS_(class Foo* __this,
                                                  class Foo&& __param_0) {
  return __this->operator=(std ::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___Z3foov() { foo(); }
extern "C" void __rust_thunk___ZN3BarC1Ev(class Bar* __this) {
  rs_api_impl_support ::construct_at(std ::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN3BarC1ERKS_(class Bar* __this,
                                             const class Bar& __param_0) {
  rs_api_impl_support ::construct_at(
      std ::forward<decltype(__this)>(__this),
      std ::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN3BarC1EOS_(class Bar* __this,
                                            class Bar&& __param_0) {
  rs_api_impl_support ::construct_at(
      std ::forward<decltype(__this)>(__this),
      std ::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN3BarD1Ev(class Bar* __this) {
  std ::destroy_at(std ::forward<decltype(__this)>(__this));
}
extern "C" class Bar& __rust_thunk___ZN3BaraSERKS_(class Bar* __this,
                                                   const class Bar& __param_0) {
  return __this->operator=(std ::forward<decltype(__param_0)>(__param_0));
}
extern "C" class Bar& __rust_thunk___ZN3BaraSEOS_(class Bar* __this,
                                                  class Bar&& __param_0) {
  return __this->operator=(std ::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN13HasNoCommentsC1Ev(
    class HasNoComments* __this) {
  rs_api_impl_support ::construct_at(std ::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN13HasNoCommentsC1ERKS_(
    class HasNoComments* __this, const class HasNoComments& __param_0) {
  rs_api_impl_support ::construct_at(
      std ::forward<decltype(__this)>(__this),
      std ::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN13HasNoCommentsC1EOS_(
    class HasNoComments* __this, class HasNoComments&& __param_0) {
  rs_api_impl_support ::construct_at(
      std ::forward<decltype(__this)>(__this),
      std ::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN13HasNoCommentsD1Ev(
    class HasNoComments* __this) {
  std ::destroy_at(std ::forward<decltype(__this)>(__this));
}
extern "C" class HasNoComments& __rust_thunk___ZN13HasNoCommentsaSERKS_(
    class HasNoComments* __this, const class HasNoComments& __param_0) {
  return __this->operator=(std ::forward<decltype(__param_0)>(__param_0));
}
extern "C" class HasNoComments& __rust_thunk___ZN13HasNoCommentsaSEOS_(
    class HasNoComments* __this, class HasNoComments&& __param_0) {
  return __this->operator=(std ::forward<decltype(__param_0)>(__param_0));
}

static_assert(sizeof(class Foo) == 8);
static_assert(alignof(class Foo) == 4);
static_assert(offsetof(class Foo, i) * 8 == 0);
static_assert(offsetof(class Foo, j) * 8 == 32);

static_assert(sizeof(class Bar) == 4);
static_assert(alignof(class Bar) == 4);
static_assert(offsetof(class Bar, i) * 8 == 0);

static_assert(sizeof(class HasNoComments) == 4);
static_assert(alignof(class HasNoComments) == 4);
static_assert(offsetof(class HasNoComments, i) * 8 == 0);

#pragma clang diagnostic pop
