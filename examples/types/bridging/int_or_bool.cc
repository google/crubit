// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "examples/types/bridging/int_or_bool.h"

#include "examples/types/bridging/either.h"

either::Either<int, bool> MakeInt(int i) {
  return {
      .is_left = true,
      .left = i,
  };
}

either::Either<int, bool> MakeBool(bool b) {
  return {
      .is_left = false,
      .right = b,
  };
}
