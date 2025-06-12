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
// TODO(b/233582219): Implement diagnosis of unreachable program points
TEST void nonnullEqualsNullptr(int *_Nonnull X) {
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
// TODO(b/233582219): The pointer is compared to nullptr,
// hence the unannotated pointer should be considered nullable.
TEST void unknownEqualsNullptr(int *X) {
  unknown(X);  // TODO: nullable
  if (X == nullptr) {
    nullable(X);
  } else {
    nonnull(X);
  }
  unknown(X);  // TODO: nullable
}
TEST void nullptrEqualsUnknown(int *X) {
  unknown(X);  // TODO: nullable
  if (nullptr == X) {
    nullable(X);
  } else {
    nonnull(X);
  }
  unknown(X);  // TODO: nullable
}
TEST void unknownNotEqualsNullptr(int *X) {
  unknown(X);  // TODO: nullable
  if (X != nullptr) {
    nonnull(X);
  } else {
    nullable(X);
  }
  unknown(X);  // TODO: nullable
}
TEST void nullptrNotEqualsUnknown(int *X) {
  unknown(X);  // TODO: nullable
  if (nullptr != X) {
    nonnull(X);
  } else {
    nullable(X);
  }
  unknown(X);  // TODO: nullable
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
