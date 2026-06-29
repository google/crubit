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

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SPECIAL_LINKING_SPECIAL_LINKING_H_
