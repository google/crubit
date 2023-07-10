// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_COMMON_TEST_BIDIRECTIONAL_DEPS_LEAF_CC_LIB_H_
#define CRUBIT_COMMON_TEST_BIDIRECTIONAL_DEPS_LEAF_CC_LIB_H_

struct LeafCcType {
  unsigned char field;
};

LeafCcType Wrap(unsigned char x) { return LeafCcType{x}; }

unsigned char Unwrap(LeafCcType x) { return x.field; }

#endif  // CRUBIT_COMMON_TEST_BIDIRECTIONAL_DEPS_LEAF_CC_LIB_H_
