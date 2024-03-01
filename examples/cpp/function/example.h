// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_CPP_FUNCTION_EXAMPLE_H_
#define THIRD_PARTY_CRUBIT_EXAMPLES_CPP_FUNCTION_EXAMPLE_H_

#include <stdint.h>

extern "C" {
inline int32_t crubit_add_two_integers(int32_t x, int32_t y) { return x + y; }
}

#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_CPP_FUNCTION_EXAMPLE_H_
