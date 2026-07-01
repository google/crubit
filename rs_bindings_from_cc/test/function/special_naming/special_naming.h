// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SPECIAL_NAMING_SPECIAL_NAMING_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SPECIAL_NAMING_SPECIAL_NAMING_H_

#include "support/annotations.h"

// LLVM identifiers use the `\01` prefix to suppress mangling:
// https://llvm.org/docs/LangRef.html#identifiers
// Test that we can import functions that have such names.
// If `__USER_LABEL_PREFIX__` is non-empty, the Clang mangler adds the `\01`
// prefix; otherwise, we add it here ourselves.
#define IS_EMPTY_HELPER 1
#define IS_EMPTY(x) IS_EMPTY2(x)
#define IS_EMPTY2(x) IS_EMPTY_HELPER##x
#if IS_EMPTY(__USER_LABEL_PREFIX__)
CRUBIT_MUST_BIND int llvm_no_mangle_marker() __asm("\01_llvm_no_mangle_marker");
#else
CRUBIT_MUST_BIND int llvm_no_mangle_marker() __asm("_llvm_no_mangle_marker");
#endif

// Test that we can import functions whose `__asm` name contains a dollar sign.
// For example, the Apple SDKs use dollar signs in their symbol versioning
// macros (e.g. `__DARWIN_EXTSN()`).
CRUBIT_MUST_BIND int asm_name_with_dollar_sign() __asm(
    "asm$name$with$dollar$sign");

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SPECIAL_NAMING_SPECIAL_NAMING_H_
