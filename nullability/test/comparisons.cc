// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for comparisons of types containing nullability annotations.

#include "nullability_test.h"

// nonnull vs nonnull
TEST void nonnullEqualsNonnull(Nonnull<int *> X, Nonnull<int *> Y) {
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
TEST void nonnullNotEqualsNonnull(Nonnull<int *> X, Nonnull<int *> Y) {
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
TEST void nullableEqualsNullable(Nullable<int *> X, Nullable<int *> Y) {
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
TEST void nullableNotEqualsNullable(Nullable<int *> X, Nullable<int *> Y) {
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
TEST void nonnullEqualsNullptr(Nonnull<int *> X) {
  nonnull(X);
  if (X == nullptr) {
    nonnull(X);  // unreachable
  } else {
    nonnull(X);
  }
  nonnull(X);
}
TEST void nullptrEqualsNonnull(Nonnull<int *> X) {
  nonnull(X);
  if (nullptr == X) {
    nonnull(X);  // unreachable
  } else {
    nonnull(X);
  }
  nonnull(X);
}
TEST void nonnullNotEqualsNullptr(Nonnull<int *> X) {
  nonnull(X);
  if (X != nullptr) {
    nonnull(X);
  } else {
    nonnull(X);  // unreachable
  }
  nonnull(X);
}
TEST void nullptrNotEqualsNonnull(Nonnull<int *> X) {
  nonnull(X);
  if (nullptr != X) {
    nonnull(X);
  } else {
    nonnull(X);  // unreachable
  }
  nonnull(X);
}

// nullable vs nullptr
TEST void nullableEqualsNullptr(Nullable<int *> X) {
  nullable(X);
  if (X == nullptr) {
    nullable(X);
  } else {
    nonnull(X);
  }
  nullable(X);
}
TEST void nullptrEqualsNullable(Nullable<int *> X) {
  nullable(X);
  if (nullptr == X) {
    nullable(X);
  } else {
    nonnull(X);
  }
  nullable(X);
}
TEST void nullableNotEqualsNullptr(Nullable<int *> X) {
  nullable(X);
  if (X != nullptr) {
    nonnull(X);
  } else {
    nullable(X);
  }
  nullable(X);
}
TEST void nullptrNotEqualsNullable(Nullable<int *> X) {
  nullable(X);
  if (nullptr != X) {
    nonnull(X);
  } else {
    nullable(X);
  }
  nullable(X);
}

// nullable vs nonnull
TEST void nullableEqualsNonnull(Nullable<int *> X, Nonnull<int *> Y) {
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
TEST void nonnullEqualsNullable(Nullable<int *> X, Nonnull<int *> Y) {
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
TEST void nullableNotEqualsNonnull(Nullable<int *> X, Nonnull<int *> Y) {
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
TEST void nonnullNotEqualsNullable(Nullable<int *> X, Nonnull<int *> Y) {
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
TEST void nullableEqualsUnknown(Nullable<int *> X, int *Y) {
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
TEST void unknownEqualsNullable(Nullable<int *> X, int *Y) {
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
TEST void nullableNotEqualsUnknown(Nullable<int *> X, int *Y) {
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
TEST void unknownNotEqualsNullable(Nullable<int *> X, int *Y) {
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
TEST void unknownEqualsNonnull(int *X, Nonnull<int *> Y) {
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
TEST void nonnullEqualsUnknown(int *X, Nonnull<int *> Y) {
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
TEST void unknownNotEqualsNonnull(int *X, Nonnull<int *> Y) {
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
TEST void nonnullNotEqualsUnknown(int *X, Nonnull<int *> Y) {
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
