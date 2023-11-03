// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability_lattice.h"

#include <optional>

#include "absl/log/check.h"
#include "nullability/type_nullability.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/Basic/LLVM.h"

namespace clang::tidy::nullability {
namespace {
// Returns overridden nullability information associated with a declaration.
// For now we only track top-level decl nullability symbolically and check for
// concrete nullability override results.
const PointerTypeNullability *getDeclNullability(
    const Decl *D,
    const PointerNullabilityLattice::NonFlowSensitiveState &NFS) {
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
}  // namespace

void PointerNullabilityLattice::overrideNullabilityFromDecl(
    const Decl *D, TypeNullability &N) const {
  // For now, overrides are always for pointer values only, and override only
  // the top-level nullability.
  if (auto *PN = getDeclNullability(D, NFS)) {
    CHECK(!N.empty());
    N.front() = *PN;
  }
}

}  // namespace clang::tidy::nullability
