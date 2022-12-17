// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_LATTICE_H_
#define THIRD_PARTY_CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_LATTICE_H_

#include <ostream>

#include "absl/container/flat_hash_map.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/DataflowLattice.h"

namespace clang {
namespace tidy {
namespace nullability {

class PointerNullabilityLattice {
 private:
  // Owned by the PointerNullabilityAnalysis object, shared by all lattice
  // elements within one analysis run.
  absl::flat_hash_map<const Expr *, std::vector<NullabilityKind>>
      *ExprToNullability;

 public:
  PointerNullabilityLattice(
      absl::flat_hash_map<const Expr *, std::vector<NullabilityKind>>
          *ExprToNullability)
      : ExprToNullability(ExprToNullability) {}

  Optional<ArrayRef<NullabilityKind>> getExprNullability(const Expr *E) {
    auto I = ExprToNullability->find(E);
    return I == ExprToNullability->end()
               ? std::nullopt
               : Optional<ArrayRef<NullabilityKind>>(I->second);
  }

  // If the `ExprToNullability` map already contains an entry for `E`, does
  // nothing. Otherwise, inserts a new entry with key `E` and value computed by
  // the provided GetNullability.
  void insertExprNullabilityIfAbsent(
      const Expr *E,
      const std::function<std::vector<NullabilityKind>()> &GetNullability) {
    auto [Iterator, Inserted] =
        ExprToNullability->insert({E, std::vector<NullabilityKind>()});
    if (Inserted) {
      Iterator->second = GetNullability();
    }
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
