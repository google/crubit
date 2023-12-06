// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for smart pointers.

#include <memory>

#include "nullability_test.h"

TEST void parameterAnnotations(Nonnull<std::unique_ptr<int>> nonnullParam,
                               Nullable<std::unique_ptr<int>> nullableParam,
                               std::unique_ptr<int> unknownParam) {
  nonnull(nonnullParam);
  nullable(nullableParam);
  unknown(unknownParam);
}

Nonnull<std::unique_ptr<int>> returnsNonnull();
Nullable<std::unique_ptr<int>> returnsNullable();
std::unique_ptr<int> returnsUnknown();

TEST void returnValueAnnotations() {
  nonnull(returnsNonnull());
  nullable(returnsNullable());
  unknown(returnsUnknown());
}
