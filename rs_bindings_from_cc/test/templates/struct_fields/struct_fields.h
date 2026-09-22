// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_STRUCT_FIELDS_STRUCT_FIELDS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_STRUCT_FIELDS_STRUCT_FIELDS_H_

#include "support/annotations.h"

template <typename T>
class CRUBIT_ALWAYS_INSTANTIATE MyTemplate {
 public:
  explicit MyTemplate(T value) : value_(value) {}
  const T& value() const { return value_; }

 private:
  T value_;
};

struct MyStruct {
  MyTemplate<int> public_field;
};

class OuterWithShadowedType {
 public:
  struct Shadowed {
    int x;
  };
  using Action = MyTemplate<Shadowed>;

  static OuterWithShadowedType Shadowed(int x) {
    return OuterWithShadowedType();
  }
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_STRUCT_FIELDS_STRUCT_FIELDS_H_
