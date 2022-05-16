// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_DEFINITION_IN_CC_DEFINITION_IN_CC_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_DEFINITION_IN_CC_DEFINITION_IN_CC_H_

#pragma clang lifetime_elision

template <typename T>
class MyTemplate {
 public:
  static MyTemplate Create(T value);
  const T& value() const;

 private:
  T value_;
};

using MyTypeAlias = MyTemplate<int>;

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_DEFINITION_IN_CC_DEFINITION_IN_CC_H_
