// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_LATTICE_H_
#define THIRD_PARTY_CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_LATTICE_H_

#include <optional>
#include <ostream>

#include "absl/container/flat_hash_map.h"
#include "absl/log/check.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowLattice.h"

namespace clang {
namespace tidy {
namespace nullability {

class PointerNullabilityLattice {
 private:
  // Owned by the PointerNullabilityAnalysis object, shared by all lattice
  // elements within one analysis run.
  absl::flat_hash_map<const Expr *, std::vector<NullabilityKind>>
      &ExprToNullability;

 public:
  PointerNullabilityLattice(
      absl::flat_hash_map<const Expr *, std::vector<NullabilityKind>>
          &ExprToNullability)
      : ExprToNullability(ExprToNullability) {}

  std::optional<ArrayRef<NullabilityKind>> getExprNullability(
      const Expr *E) const {
    auto I = ExprToNullability.find(&dataflow::ignoreCFGOmittedNodes(*E));
    return I == ExprToNullability.end()
               ? std::nullopt
               : std::optional<ArrayRef<NullabilityKind>>(I->second);
  }

  // If the `ExprToNullability` map already contains an entry for `E`, does
  // nothing. Otherwise, inserts a new entry with key `E` and value computed by
  // the provided GetNullability.
  // Returns the (cached or computed) nullability.
  ArrayRef<NullabilityKind> insertExprNullabilityIfAbsent(
      const Expr *E,
      const std::function<std::vector<NullabilityKind>()> &GetNullability) {
    E = &dataflow::ignoreCFGOmittedNodes(*E);
    if (auto It = ExprToNullability.find(E); It != ExprToNullability.end())
      return It->second;
    // Deliberately perform a separate lookup after calling GetNullability.
    // It may invalidate iterators, e.g. inserting missing vectors for children.
    auto [Iterator, Inserted] = ExprToNullability.insert({E, GetNullability()});
    CHECK(Inserted) << "GetNullability inserted same " << E->getStmtClassName();
    return Iterator->second;
  }

  bool operator==(const PointerNullabilityLattice &Other) const { return true; }

  dataflow::LatticeJoinEffect join(const PointerNullabilityLattice &Other) {
    return dataflow::LatticeJoinEffect::Unchanged;
  }
};

inline std::ostream &operator<<(std::ostream &OS,
                                const PointerNullabilityLattice &) {
  return OS << "noop";
}

}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // THIRD_PARTY_CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_LATTICE_H_
