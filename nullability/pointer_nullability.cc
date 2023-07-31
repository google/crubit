// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability.h"

#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Formula.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/Specifiers.h"
#include "llvm/ADT/StringRef.h"

namespace clang::tidy::nullability {

using dataflow::AtomicBoolValue;
using dataflow::BoolValue;
using dataflow::Environment;
using dataflow::PointerValue;
using dataflow::StorageLocation;
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

std::pair<AtomicBoolValue &, AtomicBoolValue &> getPointerNullState(
    const PointerValue &PointerVal) {
  auto &FromNullable =
      *cast<AtomicBoolValue>(PointerVal.getProperty(kFromNullable));
  auto &Null = *cast<AtomicBoolValue>(PointerVal.getProperty(kNull));
  return {FromNullable, Null};
}

void initPointerBoolProperty(PointerValue &PointerVal, llvm::StringRef Name,
                             BoolValue *BoolVal, Environment &Env) {
  if (PointerVal.getProperty(Name) != nullptr) return;
  // The property must always be a non-null boolean atom.
  if (!isa_and_nonnull<AtomicBoolValue>(BoolVal)) {
    auto &Atom = Env.makeAtomicBoolValue();
    if (BoolVal)
      Env.addToFlowCondition(
          Env.arena().makeEquals(Atom.formula(), BoolVal->formula()));
    BoolVal = &Atom;
  }
  PointerVal.setProperty(Name, BoolVal ? *BoolVal : Env.makeAtomicBoolValue());
}

void initPointerNullState(PointerValue &PointerVal, Environment &Env,
                          BoolValue *FromNullableConstraint,
                          BoolValue *NullConstraint) {
  initPointerBoolProperty(PointerVal, kFromNullable, FromNullableConstraint,
                          Env);
  initPointerBoolProperty(PointerVal, kNull, NullConstraint, Env);
}

bool isNullable(const PointerValue &PointerVal, const Environment &Env,
                const dataflow::Formula *AdditionalConstraints) {
  auto &A = Env.getDataflowAnalysisContext().arena();
  auto [FromNullable, Null] = getPointerNullState(PointerVal);
  auto *ForseeablyNull = &A.makeAnd(FromNullable.formula(), Null.formula());
  if (AdditionalConstraints)
    ForseeablyNull = &A.makeAnd(*AdditionalConstraints, *ForseeablyNull);
  return !Env.flowConditionImplies(A.makeNot(*ForseeablyNull));
}

NullabilityKind getNullability(const dataflow::PointerValue &PointerVal,
                               const dataflow::Environment &Env,
                               const dataflow::Formula *AdditionalConstraints) {
  auto &A = Env.getDataflowAnalysisContext().arena();
  auto *Null = &getPointerNullState(PointerVal).second.formula();
  if (AdditionalConstraints) Null = &A.makeAnd(*AdditionalConstraints, *Null);
  if (Env.flowConditionImplies(A.makeNot(*Null)))
    return NullabilityKind::NonNull;
  return isNullable(PointerVal, Env, AdditionalConstraints)
             ? NullabilityKind::Nullable
             : NullabilityKind::Unspecified;
}

}  // namespace clang::tidy::nullability
