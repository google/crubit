// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Defines #pragma directives controlling nullability behavior.
//
//   #pragma nullability file_default (nullable|nonnull)
//
// This controls how unannotated pointer types are interpreted.
// This is based on the governing file, where the `*` is written.
// If this file has a pragma, it determines nullability, else it is Unspecified.

#ifndef CRUBIT_NULLABILITY_PRAGMA_H_
#define CRUBIT_NULLABILITY_PRAGMA_H_

#include "clang/include/clang/Basic/SourceLocation.h"
#include "clang/include/clang/Basic/Specifiers.h"
#include "llvm/include/llvm/ADT/DenseMap.h"

namespace clang {
class Preprocessor;
namespace tidy::nullability {

using NullabilityPragmas = llvm::DenseMap<FileID, NullabilityKind>;

// Install pragma handlers which record results into `Out`.
// This must be called before parsing begins.
// `Out` must outlive the preprocessor (i.e. the compilation).
void registerPragmaHandler(clang::Preprocessor &PP, NullabilityPragmas &Out);

}  // namespace tidy::nullability
}  // namespace clang

#endif
