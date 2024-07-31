// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability_lattice.h"

#include <cassert>
#include <functional>
#include <optional>

#include "absl/base/nullability.h"
#include "absl/log/check.h"
#include "nullability/pointer_nullability.h"
#include "nullability/type_nullability.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Expr.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/DataflowLattice.h"
#include "clang/Analysis/FlowSensitive/StorageLocation.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Basic/LLVM.h"
#include "llvm/ADT/DenseMap.h"

namespace clang::tidy::nullability {
namespace {

using dataflow::LatticeJoinEffect;
using dataflow::PointerValue;
using dataflow::RecordStorageLocation;
using dataflow::StorageLocation;
using dataflow::Value;

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

template <typename T>
llvm::SmallDenseMap<const dataflow::RecordStorageLocation *,
                    llvm::SmallDenseMap<const FunctionDecl *, T *>>
joinConstMethodMap(
    const llvm::SmallDenseMap<const dataflow::RecordStorageLocation *,
                              llvm::SmallDenseMap<const FunctionDecl *, T *>>
        &Map1,
    const llvm::SmallDenseMap<const dataflow::RecordStorageLocation *,
                              llvm::SmallDenseMap<const FunctionDecl *, T *>>
        &Map2,
    LatticeJoinEffect &Effect) {
  llvm::SmallDenseMap<const dataflow::RecordStorageLocation *,
                      llvm::SmallDenseMap<const FunctionDecl *, T *>>
      Result;
  for (auto &[Loc, DeclToT] : Map1) {
    auto It = Map2.find(Loc);
    if (It == Map2.end()) {
      Effect = LatticeJoinEffect::Changed;
      continue;
    }
    const auto &OtherDeclToT = It->second;
    auto &JoinedDeclToT = Result[Loc];
    for (auto [Func, Var] : DeclToT) {
      T *OtherVar = OtherDeclToT.lookup(Func);
      if (OtherVar == nullptr || OtherVar != Var) {
        Effect = LatticeJoinEffect::Changed;
        continue;
      }
      JoinedDeclToT.insert({Func, Var});
    }
  }
  return Result;
}

}  // namespace

const TypeNullability &PointerNullabilityLattice::insertExprNullabilityIfAbsent(
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

absl::Nullable<dataflow::Value *>
PointerNullabilityLattice::getConstMethodReturnValue(
    const dataflow::RecordStorageLocation &RecordLoc,
    absl::Nonnull<const CallExpr *> CE, dataflow::Environment &Env) {
  assert(CE->getType()->isPointerType() || CE->getType()->isBooleanType());
  auto &ObjMap = ConstMethodReturnValues[&RecordLoc];
  const FunctionDecl *DirectCallee = CE->getDirectCallee();
  if (DirectCallee == nullptr) return nullptr;
  auto it = ObjMap.find(DirectCallee);
  if (it != ObjMap.end()) return it->second;
  dataflow::Value *Val = Env.createValue(CE->getType());
  if (Val != nullptr) ObjMap.insert({DirectCallee, Val});
  return Val;
}

absl::Nullable<dataflow::StorageLocation *>
PointerNullabilityLattice::getConstMethodReturnStorageLocation(
    const dataflow::RecordStorageLocation &RecordLoc,
    absl::Nonnull<const CallExpr *> CE, dataflow::Environment &Env) {
  assert(isSupportedSmartPointerType(CE->getType()));
  auto &ObjMap = ConstMethodReturnStorageLocations[&RecordLoc];
  const FunctionDecl *DirectCallee = CE->getDirectCallee();
  if (DirectCallee == nullptr) return nullptr;
  auto it = ObjMap.find(DirectCallee);
  if (it != ObjMap.end()) return it->second;
  StorageLocation &Loc = Env.createStorageLocation(CE->getType());
  setSmartPointerValue(cast<RecordStorageLocation>(Loc),
                       cast<PointerValue>(Env.createValue(
                           underlyingRawPointerType(CE->getType()))),
                       Env);
  ObjMap.insert({DirectCallee, &Loc});
  return &Loc;
}

void PointerNullabilityLattice::overrideNullabilityFromDecl(
    absl::Nullable<const Decl *> D, TypeNullability &N) const {
  // For now, overrides are always for pointer values only, and override only
  // the top-level nullability.
  if (N.empty()) return;
  if (auto *PN = getDeclNullability(D, NFS)) {
    N.front() = *PN;
  }
}

LatticeJoinEffect PointerNullabilityLattice::join(
    const PointerNullabilityLattice &Other) {
  // For simplicity, we only retain values that are identical, but not ones that
  // are non-identical but equivalent. This is likely to be sufficient in
  // practice, and it reduces implementation complexity considerably.

  LatticeJoinEffect Effect = LatticeJoinEffect::Unchanged;
  ConstMethodReturnValues = joinConstMethodMap<Value>(
      ConstMethodReturnValues, Other.ConstMethodReturnValues, Effect);
  ;

  ConstMethodReturnStorageLocations = joinConstMethodMap<StorageLocation>(
      ConstMethodReturnStorageLocations,
      Other.ConstMethodReturnStorageLocations, Effect);

  return Effect;
}

}  // namespace clang::tidy::nullability
