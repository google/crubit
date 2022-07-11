// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FUNC_RETURN_AND_PARAM_TYPES_FUNC_RETURN_AND_PARAM_TYPES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FUNC_RETURN_AND_PARAM_TYPES_FUNC_RETURN_AND_PARAM_TYPES_H_

#pragma clang lifetime_elision

template <typename T>
class MyTemplate final {
 public:
  static MyTemplate Create(T value) {
    MyTemplate result;
    result.value_ = value;
    return result;
  }

  const T& value() const { return value_; }

 private:
  T value_;
};

MyTemplate<int> CreateInstanceOfMyTemplate(int value);

int DoubleInstanceOfMyTemplate(const MyTemplate<int>& my_template);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FUNC_RETURN_AND_PARAM_TYPES_FUNC_RETURN_AND_PARAM_TYPES_H_
