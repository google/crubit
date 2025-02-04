// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_POINTER_NULLABILITY_DIAGNOSIS_H_
#define CRUBIT_NULLABILITY_POINTER_NULLABILITY_DIAGNOSIS_H_

#include <memory>
#include <optional>
#include <string>

#include "absl/base/nullability.h"
#include "nullability/pointer_nullability_analysis.h"
#include "nullability/pragma.h"
#include "clang/AST/Decl.h"
#include "clang/Analysis/FlowSensitive/Solver.h"
#include "clang/Basic/IdentifierTable.h"
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
    /// Nullability annotations are inconsistent with a previous declaration.
    /// `NoteRange` refers to the location of the previous declaration.
    InconsistentAnnotations,
    InconsistentAnnotationsForParameter,
    InconsistentAnnotationsForReturn,
    /// A moved-from nonnull pointer was accessed.
    AccessingMovedFromNonnullPointer,
    /// A pointer-typed expression was encountered with no corresponding model.
    Untracked,
    /// A nullability assertion was violated.
    AssertFailed,
  };
  ErrorCode Code;
  /// Context in which the error occurred.
  enum class Context {
    /// Dereferencing a pointer.
    NullableDereference,
    /// Initializing a variable.
    Initializer,
    /// Assigning to a pointer.
    Assignment,
    /// Value of a return statement.
    ReturnValue,
    /// Function argument.
    FunctionArgument,
    Other
  } Ctx = Context::Other;
  CharSourceRange Range;
  /// The function where the argument is being passed to.
  /// Populated only if `Ctx` is `FunctionArgument`.
  absl::Nullable<const clang::NamedDecl *> Callee = nullptr;
  /// Name of the parameter that the argument is being passed to.
  /// Populated only if `Ctx` is `FunctionArgument` and the parameter name is
  /// known.
  absl::Nullable<const clang::IdentifierInfo *> ParamName = nullptr;
  /// Source range of a note to be emitted alongside the diagnostic.
  /// The exact semantics of the note depend on `Code` and `Ctx`.
  CharSourceRange NoteRange;
};

/// Creates a solver with default parameters that is suitable for passing to
/// `diagnosePointerNullability()`.
std::unique_ptr<dataflow::Solver> makeDefaultSolverForDiagnosis();

/// Checks that nullable pointers are used safely, using nullability information
/// that is collected by `PointerNullabilityAnalysis`.
///
/// Examples of null safety violations include dereferencing nullable pointers
/// without null checks, and assignments between pointers of incompatible
/// nullability.
///
/// If `VD` is not a function, this merely checks that the annotations on `VD`
/// are consistent with the annotations on its canonical declaration.
///
/// Returns an empty vector when no issues are found in the code.
llvm::Expected<llvm::SmallVector<PointerNullabilityDiagnostic>>
diagnosePointerNullability(
    const ValueDecl *VD, const NullabilityPragmas &Pragmas,
    const SolverFactory &MakeSolver = makeDefaultSolverForDiagnosis);

}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_NULLABILITY_POINTER_NULLABILITY_DIAGNOSIS_H_
