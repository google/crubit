// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_NULLABILITY_INFERENCE_AUGMENTED_TEST_INPUTS_H_
#define THIRD_PARTY_CRUBIT_NULLABILITY_INFERENCE_AUGMENTED_TEST_INPUTS_H_

#include "nullability/pragma.h"
#include "clang/Testing/TestAST.h"
#include "llvm/ADT/StringRef.h"

namespace clang::tidy::nullability {

/// Returns `TestInputs` that include the given `Source`, nullability test
/// headers, and replaced macros, and populates `Pragmas` from nullability
/// pragmas written in `Source`.
TestInputs getAugmentedTestInputs(llvm::StringRef Source,
                                  NullabilityPragmas& Pragmas);

}  // namespace clang::tidy::nullability

#endif  // THIRD_PARTY_CRUBIT_NULLABILITY_INFERENCE_AUGMENTED_TEST_INPUTS_H_
