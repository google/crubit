// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for pointer arithmetic.

// Note that, for example, the increment and decrement operators do not cause
// their operand to become nonnull, even though we know that they may only be
// applied to nonnull pointers. This is consistent with our treatment of other
// operators, such as `*` and `->`; these also do not cause their operand to
// become nonnull.

#include "nullability_test.h"

// This test intentionally contains violations of `-Wunsafe-buffer-usage`, so
// turn it off.
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wunsafe-buffer-usage"

TEST void preIncrementType(int *_Nonnull NonnullParam,
                           int *_Nullable NullableParam, int *UnknownParam) {
  type<int *_Nonnull>(++NonnullParam);
  type<int *_Nonnull>(++NullableParam);
  type<int *_Nonnull>(++UnknownParam);
}

TEST void preIncrementValue(int *_Nonnull NonnullParam,
                            int *_Nullable NullableParam, int *UnknownParam) {
  int *NewVal = ++NonnullParam;
  provable(NonnullParam == NewVal);
  nonnull(NonnullParam);

  NewVal = ++NullableParam;
  provable(NullableParam == NewVal);
  nonnull(NullableParam);

  NewVal = ++UnknownParam;
  provable(UnknownParam == NewVal);
  nonnull(UnknownParam);
}

TEST void postIncrementType(int *_Nonnull NonnullParam,
                            int *_Nullable NullableParam, int *UnknownParam) {
  type<int *_Nonnull>(NonnullParam++);
  type<int *_Nullable>(NullableParam++);
  type<int *>(UnknownParam++);
}

TEST void postIncrementValue(int *_Nonnull NonnullParam,
                             int *_Nullable NullableParam, int *UnknownParam) {
  int *OldVal = NonnullParam;
  provable(NonnullParam++ == OldVal);
  nonnull(NonnullParam);

  OldVal = NullableParam;
  provable(NullableParam++ == OldVal);
  nonnull(NullableParam);

  OldVal = UnknownParam;
  provable(UnknownParam++ == OldVal);
  nonnull(UnknownParam);
}

TEST void add(int *_Nonnull NonnullParam, int *_Nullable NullableParam,
              int *UnknownParam, int I) {
  type<int *_Nonnull>(NonnullParam + I);
  type<int *_Nonnull>(I + NonnullParam);

  type<int *_Nonnull>(NullableParam + I);
  type<int *_Nonnull>(I + NullableParam);

  type<int *_Nonnull>(UnknownParam + I);
  type<int *_Nonnull>(I + UnknownParam);
}

TEST void subtract(int *_Nonnull NonnullParam, int *_Nullable NullableParam,
                   int *UnknownParam, int I) {
  // Pointer minus integer.
  type<int *_Nonnull>(NonnullParam - I);
  type<int *_Nonnull>(NullableParam - I);
  type<int *_Nonnull>(UnknownParam - I);

  // Pointer minus pointer.
  // Spot-checking just a few of the possible combinations.
  using Ptrdiff = decltype(UnknownParam - UnknownParam);
  type<Ptrdiff>(NonnullParam - NonnullParam);
  type<Ptrdiff>(NonnullParam - NullableParam);
  type<Ptrdiff>(NullableParam - UnknownParam);
}

TEST void addAssignType(int *_Nonnull NonnullParam,
                        int *_Nullable NullableParam, int *UnknownParam,
                        int I) {
  type<int *_Nonnull>(NonnullParam += I);
  type<int *_Nonnull>(NullableParam += I);
  type<int *_Nonnull>(UnknownParam += I);
}

TEST void addAssignValue(int *_Nonnull NonnullParam,
                         int *_Nullable NullableParam, int *UnknownParam,
                         int I) {
  int *NewVal = (NonnullParam += I);
  provable(NonnullParam == NewVal);
  nonnull(NonnullParam);

  NewVal = (NullableParam += I);
  provable(NullableParam == NewVal);
  nonnull(NullableParam);

  NewVal = (UnknownParam += I);
  provable(UnknownParam == NewVal);
  nonnull(UnknownParam);
}

TEST void subtractAssignType(int *_Nonnull NonnullParam,
                             int *_Nullable NullableParam, int *UnknownParam,
                             int I) {
  type<int *_Nonnull>(NonnullParam -= I);
  type<int *_Nonnull>(NullableParam -= I);
  type<int *_Nonnull>(UnknownParam -= I);
}

TEST void subtractAssignValue(int *_Nonnull NonnullParam,
                              int *_Nullable NullableParam, int *UnknownParam,
                              int I) {
  int *NewVal = (NonnullParam -= I);
  provable(NonnullParam == NewVal);
  nonnull(NonnullParam);

  NewVal = (NullableParam -= I);
  provable(NullableParam == NewVal);
  nonnull(NullableParam);

  NewVal = (UnknownParam -= I);
  provable(UnknownParam == NewVal);
  nonnull(UnknownParam);
}

#pragma clang diagnostic pop
