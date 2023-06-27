// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_POINTER_NULLABILITY_LATTICE_H_
#define CRUBIT_NULLABILITY_POINTER_NULLABILITY_LATTICE_H_

#include <optional>
#include <ostream>

#include "absl/container/flat_hash_map.h"
#include "absl/log/check.h"
#include "nullability/type_nullability.h"
#include "clang/AST/Expr.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowLattice.h"

namespace clang::tidy::nullability {

class PointerNullabilityLattice {
 private:
 public:
  struct NonFlowSensitiveState {
    absl::flat_hash_map<const Expr *, TypeNullability> ExprToNullability;
    // Overridden symbolic nullability for pointer-typed decls.
    absl::flat_hash_map<const ValueDecl *, PointerTypeNullability>
        DeclTopLevelNullability;
  };

  PointerNullabilityLattice(NonFlowSensitiveState &NFS) : NFS(NFS) {}

  const TypeNullability *getExprNullability(const Expr *E) const {
    auto I = NFS.ExprToNullability.find(&dataflow::ignoreCFGOmittedNodes(*E));
    return I == NFS.ExprToNullability.end() ? nullptr : &I->second;
  }

  // If the `ExprToNullability` map already contains an entry for `E`, does
  // nothing. Otherwise, inserts a new entry with key `E` and value computed by
  // the provided GetNullability.
  // Returns the (cached or computed) nullability.
  const TypeNullability &insertExprNullabilityIfAbsent(
      const Expr *E, const std::function<TypeNullability()> &GetNullability) {
    E = &dataflow::ignoreCFGOmittedNodes(*E);
    if (auto It = NFS.ExprToNullability.find(E);
        It != NFS.ExprToNullability.end())
      return It->second;
    // Deliberately perform a separate lookup after calling GetNullability.
    // It may invalidate iterators, e.g. inserting missing vectors for children.
    auto [Iterator, Inserted] =
        NFS.ExprToNullability.insert({E, GetNullability()});
    CHECK(Inserted) << "GetNullability inserted same " << E->getStmtClassName();
    return Iterator->second;
  }

  // Returns overridden nullability information associated with a declaration.
  // For now we only track top-level decl nullability symbolically.
  const PointerTypeNullability *getDeclNullability(const ValueDecl *D) const {
    auto It = NFS.DeclTopLevelNullability.find(D);
    if (It == NFS.DeclTopLevelNullability.end()) return nullptr;
    return &It->second;
  }

  bool operator==(const PointerNullabilityLattice &Other) const { return true; }

  dataflow::LatticeJoinEffect join(const PointerNullabilityLattice &Other) {
    return dataflow::LatticeJoinEffect::Unchanged;
  }

 private:
  // Owned by the PointerNullabilityAnalysis object, shared by all lattice
  // elements within one analysis run.
  NonFlowSensitiveState &NFS;
};

inline std::ostream &operator<<(std::ostream &OS,
                                const PointerNullabilityLattice &) {
  return OS << "noop";
}

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_POINTER_NULLABILITY_LATTICE_H_
