// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_POINTER_NULLABILITY_H_
#define CRUBIT_NULLABILITY_POINTER_NULLABILITY_H_

// This file extends the dataflow framework's Value model to track nullability.
// We attach two boolean properties to each modeled pointer value:
//  - is_null: whether the pointer may actually be null
//    If this is false, dereferencing is safe.
//  - from_nullable: whether the originating expression was considered nullable
//    (e.g. a nullptr literal, or a reference to a Nullable-annotated variable)
//    If this is false, dereferencing may be safe: we don't know the contract.

#include <optional>

#include "absl/base/nullability.h"
#include "nullability/type_nullability.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/ASTDumper.h"
#include "clang/AST/Expr.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Formula.h"
#include "clang/Analysis/FlowSensitive/StorageLocation.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Basic/Specifiers.h"
#include "llvm/ADT/StringRef.h"

namespace clang::tidy::nullability {

/// Name of the synthetic field that models a smart pointer's underlying
/// pointer.
/// Where possible, use accessors such as `getPointerValueFromSmartPointer()`
/// instead of accessing this field directly.
inline constexpr llvm::StringRef PtrField = "ptr";

/// Returns the `PointerValue` allocated to `PointerExpr` if available.
/// Otherwise, returns nullptr.
dataflow::PointerValue *absl_nullable getRawPointerValue(
    const Expr *absl_nonnull PointerExpr, const dataflow::Environment &Env);

/// Returns the `PointerValue` underlying a smart pointer, or null if no
/// `PointerValue` is assigned to the smart pointer in the environment.
/// If `SmartPointerLoc` is null, returns null.
dataflow::PointerValue *absl_nullable getPointerValueFromSmartPointer(
    dataflow::RecordStorageLocation *absl_nullable SmartPointerLoc,
    const dataflow::Environment &Env);

/// Returns the `PointerValue` underlying a smart pointer expression, if
/// available.
/// Returns null if the expression is not associated with a storage location or
/// the smart pointer is not associated with a `PointerValue`.
dataflow::PointerValue *absl_nullable getSmartPointerValue(
    const Expr *absl_nonnull SmartPointerExpr,
    const dataflow::Environment &Env);

/// Returns the `PointerValue` for a raw or smart pointer expression, if
/// available.
/// Use this function only if the expression can actually be either a raw or
/// smart pointer; otherwise, use `getRawPointerValue()` or
/// `getSmartPointerValue()`.
dataflow::PointerValue *absl_nullable getPointerValue(
    const Expr *absl_nonnull PointerExpr, const dataflow::Environment &Env);

/// Sets the `PointerValue` underlying a smart pointer. If `PointerValue` is
/// null, clears any association between the smart pointer and an underlying
/// `PointerValue` in the environment.
void setSmartPointerValue(dataflow::RecordStorageLocation &SmartPointerLoc,
                          dataflow::PointerValue *absl_nullable Val,
                          dataflow::Environment &Env);

// Sets the `PointerValue` underlying a smart pointer to null.
void setSmartPointerToNull(dataflow::RecordStorageLocation &SmartPointerLoc,
                           dataflow::Environment &Env);

// Returns true if the pointer has all properties necessary for representing
// complete nullness information.
// Otherwise, returns false.
//
// Pointers that are the value of some expression always have null state once
// that expression has been analyzed. Other pointers, like the values of unused
// parameters, may lack this state. This state is only set by
// PointerNullabilityAnalysis, not by the dataflow framework.
bool hasPointerNullState(const dataflow::PointerValue &PointerVal);

/// The properties representing nullness information for a pointer.
///
/// We attach these properties to every PointerValue taken by an expression.
///
/// A null pointer for `FromNullable` or `IsNull` represents "top", i.e. we have
/// no information on this property.
struct PointerNullState {
  /// Did the pointer come from a known-nullable source?
  const dataflow::Formula *absl_nullable FromNullable;
  /// Is the pointer's value null?
  const dataflow::Formula *absl_nullable IsNull;
  // These are independent: sources with unknown nullability can yield nullptr!
};

/// Returns the properties representing the nullness information of a pointer.
PointerNullState getPointerNullState(const dataflow::PointerValue &PointerVal);

/// Creates the nullness properties on `PointerVal` if not already initialised.
///
/// We call this when the framework produces a PointerValue for an expression.
/// This ensures that the variable has usable "from nullable" and "is null"
/// boolean variables, and that they are constrained based on the *original*
/// source of the PointerValue.
///
/// For example:
///    Unknown<int> *x = makeNullable();
///                      ~~~~~~~~~~~~~~ <-- initPointerNullState(Nullable)
///    *x;
///     ~ <-- initPointerNullState(Unknown) - no effect, already initialized
///
/// The constraints are added to the context as a non-flow-sensitive invariant,
/// so the source nullability may not depend on flow-sensitive information.
///
/// (We assume that the framework will not provide the same pointer from
/// different initial sources, so the `Source` nullability is the same
/// regardless of block evaluation order).
void initPointerNullState(
    dataflow::PointerValue &PointerVal, dataflow::DataflowAnalysisContext &Ctx,
    std::optional<PointerTypeNullability> Source = std::nullopt);

/// Initializes the nullness properties on `PointerVal` from `State`.
///
/// This overload may only be called on a freshly created `PointerValue` that
/// does not yet have nullability properties.
void initPointerNullState(dataflow::PointerValue &PointerVal,
                          dataflow::DataflowAnalysisContext &Ctx,
                          PointerNullState State);

/// Variant of initPointerNullState, where the pointer is guaranteed null.
/// (This is flow-insensitive, but PointerTypeNullability can't represent it).
void initNullPointer(dataflow::PointerValue &PointerVal,
                     dataflow::DataflowAnalysisContext &Ctx);

/// Creates a null pointer with the given pointee type. The null state for the
/// pointer is set to reflect that the pointer is null. This always returns the
/// same `PointerValue` for a given `PointeeType`.
dataflow::PointerValue &createNullPointer(QualType PointeeType,
                                          dataflow::Environment &Env);

/// Returns true if `PointerVal` is known to be null or is from a nullable
/// source and may be null.
bool isNullable(
    const dataflow::PointerValue &PointerVal, const dataflow::Environment &Env,
    const dataflow::Formula *absl_nullable AdditionalConstraints = nullptr);

/// Returns true if a `nullptr` literal is reachable according to `Env`'s flow
/// condition. Otherwise, returns Unspecified.
bool isReachableNullptrLiteral(
    const dataflow::Environment &Env,
    const dataflow::Formula *absl_nullable AdditionalConstraints = nullptr);

/// Returns the strongest provable assertion we can make about `PointerVal`.
/// If PointerVal may not be null, returns Nonnull.
/// If PointerVal is known to be null or is from a nullable source and may be
/// null, returns Nullable.
/// Otherwise, returns Unspecified.
clang::NullabilityKind getNullability(
    const dataflow::PointerValue &PointerVal, const dataflow::Environment &Env,
    const dataflow::Formula *absl_nullable AdditionalConstraints = nullptr);

/// Returns the strongest provable assertion we can make about the value of
/// `E` in `Env`.
clang::NullabilityKind getNullability(
    const Expr *absl_nonnull E, const dataflow::Environment &Env,
    const dataflow::Formula *absl_nullable AdditionalConstraints = nullptr);

/// Returns the strongest nullability kind for a `nullptr_t` typed value
/// (a `nullptr` literal). This will be Nullable if the value's expression is
/// reachable according to `Env`'s flow condition. Otherwise, returns
/// Unspecified.
clang::NullabilityKind getNullabilityForNullptrT(
    const dataflow::Environment &Env,
    const dataflow::Formula *absl_nullable AdditionalConstraints = nullptr);

// Work around the lack of Expr.dump() etc with an ostream but no ASTContext.
template <typename T>
void dump(const T &Node, llvm::raw_ostream &OS) {
  clang::ASTDumper(OS, /*ShowColors=*/false).Visit(Node);
}

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_POINTER_NULLABILITY_H_
