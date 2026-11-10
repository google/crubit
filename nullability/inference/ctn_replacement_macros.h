// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// This file provides macro-replacement headers embedded into nullability
// inference translation units.

#ifndef CRUBIT_NULLABILITY_INFERENCE_CTN_REPLACEMENT_MACROS_H_
#define CRUBIT_NULLABILITY_INFERENCE_CTN_REPLACEMENT_MACROS_H_

#include <utility>

#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/StringRef.h"

namespace clang::tidy::nullability {

// Returns `(filename, content)` pairs for macro-replacement headers mapped into
// nullability inference translation units.
llvm::ArrayRef<std::pair<llvm::StringRef, llvm::StringRef>>
getReplacementMacrosHeaders();

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_INFERENCE_CTN_REPLACEMENT_MACROS_H_
