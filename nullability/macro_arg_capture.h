// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// This file provides shared constants used when capturing the values passed as
// arguments to macros of interest.

#ifndef CRUBIT_NULLABILITY_INFERENCE_MACRO_ARG_CAPTURE_H_
#define CRUBIT_NULLABILITY_INFERENCE_MACRO_ARG_CAPTURE_H_

#include "llvm/include/llvm/ADT/StringRef.h"

namespace clang::tidy::nullability {
inline constexpr llvm::StringRef ArgCaptureAbortIfFalse =
    "clang_tidy_nullability_internal_abortIfFalse";

inline constexpr llvm::StringRef ArgCaptureAbortIfEqual =
    "clang_tidy_nullability_internal_abortIfEqual";
}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_INFERENCE_MACRO_ARG_CAPTURE_H_
