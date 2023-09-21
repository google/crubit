// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability.h"

#include <cassert>
#include <optional>

#include "nullability/type_nullability.h"
#include "clang/AST/Type.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Formula.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/Specifiers.h"
#include "llvm/ADT/StringRef.h"

namespace clang::tidy::nullability {

using dataflow::AtomicBoolValue;
using dataflow::BoolValue;
using dataflow::DataflowAnalysisContext;
using dataflow::Environment;
using dataflow::Formula;
using dataflow::PointerValue;
using dataflow::Value;

/// The nullness information of a pointer is represented by two properties
/// which indicate if its source was nullable, and if its value is null.
constexpr llvm::StringLiteral kFromNullable = "from_nullable";
constexpr llvm::StringLiteral kNull = "is_null";

NullabilityKind getNullabilityKind(QualType Type, ASTContext &Ctx) {
  return Type->getNullability().value_or(NullabilityKind::Unspecified);
}

PointerValue *getPointerValueFromExpr(const Expr *PointerExpr,
                                      const Environment &Env) {
  return cast_or_null<PointerValue>(Env.getValue(*PointerExpr));
}

bool hasPointerNullState(const dataflow::PointerValue &PointerVal) {
  return PointerVal.getProperty(kFromNullable) != nullptr &&
         PointerVal.getProperty(kNull) != nullptr;
}

PointerNullState getPointerNullState(const PointerValue &PointerVal) {
  auto &FromNullable =
      *cast<AtomicBoolValue>(PointerVal.getProperty(kFromNullable));
  auto &Null = *cast<AtomicBoolValue>(PointerVal.getProperty(kNull));
  return {FromNullable.formula(), Null.formula()};
}

static bool tryCreatePointerNullState(PointerValue &PointerVal,
                                      dataflow::Arena &A,
                                      const Formula *FromNullable = nullptr,
                                      const Formula *IsNull = nullptr) {
  // TODO: for now we assume that we have both nullability properties, or none.
  // We'll need to relax this when properties can be independently widened away.
  if (hasPointerNullState(PointerVal)) return false;
  if (!FromNullable) FromNullable = &A.makeAtomRef(A.makeAtom());
  if (!IsNull) IsNull = &A.makeAtomRef(A.makeAtom());
  PointerVal.setProperty(kFromNullable, A.makeBoolValue(*FromNullable));
  PointerVal.setProperty(kNull, A.makeBoolValue(*IsNull));
  return true;
}

void initPointerNullState(PointerValue &PointerVal,
                          DataflowAnalysisContext &Ctx,
                          std::optional<PointerTypeNullability> Source) {
  auto &A = Ctx.arena();
  if (tryCreatePointerNullState(PointerVal, A,
                                Source ? &Source->isNullable(A) : nullptr)) {
    // The symbolic/nonnull check is not needed for correctness, but it avoids
    // adding meaningless (false => !null) invariant clauses.
    // TODO: remove this once such clauses are recognized and dropped.
    if (Source &&
        (Source->isSymbolic() || Source == NullabilityKind::NonNull)) {
      const Formula &IsNull = getPointerNullState(PointerVal).IsNull;
      Ctx.addInvariant(A.makeImplies(Source->isNonnull(A), A.makeNot(IsNull)));
    }
  }
}

void initNullPointer(PointerValue &PointerVal, DataflowAnalysisContext &Ctx) {
  tryCreatePointerNullState(PointerVal, Ctx.arena(),
                            /*FromNullable=*/&Ctx.arena().makeLiteral(true),
                            /*IsNull=*/&Ctx.arena().makeLiteral(true));
}

bool isNullable(const PointerValue &PointerVal, const Environment &Env,
                const dataflow::Formula *AdditionalConstraints) {
  auto &A = Env.getDataflowAnalysisContext().arena();
  auto [FromNullable, Null] = getPointerNullState(PointerVal);
  auto *ForseeablyNull = &A.makeAnd(FromNullable, Null);
  if (AdditionalConstraints)
    ForseeablyNull = &A.makeAnd(*AdditionalConstraints, *ForseeablyNull);
  return !Env.flowConditionImplies(A.makeNot(*ForseeablyNull));
}

NullabilityKind getNullability(const dataflow::PointerValue &PointerVal,
                               const dataflow::Environment &Env,
                               const dataflow::Formula *AdditionalConstraints) {
  auto &A = Env.getDataflowAnalysisContext().arena();
  auto *Null = &getPointerNullState(PointerVal).IsNull;
  if (AdditionalConstraints) Null = &A.makeAnd(*AdditionalConstraints, *Null);
  if (Env.flowConditionImplies(A.makeNot(*Null)))
    return NullabilityKind::NonNull;
  return isNullable(PointerVal, Env, AdditionalConstraints)
             ? NullabilityKind::Nullable
             : NullabilityKind::Unspecified;
}

}  // namespace clang::tidy::nullability
