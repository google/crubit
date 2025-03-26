// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_POINTER_NULLABILITY_LATTICE_H_
#define CRUBIT_NULLABILITY_POINTER_NULLABILITY_LATTICE_H_

#include <functional>
#include <optional>
#include <ostream>

#include "absl/base/nullability.h"
#include "absl/container/flat_hash_map.h"
#include "nullability/type_nullability.h"
#include "clang/AST/Expr.h"
#include "clang/Analysis/FlowSensitive/ASTOps.h"
#include "clang/Analysis/FlowSensitive/CachedConstAccessorsLattice.h"
#include "clang/Analysis/FlowSensitive/DataflowLattice.h"
#include "llvm/ADT/FunctionExtras.h"

namespace clang::tidy::nullability {
class PointerNullabilityLatticeBase {
 public:
  struct NonFlowSensitiveState {
    // Nullability interpretation of types as set e.g. by per-file #pragmas.
    TypeNullabilityDefaults Defaults;

    absl::flat_hash_map<const Expr *, TypeNullability> ExprToNullability;
    // Overridden symbolic nullability for pointer-typed decls.
    // These are set by PointerNullabilityAnalysis::assignNullabilityVariable,
    // and take precedence over the declared type and over any result from
    // ConcreteNullabilityOverride.
    absl::flat_hash_map<const ValueDecl *absl_nonnull, PointerTypeNullability>
        DeclTopLevelNullability;
    // Returns overriding concrete nullability for decls. This is set by
    // PointerNullabilityAnalysis::assignNullabilityOverride, and the result, if
    // present, takes precedence over the declared type.
    llvm::unique_function<std::optional<const PointerTypeNullability *>(
        const Decl &) const>
        ConcreteNullabilityOverride = [](const Decl &) { return std::nullopt; };
  };

  PointerNullabilityLatticeBase(NonFlowSensitiveState &NFS) : NFS(NFS) {}

  const TypeNullability *absl_nullable getTypeNullability(
      const Expr *absl_nonnull E) const {
    auto I = NFS.ExprToNullability.find(&dataflow::ignoreCFGOmittedNodes(*E));
    return I == NFS.ExprToNullability.end() ? nullptr : &I->second;
  }

  // If the `ExprToNullability` map already contains an entry for `E`, does
  // nothing. Otherwise, inserts a new entry with key `E` and value computed by
  // the provided GetNullability.
  // Returns the (cached or computed) nullability.
  const TypeNullability &insertExprNullabilityIfAbsent(
      const Expr *absl_nonnull E,
      const std::function<TypeNullability()> &GetNullability);

  // If nullability for the decl D has been overridden, patch N to reflect it.
  // (N is the nullability of an access to D).
  void overrideNullabilityFromDecl(const Decl *absl_nullable D,
                                   TypeNullability &N) const;

  bool operator==(const PointerNullabilityLatticeBase &Other) const {
    return true;
  }

  dataflow::LatticeJoinEffect join(const PointerNullabilityLatticeBase &Other);

  const TypeNullabilityDefaults &defaults() const { return NFS.Defaults; }

 private:
  // Owned by the PointerNullabilityAnalysis object, shared by all lattice
  // elements within one analysis run.
  NonFlowSensitiveState &NFS;
};

using PointerNullabilityLattice =
    dataflow::CachedConstAccessorsLattice<PointerNullabilityLatticeBase>;

inline std::ostream &operator<<(std::ostream &OS,
                                const PointerNullabilityLattice &) {
  return OS << "nullability";
}

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_POINTER_NULLABILITY_LATTICE_H_
