// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability_verification/pointer_nullability.h"

#include "clang/Analysis/FlowSensitive/Value.h"
#include "llvm/ADT/StringRef.h"

namespace clang {
namespace tidy {
namespace nullability {

using dataflow::AtomicBoolValue;
using dataflow::BoolValue;
using dataflow::Environment;
using dataflow::PointerValue;
using dataflow::SkipPast;

/// The nullness information of a pointer is represented by two properties
/// which indicate if a pointer's nullability (i.e., if the pointer can hold
/// null) is `Known` and if the pointer's value is `NotNull`.
constexpr llvm::StringLiteral kKnown = "is_known";
constexpr llvm::StringLiteral kNotNull = "is_notnull";

PointerValue* getPointerValueFromExpr(const Expr* PointerExpr,
                                      const Environment& Env) {
  return cast_or_null<PointerValue>(
      Env.getValue(*PointerExpr, SkipPast::Reference));
}

std::pair<AtomicBoolValue&, AtomicBoolValue&> getPointerNullState(
    const PointerValue& PointerVal, const Environment& Env) {
  auto& PointerKnown = *cast<AtomicBoolValue>(PointerVal.getProperty(kKnown));
  auto& PointerNotNull =
      *cast<AtomicBoolValue>(PointerVal.getProperty(kNotNull));
  return {PointerKnown, PointerNotNull};
}

void initPointerBoolProperty(PointerValue& PointerVal, llvm::StringRef Name,
                             BoolValue* BoolVal, Environment& Env) {
  if (PointerVal.getProperty(Name) == nullptr) {
    PointerVal.setProperty(Name,
                           BoolVal ? *BoolVal : Env.makeAtomicBoolValue());
  }
}

void initPointerNullState(PointerValue& PointerVal, Environment& Env,
                          BoolValue* KnownConstraint,
                          BoolValue* NotNullConstraint) {
  initPointerBoolProperty(PointerVal, kKnown, KnownConstraint, Env);
  initPointerBoolProperty(PointerVal, kNotNull, NotNullConstraint, Env);
}

bool isNullable(const PointerValue& PointerVal, const Environment& Env) {
  auto [PointerKnown, PointerNotNull] = getPointerNullState(PointerVal, Env);
  auto& PointerNotKnownNull =
      Env.makeNot(Env.makeAnd(PointerKnown, Env.makeNot(PointerNotNull)));
  return !Env.flowConditionImplies(PointerNotKnownNull);
}

}  // namespace nullability
}  // namespace tidy
}  // namespace clang
