// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_NO_UNIQUE_ADDRESS_NO_UNIQUE_ADDRESS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_NO_UNIQUE_ADDRESS_NO_UNIQUE_ADDRESS_H_
#pragma clang lifetime_elision

struct Struct final {
  static Struct Make(int f1, char f2) { return Struct{f1, f2}; }
  // Nobody would ever use a no_unique_address int/char field, this is just
  // enough to test that the transmute is correct.
  [[no_unique_address]] int field1 = 1;
  [[no_unique_address]] char field2 = 2;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_NO_UNIQUE_ADDRESS_NO_UNIQUE_ADDRESS_H_
