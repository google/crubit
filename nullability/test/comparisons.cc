// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for comparisons of types containing nullability annotations.

#include "nullability_test.h"

// nonnull vs nonnull
TEST void nonnullEqualsNonnull(int *_Nonnull X, int *_Nonnull Y) {
  nonnull(X);
  nonnull(Y);
  if (X == Y) {
    nonnull(X);
    nonnull(Y);
  } else {
    nonnull(X);
    nonnull(Y);
  }
  nonnull(X);
  nonnull(Y);
}
TEST void nonnullNotEqualsNonnull(int *_Nonnull X, int *_Nonnull Y) {
  nonnull(X);
  nonnull(Y);
  if (X != Y) {
    nonnull(X);
    nonnull(Y);
  } else {
    nonnull(X);
    nonnull(Y);
  }
  nonnull(X);
  nonnull(Y);
}

// nullable vs nullable
TEST void nullableEqualsNullable(int *_Nullable X, int *_Nullable Y) {
  nullable(X);
  nullable(Y);
  if (X == Y) {
    nullable(X);
    nullable(Y);
  } else {
    nullable(X);
    nullable(Y);
  }
  nullable(X);
  nullable(Y);
}
TEST void nullableNotEqualsNullable(int *_Nullable X, int *_Nullable Y) {
  nullable(X);
  nullable(Y);
  if (X != Y) {
    nullable(X);
    nullable(Y);
  } else {
    nullable(X);
    nullable(Y);
  }
  nullable(X);
  nullable(Y);
}

// unknown vs unknown
TEST void unknownEqualsUnknown(int *X, int *Y) {
  unknown(X);
  unknown(Y);
  if (X == Y) {
    unknown(X);
    unknown(Y);
  } else {
    unknown(X);
    unknown(Y);
  }
  unknown(X);
  unknown(Y);
}
TEST void unknownNotEqualsUnknown(int *X, int *Y) {
  unknown(X);
  unknown(Y);
  if (X != Y) {
    unknown(X);
    unknown(Y);
  } else {
    unknown(X);
    unknown(Y);
  }
  unknown(X);
  unknown(Y);
}

// nonnull vs nullptr
// NOTE: The following examples involve unreachable code, in the eye of the
// analyzer, since `X` is `_Nonnull` (assumes no contract violations at
// runtime). For unreachable code, the analyzer's environment will have
// unsatisfiable flow conditions, which allow the analyzer to prove anything,
// including that the value nullability is nonnull.
TEST void nonnullEqualsNullptr(int* _Nonnull X) {
  nonnull(X);
  if (X == nullptr) {
    nonnull(X);  // unreachable
  } else {
    nonnull(X);
  }
  nonnull(X);
}
TEST void nullptrEqualsNonnull(int *_Nonnull X) {
  nonnull(X);
  if (nullptr == X) {
    nonnull(X);  // unreachable
  } else {
    nonnull(X);
  }
  nonnull(X);
}
TEST void nonnullNotEqualsNullptr(int *_Nonnull X) {
  nonnull(X);
  if (X != nullptr) {
    nonnull(X);
  } else {
    nonnull(X);  // unreachable
  }
  nonnull(X);
}
TEST void nullptrNotEqualsNonnull(int *_Nonnull X) {
  nonnull(X);
  if (nullptr != X) {
    nonnull(X);
  } else {
    nonnull(X);  // unreachable
  }
  nonnull(X);
}

// nullable vs nullptr
TEST void nullableEqualsNullptr(int *_Nullable X) {
  nullable(X);
  if (X == nullptr) {
    nullable(X);
  } else {
    nonnull(X);
  }
  nullable(X);
}
TEST void nullptrEqualsNullable(int *_Nullable X) {
  nullable(X);
  if (nullptr == X) {
    nullable(X);
  } else {
    nonnull(X);
  }
  nullable(X);
}
TEST void nullableNotEqualsNullptr(int *_Nullable X) {
  nullable(X);
  if (X != nullptr) {
    nonnull(X);
  } else {
    nullable(X);
  }
  nullable(X);
}
TEST void nullptrNotEqualsNullable(int *_Nullable X) {
  nullable(X);
  if (nullptr != X) {
    nonnull(X);
  } else {
    nullable(X);
  }
  nullable(X);
}

