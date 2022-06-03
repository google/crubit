// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BITFIELDS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BITFIELDS_H_

#pragma clang lifetime_elision

struct WithBitfields {
  int f1 : 2;
  int f2;
  int f3 : 4;
  int f4 : 8;
  int : 45;
  int f5;
  int f6 : 23;
  [[no_unique_address]] char f7 = 2;
  int f8 : 2;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BITFIELDS_H_
