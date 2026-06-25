// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SPECIAL_LINKING_SPECIAL_LINKING_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SPECIAL_LINKING_SPECIAL_LINKING_H_

// This function is weakly imported.
//
// Crubit shouldn't unconditionally create thunks that refer to a weak import,
// because this can cause the following build errors (since the function's
// address may be unknown):
//
// ```
//     error: static assertion expression is not an integral constant expression
//       14 | static_assert((void (*)()) &::weak_import_func);
//          |               ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ```
void weak_import_func() __attribute__((weak_import));

// This test function is only available in Android API 9999+.
// If we compile targeting Android API 23, this is a weak import
// (so the caveats of `weak_import_func` apply).
//
// This problem was discovered when trying to enable build tests targeting
// Android in cl/937668344.
//
// Original scenario: Functions like `sighold`, `sigignore`, `sigpause`,
// `sigrelse`, and `sigset` in Bionic's `<signal.h>` are introduced in API 26.
// When compiling targeting API 23 with
// `__ANDROID_UNAVAILABLE_SYMBOLS_ARE_WEAK__` defined (standard for NDK), Clang
// treats them as weakly imported (unavailable, but compile-able as weak).
// See Bionic's signal.h:
// https://github.com/aosp-mirror/platform_bionic/blob/android-14.0.0_r1/libc/include/signal.h
void min_android_version_9999()
    __attribute__((availability(android, introduced = 9999)));

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
// name but have different signatures (e.g. different return types), Crubit
// generates thunks with the same name, which may lead to "conflicting types for
// '__rust_thunk__...'" compile errors in the generated C++ implementation.
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

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SPECIAL_LINKING_SPECIAL_LINKING_H_
