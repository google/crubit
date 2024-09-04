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

TEST void preIncrementType(Nonnull<int *> NonnullParam,
                           Nullable<int *> NullableParam, int *UnknownParam) {
  type<Nonnull<int *>>(++NonnullParam);
  type<Nonnull<int *>>(++NullableParam);
  type<Nonnull<int *>>(++UnknownParam);
}

TEST void preIncrementValue(Nonnull<int *> NonnullParam,
                            Nullable<int *> NullableParam, int *UnknownParam) {
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

TEST void postIncrementType(Nonnull<int *> NonnullParam,
                            Nullable<int *> NullableParam, int *UnknownParam) {
  type<Nonnull<int *>>(NonnullParam++);
  type<Nullable<int *>>(NullableParam++);
  type<int *>(UnknownParam++);
}

TEST void postIncrementValue(Nonnull<int *> NonnullParam,
                             Nullable<int *> NullableParam, int *UnknownParam) {
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

TEST void add(Nonnull<int *> NonnullParam, Nullable<int *> NullableParam,
              int *UnknownParam, int I) {
  type<Nonnull<int *>>(NonnullParam + I);
  type<Nonnull<int *>>(I + NonnullParam);

  type<Nonnull<int *>>(NullableParam + I);
  type<Nonnull<int *>>(I + NullableParam);

  type<Nonnull<int *>>(UnknownParam + I);
  type<Nonnull<int *>>(I + UnknownParam);
}

TEST void subtract(Nonnull<int *> NonnullParam, Nullable<int *> NullableParam,
                   int *UnknownParam, int I) {
  type<Nonnull<int *>>(NonnullParam - I);
  type<Nonnull<int *>>(NullableParam - I);
  type<Nonnull<int *>>(UnknownParam - I);
}

TEST void addAssignType(Nonnull<int *> NonnullParam,
                        Nullable<int *> NullableParam, int *UnknownParam,
                        int I) {
  type<Nonnull<int *>>(NonnullParam += I);
  type<Nonnull<int *>>(NullableParam += I);
  type<Nonnull<int *>>(UnknownParam += I);
}

TEST void addAssignValue(Nonnull<int *> NonnullParam,
                         Nullable<int *> NullableParam, int *UnknownParam,
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

TEST void subtractAssignType(Nonnull<int *> NonnullParam,
                             Nullable<int *> NullableParam, int *UnknownParam,
                             int I) {
  type<Nonnull<int *>>(NonnullParam -= I);
  type<Nonnull<int *>>(NullableParam -= I);
  type<Nonnull<int *>>(UnknownParam -= I);
}

TEST void subtractAssignValue(Nonnull<int *> NonnullParam,
                              Nullable<int *> NullableParam, int *UnknownParam,
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
