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

// This is a regression test for b/283835873 where the alignment of the
// generated struct was wrong/missing.
struct AlignmentRegressionTest {
  char32_t code_point : 31;
  enum : char32_t { ok = 0, error = 1 } status : 1;
};
static_assert(sizeof(AlignmentRegressionTest) == sizeof(char32_t));
static_assert(sizeof(AlignmentRegressionTest) == 4);
static_assert(alignof(AlignmentRegressionTest) == alignof(char32_t));
static_assert(alignof(AlignmentRegressionTest) == 4);

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BITFIELDS_H_
