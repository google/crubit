// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_POINTER_NULLABILITY_DIAGNOSIS_H_
#define CRUBIT_NULLABILITY_POINTER_NULLABILITY_DIAGNOSIS_H_

#include <functional>

#include "nullability/pointer_nullability_lattice.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Stmt.h"
#include "clang/Analysis/FlowSensitive/MatchSwitch.h"
#include "clang/Basic/SourceLocation.h"
#include "llvm/ADT/SmallVector.h"

namespace clang {
namespace tidy {
namespace nullability {

/// Diagnoses a nullability-related issue in the associated CFG element.
struct PointerNullabilityDiagnostic {
  enum class ErrorCode {
    /// A nullable pointer was used where a nonnull pointer was expected.
    ExpectedNonnull,
    /// A pointer-typed expression was encountered with no corresponding model.
    Untracked,
    /// A nullability assertion was violated.
    AssertFailed,
  };
  ErrorCode Code;
  CharSourceRange Range;
};

/// Checks that nullable pointers are used safely, using nullability information
/// that is collected by `PointerNullabilityAnalysis`.
///
/// Examples of null safety violations include dereferencing nullable pointers
/// without null checks, and assignments between pointers of incompatible
/// nullability.
///
/// The diagnoser returns an empty vector when no issues are found in the code.
using PointerNullabilityDiagnoser =
    std::function<llvm::SmallVector<PointerNullabilityDiagnostic>(
        const CFGElement &, ASTContext &,
        const dataflow::TransferStateForDiagnostics<PointerNullabilityLattice>
            &)>;

PointerNullabilityDiagnoser pointerNullabilityDiagnoser();

}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_NULLABILITY_POINTER_NULLABILITY_DIAGNOSIS_H_
