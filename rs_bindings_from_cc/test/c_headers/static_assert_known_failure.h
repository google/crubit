// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef STATIC_ASSERT_KNOWN_FAILURE_H_
#define STATIC_ASSERT_KNOWN_FAILURE_H_

#ifdef __cplusplus
extern "C" {
#endif

_Static_assert(sizeof(int) >= 4, "int must be at least 32 bits");

#ifdef __cplusplus
}
#endif

#endif  // STATIC_ASSERT_KNOWN_FAILURE_H_
