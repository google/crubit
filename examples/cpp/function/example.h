// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_CPP_FUNCTION_EXAMPLE_H_
#define THIRD_PARTY_CRUBIT_EXAMPLES_CPP_FUNCTION_EXAMPLE_H_

#include <stdint.h>

// Crubit only supports extern C functions right now. As a consequence, the
// functions need a unique name. (Including the namespace name in the symbol,
// e.g., `gshoe`, below, is one approach to this.)
extern "C" {
inline int32_t gshoe_add_two_integers(int32_t x, int32_t y) { return x + y; }
}

#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_CPP_FUNCTION_EXAMPLE_H_
