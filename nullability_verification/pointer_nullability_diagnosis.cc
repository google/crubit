// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability_verification/pointer_nullability_diagnosis.h"

#include "nullability_verification/pointer_nullability.h"
#include "nullability_verification/pointer_nullability_matchers.h"
#include "clang/AST/Expr.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/Basic/Specifiers.h"

namespace clang {
namespace tidy {
namespace nullability {

using ast_matchers::MatchFinder;
using dataflow::Environment;

namespace {

llvm::Optional<const Stmt*> diagnosePointerAccess(const Stmt* PointerAccessExpr,
                                                  const Expr* PointerExpr,
                                                  const Environment& Env) {
  auto* PointerVal = getPointerValueFromExpr(PointerExpr, Env);
  if (!PointerVal || isNullable(*PointerVal, Env)) {
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

// TODO(b/233582219): Handle call expressions whose callee is not a decl (e.g.
// a function returned from another function), or when the callee cannot be
// interpreted as a function type (e.g. a pointer to a function pointer).
llvm::Optional<const Stmt*> diagnoseCallExpr(
    const CallExpr* CE, const MatchFinder::MatchResult& Result,
    const Environment& Env) {
  auto* Callee = CE->getCalleeDecl();
  if (!Callee) return llvm::None;

  auto* CalleeType = Callee->getFunctionType();
  if (!CalleeType) return llvm::None;

  auto ParamTypes = CalleeType->getAs<FunctionProtoType>()->getParamTypes();

  for (unsigned int I = 0; I < ParamTypes.size(); ++I) {
    auto ParamType = ParamTypes[I];
    if (!ParamType->isAnyPointerType() ||
        ParamType->getNullability(*Result.Context)
                .value_or(NullabilityKind::Unspecified) !=
            NullabilityKind::NonNull) {
      continue;
    }
    auto* Arg = CE->getArg(I);
    auto* PointerVal = getPointerValueFromExpr(Arg, Env);
    if (!PointerVal || isNullable(*PointerVal, Env)) {
      return llvm::Optional<const Stmt*>(CE);
    }
  }

  return llvm::None;
}

auto buildDiagnoser() {
  return dataflow::MatchSwitchBuilder<const Environment,
                                      llvm::Optional<const Stmt*>>()
      // (*)
      .CaseOf<UnaryOperator>(isPointerDereference(), diagnoseDereference)
      // (->)
      .CaseOf<MemberExpr>(isPointerArrow(), diagnoseArrow)
      // Check compatibility of parameter assignments
      .CaseOf<CallExpr>(isCallExpr(), diagnoseCallExpr)
      .Build();
}

}  // namespace

PointerNullabilityDiagnoser::PointerNullabilityDiagnoser()
    : Diagnoser(buildDiagnoser()) {}

}  // namespace nullability
}  // namespace tidy
}  // namespace clang
