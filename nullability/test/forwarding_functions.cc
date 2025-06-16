// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for forwarding functions like `std::make_unique`.

#include <memory>

#include "nullability_test.h"

struct CtorWithPointers {
  CtorWithPointers(int *_Nonnull X) : X(X) {}
  int *_Nonnull X;
};

void outParam(int *&X);

// Test that pointer variables passed to the real `std::make_unique` (not mock)
// are not clobbered by the call.
TEST void makeUniqueDoesNotClobberLvalueArguments(int *_Nonnull NonnullP) {
  nonnull(NonnullP);
  nonnull(std::make_unique<CtorWithPointers>(NonnullP));
  // Still nonnull because the param is not clobbered.
  nonnull(NonnullP);
  // In contrast, functions that have outparams clobber the arguments.
  outParam(NonnullP);
  unknown(NonnullP);
}
