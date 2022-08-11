// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_H_
#define CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_H_

#include <utility>

#include "clang/AST/Expr.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Value.h"

namespace clang {
namespace tidy {
namespace nullability {

/// Returns the properties representing the nullness information of a pointer.
///
/// The first boolean indicates if the pointer's nullability is known.
/// The second boolean indicates if the pointer's value is not null.
std::pair<dataflow::AtomicBoolValue&, dataflow::AtomicBoolValue&>
getPointerNullState(const Expr* PointerExpr, const dataflow::Environment& Env);

/// Returns the properties representing the nullness information of a pointer.
///
/// The first boolean indicates if the pointer's nullability is known.
/// The second boolean indicates if the pointer's value is not null.
std::pair<dataflow::AtomicBoolValue&, dataflow::AtomicBoolValue&>
getPointerNullState(const dataflow::PointerValue& PointerVal,
                    const dataflow::Environment& Env);

/// Sets the nullness properties on the PointerValue assigned to `PointerExpr`
/// if not already initialised.
///
/// The boolean properties may be constrained by specifying `KnownConstraint`
/// and `NotNullConstraint`. Otherwise, the properties are set to freshly
/// created atomic booleans.
void initPointerNullState(const Expr* PointerExpr, dataflow::Environment& Env,
                          dataflow::BoolValue* KnownConstraint = nullptr,
                          dataflow::BoolValue* NotNullConstraint = nullptr);

/// Sets the nullness properties on `PointerVal` if not already initialised.
///
/// The boolean properties may be constrained by specifying `KnownConstraint`
/// and `NotNullConstraint`. Otherwise, the properties are set to freshly
/// created atomic booleans.
void initPointerNullState(dataflow::PointerValue& PointerVal,
                          dataflow::Environment& Env,
                          dataflow::BoolValue* KnownConstraint = nullptr,
                          dataflow::BoolValue* NotNullConstraint = nullptr);

}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_H_
