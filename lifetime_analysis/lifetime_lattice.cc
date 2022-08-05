// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/lifetime_lattice.h"

#include <assert.h>

#include <string>
#include <tuple>
#include <utility>

#include "clang/Analysis/FlowSensitive/DataflowLattice.h"
#include "llvm/Support/ErrorHandling.h"

namespace clang {
namespace tidy {
namespace lifetimes {

std::string LifetimeLattice::ToString() const {
  if (IsError()) {
    return Error().str();
  }
  return PointsTo().DebugString();
}

PointsToMap& LifetimeLattice::PointsTo() {
  assert(!IsError());
  return std::get<0>(var_).first;
}

const PointsToMap& LifetimeLattice::PointsTo() const {
  assert(!IsError());
  return std::get<0>(var_).first;
}

LifetimeConstraints& LifetimeLattice::Constraints() {
  assert(!IsError());
  return std::get<0>(var_).second;
}

const LifetimeConstraints& LifetimeLattice::Constraints() const {
  assert(!IsError());
  return std::get<0>(var_).second;
}

llvm::StringRef LifetimeLattice::Error() const {
  assert(IsError());
  if (!IsError()) {
    llvm::report_fatal_error(
        "Trying to access error on non-error LifetimeLattice");
  }
  return std::get<std::string>(var_);
}

clang::dataflow::LatticeJoinEffect LifetimeLattice::join(
    const LifetimeLattice& other) {
  if (IsError()) {
    return clang::dataflow::LatticeJoinEffect::Unchanged;
  }
  if (other.IsError()) {
    *this = other;
    return clang::dataflow::LatticeJoinEffect::Changed;
  }

  auto constraints_effect = Constraints().join(other.Constraints());

  PointsToMap joined_points_to_map = PointsTo().Union(other.PointsTo());
  if (PointsTo() == joined_points_to_map &&
      constraints_effect == clang::dataflow::LatticeJoinEffect::Unchanged) {
    return clang::dataflow::LatticeJoinEffect::Unchanged;
  }

  PointsTo() = std::move(joined_points_to_map);
  return clang::dataflow::LatticeJoinEffect::Changed;
}

bool LifetimeLattice::operator==(const LifetimeLattice& other) const {
  if (IsError() || other.IsError()) {
    // Any error compares equal to any other error.
    return IsError() && other.IsError();
  }
  return PointsTo() == other.PointsTo();
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
