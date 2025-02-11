// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_NULLABILITY_ANNOTATIONS_H_
#define THIRD_PARTY_CRUBIT_NULLABILITY_ANNOTATIONS_H_

#include "llvm/ADT/StringRef.h"

namespace clang::tidy::nullability {
inline constexpr llvm::StringLiteral ClangNullable = "_Nullable";
inline constexpr llvm::StringLiteral ClangNonnull = "_Nonnull";
inline constexpr llvm::StringLiteral ClangUnknown = "_Null_unspecified";

inline constexpr llvm::StringLiteral AbslMacroNullable = "absl_nullable";
inline constexpr llvm::StringLiteral AbslMacroNonnull = "absl_nonnull";
inline constexpr llvm::StringLiteral AbslMacroUnknown =
    "absl_nullability_unknown";

inline constexpr llvm::StringLiteral AbslTemplateNamespace = "absl";
inline constexpr llvm::StringLiteral AbslTemplateNullable = "Nullable";
inline constexpr llvm::StringLiteral AbslTemplateNonnull = "Nonnull";
inline constexpr llvm::StringLiteral AbslTemplateUnknown = "NullabilityUnknown";
}  // namespace clang::tidy::nullability

#endif  // THIRD_PARTY_CRUBIT_NULLABILITY_ANNOTATIONS_H_
