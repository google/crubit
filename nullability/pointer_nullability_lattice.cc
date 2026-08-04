// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability_lattice.h"

#include <cassert>
#include <functional>

#include "absl/base/nullability.h"
#include "nullability/type_nullability.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/Analysis/FlowSensitive/ASTOps.h"
#include "clang/Analysis/FlowSensitive/DataflowLattice.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/OperatorKinds.h"
#include "llvm/Support/ErrorHandling.h"

namespace clang::tidy::nullability {
namespace {

using dataflow::LatticeJoinEffect;

// Returns overridden nullability information associated with a declaration.
// For now we only track top-level decl nullability symbolically.
const PointerTypeNullability *absl_nullable getDeclNullability(
    const Decl *absl_nullable D,
    const PointerNullabilityLattice::NonFlowSensitiveState &NFS) {
  if (!D) return nullptr;
  if (const auto *VD = dyn_cast_or_null<ValueDecl>(D->getCanonicalDecl())) {
    auto It = NFS.DeclTopLevelNullability.find(VD);
    if (It != NFS.DeclTopLevelNullability.end()) return &It->second;
  }
  return nullptr;
}

}  // namespace

const TypeNullability &
PointerNullabilityLatticeBase::insertExprNullabilityIfAbsent(
    const Expr *absl_nonnull E,
    const std::function<TypeNullability()> &GetNullability) {
  E = &dataflow::ignoreCFGOmittedNodes(*E);
  if (auto It = NFS.ExprToNullability.find(E);
      It != NFS.ExprToNullability.end())
    return It->second;
  // Deliberately perform a separate lookup after calling GetNullability.
  // It may invalidate iterators, e.g. inserting missing vectors for children.
  auto [Iterator, Inserted] =
      NFS.ExprToNullability.insert({E, GetNullability()});
  if (!Inserted)
    reportFatalInternalError("GetNullability inserted same " +
                             Twine(E->getStmtClassName()));
  return Iterator->second;
}

void PointerNullabilityLatticeBase::overrideNullabilityFromDecl(
    const Decl *absl_nullable D, TypeNullability &N) const {
  // For now, overrides are always for pointer values only, and override only
  // the top-level nullability.
  if (N.empty()) return;
  if (auto *PN = getDeclNullability(D, NFS)) {
    N.front() = *PN;
  }
}

const FieldDecl* absl_nullable fieldTreatedAsNullableAtDestructorEntry(
    const Expr* absl_nonnull E, const PointerNullabilityLatticeBase& Lattice) {
  const auto& Fields = Lattice.fieldsToTreatAsNullableAtDestructorEntry();
  if (Fields.empty()) return nullptr;
  E = E->IgnoreParenImpCasts();
  // `field->member` on a smart pointer lowers to `field.operator->()->member`;
  // the arrow's base is the `operator->()` call. Unwrap it to the field expr.
  if (const auto* Op = dyn_cast<CXXOperatorCallExpr>(E);
      Op != nullptr && Op->getOperator() == OO_Arrow && Op->getNumArgs() == 1)
    E = Op->getArg(0)->IgnoreParenImpCasts();
  const auto* ME = dyn_cast<MemberExpr>(E);
  if (ME == nullptr) return nullptr;
  // Only downgrade fields accessed through `*this` (the object being
  // destroyed); an access like `other.field` on a different instance keeps its
  // declared nullability.
  if (!isa<CXXThisExpr>(ME->getBase()->IgnoreParenImpCasts())) return nullptr;
  const auto* FD = dyn_cast<FieldDecl>(ME->getMemberDecl());
  if (FD == nullptr || !Fields.contains(FD)) return nullptr;
  return FD;
}

bool shouldTreatFieldAsNullableAtDestructorEntry(
    const Expr* absl_nonnull E, const PointerNullabilityLatticeBase& Lattice) {
  return fieldTreatedAsNullableAtDestructorEntry(E, Lattice) != nullptr;
}

LatticeJoinEffect PointerNullabilityLatticeBase::join(
    const PointerNullabilityLatticeBase &Other) {
  return LatticeJoinEffect::Unchanged;
}

}  // namespace clang::tidy::nullability
