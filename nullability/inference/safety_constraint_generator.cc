// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/safety_constraint_generator.h"

#include "nullability/pointer_nullability.h"
#include "nullability/pointer_nullability_matchers.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Expr.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/MatchSwitch.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "llvm/ADT/DenseSet.h"

namespace clang::tidy::nullability {
namespace {
clang::dataflow::BoolValue* collectFromDereference(
    const clang::UnaryOperator* Op,
    const clang::ast_matchers::MatchFinder::MatchResult&,
    const clang::dataflow::TransferStateForDiagnostics<
        SafetyConstraintGenerator::LatticeType>& State) {
  if (clang::dataflow::PointerValue* DereferencedValue =
          getPointerValueFromExpr(Op->getSubExpr(), State.Env)) {
    auto& NotIsNull =
        State.Env.makeNot(getPointerNullState(*DereferencedValue).second);
    // If the flow condition at this point in the code implies that the
    // dereferenced value is not null, we can avoid collecting complex flow
    // condition tokens and recognize that regardless of any annotation we could
    // add, the current value is safe to be dereferenced.
    if (!State.Env.flowConditionImplies(NotIsNull)) {
      // If the flow condition is not enough to imply that the dereferenced
      // value is not null, we need to constrain it to be so, and can avoid
      // collecting complex flow condition tokens by simply collecting !is_null.
      //
      // Intuition suggests the alternative of unconditionally collecting the
      // safety condition FlowConditions => !null or the equivalent
      // !(FlowConditions && null), but these can potentially be satisfied by
      // the not-fully-constrained flow conditions being false and the value
      // being null. That expression being satisfiable doesn't mean that it is
      // provably true in all cases. We are collecting safety constraints for
      // which satisfiability is required and of which collective satisfiability
      // is sufficient for null-safety.
      return &NotIsNull;
    }
  }
  return nullptr;
}

auto buildConstraintCollector() {
  return clang::dataflow::CFGMatchSwitchBuilder<
             const clang::dataflow::TransferStateForDiagnostics<
                 SafetyConstraintGenerator::LatticeType>,
             clang::dataflow::BoolValue*>()
      .CaseOfCFGStmt<clang::UnaryOperator>(isPointerDereference(),
                                           collectFromDereference)
      .Build();
}
}  // namespace

SafetyConstraintGenerator::SafetyConstraintGenerator()
    : ConstraintCollector(buildConstraintCollector()) {}

void SafetyConstraintGenerator::collectConstraints(
    const clang::CFGElement& Element,
    const clang::dataflow::DataflowAnalysisState<LatticeType>& State,
    clang::ASTContext& Context) {
  if (auto* Constraint =
          ConstraintCollector(Element, Context, {State.Lattice, State.Env})) {
    Constraints.insert(Constraint);
  }
}
}  // namespace clang::tidy::nullability
