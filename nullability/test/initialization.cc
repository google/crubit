// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for various forms of initialization.

#include "nullability_test.h"

TEST void valueInitializedPointerIsNull() {
  using Pointer = int *;
  provable(Pointer() == nullptr);
}
