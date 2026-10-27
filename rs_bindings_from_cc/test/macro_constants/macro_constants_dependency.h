// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_MACRO_CONSTANTS_MACRO_CONSTANTS_DEPENDENCY_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_MACRO_CONSTANTS_MACRO_CONSTANTS_DEPENDENCY_H_

// This target doesn't enable the `macro_constants` feature, so no Rust constant
// is generated for this macro. It can still be used by macros in other targets.
#define DEPENDENCY_CONSTANT 100
// Nested macros that are never expanded in C++ code, so their inner tokens
// remain out-of-date in the precompiled header module until Crubit expands
// them.
#define UNEXPANDED_INNER_DEPENDENCY_CONSTANT 99
#define UNEXPANDED_OUTER_DEPENDENCY_CONSTANT \
  (UNEXPANDED_INNER_DEPENDENCY_CONSTANT + 1)
// If `InnerShadowedType` is not refreshed from the precompiled header module
// when `UNEXPANDED_CAST_HELPER` is expanded, the cast resolves to the `short`
// typedef and truncates `0x10000` to `0` instead of expanding to `long long`.
typedef short InnerShadowedType;     // NOLINT(runtime/int)
#define InnerShadowedType long long  // NOLINT(runtime/int)
#define UNEXPANDED_CAST_HELPER ((InnerShadowedType)0x10000)

// Ensures that the bindings of this target are not empty.
inline int DependencyFunction() { return DEPENDENCY_CONSTANT; }

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_MACRO_CONSTANTS_MACRO_CONSTANTS_DEPENDENCY_H_
