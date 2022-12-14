// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_DIAGNOSIS_H_
#define CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_DIAGNOSIS_H_

#include "nullability_verification/pointer_nullability_lattice.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Stmt.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "llvm/ADT/Optional.h"

namespace clang {
namespace tidy {
namespace nullability {

/// Checks that nullable pointers are used safely, using nullability information
/// that is collected by `PointerNullabilityAnalysis`.
///
/// Examples of null safety violations include dereferencing nullable pointers
/// without null checks, and assignments between pointers of incompatible
/// nullability.
class PointerNullabilityDiagnoser {
 public:
  PointerNullabilityDiagnoser();

  /// Returns the pointer to the statement if null safety is violated, otherwise
  /// the optional is empty.
  ///
  /// TODO(b/233582219): Extend diagnosis to return more information, e.g. the
  /// type of violation.
  llvm::Optional<CFGElement> diagnose(
      const CFGElement* Elt, ASTContext& Ctx,
      const dataflow::TransferStateForDiagnostics<PointerNullabilityLattice>&
          State) {
    return Diagnoser(*Elt, Ctx, State);
  }

 private:
  dataflow::CFGMatchSwitch<
      const dataflow::TransferStateForDiagnostics<PointerNullabilityLattice>,
      llvm::Optional<CFGElement>>
      Diagnoser;
};

}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_DIAGNOSIS_H_
