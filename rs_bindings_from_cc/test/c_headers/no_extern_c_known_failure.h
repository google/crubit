// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef NO_EXTERN_C_KNOWN_FAILURE_H_
#define NO_EXTERN_C_KNOWN_FAILURE_H_

// No extern "C" here. Crubit assumes mangled C++ symbol
// _Z30no_extern_c_func_unmangledv, which fails linking to C object.
int no_extern_c_func_unmangled(void);

#endif  // NO_EXTERN_C_KNOWN_FAILURE_H_
