// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:namespace_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/namespace.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct test_namespace_bindings::S) == 4);
static_assert(alignof(struct test_namespace_bindings::S) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct test_namespace_bindings::S) == 0);

extern "C" int __rust_thunk___ZN23test_namespace_bindings1fENS_1SE(
    struct test_namespace_bindings::S* s) {
  return test_namespace_bindings::f(std::move(*s));
}

static_assert(
    (int (*)(struct test_namespace_bindings::S))&test_namespace_bindings::f);

extern "C" void
__rust_thunk___ZN23test_namespace_bindings15inline_functionEv() {
  test_namespace_bindings::inline_function();
}

static_assert((void (*)())&test_namespace_bindings::inline_function);

static_assert((void (*)())&test_namespace_bindings::inner::i);

extern "C" void __rust_thunk___Z8identityN23test_namespace_bindings1SE(
    struct test_namespace_bindings::S* __return,
    struct test_namespace_bindings::S* s) {
  new (__return) auto(identity(std::move(*s)));
}

static_assert((struct test_namespace_bindings::S (*)(
    struct test_namespace_bindings::S))&identity);

static_assert((void (*)())&test_namespace_bindings_reopened::x);

static_assert(sizeof(struct test_namespace_bindings_reopened::inner::S) == 1);
static_assert(alignof(struct test_namespace_bindings_reopened::inner::S) == 1);

static_assert((void (*)())&test_namespace_bindings_reopened::y);

extern "C" void
__rust_thunk___ZN32test_namespace_bindings_reopened5inner1zENS0_1SE(
    struct test_namespace_bindings_reopened::inner::S* s) {
  test_namespace_bindings_reopened::inner::z(std::move(*s));
}

static_assert((void (*)(struct test_namespace_bindings_reopened::inner::
                            S))&test_namespace_bindings_reopened::inner::z);

static_assert(sizeof(struct test_namespace_bindings_inline::inner::
                         StructInInlineNamespace) == 1);
static_assert(alignof(struct test_namespace_bindings_inline::inner::
                          StructInInlineNamespace) == 1);

extern "C" void
__rust_thunk___Z43useStructInInlineNamespaceWithFullQualifierN30test_namespace_bindings_inline5inner23StructInInlineNamespaceE(
    struct test_namespace_bindings_inline::inner::StructInInlineNamespace* s) {
  useStructInInlineNamespaceWithFullQualifier(std::move(*s));
}

static_assert((void (*)(
    struct test_namespace_bindings_inline::inner::
        StructInInlineNamespace))&useStructInInlineNamespaceWithFullQualifier);

extern "C" void
__rust_thunk___Z45useStructInInlineNamespaceSkipInlineQualifierN30test_namespace_bindings_inline5inner23StructInInlineNamespaceE(
    struct test_namespace_bindings_inline::inner::StructInInlineNamespace* s) {
  useStructInInlineNamespaceSkipInlineQualifier(std::move(*s));
}

static_assert((void (*)(
    struct test_namespace_bindings_inline::inner::
        StructInInlineNamespace))&useStructInInlineNamespaceSkipInlineQualifier);

extern "C" void __rust_thunk___ZN4impl3fooEv() { impl::foo(); }

static_assert((void (*)())&impl::foo);

#pragma clang diagnostic pop
