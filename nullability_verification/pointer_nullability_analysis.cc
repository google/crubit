// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability_verification/pointer_nullability_analysis.h"

#include <string>

#include "common/check.h"
#include "nullability_verification/pointer_nullability_lattice.h"
#include "nullability_verification/pointer_nullability_matchers.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Expr.h"
#include "clang/AST/OperationKinds.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Type.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/MatchSwitch.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Basic/LLVM.h"

namespace clang {
namespace tidy {
namespace nullability {

using ast_matchers::MatchFinder;
using dataflow::AtomicBoolValue;
using dataflow::BoolValue;
using dataflow::Environment;
using dataflow::MatchSwitchBuilder;
using dataflow::PointerValue;
using dataflow::SkipPast;
using dataflow::TransferState;
using dataflow::Value;

namespace {

constexpr llvm::StringLiteral kKnown = "is_known";
constexpr llvm::StringLiteral kNotNull = "is_notnull";

std::pair<AtomicBoolValue&, AtomicBoolValue&> getPointerNullState(
    const Expr* PointerExpr, TransferState<PointerNullabilityLattice>& State) {
  auto* PointerVal =
      cast<PointerValue>(State.Env.getValue(*PointerExpr, SkipPast::Reference));
  auto& PointerKnown = *cast<AtomicBoolValue>(PointerVal->getProperty(kKnown));
  auto& PointerNotNull =
      *cast<AtomicBoolValue>(PointerVal->getProperty(kNotNull));
  return {PointerKnown, PointerNotNull};
}

void initPointerBoolProperty(PointerValue& PointerVal, llvm::StringRef Name,
                             BoolValue* BoolVal, Environment& Env) {
  if (PointerVal.getProperty(Name) == nullptr) {
    PointerVal.setProperty(Name,
                           BoolVal ? *BoolVal : Env.makeAtomicBoolValue());
  }
}

/// The nullness information of a pointer is represented by two properties which
/// indicate if a pointer's nullability (i.e. if the pointer can hold null) is
/// `Known` and if the pointer's value is `NotNull`.
void initPointerNullState(const Expr* PointerExpr,
                          TransferState<PointerNullabilityLattice>& State,
                          BoolValue* Known, BoolValue* NotNull = nullptr) {
  if (auto* PointerVal = cast_or_null<PointerValue>(
          State.Env.getValue(*PointerExpr, SkipPast::Reference))) {
    initPointerBoolProperty(*PointerVal, kKnown, Known, State.Env);
    initPointerBoolProperty(*PointerVal, kNotNull, NotNull, State.Env);
  }
}

void transferInitNotNullPointer(
    const Expr* NotNullPointer, const MatchFinder::MatchResult&,
    TransferState<PointerNullabilityLattice>& State) {
  initPointerNullState(NotNullPointer, State,
                       /*Known=*/&State.Env.getBoolLiteralValue(true),
                       /*NotNull=*/&State.Env.getBoolLiteralValue(true));
}

void transferInitNullPointer(const Expr* NullPointer,
                             const MatchFinder::MatchResult&,
                             TransferState<PointerNullabilityLattice>& State) {
  initPointerNullState(NullPointer, State,
                       /*Known=*/&State.Env.getBoolLiteralValue(true),
                       /*NotNull=*/&State.Env.getBoolLiteralValue(false));
}

void transferInitNullablePointer(
    const Expr* NullablePointer,
    TransferState<PointerNullabilityLattice>& State) {
  initPointerNullState(NullablePointer, State,
                       /*Known=*/&State.Env.getBoolLiteralValue(true));
}

void transferInitPointerFromDecl(
    const Expr* PointerExpr, const MatchFinder::MatchResult&,
    TransferState<PointerNullabilityLattice>& State) {
  // TODO(wyt): Implement processing of nullability annotations. The current
  // implementation treats unnannotated pointers as nullable.
  transferInitNullablePointer(PointerExpr, State);
}

void transferPointerAccess(const Expr* PointerExpr,
                           TransferState<PointerNullabilityLattice>& State) {
  auto [PointerKnown, PointerNotNull] = getPointerNullState(PointerExpr, State);
  auto& PointerNotKnownNull = State.Env.makeNot(
      State.Env.makeAnd(PointerKnown, State.Env.makeNot(PointerNotNull)));
  if (!State.Env.flowConditionImplies(PointerNotKnownNull)) {
    State.Lattice.addViolation(PointerExpr);
  }
}

void transferDereference(const UnaryOperator* UnaryOp,
                         const MatchFinder::MatchResult&,
                         TransferState<PointerNullabilityLattice>& State) {
  transferPointerAccess(UnaryOp->getSubExpr(), State);
}

void transferMemberExprInvolvingPointers(
    const MemberExpr* MemberExpr, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  if (MemberExpr->isArrow()) {
    // Base expr is a pointer, check that (->) access is safe
    transferPointerAccess(MemberExpr->getBase(), State);
  }
  if (MemberExpr->getType()->isAnyPointerType()) {
    // Accessed member is a pointer, initialise its nullability
    transferInitPointerFromDecl(MemberExpr, Result, State);
  }
}

// TODO(wyt): Implement promotion of nullability knownness for initially unknown
// pointers when there is evidence that it is nullable, for example when the
// pointer is compared to nullptr, or casted to boolean.
void transferNullCheckComparison(
    const BinaryOperator* BinaryOp, const MatchFinder::MatchResult& result,
    TransferState<PointerNullabilityLattice>& State) {
  // Boolean representing the comparison between the two pointer values,
  // automatically created by the dataflow framework
  auto& PointerComparison =
      *cast<BoolValue>(State.Env.getValue(*BinaryOp, SkipPast::None));

  CHECK(BinaryOp->getOpcode() == BO_EQ || BinaryOp->getOpcode() == BO_NE);
  auto& PointerEQ = BinaryOp->getOpcode() == BO_EQ
                        ? PointerComparison
                        : State.Env.makeNot(PointerComparison);
  auto& PointerNE = BinaryOp->getOpcode() == BO_EQ
                        ? State.Env.makeNot(PointerComparison)
                        : PointerComparison;

  auto [LHSKnown, LHSNotNull] = getPointerNullState(BinaryOp->getLHS(), State);
  auto [RHSKnown, RHSNotNull] = getPointerNullState(BinaryOp->getRHS(), State);
  auto& LHSKnownNotNull = State.Env.makeAnd(LHSKnown, LHSNotNull);
  auto& RHSKnownNotNull = State.Env.makeAnd(RHSKnown, RHSNotNull);
  auto& LHSKnownNull =
      State.Env.makeAnd(LHSKnown, State.Env.makeNot(LHSNotNull));
  auto& RHSKnownNull =
      State.Env.makeAnd(RHSKnown, State.Env.makeNot(RHSNotNull));

  // nullptr == nullptr
  State.Env.addToFlowCondition(State.Env.makeImplication(
      State.Env.makeAnd(LHSKnownNull, RHSKnownNull), PointerEQ));
  // nullptr != notnull
  State.Env.addToFlowCondition(State.Env.makeImplication(
      State.Env.makeAnd(LHSKnownNull, RHSKnownNotNull), PointerNE));
  // notnull != nullptr
  State.Env.addToFlowCondition(State.Env.makeImplication(
      State.Env.makeAnd(LHSKnownNotNull, RHSKnownNull), PointerNE));
}

void transferNullCheckImplicitCastPtrToBool(
    const Expr* CastExpr, const MatchFinder::MatchResult&,
    TransferState<PointerNullabilityLattice>& State) {
  auto [PointerKnown, PointerNotNull] =
      getPointerNullState(CastExpr->IgnoreImplicit(), State);
  auto& CastExprLoc = State.Env.createStorageLocation(*CastExpr);
  State.Env.setValue(CastExprLoc, PointerNotNull);
  State.Env.setStorageLocation(*CastExpr, CastExprLoc);
}

auto buildTransferer() {
  return MatchSwitchBuilder<TransferState<PointerNullabilityLattice>>()
      // Handles initialization of the null states of pointers
      .CaseOf<Expr>(isPointerVariableReference(), transferInitPointerFromDecl)
      .CaseOf<Expr>(isCXXThisExpr(), transferInitNotNullPointer)
      .CaseOf<Expr>(isAddrOf(), transferInitNotNullPointer)
      .CaseOf<Expr>(isNullPointerLiteral(), transferInitNullPointer)
      // Handles initialization of null states of member pointers and safety of
      // member access (->) on pointers
      .CaseOf<MemberExpr>(isMemberExprInvolvingPointers(),
                          transferMemberExprInvolvingPointers)
      // Handles pointer dereferencing (*ptr)
      .CaseOf<UnaryOperator>(isPointerDereference(), transferDereference)
      // Handles comparison between 2 pointers
      .CaseOf<BinaryOperator>(isPointerCheckBinOp(),
                              transferNullCheckComparison)
      // Handles checking of pointer as boolean
      .CaseOf<Expr>(isImplicitCastPointerToBool(),
                    transferNullCheckImplicitCastPtrToBool)
      .Build();
}
}  // namespace

PointerNullabilityAnalysis::PointerNullabilityAnalysis(ASTContext& Context)
    : DataflowAnalysis<PointerNullabilityAnalysis, PointerNullabilityLattice>(
          Context),
      Transferer(buildTransferer()) {}

void PointerNullabilityAnalysis::transfer(const Stmt* Stmt,
                                          PointerNullabilityLattice& Lattice,
                                          Environment& Env) {
  TransferState<PointerNullabilityLattice> State(Lattice, Env);
  Transferer(*Stmt, getASTContext(), State);
}

bool PointerNullabilityAnalysis::merge(QualType Type, const Value& Val1,
                                       const Environment& Env1,
                                       const Value& Val2,
                                       const Environment& Env2,
                                       Value& MergedVal,
                                       Environment& MergedEnv) {
  return false;
}
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
