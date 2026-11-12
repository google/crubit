// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef ATTRIBUTES_H_
#define ATTRIBUTES_H_

#ifdef __cplusplus
extern "C" {
#endif

void deprecated_func(void) __attribute__((deprecated));
void unavailable_func(void) __attribute__((unavailable));

#ifdef __cplusplus
}
#endif

#endif  // ATTRIBUTES_H_
