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

PointerValue *absl_nullable getRawPointerValue(
    const Expr *absl_nonnull PointerExpr, const Environment &Env) {
  return Env.get<PointerValue>(*PointerExpr);
}

PointerValue *absl_nullable getPointerValueFromSmartPointer(
    RecordStorageLocation *absl_nullable SmartPointerLoc,
    const Environment &Env) {
  if (SmartPointerLoc == nullptr) return nullptr;
  return Env.get<PointerValue>(SmartPointerLoc->getSyntheticField(PtrField));
}

PointerValue *absl_nullable getSmartPointerValue(
    const Expr *absl_nonnull SmartPointerExpr, const Environment &Env) {
  RecordStorageLocation *Loc = nullptr;
  if (SmartPointerExpr->isPRValue())
    Loc = &Env.getResultObjectLocation(*SmartPointerExpr);
  else
    Loc = Env.get<RecordStorageLocation>(*SmartPointerExpr);
  return getPointerValueFromSmartPointer(Loc, Env);
}

dataflow::PointerValue *absl_nullable getPointerValue(
    const Expr *absl_nonnull PointerExpr, const Environment &Env) {
  QualType Ty = PointerExpr->getType();
  if (Ty->isNullPtrType() || isSupportedRawPointerType(Ty))
    return getRawPointerValue(PointerExpr, Env);
  return getSmartPointerValue(PointerExpr, Env);
}

void setSmartPointerValue(dataflow::RecordStorageLocation &SmartPointerLoc,
                          dataflow::PointerValue *absl_nullable Val,
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

  assert(FromNullableProp != nullptr && NullProp != nullptr &&
         "PointerVal is missing null state!");

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
    const Formula *absl_nullable FromNullable = nullptr,
    const Formula *absl_nullable IsNull = nullptr) {
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

void initPointerNullState(PointerValue &PointerVal,
                          DataflowAnalysisContext &Ctx,
                          PointerNullState State) {
  assert(!hasPointerNullState(PointerVal));

  auto &A = Ctx.arena();

  // Internally, we encode "top" as `TopBoolValue`.
  BoolValue &FromNullable = State.FromNullable != nullptr
                                ? A.makeBoolValue(*State.FromNullable)
                                : A.makeTopValue();
  BoolValue &IsNull = State.IsNull != nullptr ? A.makeBoolValue(*State.IsNull)
                                              : A.makeTopValue();
  PointerVal.setProperty(kFromNullable, FromNullable);
  PointerVal.setProperty(kNull, IsNull);
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

static bool isPointerStateNullable(
    PointerNullState PointerNullState, const Environment &Env,
    const dataflow::Formula *absl_nullable AdditionalConstraints) {
  auto &A = Env.getDataflowAnalysisContext().arena();
  auto [FromNullable, Null] = PointerNullState;

  // A value is nullable if either of two things is true:
  // - The value is provably null under satisfiable flow conditions
  //   (a value from an unknown source becomes nullable if provably null)
  // - We got it from a nullable source and it may be actually null
  //   (if a value from a nullable source was checked, it's not nullable)
  //
  // Notably, a value from an unknown source that may be null, but is not
  // provably null, is not considered nullable. Values from non-null sources are
  // never considered nullable because being able to prove them null can only
  // occur under unsatisfiable flow conditions.
  if (Null) {
    const Formula *ProvablyNull = Null;
    if (AdditionalConstraints)
      ProvablyNull = &A.makeImplies(*AdditionalConstraints, *ProvablyNull);

    if (Env.proves(*ProvablyNull)) {
      // If we're in an environment with false flow conditions, we can prove
      // anything, but don't want to consider this value Nullable and end up
      // producing diagnostics in unreachable code.
      if (Env.proves(A.makeLiteral(false))) return false;
      return true;
    }
  }

  const Formula *NullableAndMaybeNull = &A.makeLiteral(true);
  if (FromNullable)
    NullableAndMaybeNull = &A.makeAnd(*NullableAndMaybeNull, *FromNullable);
  if (Null) NullableAndMaybeNull = &A.makeAnd(*NullableAndMaybeNull, *Null);
  if (AdditionalConstraints)
    NullableAndMaybeNull =
        &A.makeAnd(*NullableAndMaybeNull, *AdditionalConstraints);

  return Env.allows(*NullableAndMaybeNull);
}

bool isNullable(const PointerValue &PointerVal, const Environment &Env,
                const dataflow::Formula *absl_nullable AdditionalConstraints) {
  return isPointerStateNullable(getPointerNullState(PointerVal), Env,
                                AdditionalConstraints);
}

bool isReachableNullptrLiteral(
    const Environment &Env,
    const dataflow::Formula *absl_nullable AdditionalConstraints) {
  auto &A = Env.getDataflowAnalysisContext().arena();
  PointerNullState NullState = {/*FromNullable=*/&A.makeLiteral(true),
                                /*IsNull=*/&A.makeLiteral(true)};
  return isPointerStateNullable(NullState, Env, AdditionalConstraints);
}

NullabilityKind getNullability(
    const dataflow::PointerValue &PointerVal, const dataflow::Environment &Env,
    const dataflow::Formula *absl_nullable AdditionalConstraints) {
  auto &A = Env.getDataflowAnalysisContext().arena();
  if (auto *Null = getPointerNullState(PointerVal).IsNull) {
    if (AdditionalConstraints) Null = &A.makeAnd(*AdditionalConstraints, *Null);
    if (Env.proves(A.makeNot(*Null))) return NullabilityKind::NonNull;
  }
  return isNullable(PointerVal, Env, AdditionalConstraints)
             ? NullabilityKind::Nullable
             : NullabilityKind::Unspecified;
}

NullabilityKind getNullability(
    const Expr *absl_nonnull E, const dataflow::Environment &Env,
    const dataflow::Formula *absl_nullable AdditionalConstraints) {
  if (dataflow::PointerValue *P = getPointerValue(E, Env))
    return getNullability(*P, Env, AdditionalConstraints);
  return clang::NullabilityKind::Unspecified;
}

NullabilityKind getNullabilityForNullptrT(
    const dataflow::Environment &Env,
    const dataflow::Formula *absl_nullable AdditionalConstraints) {
  return isReachableNullptrLiteral(Env, AdditionalConstraints)
             ? NullabilityKind::Nullable
             : NullabilityKind::Unspecified;
}

}  // namespace clang::tidy::nullability
