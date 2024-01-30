// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_EXAMPLES_RS_BINDINGS_FROM_CC_BASICS_EXAMPLE_H_
#define CRUBIT_EXAMPLES_RS_BINDINGS_FROM_CC_BASICS_EXAMPLE_H_

#include <stdint.h>

extern "C" {
inline int32_t crubit_add_two_integers(int32_t x, int32_t y) { return x + y; }
}

#endif  // CRUBIT_EXAMPLES_RS_BINDINGS_FROM_CC_BASICS_EXAMPLE_H_
