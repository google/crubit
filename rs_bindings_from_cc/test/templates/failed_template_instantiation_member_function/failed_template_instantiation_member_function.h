// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_H_

#pragma clang lifetime_elision

struct NoMethod final {};

template <typename T>
struct A final {
  void NoOp(){};
  // It's important that the return type is `auto`, to exercise the return type
  // deduction logic in Crubit's importers/function.cc, which use to cause a
  // CHECK_OK! crash in Crubit.
  // TODO(b/248542210): Note: Changing the return type from `auto` to `int`,
  // i.e., `int CallMethod(T t) { return t.method(); }` still causes Crubit to
  // fail ("no member named 'method' in 'NoMethod'") while it doesn't cause the
  // header to fail to compile.
  auto CallMethod(T t) { return t.method(); }
};

using B = A<NoMethod>;
void inline Func(B a) { a.NoOp(); }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_H_
