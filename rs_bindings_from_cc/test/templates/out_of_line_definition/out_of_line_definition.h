// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_OUT_OF_LINE_DEFINITION_OUT_OF_LINE_DEFINITION_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_OUT_OF_LINE_DEFINITION_OUT_OF_LINE_DEFINITION_H_

#pragma clang lifetime_elision

template <typename T>
class MyTemplate final {
 public:
  static MyTemplate Create(T value);
  const T& value() const;

 private:
  T value_;
};

template <typename T>
MyTemplate<T> MyTemplate<T>::Create(T value) {
  MyTemplate<T> result;
  result.value_ = value;
  return result;
}

template <typename T>
const T& MyTemplate<T>::value() const {
  return value_;
}

using MyTypeAlias = MyTemplate<int>;

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_OUT_OF_LINE_DEFINITION_OUT_OF_LINE_DEFINITION_H_
