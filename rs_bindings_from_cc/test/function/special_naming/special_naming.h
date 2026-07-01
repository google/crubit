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

// The problem replicated by the functions below was originally discovered when
// trying to enable build tests targeting Android in cl/937668344.
//
// This test replicates Android's Bionic `mallinfo` / `mallinfo2` symbol
// redirection conflict. Original scenario: Bionic's NDK header `<malloc.h>`
// declares (with the help of a `__RENAME` macro):
//   struct mallinfo mallinfo(void);
//   struct mallinfo2 mallinfo2(void) __asm__("mallinfo");
// This renames `mallinfo2` at the assembler level to `mallinfo` to reuse the
// same underlying symbol. See Bionic's malloc.h:
// https://github.com/aosp-mirror/platform_bionic/blob/android-14.0.0_r1/libc/include/malloc.h
//
// See also GCC documentation about controlling names in assembler code via
// `__asm__`: https://gcc.gnu.org/onlinedocs/gcc/Asm-Labels.html
//
// Crubit's rs_bindings_from_cc generates C++ thunk names based on the assembler
// linkage name of the C++ functions. If two functions share the same assembler
// name but have different signatures (e.g. different return types), Crubit used
// to generate thunks with the same name, which may lead to "conflicting types
// for '__rust_thunk__...'" compile errors in the generated C++ implementation.
struct SimpleStruct {
  int x;
};
struct OtherStruct {
  int y;
};
extern "C" {
SimpleStruct my_asm_conflict_func1() __asm__("my_asm_conflict_func");
OtherStruct my_asm_conflict_func2() __asm__("my_asm_conflict_func");
}

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SPECIAL_NAMING_SPECIAL_NAMING_H_
