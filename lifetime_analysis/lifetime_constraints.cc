// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/lifetime_constraints.h"

namespace clang {
namespace tidy {
namespace lifetimes {

clang::dataflow::LatticeJoinEffect LifetimeConstraints::join(
    const LifetimeConstraints& other) {
  bool changed = false;
  for (auto p : other.outlives_constraints_) {
    changed |= outlives_constraints_.insert(p).second;
  }
  return changed ? clang::dataflow::LatticeJoinEffect::Changed
                 : clang::dataflow::LatticeJoinEffect::Unchanged;
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
