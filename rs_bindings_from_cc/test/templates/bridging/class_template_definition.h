// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_BRIDGING_CLASS_TEMPLATE_DEFINITION_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_BRIDGING_CLASS_TEMPLATE_DEFINITION_H_

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

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_BRIDGING_CLASS_TEMPLATE_DEFINITION_H_
