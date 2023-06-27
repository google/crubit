// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_POINTER_NULLABILITY_H_
#define CRUBIT_NULLABILITY_POINTER_NULLABILITY_H_

// This file extends the dataflow framework's Value model to track nullability.
// We attach two boolean properties to each modeled pointer value:
//  - is_null: whether the pointer may actually be null
//    If this is false, dereferencing is safe.
//  - is_known: whether the source had defined nullability (Nullable or Nonnull)
//    If this is false, dereferencing may be safe: we don't know the contract.

#include <utility>

#include "clang/AST/ASTContext.h"
#include "clang/AST/ASTDumper.h"
#include "clang/AST/Expr.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Basic/Specifiers.h"

namespace clang::tidy::nullability {

/// Returns the `PointerValue` allocated to `PointerExpr` if available.
/// Otherwise, returns nullptr.
dataflow::PointerValue *getPointerValueFromExpr(
    const Expr *PointerExpr, const dataflow::Environment &Env);

// Returns true if the pointer has all properties necessary for representing
// complete nullness information.
// Otherwise, returns false.
//
// Pointers that are the value of some expression always have null state once
// that expression has been analyzed. Other pointers, like the values of unused
// parameters, may lack this state. This state is only set by
// PointerNullabilityAnalysis, not by the dataflow framework.
bool hasPointerNullState(const dataflow::PointerValue &PointerVal);

/// Returns the properties representing the nullness information of a pointer.
///
/// The first boolean indicates if the pointer's nullability is known.
/// The second boolean indicates if the pointer's value is null.
std::pair<dataflow::AtomicBoolValue &, dataflow::AtomicBoolValue &>
getPointerNullState(const dataflow::PointerValue &PointerVal);

/// Sets the nullness properties on `PointerVal` if not already initialised.
///
/// The boolean properties may be constrained by specifying `KnownConstraint`
/// and `NullConstraint`. Otherwise, the properties are set to freshly
/// created atomic booleans.
void initPointerNullState(dataflow::PointerValue &PointerVal,
                          dataflow::Environment &Env,
                          dataflow::BoolValue *KnownConstraint = nullptr,
                          dataflow::BoolValue *NullConstraint = nullptr);

/// Sets the nullness properties on `PointerVal` representing a nullptr if not
/// already initialised.
///
/// `Known` is constrained to true, `Null` is constrained to true.
inline void initNullPointer(dataflow::PointerValue &PointerVal,
                            dataflow::Environment &Env) {
  initPointerNullState(PointerVal, Env,
                       /*KnownConstraint=*/&Env.getBoolLiteralValue(true),
                       /*NullConstraint=*/&Env.getBoolLiteralValue(true));
}

/// Sets the nullness properties on `PointerVal` representing a pointer that is
/// not null if not already initialised.
///
/// `Known` is constrained to true, `Null` is constrained to false.
inline void initNotNullPointer(dataflow::PointerValue &PointerVal,
                               dataflow::Environment &Env) {
  initPointerNullState(PointerVal, Env,
                       /*KnownConstraint=*/&Env.getBoolLiteralValue(true),
                       /*NullConstraint=*/&Env.getBoolLiteralValue(false));
}

/// Sets the nullness properties on `PointerVal` representing a pointer that is
/// nullable if not already initialised.
///
/// `Known` is constrained to true, `Null` is unconstrained.
inline void initNullablePointer(dataflow::PointerValue &PointerVal,
                                dataflow::Environment &Env) {
  initPointerNullState(PointerVal, Env,
                       /*KnownConstraint=*/&Env.getBoolLiteralValue(true));
}

/// Sets the nullness properties on `PointerVal` representing a pointer with
/// unknown nullability if not already initialised.
///
/// `Known` is constrained to false, `Null` is unconstrained.
inline void initUnknownPointer(dataflow::PointerValue &PointerVal,
                               dataflow::Environment &Env) {
  initPointerNullState(PointerVal, Env,
                       /*KnownConstraint=*/&Env.getBoolLiteralValue(false));
}

/// Returns true if there is evidence that `PointerVal` may hold a nullptr.
bool isNullable(const dataflow::PointerValue &PointerVal,
                const dataflow::Environment &Env);

/// Returns the strongest provable assertion we can make about `PointerVal`.
/// If PointerVal may not be null, returns Nonnull.
/// If PointerVal may be both null and known-nullability, returns Nullable.
/// Otherwise, returns Unspecified.
clang::NullabilityKind getNullability(const dataflow::PointerValue &PointerVal,
                                      const dataflow::Environment &Env);

/// Returns the strongest provable assertion we can make about the value of
/// `E` in `Env`.
inline clang::NullabilityKind getNullability(const Expr *E,
                                             const dataflow::Environment &Env) {
  if (auto *P = getPointerValueFromExpr(E, Env)) return getNullability(*P, Env);
  return clang::NullabilityKind::Unspecified;
}

// Work around the lack of Expr.dump() etc with an ostream but no ASTContext.
template <typename T>
void dump(const T &Node, llvm::raw_ostream &OS) {
  clang::ASTDumper(OS, /*ShowColors=*/false).Visit(Node);
}

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_POINTER_NULLABILITY_H_
