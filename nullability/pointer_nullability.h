// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_POINTER_NULLABILITY_H_
#define CRUBIT_NULLABILITY_POINTER_NULLABILITY_H_

#include <utility>

#include "nullability/pointer_nullability_lattice.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/ASTDumper.h"
#include "clang/AST/Expr.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Basic/Specifiers.h"

namespace clang {
namespace tidy {
namespace nullability {

using dataflow::TransferState;

/// Returns the `NullabilityKind` corresponding to the nullability annotation on
/// `Type` if present. Otherwise, returns `NullabilityKind::Unspecified`.
NullabilityKind getNullabilityKind(QualType Type, ASTContext& Ctx);

/// Returns the `PointerValue` allocated to `PointerExpr` if available.
/// Otherwise, returns nullptr.
dataflow::PointerValue* getPointerValueFromExpr(
    const Expr* PointerExpr, const dataflow::Environment& Env);

/// Returns the properties representing the nullness information of a pointer.
///
/// The first boolean indicates if the pointer's nullability is known.
/// The second boolean indicates if the pointer's value is null.
std::pair<dataflow::AtomicBoolValue&, dataflow::AtomicBoolValue&>
getPointerNullState(const dataflow::PointerValue& PointerVal,
                    const dataflow::Environment& Env);

/// Sets the nullness properties on `PointerVal` if not already initialised.
///
/// The boolean properties may be constrained by specifying `KnownConstraint`
/// and `NullConstraint`. Otherwise, the properties are set to freshly
/// created atomic booleans.
void initPointerNullState(dataflow::PointerValue& PointerVal,
                          dataflow::Environment& Env,
                          dataflow::BoolValue* KnownConstraint = nullptr,
                          dataflow::BoolValue* NullConstraint = nullptr);

/// Sets the nullness properties on `PointerVal` representing a nullptr if not
/// already initialised.
///
/// `Known` is constrained to true, `Null` is constrained to true.
inline void initNullPointer(dataflow::PointerValue& PointerVal,
                            dataflow::Environment& Env) {
  initPointerNullState(PointerVal, Env,
                       /*KnownConstraint=*/&Env.getBoolLiteralValue(true),
                       /*NullConstraint=*/&Env.getBoolLiteralValue(true));
}

/// Sets the nullness properties on `PointerVal` representing a pointer that is
/// not null if not already initialised.
///
/// `Known` is constrained to true, `Null` is constrained to false.
inline void initNotNullPointer(dataflow::PointerValue& PointerVal,
                               dataflow::Environment& Env) {
  initPointerNullState(PointerVal, Env,
                       /*KnownConstraint=*/&Env.getBoolLiteralValue(true),
                       /*NullConstraint=*/&Env.getBoolLiteralValue(false));
}

/// Sets the nullness properties on `PointerVal` representing a pointer that is
/// nullable if not already initialised.
///
/// `Known` is constrained to true, `Null` is unconstrained.
inline void initNullablePointer(dataflow::PointerValue& PointerVal,
                                dataflow::Environment& Env) {
  initPointerNullState(PointerVal, Env,
                       /*KnownConstraint=*/&Env.getBoolLiteralValue(true));
}

/// Sets the nullness properties on `PointerVal` representing a pointer with
/// unknown nullability if not already initialised.
///
/// `Known` is constrained to false, `Null` is unconstrained.
inline void initUnknownPointer(dataflow::PointerValue& PointerVal,
                               dataflow::Environment& Env) {
  initPointerNullState(PointerVal, Env,
                       /*KnownConstraint=*/&Env.getBoolLiteralValue(false));
}

/// Returns true if there is evidence that `PointerVal` may hold a nullptr.
bool isNullable(const dataflow::PointerValue& PointerVal,
                const dataflow::Environment& Env);

/// Returns a human-readable debug representation of a nullability vector.
std::string nullabilityToString(ArrayRef<NullabilityKind> Nullability);

/// A function that may provide enhanced nullability information for a
/// substituted template parameter (which has no sugar of its own).
using GetTypeParamNullability = std::optional<std::vector<NullabilityKind>>(
    const SubstTemplateTypeParmType* ST);
/// Traverse over a type to get its nullability. For example, if T is the type
/// Struct3Arg<int * _Nonnull, int, pair<int * _Nullable, int *>> * _Nonnull,
/// the resulting nullability annotations will be {_Nonnull, _Nonnull,
/// _Nullable, _Unknown}. Note that non-pointer elements (e.g., the second
/// argument of Struct3Arg) do not get a nullability annotation.
std::vector<NullabilityKind> getNullabilityAnnotationsFromType(
    QualType T,
    llvm::function_ref<GetTypeParamNullability> SubstituteTypeParam = nullptr);

/// Prints QualType's underlying canonical type, annotated with nullability.
/// See rebuildWithNullability().
std::string printWithNullability(QualType, ArrayRef<NullabilityKind>,
                                 ASTContext&);
/// Returns an equivalent type annotated with the provided nullability.
/// Any existing sugar (including nullability) is discarded.
/// rebuildWithNullability(int *, {Nullable}) ==> int * _Nullable.
QualType rebuildWithNullability(QualType, ArrayRef<NullabilityKind>,
                                ASTContext&);

/// Computes the number of pointer slots within a type.
/// Each of these could conceptually be nullable, so this is the length of
/// the nullability vector computed by getNullabilityAnnotationsFromType().
unsigned countPointersInType(QualType T);
unsigned countPointersInType(const Expr* E);
unsigned countPointersInType(TemplateArgument TA);
unsigned countPointersInType(const DeclContext* DC);

QualType exprType(const Expr* E);

std::vector<NullabilityKind> unspecifiedNullability(const Expr* E);

// Work around the lack of Expr.dump() etc with an ostream but no ASTContext.
template <typename T>
void dump(const T& Node, llvm::raw_ostream& OS) {
  clang::ASTDumper(OS, /*ShowColors=*/false).Visit(Node);
}

// Returns the computed nullability for a subexpr of the current expression.
// This is always available as we compute bottom-up.
ArrayRef<NullabilityKind> getNullabilityForChild(
    const Expr* E, TransferState<PointerNullabilityLattice>& State);

}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_NULLABILITY_POINTER_NULLABILITY_H_
