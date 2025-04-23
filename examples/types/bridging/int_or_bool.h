// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_TYPES_BRIDGING_INT_OR_BOOL_H_
#define THIRD_PARTY_CRUBIT_EXAMPLES_TYPES_BRIDGING_INT_OR_BOOL_H_

#include "examples/types/bridging/either.h"

either::Either<int, bool> MakeInt(int i);

either::Either<int, bool> MakeBool(bool b);

#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_TYPES_BRIDGING_INT_OR_BOOL_H_
