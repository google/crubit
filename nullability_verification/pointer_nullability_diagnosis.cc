// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability_verification/pointer_nullability_diagnosis.h"

#include "nullability_verification/pointer_nullability.h"
#include "nullability_verification/pointer_nullability_matchers.h"
#include "clang/AST/Expr.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"

namespace clang {
namespace tidy {
namespace nullability {

using ast_matchers::MatchFinder;
using dataflow::Environment;

namespace {

llvm::Optional<const Stmt*> diagnosePointerAccess(const Stmt* PointerAccessExpr,
                                                  const Expr* PointerExpr,
                                                  const Environment& Env) {
  auto [PointerKnown, PointerNotNull] = getPointerNullState(PointerExpr, Env);
  auto& PointerNotKnownNull =
      Env.makeNot(Env.makeAnd(PointerKnown, Env.makeNot(PointerNotNull)));
  if (!Env.flowConditionImplies(PointerNotKnownNull)) {
    return PointerAccessExpr;
  }
  return llvm::None;
}

llvm::Optional<const Stmt*> diagnoseDereference(const UnaryOperator* UnaryOp,
                                                const MatchFinder::MatchResult&,
                                                const Environment& Env) {
  return diagnosePointerAccess(UnaryOp, UnaryOp->getSubExpr(), Env);
}

llvm::Optional<const Stmt*> diagnoseArrow(
    const MemberExpr* MemberExpr, const MatchFinder::MatchResult& Result,
    const Environment& Env) {
  return diagnosePointerAccess(MemberExpr, MemberExpr->getBase(), Env);
}

auto buildDiagnoser() {
  return dataflow::MatchSwitchBuilder<const Environment,
                                      llvm::Optional<const Stmt*>>()
      // (*)
      .CaseOf<UnaryOperator>(isPointerDereference(), diagnoseDereference)
      // (->)
      .CaseOf<MemberExpr>(isPointerArrow(), diagnoseArrow)
      .Build();
}

}  // namespace

PointerNullabilityDiagnoser::PointerNullabilityDiagnoser()
    : Diagnoser(buildDiagnoser()) {}

}  // namespace nullability
}  // namespace tidy
}  // namespace clang