// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability_verification/pointer_nullability_diagnosis.h"

#include "nullability_verification/pointer_nullability.h"
#include "nullability_verification/pointer_nullability_matchers.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/Stmt.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Basic/Specifiers.h"

namespace clang {
namespace tidy {
namespace nullability {

using ast_matchers::MatchFinder;
using dataflow::Environment;

namespace {

// Returns true if `Expr` is uninterpreted or known to be nullable.
bool isNullableOrUntracked(const Expr* E, const Environment& Env) {
  auto* ActualVal = getPointerValueFromExpr(E, Env);
  return !ActualVal || isNullable(*ActualVal, Env);
}

// Returns true if an uninterpreted or nullable `Expr` was assigned to a
// construct with a non-null `DeclaredType`.
bool isIncompatibleAssignment(QualType DeclaredType, const Expr* E,
                              const Environment& Env, ASTContext& Ctx) {
  assert(DeclaredType->isAnyPointerType());
  return getNullabilityKind(DeclaredType, Ctx) == NullabilityKind::NonNull &&
         isNullableOrUntracked(E, Env);
}

llvm::Optional<const Stmt*> diagnoseDereference(const UnaryOperator* UnaryOp,
                                                const MatchFinder::MatchResult&,
                                                const Environment& Env) {
  if (isNullableOrUntracked(UnaryOp->getSubExpr(), Env)) {
    return UnaryOp;
  }
  return llvm::None;
}

llvm::Optional<const Stmt*> diagnoseArrow(
    const MemberExpr* MemberExpr, const MatchFinder::MatchResult& Result,
    const Environment& Env) {
  if (isNullableOrUntracked(MemberExpr->getBase(), Env)) {
    return MemberExpr;
  }
  return llvm::None;
}

bool isIncompatibleArgumentList(ArrayRef<QualType> ParamTypes,
                                ArrayRef<const Expr*> Args,
                                const Environment& Env, ASTContext& Ctx) {
  assert(ParamTypes.size() == Args.size());
  for (unsigned int I = 0; I < Args.size(); ++I) {
    auto ParamType = ParamTypes[I].getNonReferenceType();
    if (!ParamType->isAnyPointerType()) {
      continue;
    }
    if (isIncompatibleAssignment(ParamType, Args[I], Env, Ctx)) {
      return true;
    }
  }
  return false;
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
  ArrayRef<const Expr*> Args(CE->getArgs(), CE->getNumArgs());
  if (isa<CXXOperatorCallExpr>(CE)) {
    // The first argument of an operator call expression is the operand which
    // does not appear in the list of parameter types.
    Args = Args.drop_front();
  }

  return isIncompatibleArgumentList(ParamTypes, Args, Env, *Result.Context)
             ? llvm::Optional<const Stmt*>(CE)
             : llvm::None;
}

llvm::Optional<const Stmt*> diagnoseReturn(
    const ReturnStmt* RS, const MatchFinder::MatchResult& Result,
    const Environment& Env) {
  auto ReturnType = cast<FunctionDecl>(Env.getDeclCtx())->getReturnType();
  assert(ReturnType->isPointerType());

  auto* ReturnExpr = RS->getRetValue();
  assert(ReturnExpr->getType()->isPointerType());

  return isIncompatibleAssignment(ReturnType, ReturnExpr, Env, *Result.Context)
             ? llvm::Optional<const Stmt*>(RS)
             : llvm::None;
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
      .CaseOf<ReturnStmt>(isPointerReturn(), diagnoseReturn)
      .Build();
}

}  // namespace

PointerNullabilityDiagnoser::PointerNullabilityDiagnoser()
    : Diagnoser(buildDiagnoser()) {}

}  // namespace nullability
}  // namespace tidy
}  // namespace clang
