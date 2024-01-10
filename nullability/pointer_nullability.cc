// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability.h"

#include <cassert>
#include <optional>

#include "absl/base/nullability.h"
#include "nullability/type_nullability.h"
#include "clang/AST/Type.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Formula.h"
#include "clang/Analysis/FlowSensitive/StorageLocation.h"
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
using dataflow::RecordStorageLocation;
using dataflow::StorageLocation;
using dataflow::TopBoolValue;
using dataflow::Value;

/// The nullness information of a pointer is represented by two properties
/// which indicate if its source was nullable, and if its value is null.
constexpr llvm::StringLiteral kFromNullable = "from_nullable";
constexpr llvm::StringLiteral kNull = "is_null";

absl::Nullable<PointerValue *> getPointerValueFromExpr(
    absl::Nonnull<const Expr *> PointerExpr, const Environment &Env) {
  return Env.get<PointerValue>(*PointerExpr);
}

absl::Nullable<PointerValue *> getPointerValueFromSmartPointer(
    absl::Nullable<RecordStorageLocation *> SmartPointerLoc,
    const Environment &Env) {
  if (SmartPointerLoc == nullptr) return nullptr;
  return Env.get<PointerValue>(SmartPointerLoc->getSyntheticField(PtrField));
}

absl::Nullable<PointerValue *> getPointerValueFromSmartPointerExpr(
    absl::Nonnull<const Expr *> SmartPointerExpr, const Environment &Env) {
  RecordStorageLocation *Loc = nullptr;
  if (SmartPointerExpr->isPRValue())
    Loc = &Env.getResultObjectLocation(*SmartPointerExpr);
  else
    Loc = Env.get<RecordStorageLocation>(*SmartPointerExpr);
  return getPointerValueFromSmartPointer(Loc, Env);
}

void setSmartPointerValue(dataflow::RecordStorageLocation &SmartPointerLoc,
                          absl::Nullable<dataflow::PointerValue *> Val,
                          Environment &Env) {
  StorageLocation &PointerLoc = SmartPointerLoc.getSyntheticField(PtrField);
  if (Val)
    Env.setValue(PointerLoc, *Val);
  else
    Env.clearValue(PointerLoc);
}

void setSmartPointerToNull(dataflow::RecordStorageLocation &SmartPointerLoc,
                           Environment &Env) {
  StorageLocation &PointerLoc = SmartPointerLoc.getSyntheticField(PtrField);
  Env.setValue(PointerLoc,
               createNullPointer(PointerLoc.getType()->getPointeeType(), Env));
}

bool hasPointerNullState(const dataflow::PointerValue &PointerVal) {
  return PointerVal.getProperty(kFromNullable) != nullptr &&
         PointerVal.getProperty(kNull) != nullptr;
}

PointerNullState getPointerNullState(const PointerValue &PointerVal) {
  Value *FromNullableProp = PointerVal.getProperty(kFromNullable);
  Value *NullProp = PointerVal.getProperty(kNull);

  return {
      isa<TopBoolValue>(FromNullableProp)
          ? nullptr
          : &cast<BoolValue>(FromNullableProp)->formula(),
      isa<TopBoolValue>(NullProp) ? nullptr
                                  : &cast<BoolValue>(NullProp)->formula(),
  };
}

static bool tryCreatePointerNullState(
    PointerValue &PointerVal, dataflow::Arena &A,
    absl::Nullable<const Formula *> FromNullable = nullptr,
    absl::Nullable<const Formula *> IsNull = nullptr) {
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
  if (tryCreatePointerNullState(
          PointerVal, A, Source ? &Source->isNullable(A) : nullptr,
          Source == NullabilityKind::NonNull ? &A.makeLiteral(false)
                                             : nullptr)) {
    // The `isSymbolic()` check is not needed for correctness, but it avoids
    // adding meaningless (false => !null) or (true => true) invariant clauses.
    // TODO: remove this once such clauses are recognized and dropped.
    if (Source && Source->isSymbolic()) {
      if (const Formula *IsNull = getPointerNullState(PointerVal).IsNull)
        Ctx.addInvariant(
            A.makeImplies(Source->isNonnull(A), A.makeNot(*IsNull)));
    }
  }
}

void forgetFromNullable(dataflow::PointerValue &PointerVal,
                        DataflowAnalysisContext &Ctx) {
  PointerVal.setProperty(kFromNullable, Ctx.arena().makeTopValue());
}

void forgetIsNull(dataflow::PointerValue &PointerVal,
                  DataflowAnalysisContext &Ctx) {
  PointerVal.setProperty(kNull, Ctx.arena().makeTopValue());
}

void initNullPointer(PointerValue &PointerVal, DataflowAnalysisContext &Ctx) {
  tryCreatePointerNullState(PointerVal, Ctx.arena(),
                            /*FromNullable=*/&Ctx.arena().makeLiteral(true),
                            /*IsNull=*/&Ctx.arena().makeLiteral(true));
}

PointerValue &createNullPointer(QualType PointeeType, Environment &Env) {
  PointerValue &PointerVal = Env.getOrCreateNullPointerValue(PointeeType);
  initNullPointer(PointerVal, Env.getDataflowAnalysisContext());
  return PointerVal;
}

bool isNullable(
    const PointerValue &PointerVal, const Environment &Env,
    absl::Nullable<const dataflow::Formula *> AdditionalConstraints) {
  auto &A = Env.getDataflowAnalysisContext().arena();
  auto [FromNullable, Null] = getPointerNullState(PointerVal);

  // A value is nullable if two things can be simultaneously true:
  // - We got it from a nullable source
  //   (values from unknown sources may be null, but are not nullable)
  // - The value is actually null
  //   (if a value from a nullable source was checked, it's not nullable)
  const Formula *ForseeablyNull = &A.makeLiteral(true);
  if (FromNullable) ForseeablyNull = &A.makeAnd(*ForseeablyNull, *FromNullable);
  if (Null) ForseeablyNull = &A.makeAnd(*ForseeablyNull, *Null);
  if (AdditionalConstraints)
    ForseeablyNull = &A.makeAnd(*ForseeablyNull, *AdditionalConstraints);

  return Env.allows(*ForseeablyNull);
}

NullabilityKind getNullability(
    const dataflow::PointerValue &PointerVal, const dataflow::Environment &Env,
    absl::Nullable<const dataflow::Formula *> AdditionalConstraints) {
  auto &A = Env.getDataflowAnalysisContext().arena();
  if (auto *Null = getPointerNullState(PointerVal).IsNull) {
    if (AdditionalConstraints) Null = &A.makeAnd(*AdditionalConstraints, *Null);
    if (Env.proves(A.makeNot(*Null))) return NullabilityKind::NonNull;
  }
  return isNullable(PointerVal, Env, AdditionalConstraints)
             ? NullabilityKind::Nullable
             : NullabilityKind::Unspecified;
}

NullabilityKind getNullability(const Expr *E, const dataflow::Environment &Env,
                               const dataflow::Formula *AdditionalConstraints) {
  dataflow::PointerValue *P = nullptr;
  if (isSupportedRawPointerType(E->getType()))
    P = getPointerValueFromExpr(E, Env);
  else if (isSupportedSmartPointerType(E->getType()))
    P = getPointerValueFromSmartPointer(Env.get<RecordStorageLocation>(*E),
                                        Env);
  if (P != nullptr) return getNullability(*P, Env, AdditionalConstraints);
  return clang::NullabilityKind::Unspecified;
}

}  // namespace clang::tidy::nullability
