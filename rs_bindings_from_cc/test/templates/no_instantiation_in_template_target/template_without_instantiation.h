// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_NO_INSTANTIATION_IN_TEMPLATE_TARGET_TEMPLATE_WITHOUT_INSTANTIATION_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_NO_INSTANTIATION_IN_TEMPLATE_TARGET_TEMPLATE_WITHOUT_INSTANTIATION_H_

#pragma clang lifetime_elision

// This template is not instantiated anywhere in this header file
// (this is what the test scenario exercised here cares about).
template <typename T>
class MyTemplate {
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

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_NO_INSTANTIATION_IN_TEMPLATE_TARGET_TEMPLATE_WITHOUT_INSTANTIATION_H_
