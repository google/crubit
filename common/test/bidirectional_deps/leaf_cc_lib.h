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

// Move-only C++ type with non-trivial destructor and move constructor.
struct MoveOnlyCcType final {
  int val = 0;

  explicit MoveOnlyCcType(int v) : val(v) {}

  // Explicitly provide a user provided destructor to force a Drop impl.
  ~MoveOnlyCcType() {}  // NOLINT(modernize-use-equals-default)

  MoveOnlyCcType(MoveOnlyCcType&&) = default;
  MoveOnlyCcType& operator=(MoveOnlyCcType&&) = default;

  MoveOnlyCcType(const MoveOnlyCcType&) = delete;
  MoveOnlyCcType& operator=(const MoveOnlyCcType&) = delete;
};

inline MoveOnlyCcType MakeMoveOnly(int val) { return MoveOnlyCcType(val); }

inline int InspectMoveOnly(const MoveOnlyCcType& x) { return x.val; }
}  // namespace crubit

#endif  // CRUBIT_COMMON_TEST_BIDIRECTIONAL_DEPS_LEAF_CC_LIB_H_
