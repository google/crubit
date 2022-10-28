// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_VOID_POINTERS_VOID_POINTERS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_VOID_POINTERS_VOID_POINTERS_H_

#include <cstring>

#pragma clang lifetime_elision

// Use `inline` to force a thunk to be generated so we can test that it
// compiles.
inline void* invoke_memcpy(void* dst, const void* src, size_t size) {
  return memcpy(dst, src, size);
}

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_VOID_POINTERS_VOID_POINTERS_H_
