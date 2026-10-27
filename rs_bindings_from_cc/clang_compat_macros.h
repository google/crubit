// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_CLANG_COMPAT_MACROS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_CLANG_COMPAT_MACROS_H_

// `LLVM_DEV_DATE_GE(YYYYMMDD)` and `LLVM_DEV_DATE_LT(YYYYMMDD)` let Crubit
// build against LLVM versions from before and after a Clang API change.
// `CRUBIT_LLVM_DEV_DATE` is the UTC commit date of the LLVM version that Crubit
// is built against.
//
// Crubit supports building against older LLVM versions (e.g. the LLVM version
// pinned by Chromium), not only the newest one.  So do not remove
// `#if LLVM_DEV_DATE_GE(...)` guards (or their `#else` branches) only because
// the newest LLVM no longer needs them.  If a build compiles the wrong branch,
// then fix the `CRUBIT_LLVM_DEV_DATE` value of that build instead.

#if !defined(CRUBIT_LLVM_DEV_DATE)
#error "CRUBIT_LLVM_DEV_DATE must be defined when compiling Crubit."
#endif

#define LLVM_DEV_DATE_GE(date) (CRUBIT_LLVM_DEV_DATE >= (date))
#define LLVM_DEV_DATE_LT(date) (CRUBIT_LLVM_DEV_DATE < (date))

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_CLANG_COMPAT_MACROS_H_
