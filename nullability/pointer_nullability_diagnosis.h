// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_POINTER_NULLABILITY_DIAGNOSIS_H_
#define CRUBIT_NULLABILITY_POINTER_NULLABILITY_DIAGNOSIS_H_

#include <memory>
#include <string>
#include <utility>

#include "absl/base/nullability.h"
#include "nullability/pointer_nullability_analysis.h"
#include "nullability/pragma.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"
#include "clang/Analysis/FlowSensitive/Solver.h"
#include "clang/Basic/IdentifierTable.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/Specifiers.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/Support/Error.h"

namespace clang::tidy::nullability {

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
    /// A nonnull pointer field is nullable at method exit.
    /// `NoteRange` refers to the field declaration.
    NonnullPointerFieldNullableAtExit,
    /// A pointer-typed expression was encountered with no corresponding model.
    Untracked,
    /// A nullability assertion was violated.
    AssertFailed,
    /// Two types were expected to have the same nullability, but they did not.
    ExpectedEqualNullability,
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
  const clang::NamedDecl *absl_nullable Callee = nullptr;
  /// Name of the parameter that the argument is being passed to.
  /// Populated only if `Ctx` is `FunctionArgument` and the parameter name is
  /// known.
  const clang::IdentifierInfo *absl_nullable ParamName = nullptr;
  /// Source range and message of a note to be emitted alongside the diagnostic.
  CharSourceRange NoteRange;
  std::string NoteMessage;
};

/// Creates a solver with default parameters that is suitable for passing to
/// `diagnosePointerNullability()`.
std::unique_ptr<dataflow::Solver> makeDefaultSolverForDiagnosis();

/// TU-scoped side table recording the flow-sensitive nullability of by-value
/// lambda captures, keyed by (closure record, capture field). Only captures
/// proven nonnull or nullable at the capture site are recorded.
using LambdaCaptureNullabilityMap = llvm::DenseMap<
    std::pair<const clang::CXXRecordDecl*, const clang::FieldDecl*>,
    clang::NullabilityKind>;

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
/// When analyzing a function that constructs lambdas, `CaptureMap` is populated
/// with the flow-sensitive nullability of their by-value captures; when
/// analyzing a lambda call operator, it is consulted to preserve that
/// nullability. Callers analyzing a whole translation unit should share one map
/// across all calls; other callers may pass a throwaway map.
///
/// Returns an empty vector when no issues are found in the code.
llvm::Expected<llvm::SmallVector<PointerNullabilityDiagnostic>>
diagnosePointerNullability(
    const ValueDecl* absl_nonnull VD, const NullabilityPragmas& Pragmas,
    LambdaCaptureNullabilityMap& CaptureMap,
    const SolverFactory& MakeSolver = makeDefaultSolverForDiagnosis);

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_POINTER_NULLABILITY_DIAGNOSIS_H_
