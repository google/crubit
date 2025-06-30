// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_LIFETIME_LATTICE_H_
#define DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_LIFETIME_LATTICE_H_

#include <string>
#include <tuple>
#include <utility>
#include <variant>

#include "lifetime_analysis/lifetime_constraints.h"
#include "lifetime_analysis/object_set.h"
#include "lifetime_analysis/points_to_map.h"
#include "clang/Analysis/FlowSensitive/DataflowLattice.h"
#include "llvm/ADT/StringRef.h"

namespace clang {
namespace tidy {
namespace lifetimes {

class LifetimeLattice {
 public:
  // Creates a lattice holding an empty points-to map and empty constraints.
  LifetimeLattice() = default;

  LifetimeLattice(const LifetimeLattice&) = default;
  LifetimeLattice(LifetimeLattice&&) = default;
  LifetimeLattice& operator=(const LifetimeLattice&) = default;
  LifetimeLattice& operator=(LifetimeLattice&&) = default;

  // Creates a lattice containing the given points-to map, single-valued object
  // set, and empty constraints.
  explicit LifetimeLattice(PointsToMap points_to_map,
                           ObjectSet single_valued_objects)
      : var_(std::make_tuple(std::move(points_to_map), LifetimeConstraints(),
                             std::move(single_valued_objects))) {}

  // Creates an error state containing the error message `err`.
  explicit LifetimeLattice(std::string err) : var_(err) {}

  // Returns the points-to map.
  // Precondition: !IsError().
  PointsToMap& PointsTo();
  const PointsToMap& PointsTo() const;

  // Returns the lifetime constraints.
  // Precondition: !IsError().
  LifetimeConstraints& Constraints();
  const LifetimeConstraints& Constraints() const;

  // Returns the set of single-valued objects, i.e. objects that will be
  // guaranteed to be overwritten completely by a write operation.
  // For example, all local variables are single-valued unless they are
  // conditionally overwritten. Values that represent pointees of pointers are
  // not (as they could be arrays), but values that represent pointees of
  // references can be.
  // Precondition: !IsError().
  ObjectSet& SingleValuedObjects();
  const ObjectSet& SingleValuedObjects() const;

  // Returns whether the lattice is in the error state.
  bool IsError() const { return std::holds_alternative<std::string>(var_); }

  // Returns the error string.
  // Precondition: IsError().
  llvm::StringRef Error() const;

  // Returns a human-readable representation of the lattice.
  std::string ToString() const;

  // Sets the lattice to the result of the "join" operation with `other` and
  // returns the effect of the operation.
  // If either of the lattices contains an error, sets this lattice to the
  // first error encountered.
  clang::dataflow::LatticeJoinEffect join(const LifetimeLattice& other);

  // Compares for (in-)equality.
  // All error states are considered to be equal.
  bool operator==(const LifetimeLattice& other) const;
  bool operator!=(const LifetimeLattice& other) const {
    return !(*this == other);
  }

 private:
  std::variant<std::tuple<PointsToMap, LifetimeConstraints, ObjectSet>,
               std::string>
      var_;
};

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

#endif  // DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_LIFETIME_LATTICE_H_
