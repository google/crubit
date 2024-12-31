// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_UNSAFE_ATTRIBUTES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_UNSAFE_ATTRIBUTES_H_

#include "support/annotations.h"

inline void SafeSignatureWithoutAnnotation() {}
CRUBIT_UNSAFE inline void SafeSignatureButAnnotatedUnsafe() {}

inline void UnsafeSignatureWithoutAnnotation(void*) {}
CRUBIT_UNSAFE_MARK_SAFE inline void UnsafeSignatureButAnnotatedSafe(void*) {}

CRUBIT_OVERRIDE_UNSAFE(/*is_unsafe=*/false) inline void SafeBasedOnBoolean() {}
CRUBIT_OVERRIDE_UNSAFE(/*is_unsafe=*/true) inline void UnsafeBasedOnBoolean() {}

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_UNSAFE_ATTRIBUTES_H_
