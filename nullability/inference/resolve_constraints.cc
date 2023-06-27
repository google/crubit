// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/resolve_constraints.h"

#include "nullability/inference/inference.proto.h"
#include "nullability/pointer_nullability.h"
#include "clang/Analysis/FlowSensitive/Solver.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Analysis/FlowSensitive/WatchedLiteralsSolver.h"
#include "llvm/ADT/DenseSet.h"

namespace clang::tidy::nullability {
namespace {
bool isSatisfiable(
    const llvm::DenseSet<clang::dataflow::BoolValue *> &ConstraintSet) {
  clang::dataflow::WatchedLiteralsSolver Solver;
  std::vector<clang::dataflow::BoolValue *> Vec(ConstraintSet.begin(),
                                                ConstraintSet.end());
  return Solver.solve(Vec).getStatus() ==
         clang::dataflow::Solver::Result::Status::Satisfiable;
}

bool isUnsatisfiable(
    const llvm::DenseSet<clang::dataflow::BoolValue *> &ConstraintSet) {
  clang::dataflow::WatchedLiteralsSolver Solver;
  std::vector<clang::dataflow::BoolValue *> Vec(ConstraintSet.begin(),
                                                ConstraintSet.end());
  return Solver.solve(Vec).getStatus() ==
         clang::dataflow::Solver::Result::Status::Unsatisfiable;
}
}  // namespace

NullabilityConstraint resolveConstraints(
    const llvm::DenseSet<clang::dataflow::BoolValue *> &SafetyConstraints,
    const clang::dataflow::PointerValue &Pointer) {
  // If the safety constraints are satisfiable, then we can potentially
  // add annotations that are necessary for them to be satisfied. If they
  // are not all satisfiable together, we need to prune conditions in a
  // targeted way to do the best we can with the code we have.
  // TODO(b/268440048) Handle unsatisfiable safety constraints
  if (!isSatisfiable(SafetyConstraints)) return {};

  clang::dataflow::AtomicBoolValue &IsNull =
      getPointerNullState(Pointer).second;
  auto SafetyConstraintsAndIsNull = SafetyConstraints;
  SafetyConstraintsAndIsNull.insert(&IsNull);

  // If the safety constraints are satisfiable, but the conjunction of
  // the safety constraints and this pointer being null is not
  // satisfiable, then the safety constraints imply that the pointer
  // must be Nonnull.
  //
  // Example code behaviors that could create safety conditions implying
  // Nonnull:
  // - For return values, a Nonnull variable assigned to the return
  //  value of the function under analysis.
  // - For a parameter, an unconditional dereference.
  // TODO(b/268440048) implement safety constraints that imply Nonnull.
  // The examples above are not implemented.
  NullabilityConstraint NullabilityConstraint;
  NullabilityConstraint.set_must_be_nonnull(
      isUnsatisfiable(SafetyConstraintsAndIsNull));
  return NullabilityConstraint;
}
}  // namespace clang::tidy::nullability
