// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_STATIC_METHODS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_STATIC_METHODS_H_

class SomeClass final {
 public:
  // Example of a factory method.
  static SomeClass static_factory_method(int initial_value_of_field);

  // Static method working on primitive types (and unrelated to the struct).
  static int static_method_that_multiplies_its_args(int x, int y);

 private:
  int field_;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_STATIC_METHODS_H_
