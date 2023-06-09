// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for comparisons of types containing nullability annotations.

#include "nullability_test.h"

// nonnull vs nonnull
TEST void nonnullEqualsNonnull(Nonnull<int*> x, Nonnull<int*> y) {
  nonnull(x);
  nonnull(y);
  if (x == y) {
    nonnull(x);
    nonnull(y);
  } else {
    nonnull(x);
    nonnull(y);
  }
  nonnull(x);
  nonnull(y);
}
TEST void nonnullNotEqualsNonnull(Nonnull<int*> x, Nonnull<int*> y) {
  nonnull(x);
  nonnull(y);
  if (x != y) {
    nonnull(x);
    nonnull(y);
  } else {
    nonnull(x);
    nonnull(y);
  }
  nonnull(x);
  nonnull(y);
}

// nullable vs nullable
TEST void nullableEqualsNullable(Nullable<int*> x, Nullable<int*> y) {
  nullable(x);
  nullable(y);
  if (x == y) {
    nullable(x);
    nullable(y);
  } else {
    nullable(x);
    nullable(y);
  }
  nullable(x);
  nullable(y);
}
TEST void nullableNotEqualsNullable(Nullable<int*> x, Nullable<int*> y) {
  nullable(x);
  nullable(y);
  if (x != y) {
    nullable(x);
    nullable(y);
  } else {
    nullable(x);
    nullable(y);
  }
  nullable(x);
  nullable(y);
}

// unknown vs unknown
TEST void unknownEqualsUnknown(int* x, int* y) {
  unknown(x);
  unknown(y);
  if (x == y) {
    unknown(x);
    unknown(y);
  } else {
    unknown(x);
    unknown(y);
  }
  unknown(x);
  unknown(y);
}
TEST void unknownNotEqualsUnknown(int* x, int* y) {
  unknown(x);
  unknown(y);
  if (x != y) {
    unknown(x);
    unknown(y);
  } else {
    unknown(x);
    unknown(y);
  }
  unknown(x);
  unknown(y);
}

// nonnull vs nullptr
// TODO(b/233582219): Implement diagnosis of unreachable program points
TEST void nonnullEqualsNullptr(Nonnull<int*> x) {
  nonnull(x);
  if (x == nullptr) {
    nonnull(x);  // unreachable
  } else {
    nonnull(x);
  }
  nonnull(x);
}
TEST void nullptrEqualsNonnull(Nonnull<int*> x) {
  nonnull(x);
  if (nullptr == x) {
    nonnull(x);  // unreachable
  } else {
    nonnull(x);
  }
  nonnull(x);
}
TEST void nonnullNotEqualsNullptr(Nonnull<int*> x) {
  nonnull(x);
  if (x != nullptr) {
    nonnull(x);
  } else {
    nonnull(x);  // unreachable
  }
  nonnull(x);
}
TEST void nullptrNotEqualsNonnull(Nonnull<int*> x) {
  nonnull(x);
  if (nullptr != x) {
    nonnull(x);
  } else {
    nonnull(x);  // unreachable
  }
  nonnull(x);
}

// nullable vs nullptr
TEST void nullableEqualsNullptr(Nullable<int*> x) {
  nullable(x);
  if (x == nullptr) {
    nullable(x);
  } else {
    nonnull(x);
  }
  nullable(x);
}
TEST void nullptrEqualsNullable(Nullable<int*> x) {
  nullable(x);
  if (nullptr == x) {
    nullable(x);
  } else {
    nonnull(x);
  }
  nullable(x);
}
TEST void nullableNotEqualsNullptr(Nullable<int*> x) {
  nullable(x);
  if (x != nullptr) {
    nonnull(x);
  } else {
    nullable(x);
  }
  nullable(x);
}
TEST void nullptrNotEqualsNullable(Nullable<int*> x) {
  nullable(x);
  if (nullptr != x) {
    nonnull(x);
  } else {
    nullable(x);
  }
  nullable(x);
}