// nullable vs nonnull
TEST void nullableEqualsNonnull(int *_Nullable X, int *_Nonnull Y) {
  nullable(X);
  nonnull(Y);
  if (X == Y) {
    nonnull(X);
    nonnull(Y);
  } else {
    nullable(X);
    nonnull(Y);
  }
  nullable(X);
  nonnull(Y);
}
TEST void nonnullEqualsNullable(int *_Nullable X, int *_Nonnull Y) {
  nullable(X);
  nonnull(Y);
  if (Y == X) {
    nonnull(X);
    nonnull(Y);
  } else {
    nullable(X);
    nonnull(Y);
  }
  nullable(X);
  nonnull(Y);
}
TEST void nullableNotEqualsNonnull(int *_Nullable X, int *_Nonnull Y) {
  nullable(X);
  nonnull(Y);
  if (X != Y) {
    nullable(X);
    nonnull(Y);
  } else {
    nonnull(X);
    nonnull(Y);
  }
  nullable(X);
  nonnull(Y);
}
TEST void nonnullNotEqualsNullable(int *_Nullable X, int *_Nonnull Y) {
  nullable(X);
  nonnull(Y);
  if (Y != X) {
    nullable(X);
    nonnull(Y);
  } else {
    nonnull(X);
    nonnull(Y);
  }
  nullable(X);
  nonnull(Y);
}

// nullable vs unknown
TEST void nullableEqualsUnknown(int *_Nullable X, int *Y) {
  nullable(X);
  unknown(Y);
  if (X == Y) {
    nullable(X);
    unknown(Y);
  } else {
    nullable(X);
    unknown(Y);
  }
  nullable(X);
  unknown(Y);
}
TEST void unknownEqualsNullable(int *_Nullable X, int *Y) {
  nullable(X);
  unknown(Y);
  if (Y == X) {
    nullable(X);
    unknown(Y);
  } else {
    nullable(X);
    unknown(Y);
  }
  nullable(X);
  unknown(Y);
}
TEST void nullableNotEqualsUnknown(int *_Nullable X, int *Y) {
  nullable(X);
  unknown(Y);
  if (X != Y) {
    nullable(X);
    unknown(Y);
  } else {
    nullable(X);
    unknown(Y);
  }
  nullable(X);
  unknown(Y);
}
TEST void unknownNotEqualsNullable(int *_Nullable X, int *Y) {
  nullable(X);
  unknown(Y);
  if (Y != X) {
    nullable(X);
    unknown(Y);
  } else {
    nullable(X);
    unknown(Y);
  }
  nullable(X);
  unknown(Y);
}

// unknown vs nullptr
// NOTE: We don't promote an unknown pointer to nullable based on a null check.
// The pointer could still be non-null, but with a defensive/redundant check.
// Thus, the pointer is still unknown before the null check, or after the join
// point, in the following examples.
TEST void unknownEqualsNullptr(int* X) {
  unknown(X);
  if (X == nullptr) {
    nullable(X);
  } else {
    nonnull(X);
  }
  unknown(X);
}
TEST void nullptrEqualsUnknown(int *X) {
  unknown(X);
  if (nullptr == X) {
    nullable(X);
  } else {
    nonnull(X);
  }
  unknown(X);
}
TEST void unknownNotEqualsNullptr(int *X) {
  unknown(X);
  if (X != nullptr) {
    nonnull(X);
  } else {
    nullable(X);
  }
  unknown(X);
}
TEST void nullptrNotEqualsUnknown(int *X) {
  unknown(X);
  if (nullptr != X) {
    nonnull(X);
  } else {
    nullable(X);
  }
  unknown(X);
}

// unknown vs nonnull
TEST void unknownEqualsNonnull(int *X, int *_Nonnull Y) {
  unknown(X);
  nonnull(Y);
  if (X == Y) {
    nonnull(X);
    nonnull(Y);
  } else {
    unknown(X);
    nonnull(Y);
  }
  unknown(X);
  nonnull(Y);
}
TEST void nonnullEqualsUnknown(int *X, int *_Nonnull Y) {
  unknown(X);
  nonnull(Y);
  if (Y == X) {
    nonnull(X);
    nonnull(Y);
  } else {
    unknown(X);
    nonnull(Y);
  }
  unknown(X);
  nonnull(Y);
}
TEST void unknownNotEqualsNonnull(int *X, int *_Nonnull Y) {
  unknown(X);
  nonnull(Y);
  if (X != Y) {
    unknown(X);
    nonnull(Y);
  } else {
    nonnull(X);
    nonnull(Y);
  }
  unknown(X);
  nonnull(Y);
}
TEST void nonnullNotEqualsUnknown(int *X, int *_Nonnull Y) {
  unknown(X);
  nonnull(Y);
  if (Y != X) {
    unknown(X);
    nonnull(Y);
  } else {
    nonnull(X);
    nonnull(Y);
  }
  unknown(X);
  nonnull(Y);
}
