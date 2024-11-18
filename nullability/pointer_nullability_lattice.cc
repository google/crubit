// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability_lattice.h"

#include <cassert>
#include <functional>
#include <optional>

#include "absl/base/nullability.h"
#include "absl/log/check.h"
#include "nullability/type_nullability.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Expr.h"
#include "clang/Analysis/FlowSensitive/DataflowLattice.h"
#include "clang/Basic/LLVM.h"

namespace clang::tidy::nullability {
namespace {

using dataflow::LatticeJoinEffect;

// Returns overridden nullability information associated with a declaration.
// For now we only track top-level decl nullability symbolically and check for
// concrete nullability override results.
absl::Nullable<const PointerTypeNullability *> getDeclNullability(
    absl::Nullable<const Decl *> D,
    const PointerNullabilityLattice::NonFlowSensitiveState &NFS) {
  if (!D) return nullptr;
  if (const auto *VD = dyn_cast_or_null<ValueDecl>(D->getCanonicalDecl())) {
    auto It = NFS.DeclTopLevelNullability.find(VD);
    if (It != NFS.DeclTopLevelNullability.end()) return &It->second;
  }
  if (const std::optional<const PointerTypeNullability *> N =
          NFS.ConcreteNullabilityOverride(*D->getCanonicalDecl()))
    return *N;
  return nullptr;
}

}  // namespace

const TypeNullability &
PointerNullabilityLatticeBase::insertExprNullabilityIfAbsent(
    absl::Nonnull<const Expr *> E,
    const std::function<TypeNullability()> &GetNullability) {
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

void PointerNullabilityLatticeBase::overrideNullabilityFromDecl(
    absl::Nullable<const Decl *> D, TypeNullability &N) const {
  // For now, overrides are always for pointer values only, and override only
  // the top-level nullability.
  if (N.empty()) return;
  if (auto *PN = getDeclNullability(D, NFS)) {
    N.front() = *PN;
  }
}

LatticeJoinEffect PointerNullabilityLatticeBase::join(
    const PointerNullabilityLatticeBase &Other) {
  return LatticeJoinEffect::Unchanged;
}

}  // namespace clang::tidy::nullability
