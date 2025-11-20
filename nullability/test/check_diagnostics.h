// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_TEST_CHECK_DIAGNOSTICS_H_
#define CRUBIT_NULLABILITY_TEST_CHECK_DIAGNOSTICS_H_

#include <optional>
#include <vector>

#include "nullability/pointer_nullability_diagnosis.h"
#include "nullability/pragma.h"
#include "clang/AST/ASTContext.h"
#include "clang/Testing/CommandLineArgs.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Testing/Annotations/Annotations.h"

namespace clang {
namespace tidy {
namespace nullability {

/// Runs nullability verification on `SourceCode` and returns whether
/// diagnostics are produced on those lines marked in the source code with
/// `llvm::Annotations` style annotations (and no other lines).
bool checkDiagnostics(llvm::StringRef SourceCode);

/// Same as `checkDiagnostics`, but allows for untracked errors.
bool checkDiagnosticsHasUntracked(llvm::StringRef SourceCode);

bool checkDiagnosticsWithMin(llvm::StringRef SourceCode, TestLanguage Min);

bool checkDiagnostics(ASTContext& AST, llvm::Annotations AnnotatedCode,
                      const NullabilityPragmas& Pragmas = NullabilityPragmas(),
                      bool AllowUntracked = false);

/// Variation of `checkDiagnostics` which returns the ordered list of all actual
/// diagnostics if their locations match those in the source code annotations,
/// and otherwise returns `std::nullopt` to indicate a mismatch or failure.
/// Note that this iterates over the target C++ versions, so produces repeated
/// diagnostics if they occur in multiple versions.
std::optional<std::vector<PointerNullabilityDiagnostic>> checkAndGetDiagnostics(
    llvm::StringRef SourceCode);

}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_NULLABILITY_TEST_CHECK_DIAGNOSTICS_H_
