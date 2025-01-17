// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_UNSAFE_ATTRIBUTES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_UNSAFE_ATTRIBUTES_H_

#include "support/annotations.h"

void SafeSignatureWithoutAnnotation();
CRUBIT_UNSAFE void SafeSignatureButAnnotatedUnsafe();
[[clang::unsafe_buffer_usage]] void SafeSignatureButAnnotatedUnsafeBuffer();
CRUBIT_UNSAFE_MARK_SAFE void SafeSignatureButAnnotatedSafe();
void UnsafeSignatureWithoutAnnotation(void*);
CRUBIT_UNSAFE void UnsafeSignatureButAnnotatedUnsafe(void*);
[[clang::unsafe_buffer_usage]] void UnsafeSignatureButAnnotatedUnsafeBuffer(
    void*);
CRUBIT_UNSAFE_MARK_SAFE void UnsafeSignatureButAnnotatedSafe(void*);

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_UNSAFE_ATTRIBUTES_H_
