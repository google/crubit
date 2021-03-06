// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_TYPE_ALIAS_TYPE_ALIAS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_TYPE_ALIAS_TYPE_ALIAS_H_

#pragma clang lifetime_elision

template <typename T1, typename T2>
struct TemplateWithTwoParams final {
  T1 value1;
  T2 value2;
};

using AliasToTemplateWithTwoParams = TemplateWithTwoParams<int, float>;

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_TYPE_ALIAS_TYPE_ALIAS_H_
