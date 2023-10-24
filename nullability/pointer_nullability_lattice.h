// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_POINTER_NULLABILITY_LATTICE_H_
#define CRUBIT_NULLABILITY_POINTER_NULLABILITY_LATTICE_H_

#include <functional>
#include <optional>
#include <ostream>

#include "absl/container/flat_hash_map.h"
#include "absl/log/check.h"
#include "nullability/type_nullability.h"
#include "clang/AST/Expr.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowLattice.h"
#include "clang/Basic/LLVM.h"
#include "llvm/ADT/FunctionExtras.h"

namespace clang::tidy::nullability {
class PointerNullabilityLattice {
 public:
  struct NonFlowSensitiveState {
    absl::flat_hash_map<const Expr *, TypeNullability> ExprToNullability;
    // Overridden symbolic nullability for pointer-typed decls.
    // These are set by PointerNullabilityAnalysis::assignNullabilityVariable,
    // and take precedence over the declared type and over any result from
    // ConcreteNullabilityOverride.
    absl::flat_hash_map<const ValueDecl *, PointerTypeNullability>
        DeclTopLevelNullability;
    // Returns overriding concrete nullability for decls. This is set by
    // PointerNullabilityAnalysis::assignNullabilityOverride, and the result, if
    // present, takes precedence over the declared type.
    llvm::unique_function<std::optional<const PointerTypeNullability *>(
        const Decl &) const>
        ConcreteNullabilityOverride = [](const Decl &) { return std::nullopt; };
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
  // For now we only track top-level decl nullability symbolically and check for
  // concrete nullability override results.
  const PointerTypeNullability *getDeclNullability(const Decl *D) const {
    if (!D) return nullptr;
    if (const auto *VD = dyn_cast_or_null<ValueDecl>(D)) {
      auto It = NFS.DeclTopLevelNullability.find(VD);
      if (It != NFS.DeclTopLevelNullability.end()) return &It->second;
    }
    if (const std::optional<const PointerTypeNullability *> N =
            NFS.ConcreteNullabilityOverride(*D))
      return *N;
    return nullptr;
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
