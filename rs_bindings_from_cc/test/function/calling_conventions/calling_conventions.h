// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_CALLING_CONVENTIONS_CALLING_CONVENTIONS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_CALLING_CONVENTIONS_CALLING_CONVENTIONS_H_

#include <stdint.h>

#pragma clang lifetime_elision

// A struct that is passed in a non-default way in the swiftcall calling
// convention.
//
// On most platforms structs that are 3 words or smaller are passed directly
// in swiftcall.
struct UnusualSwiftcallStruct final {
  uintptr_t x0, x1, x2;
};

uintptr_t function_with_default_cc(UnusualSwiftcallStruct s);

__attribute__((swiftcall)) uintptr_t function_with_swiftcall_cc(
    UnusualSwiftcallStruct s);

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_CALLING_CONVENTIONS_CALLING_CONVENTIONS_H_
