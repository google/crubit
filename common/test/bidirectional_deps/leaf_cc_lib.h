// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_COMMON_TEST_BIDIRECTIONAL_DEPS_LEAF_CC_LIB_H_
#define CRUBIT_COMMON_TEST_BIDIRECTIONAL_DEPS_LEAF_CC_LIB_H_

#pragma clang lifetime_elision

namespace crubit {

struct LeafCcType final {
  unsigned char field;
};

inline LeafCcType Wrap(unsigned char x) { return LeafCcType{x}; }

inline unsigned char Unwrap(LeafCcType x) { return x.field; }

enum class LeafCcEnum { kUnknown, kValue1, kValue2, kValue3 };

inline LeafCcEnum WrapEnum(unsigned char x) {
  return static_cast<LeafCcEnum>(x);
}

inline unsigned char UnwrapEnum(LeafCcEnum x) {
  return static_cast<unsigned char>(x);
}

using LeafCcTypeAlias = LeafCcType;
}  // namespace crubit

#endif  // CRUBIT_COMMON_TEST_BIDIRECTIONAL_DEPS_LEAF_CC_LIB_H_
