// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_H_

struct NoMethod final {};

template <typename T>
struct A final {
  void NoOp(){};
  // It's important that the return type is `auto`, to exercise the return type
  // deduction logic in Crubit's importers/function.cc, which causes the build
  // failure.
  auto CallMethod(T t) { return t.method(); }
};

void inline Func(A<NoMethod> a) { a.NoOp(); }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_H_
