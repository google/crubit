// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/lifetime_lattice.h"

#include <string>
#include <utility>

#include "lifetime_analysis/lifetime_constraints.h"
#include "lifetime_analysis/object_set.h"
#include "lifetime_analysis/points_to_map.h"
#include "clang/include/clang/Analysis/FlowSensitive/DataflowLattice.h"
#include "llvm/include/llvm/ADT/StringRef.h"

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
  return std::get<PointsToMap>(std::get<0>(var_));
}

const PointsToMap& LifetimeLattice::PointsTo() const {
  return std::get<PointsToMap>(std::get<0>(var_));
}

LifetimeConstraints& LifetimeLattice::Constraints() {
  return std::get<LifetimeConstraints>(std::get<0>(var_));
}

const LifetimeConstraints& LifetimeLattice::Constraints() const {
  return std::get<LifetimeConstraints>(std::get<0>(var_));
}

ObjectSet& LifetimeLattice::SingleValuedObjects() {
  return std::get<ObjectSet>(std::get<0>(var_));
}

const ObjectSet& LifetimeLattice::SingleValuedObjects() const {
  return std::get<ObjectSet>(std::get<0>(var_));
}

llvm::StringRef LifetimeLattice::Error() const {
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

  auto effect = Constraints().join(other.Constraints());

  PointsToMap joined_points_to_map = PointsTo().Union(other.PointsTo());
  if (PointsTo() != joined_points_to_map) {
    PointsTo() = std::move(joined_points_to_map);
    effect = clang::dataflow::LatticeJoinEffect::Changed;
  }

  ObjectSet joined_single_valued_objects =
      SingleValuedObjects().Intersection(other.SingleValuedObjects());
  if (SingleValuedObjects() != joined_single_valued_objects) {
    SingleValuedObjects() = std::move(joined_single_valued_objects);
    effect = clang::dataflow::LatticeJoinEffect::Changed;
  }

  return effect;
}

bool LifetimeLattice::operator==(const LifetimeLattice& other) const {
  if (IsError() || other.IsError()) {
    // Any error compares equal to any other error.
    return IsError() && other.IsError();
  }
  return PointsTo() == other.PointsTo() && Constraints() == other.Constraints();
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
