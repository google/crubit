// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef QUALIFIERS_KNOWN_FAILURE_H_
#define QUALIFIERS_KNOWN_FAILURE_H_

#ifdef __cplusplus
extern "C" {
#endif

void process_restrict(int* restrict p);  // Error: expected ')' (restrict is
                                         // C99, C++ doesn't recognize)

#ifdef __cplusplus
}
#endif

#endif  // QUALIFIERS_KNOWN_FAILURE_H_
