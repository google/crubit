// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_LIFETIME_LATTICE_H_
#define DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_LIFETIME_LATTICE_H_

#include <string>
#include <utility>
#include <variant>

#include "lifetime_analysis/lifetime_constraints.h"
#include "lifetime_analysis/points_to_map.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
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

  // Creates a lattice containing the given points-to map and empty constraints.
  explicit LifetimeLattice(PointsToMap points_to_map)
      : var_(std::make_pair(std::move(points_to_map), LifetimeConstraints())) {}

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
  std::variant<std::pair<PointsToMap, LifetimeConstraints>, std::string> var_;
};

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

#endif  // DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_LIFETIME_LATTICE_H_
