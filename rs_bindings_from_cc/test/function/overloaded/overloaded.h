// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_OVERLOADED_OVERLOADED_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_OVERLOADED_OVERLOADED_H_

inline void Ambiguous() {}
inline void Ambiguous(int) {}

[[deprecated]] inline void AmbiguousDeprecated() {}
[[deprecated]] inline void AmbiguousDeprecated(int) {}

inline void CanonicalNonDeprecated() {}
[[deprecated]] inline void CanonicalNonDeprecated(int) {}

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_OVERLOADED_OVERLOADED_H_
