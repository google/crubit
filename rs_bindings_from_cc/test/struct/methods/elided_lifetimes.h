// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_METHODS_ELIDED_LIFETIMES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_METHODS_ELIDED_LIFETIMES_H_

#pragma clang lifetime_elision

struct ElidedLifetimes {
  int get_int_field() const;
  void set_int_field(int new_value);

  inline int inline_get_int_field() const { return int_field; }
  inline void inline_set_int_field(int new_value) { int_field = new_value; }

  int int_field;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_METHODS_ELIDED_LIFETIMES_H_
