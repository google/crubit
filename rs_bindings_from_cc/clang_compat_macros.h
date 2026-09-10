// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_CLANG_COMPAT_MACROS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_CLANG_COMPAT_MACROS_H_

#if !defined(CRUBIT_LLVM_DEV_DATE)
#error "CRUBIT_LLVM_DEV_DATE must be defined when compiling Crubit."
#endif

#define LLVM_DEV_DATE_GE(date) (CRUBIT_LLVM_DEV_DATE >= (date))
#define LLVM_DEV_DATE_LT(date) (CRUBIT_LLVM_DEV_DATE < (date))

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_CLANG_COMPAT_MACROS_H_
