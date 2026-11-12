// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef QUALIFIERS_H_
#define QUALIFIERS_H_

#ifdef __cplusplus
extern "C" {
#endif

extern volatile int vol_var;
extern const volatile int const_vol_var;
void process_ptr_to_ptr(int** pp);
void process_ptr_to_const(const int* p);

#ifdef __cplusplus
}
#endif

#endif  // QUALIFIERS_H_
