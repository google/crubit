// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef GLOBALS_H_
#define GLOBALS_H_

#ifdef __cplusplus
extern "C" {
#endif

extern int global_var;
extern const int const_global_var;
int tentative_def;

#ifdef __cplusplus
}
#endif

#endif  // GLOBALS_H_
