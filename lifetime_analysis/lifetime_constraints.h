// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_LIFETIME_ANALYSIS_LIFETIME_CONSTRAINTS_H_
#define THIRD_PARTY_CRUBIT_LIFETIME_ANALYSIS_LIFETIME_CONSTRAINTS_H_

#include "lifetime_annotations/function_lifetimes.h"
#include "lifetime_annotations/lifetime.h"
#include "clang/Analysis/FlowSensitive/DataflowLattice.h"
#include "llvm/ADT/DenseSet.h"

namespace clang {
namespace tidy {
namespace lifetimes {

class LifetimeConstraints {
 public:
  // Creates empty constraints.
  LifetimeConstraints() {}

  // Returns the constraints on `callable` that allow `replacement_callable` to
  // be used where `callable` is requested.
  static LifetimeConstraints ForCallableSubstitution(
      const FunctionLifetimes& callable,
      const FunctionLifetimes& replacement_callable);

  // Returns the constraints on `callable` and `replacement_callable` that allow
  // `replacement_callable` to be used where `callable` is requested.
  static LifetimeConstraints ForCallableSubstitutionFull(
      const FunctionLifetimes& callable,
      const FunctionLifetimes& replacement_callable);

  // Imposes the constraint shorter <= longer.
  void AddOutlivesConstraint(Lifetime shorter, Lifetime longer) {
    outlives_constraints_.insert({shorter, longer});
  }

  // Returns all the lifetimes that this set of constraints implies must outlive
  // the given lifetime l.
  llvm::DenseSet<Lifetime> GetOutlivingLifetimes(Lifetime l) const;

  // Merges this set of constraints with the provided constraints, returning
  // the effect of the operation.
  clang::dataflow::LatticeJoinEffect join(const LifetimeConstraints& other);

  // Applies this set of constraints to the given FunctionLifetimes.
  llvm::Error ApplyToFunctionLifetimes(FunctionLifetimes& function_lifetimes);

  bool operator==(const LifetimeConstraints& other) const {
    return outlives_constraints_ == other.outlives_constraints_;
  }

  // Accessor for debug purposes.
  const llvm::DenseSet<std::pair<Lifetime, Lifetime>>& AllConstraints() const {
    return outlives_constraints_;
  }

 private:
  // Constraints of the form p.first <= p.second
  llvm::DenseSet<std::pair<Lifetime, Lifetime>> outlives_constraints_;
};

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

#endif  // THIRD_PARTY_CRUBIT_LIFETIME_ANALYSIS_LIFETIME_CONSTRAINTS_H_
