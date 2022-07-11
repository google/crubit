// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_TYPE_ALIAS_TYPE_ALIAS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_TYPE_ALIAS_TYPE_ALIAS_H_

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

using MyTypeAlias = MyTemplate<int>;
using OtherTypeAliasInSameTarget = MyTemplate<int>;

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_TYPE_ALIAS_TYPE_ALIAS_H_