// nullable vs nonnull
TEST void nullableEqualsNonnull(Nullable<int*> x, Nonnull<int*> y) {
  nullable(x);
  nonnull(y);
  if (x == y) {
    nonnull(x);
    nonnull(y);
  } else {
    nullable(x);
    nonnull(y);
  }
  nullable(x);
  nonnull(y);
}
TEST void nonnullEqualsNullable(Nullable<int*> x, Nonnull<int*> y) {
  nullable(x);
  nonnull(y);
  if (y == x) {
    nonnull(x);
    nonnull(y);
  } else {
    nullable(x);
    nonnull(y);
  }
  nullable(x);
  nonnull(y);
}
TEST void nullableNotEqualsNonnull(Nullable<int*> x, Nonnull<int*> y) {
  nullable(x);
  nonnull(y);
  if (x != y) {
    nullable(x);
    nonnull(y);
  } else {
    nonnull(x);
    nonnull(y);
  }
  nullable(x);
  nonnull(y);
}
TEST void nonnullNotEqualsNullable(Nullable<int*> x, Nonnull<int*> y) {
  nullable(x);
  nonnull(y);
  if (y != x) {
    nullable(x);
    nonnull(y);
  } else {
    nonnull(x);
    nonnull(y);
  }
  nullable(x);
  nonnull(y);
}

// nullable vs unknown
TEST void nullableEqualsUnknown(Nullable<int*> x, int* y) {
  nullable(x);
  unknown(y);
  if (x == y) {
    nullable(x);
    unknown(y);
  } else {
    nullable(x);
    unknown(y);
  }
  nullable(x);
  unknown(y);
}
TEST void unknownEqualsNullable(Nullable<int*> x, int* y) {
  nullable(x);
  unknown(y);
  if (y == x) {
    nullable(x);
    unknown(y);
  } else {
    nullable(x);
    unknown(y);
  }
  nullable(x);
  unknown(y);
}
TEST void nullableNotEqualsUnknown(Nullable<int*> x, int* y) {
  nullable(x);
  unknown(y);
  if (x != y) {
    nullable(x);
    unknown(y);
  } else {
    nullable(x);
    unknown(y);
  }
  nullable(x);
  unknown(y);
}
TEST void unknownNotEqualsNullable(Nullable<int*> x, int* y) {
  nullable(x);
  unknown(y);
  if (y != x) {
    nullable(x);
    unknown(y);
  } else {
    nullable(x);
    unknown(y);
  }
  nullable(x);
  unknown(y);
}

// unknown vs nullptr
// TODO(b/233582219): The pointer is compared to nullptr,
// hence the unnannotated pointer should be considered nullable.
TEST void unknownEqualsNullptr(int* x) {
  unknown(x);  // TODO: nullable
  if (x == nullptr) {
    unknown(x);  // TODO: nullable
  } else {
    nonnull(x);
  }
  unknown(x);  // TODO: nullable
}
TEST void nullptrEqualsUnknown(int* x) {
  unknown(x);  // TODO: nullable
  if (nullptr == x) {
    unknown(x);  // TODO: nullable
  } else {
    nonnull(x);
  }
  unknown(x);  // TODO: nullable
}
TEST void unknownNotEqualsNullptr(int* x) {
  unknown(x);  // TODO: nullable
  if (x != nullptr) {
    nonnull(x);
  } else {
    unknown(x);  // TODO: nullable
  }
  unknown(x);  // TODO: nullable
}
TEST void nullptrNotEqualsUnknown(int* x) {
  unknown(x);  // TODO: nullable
  if (nullptr != x) {
    nonnull(x);
  } else {
    unknown(x);  // TODO: nullable
  }
  unknown(x);  // TODO: nullable
}

// unknown vs nonnull
TEST void unknownEqualsNonnull(int* x, Nonnull<int*> y) {
  unknown(x);
  nonnull(y);
  if (x == y) {
    nonnull(x);
    nonnull(y);
  } else {
    unknown(x);
    nonnull(y);
  }
  unknown(x);
  nonnull(y);
}
TEST void nonnullEqualsUnknown(int* x, Nonnull<int*> y) {
  unknown(x);
  nonnull(y);
  if (y == x) {
    nonnull(x);
    nonnull(y);
  } else {
    unknown(x);
    nonnull(y);
  }
  unknown(x);
  nonnull(y);
}
TEST void unknownNotEqualsNonnull(int* x, Nonnull<int*> y) {
  unknown(x);
  nonnull(y);
  if (x != y) {
    unknown(x);
    nonnull(y);
  } else {
    nonnull(x);
    nonnull(y);
  }
  unknown(x);
  nonnull(y);
}
TEST void nonnullNotEqualsUnknown(int* x, Nonnull<int*> y) {
  unknown(x);
  nonnull(y);
  if (y != x) {
    unknown(x);
    nonnull(y);
  } else {
    nonnull(x);
    nonnull(y);
  }
  unknown(x);
  nonnull(y);
}
