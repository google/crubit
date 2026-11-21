// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_RECURSIVE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_RECURSIVE_H_

#include "support/annotations.h"

template <typename T>
struct CRUBIT_ALWAYS_INSTANTIATE A final {
  void NoOp() {}

  // `FailMethod` passes initial type checking, so Crubit must check function
  // template bodies recursively to avoid importing `Call_FailMethod`.
  void Call_FailMethod(T t) {
    FailMethod();  // Similarly, this also fails for `FailStaticMethod();`.
  }
  void FailMethod() { static_assert(false); }
  static void FailStaticMethod() { static_assert(false); }
};

using AForInt = A<int>;

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_RECURSIVE_H_
