// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef STATIC_INLINE_H_
#define STATIC_INLINE_H_

#ifdef __cplusplus
extern "C" {
#endif

static inline int static_inline_func(void) { return 42; }

#ifdef __cplusplus
}
#endif

#endif  // STATIC_INLINE_H_
