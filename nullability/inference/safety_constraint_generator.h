// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_INFERENCE_SAFETY_CONSTRAINT_GENERATOR_H_
#define CRUBIT_NULLABILITY_INFERENCE_SAFETY_CONSTRAINT_GENERATOR_H_

#include "nullability/pointer_nullability_lattice.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Expr.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/MatchSwitch.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "llvm/ADT/DenseSet.h"

namespace clang::tidy::nullability {
// Collects constraints that must be satisfiable to make a piece of code
// null-safe.
//
// The nullability properties resulting from prospective new annotations can
// then be combined with the constraints to determine if there is only one valid
// annotation for an unannotated nullability slot.
//
// Intended for use with PointerNullabilityAnalysis, a DataflowAnalysis which
// stores nullability information in properties on PointerValues. The boolean
// expressions collected by CollectConstraints will utilize the boolean
// expressions stored in those properties.
class SafetyConstraintGenerator {
 public:
  using LatticeType = PointerNullabilityLattice;

  SafetyConstraintGenerator();

  // Collects constraints implied by pointer usage in `Element`.
  //
  // Intended for use as a PostVisitCFG after running
  // PointerNullabilityAnalysis. Assumes that `State` includes pointer
  // nullability state as set by PointerNullabilityAnalysis.
  void collectConstraints(
      const clang::CFGElement &Element,
      const clang::dataflow::DataflowAnalysisState<LatticeType> &State,
      clang::ASTContext &Context);

  // Retrieves constraints gathered thus far. Until all analyzed CFGElements
  // have been processed by `collectConstraints`, the return value will not
  // represent all safety constraints implied by the code.
  //
  // Intended for use after the completion of the DataflowAnalysis and
  // PostVisitCFG process.
  //
  // Constraints take the form of boolean expressions that must be satisfiable
  // in order for the processed code to be null-safe.
  const llvm::DenseSet<clang::dataflow::BoolValue *> &constraints() const {
    return Constraints;
  }

 private:
  llvm::DenseSet<clang::dataflow::BoolValue *> Constraints;
  clang::dataflow::CFGMatchSwitch<
      const clang::dataflow::TransferStateForDiagnostics<LatticeType>,
      clang::dataflow::BoolValue *>
      ConstraintCollector;
};
}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_INFERENCE_SAFETY_CONSTRAINT_GENERATOR_H_
