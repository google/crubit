// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability.h"

#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "llvm/ADT/StringRef.h"

namespace clang {
namespace tidy {
namespace nullability {

using dataflow::AtomicBoolValue;
using dataflow::BoolValue;
using dataflow::Environment;
using dataflow::PointerValue;
using dataflow::StorageLocation;
using dataflow::Value;

/// The nullness information of a pointer is represented by two properties
/// which indicate if a pointer's nullability (i.e., if the pointer can hold
/// null) is `Known` and if the pointer's value is `Null`.
constexpr llvm::StringLiteral kKnown = "is_known";
constexpr llvm::StringLiteral kNull = "is_null";

NullabilityKind getNullabilityKind(QualType Type, ASTContext& Ctx) {
  return Type->getNullability().value_or(NullabilityKind::Unspecified);
}

PointerValue* getPointerValueFromExpr(const Expr* PointerExpr,
                                      const Environment& Env) {
  Value* Val = nullptr;
  if (PointerExpr->isGLValue()) {
    StorageLocation* Loc = Env.getStorageLocationStrict(*PointerExpr);
    if (Loc == nullptr) return nullptr;
    Val = Env.getValue(*Loc);
  } else {
    Val = Env.getValueStrict(*PointerExpr);
  }
  return cast_or_null<PointerValue>(Val);
}

bool hasPointerNullState(const dataflow::PointerValue& PointerVal) {
  return PointerVal.getProperty(kKnown) != nullptr &&
         PointerVal.getProperty(kNull) != nullptr;
}

std::pair<AtomicBoolValue&, AtomicBoolValue&> getPointerNullState(
    const PointerValue& PointerVal) {
  auto& PointerKnown = *cast<AtomicBoolValue>(PointerVal.getProperty(kKnown));
  auto& PointerNull = *cast<AtomicBoolValue>(PointerVal.getProperty(kNull));
  return {PointerKnown, PointerNull};
}

void initPointerBoolProperty(PointerValue& PointerVal, llvm::StringRef Name,
                             BoolValue* BoolVal, Environment& Env) {
  if (PointerVal.getProperty(Name) != nullptr) return;
  // The property must always be a non-null boolean atom.
  if (!isa_and_nonnull<AtomicBoolValue>(BoolVal)) {
    auto& Atom = Env.makeAtomicBoolValue();
    if (BoolVal) Env.addToFlowCondition(Env.makeIff(Atom, *BoolVal));
    BoolVal = &Atom;
  }
  PointerVal.setProperty(Name, BoolVal ? *BoolVal : Env.makeAtomicBoolValue());
}

void initPointerNullState(PointerValue& PointerVal, Environment& Env,
                          BoolValue* KnownConstraint,
                          BoolValue* NullConstraint) {
  initPointerBoolProperty(PointerVal, kKnown, KnownConstraint, Env);
  initPointerBoolProperty(PointerVal, kNull, NullConstraint, Env);
}

bool isNullable(const PointerValue& PointerVal, const Environment& Env) {
  auto [PointerKnown, PointerNull] = getPointerNullState(PointerVal);
  auto& PointerNotKnownNull =
      Env.makeNot(Env.makeAnd(PointerKnown, PointerNull));
  return !Env.flowConditionImplies(PointerNotKnownNull);
}

}  // namespace nullability
}  // namespace tidy
}  // namespace clang
