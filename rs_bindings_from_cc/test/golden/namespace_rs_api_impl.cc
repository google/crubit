// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:namespace_cc
// Features: experimental, extern_c, supported

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

extern "C" void __rust_thunk___ZN23test_namespace_bindings1SC1Ev(
    struct test_namespace_bindings::S* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN23test_namespace_bindings1SC1EOS0_(
    struct test_namespace_bindings::S* __this,
    struct test_namespace_bindings::S* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct test_namespace_bindings::S*
__rust_thunk___ZN23test_namespace_bindings1SaSERKS0_(
    struct test_namespace_bindings::S* __this,
    const struct test_namespace_bindings::S* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct test_namespace_bindings::S*
__rust_thunk___ZN23test_namespace_bindings1SaSEOS0_(
    struct test_namespace_bindings::S* __this,
    struct test_namespace_bindings::S* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" int __rust_thunk___ZN23test_namespace_bindings1fENS_1SE(
    struct test_namespace_bindings::S* s) {
  return test_namespace_bindings::f(std::move(*s));
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings15inline_functionEv() {
  test_namespace_bindings::inline_function();
}

extern "C" void __rust_thunk___Z8identityN23test_namespace_bindings1SE(
    struct test_namespace_bindings::S* __return,
    struct test_namespace_bindings::S* s) {
  new (__return) auto(identity(std::move(*s)));
}

static_assert(sizeof(struct test_namespace_bindings_reopened::inner::S) == 1);
static_assert(alignof(struct test_namespace_bindings_reopened::inner::S) == 1);

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

extern "C" struct test_namespace_bindings_reopened::inner::S*
__rust_thunk___ZN32test_namespace_bindings_reopened5inner1SaSERKS1_(
    struct test_namespace_bindings_reopened::inner::S* __this,
    const struct test_namespace_bindings_reopened::inner::S* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct test_namespace_bindings_reopened::inner::S*
__rust_thunk___ZN32test_namespace_bindings_reopened5inner1SaSEOS1_(
    struct test_namespace_bindings_reopened::inner::S* __this,
    struct test_namespace_bindings_reopened::inner::S* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" void
__rust_thunk___ZN32test_namespace_bindings_reopened5inner1zENS0_1SE(
    struct test_namespace_bindings_reopened::inner::S* s) {
  test_namespace_bindings_reopened::inner::z(std::move(*s));
}

static_assert(sizeof(struct test_namespace_bindings_inline::inner::
                         StructInInlineNamespace) == 1);
static_assert(alignof(struct test_namespace_bindings_inline::inner::
                          StructInInlineNamespace) == 1);

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

extern "C" struct test_namespace_bindings_inline::inner::StructInInlineNamespace*
__rust_thunk___ZN30test_namespace_bindings_inline5inner23StructInInlineNamespaceaSERKS1_(
    struct test_namespace_bindings_inline::inner::StructInInlineNamespace*
        __this,
    const struct test_namespace_bindings_inline::inner::StructInInlineNamespace*
        __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct test_namespace_bindings_inline::inner::StructInInlineNamespace*
__rust_thunk___ZN30test_namespace_bindings_inline5inner23StructInInlineNamespaceaSEOS1_(
    struct test_namespace_bindings_inline::inner::StructInInlineNamespace*
        __this,
    struct test_namespace_bindings_inline::inner::StructInInlineNamespace*
        __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" void
__rust_thunk___Z43useStructInInlineNamespaceWithFullQualifierN30test_namespace_bindings_inline5inner23StructInInlineNamespaceE(
    struct test_namespace_bindings_inline::inner::StructInInlineNamespace* s) {
  useStructInInlineNamespaceWithFullQualifier(std::move(*s));
}

extern "C" void
__rust_thunk___Z45useStructInInlineNamespaceSkipInlineQualifierN30test_namespace_bindings_inline5inner23StructInInlineNamespaceE(
    struct test_namespace_bindings_inline::inner::StructInInlineNamespace* s) {
  useStructInInlineNamespaceSkipInlineQualifier(std::move(*s));
}

extern "C" void __rust_thunk___ZN4impl3fooEv() { impl::foo(); }

#pragma clang diagnostic pop
