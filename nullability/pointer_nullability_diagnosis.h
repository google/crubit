// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_POINTER_NULLABILITY_DIAGNOSIS_H_
#define CRUBIT_NULLABILITY_POINTER_NULLABILITY_DIAGNOSIS_H_

#include <optional>
#include <string>

#include "clang/AST/Decl.h"
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
  /// Context in which the error occurred.
  enum class Context {
    /// Dereferencing a pointer.
    NullableDereference,
    /// Initializing a variable.
    Initializer,
    /// Value of a return statement.
    ReturnValue,
    /// Function argument.
    FunctionArgument,
    Other
  } Ctx = Context::Other;
  CharSourceRange Range;
  /// Name of the parameter that the argument is being passed to.
  /// Populated only if `Ctx` is `FunctionArgument` and the parameter name is
  /// known.
  std::optional<std::string> ParamName;
};

/// Checks that nullable pointers are used safely, using nullability information
/// that is collected by `PointerNullabilityAnalysis`.
///
/// Examples of null safety violations include dereferencing nullable pointers
/// without null checks, and assignments between pointers of incompatible
/// nullability.
///
/// Returns an empty vector when no issues are found in the code.
llvm::Expected<llvm::SmallVector<PointerNullabilityDiagnostic>>
diagnosePointerNullability(const FunctionDecl *Func);

}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_NULLABILITY_POINTER_NULLABILITY_DIAGNOSIS_H_
