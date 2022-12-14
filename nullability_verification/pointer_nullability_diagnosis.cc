// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability_verification/pointer_nullability_diagnosis.h"

#include "nullability_verification/pointer_nullability.h"
#include "nullability_verification/pointer_nullability_matchers.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/Stmt.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Basic/Specifiers.h"

namespace clang {
namespace tidy {
namespace nullability {

using ast_matchers::MatchFinder;
using dataflow::CFGMatchSwitchBuilder;
using dataflow::Environment;
using dataflow::TransferStateForDiagnostics;

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

llvm::Optional<CFGElement> diagnoseDereference(
    const UnaryOperator* UnaryOp, const MatchFinder::MatchResult&,
    const TransferStateForDiagnostics<PointerNullabilityLattice>& State) {
  if (isNullableOrUntracked(UnaryOp->getSubExpr(), State.Env)) {
    return llvm::Optional<CFGElement>(CFGStmt(UnaryOp));
  }
  return llvm::None;
}

llvm::Optional<CFGElement> diagnoseArrow(
    const MemberExpr* MemberExpr, const MatchFinder::MatchResult& Result,
    const TransferStateForDiagnostics<PointerNullabilityLattice>& State) {
  if (isNullableOrUntracked(MemberExpr->getBase(), State.Env)) {
    return llvm::Optional<CFGElement>(CFGStmt(MemberExpr));
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
llvm::Optional<CFGElement> diagnoseCallExpr(
    const CallExpr* CE, const MatchFinder::MatchResult& Result,
    const TransferStateForDiagnostics<PointerNullabilityLattice>& State) {
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

  return isIncompatibleArgumentList(ParamTypes, Args, State.Env,
                                    *Result.Context)
             ? llvm::Optional<CFGElement>(CFGStmt(CE))
             : llvm::None;
}

llvm::Optional<CFGElement> diagnoseConstructExpr(
    const CXXConstructExpr* CE, const MatchFinder::MatchResult& Result,
    const TransferStateForDiagnostics<PointerNullabilityLattice>& State) {
  auto ConstructorParamTypes = CE->getConstructor()
                                   ->getType()
                                   ->getAs<FunctionProtoType>()
                                   ->getParamTypes();
  ArrayRef<const Expr*> ConstructorArgs(CE->getArgs(), CE->getNumArgs());
  return isIncompatibleArgumentList(ConstructorParamTypes, ConstructorArgs,
                                    State.Env, *Result.Context)
             ? llvm::Optional<CFGElement>(CFGStmt(CE))
             : llvm::None;
}

llvm::Optional<CFGElement> diagnoseReturn(
    const ReturnStmt* RS, const MatchFinder::MatchResult& Result,
    const TransferStateForDiagnostics<PointerNullabilityLattice>& State) {
  auto ReturnType = cast<FunctionDecl>(State.Env.getDeclCtx())->getReturnType();
  assert(ReturnType->isPointerType());

  auto* ReturnExpr = RS->getRetValue();
  assert(ReturnExpr->getType()->isPointerType());

  return isIncompatibleAssignment(ReturnType, ReturnExpr, State.Env,
                                  *Result.Context)
             ? llvm::Optional<CFGElement>(CFGStmt(RS))
             : llvm::None;
}

llvm::Optional<CFGElement> diagnoseMemberInitializer(
    const CXXCtorInitializer* CI, const MatchFinder::MatchResult& Result,
    const TransferStateForDiagnostics<PointerNullabilityLattice>& State) {
  assert(CI->isAnyMemberInitializer());
  auto MemberType = CI->getAnyMember()->getType();
  if (!MemberType->isAnyPointerType()) {
    return llvm::None;
  }
  auto MemberInitExpr = CI->getInit();
  return isIncompatibleAssignment(MemberType, MemberInitExpr, State.Env,
                                  *Result.Context)
             ? llvm::Optional<CFGElement>(CFGInitializer(CI))
             : llvm::None;
}

auto buildDiagnoser() {
  return CFGMatchSwitchBuilder<const dataflow::TransferStateForDiagnostics<
                                   PointerNullabilityLattice>,
                               llvm::Optional<CFGElement>>()
      // (*)
      .CaseOfCFGStmt<UnaryOperator>(isPointerDereference(), diagnoseDereference)
      // (->)
      .CaseOfCFGStmt<MemberExpr>(isPointerArrow(), diagnoseArrow)
      // Check compatibility of parameter assignments
      .CaseOfCFGStmt<CallExpr>(isCallExpr(), diagnoseCallExpr)
      .CaseOfCFGStmt<ReturnStmt>(isPointerReturn(), diagnoseReturn)
      .CaseOfCFGStmt<CXXConstructExpr>(isConstructExpr(), diagnoseConstructExpr)
      .CaseOfCFGInit<CXXCtorInitializer>(isCtorMemberInitializer(),
                                         diagnoseMemberInitializer)
      .Build();
}

}  // namespace

PointerNullabilityDiagnoser::PointerNullabilityDiagnoser()
    : Diagnoser(buildDiagnoser()) {}

}  // namespace nullability
}  // namespace tidy
}  // namespace clang
