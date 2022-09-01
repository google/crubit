// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/namespace.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN23test_namespace_bindings1SC1Ev(
    struct test_namespace_bindings::S* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN23test_namespace_bindings1SC1EOS0_(
    struct test_namespace_bindings::S* __this,
    struct test_namespace_bindings::S* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings15inline_functionEv() {
  test_namespace_bindings::inline_function();
}
extern "C" void __rust_thunk___ZN32test_namespace_bindings_reopened5inner1SC1Ev(
    struct test_namespace_bindings_reopened::inner::S* __this) {
  crubit::construct_at(__this);
}
extern "C" void
__rust_thunk___ZN32test_namespace_bindings_reopened5inner1SC1EOS1_(
    struct test_namespace_bindings_reopened::inner::S* __this,
    struct test_namespace_bindings_reopened::inner::S* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void
__rust_thunk___ZN30test_namespace_bindings_inline5inner23StructInInlineNamespaceC1Ev(
    struct test_namespace_bindings_inline::inner::StructInInlineNamespace*
        __this) {
  crubit::construct_at(__this);
}
extern "C" void
__rust_thunk___ZN30test_namespace_bindings_inline5inner23StructInInlineNamespaceC1EOS1_(
    struct test_namespace_bindings_inline::inner::StructInInlineNamespace*
        __this,
    struct test_namespace_bindings_inline::inner::StructInInlineNamespace*
        __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN4impl3fooEv() { impl::foo(); }

static_assert(sizeof(struct test_namespace_bindings::S) == 4);
static_assert(alignof(struct test_namespace_bindings::S) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct test_namespace_bindings::S) == 0);

static_assert(sizeof(struct test_namespace_bindings_reopened::inner::S) == 1);
static_assert(alignof(struct test_namespace_bindings_reopened::inner::S) == 1);

static_assert(sizeof(struct test_namespace_bindings_inline::inner::
                         StructInInlineNamespace) == 1);
static_assert(alignof(struct test_namespace_bindings_inline::inner::
                          StructInInlineNamespace) == 1);

#pragma clang diagnostic pop
