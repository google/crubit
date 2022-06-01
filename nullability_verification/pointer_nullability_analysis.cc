// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability_verification/pointer_nullability_analysis.h"

#include <iostream>
#include <string>

#include "common/check.h"
#include "nullability_verification/pointer_nullability_lattice.h"
#include "nullability_verification/pointer_nullability_matchers.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Expr.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/MatchSwitch.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Basic/LLVM.h"

namespace clang {
namespace tidy {
namespace nullability {

using ast_matchers::MatchFinder;
using dataflow::BoolValue;
using dataflow::Environment;
using dataflow::MatchSwitchBuilder;
using dataflow::PointerValue;
using dataflow::SkipPast;
using dataflow::TransferState;
using dataflow::Value;

namespace {

void initialisePointerNullability(
    const Expr* Expr, const MatchFinder::MatchResult&,
    TransferState<PointerNullabilityLattice>& State) {
  if (auto* PointerVal = cast_or_null<PointerValue>(
          State.Env.getValue(*Expr, SkipPast::Reference))) {
    if (!State.Lattice.hasPointerNullability(PointerVal)) {
      State.Lattice.setPointerNullability(PointerVal,
                                          &State.Env.makeAtomicBoolValue());
    }
  }
}

void transferDereference(const UnaryOperator* UnaryOp,
                         const MatchFinder::MatchResult&,
                         TransferState<PointerNullabilityLattice>& State) {
  auto* PointerExpr = UnaryOp->getSubExpr();
  if (auto* PointerVal = cast_or_null<PointerValue>(
          State.Env.getValue(*PointerExpr, SkipPast::Reference))) {
    auto PointerNullability = State.Lattice.getPointerNullability(PointerVal);
    CHECK(PointerNullability != nullptr);
    if (State.Env.flowConditionImplies(*PointerNullability)) {
      return;
    }
  }
  State.Lattice.addViolation(PointerExpr);
}

void transferNullCheckComparison(
    const Expr* NullCheck, const Expr* PointerExpr,
    TransferState<PointerNullabilityLattice>& State) {
  if (auto* PointerVal = cast_or_null<PointerValue>(
          State.Env.getValue(*PointerExpr, SkipPast::Reference))) {
    auto* PointerNullability = State.Lattice.getPointerNullability(PointerVal);
    CHECK(PointerNullability != nullptr);

    // For binary operations, the dataflow framework automatically creates a
    // corresponding BoolVal
    auto* ExistingDFVal =
        cast_or_null<BoolValue>(State.Env.getValue(*NullCheck, SkipPast::None));
    CHECK(ExistingDFVal != nullptr);
    State.Env.addToFlowCondition(
        State.Env.makeIff(*ExistingDFVal, *PointerNullability));
  }
}

void transferNullCheckImplicitCastPtrToBool(
    const Expr* CastExpr, const MatchFinder::MatchResult&,
    TransferState<PointerNullabilityLattice>& State) {
  if (auto* PointerVal = cast_or_null<PointerValue>(State.Env.getValue(
          *CastExpr->IgnoreImplicit(), SkipPast::Reference))) {
    auto* PointerNullability = State.Lattice.getPointerNullability(PointerVal);
    CHECK(PointerNullability != nullptr);

    auto& CastExprLoc = State.Env.createStorageLocation(*CastExpr);
    State.Env.setValue(CastExprLoc, *PointerNullability);
    State.Env.setStorageLocation(*CastExpr, CastExprLoc);
  }
}

auto buildTransferer() {
  return MatchSwitchBuilder<TransferState<PointerNullabilityLattice>>()
      // Initialise nullability state of pointers
      .CaseOf<Expr>(isPointerExpr(), initialisePointerNullability)
      // Pointer dereference
      .CaseOf<UnaryOperator>(isPointerDereference(), transferDereference)
      // Nullability check
      .CaseOf<BinaryOperator>(
          isNEQNullBinOp(/*BindID=*/"pointer"),
          [](const BinaryOperator* binOp,
             const MatchFinder::MatchResult& result,
             TransferState<PointerNullabilityLattice>& State) {
            transferNullCheckComparison(
                binOp, result.Nodes.getNodeAs<Expr>("pointer"), State);
          })
      .CaseOf<Expr>(isImplicitCastPtrToBool(),
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
