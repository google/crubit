// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_METHODS_METHODS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_METHODS_METHODS_H_

class SomeClass final {
 public:
  static SomeClass static_factory_method(int int_var_initial_value);
  static int static_method_that_multiplies_its_args(int x, int y);
  int int_var;

  // Using an `inline` method forces generation of a C++ thunk in
  // methods_rs_api_impl.cc (helping add test coverage for such thunks).
  static inline int static_inline_method(int arg) { return arg * 42; }

 protected:
  static inline int protected_static_inline_method(int arg) { return arg * 42; }

 private:
  static inline int private_static_inline_method(int arg) { return arg * 42; }
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_METHODS_METHODS_H_
