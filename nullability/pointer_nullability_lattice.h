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
#include "absl/log/check.h"
#include "nullability/type_nullability.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/Expr.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/DataflowLattice.h"
#include "clang/Analysis/FlowSensitive/StorageLocation.h"
#include "clang/Analysis/FlowSensitive/Value.h"
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
    absl::flat_hash_map<absl::Nonnull<const ValueDecl *>,
                        PointerTypeNullability>
        DeclTopLevelNullability;
    // Returns overriding concrete nullability for decls. This is set by
    // PointerNullabilityAnalysis::assignNullabilityOverride, and the result, if
    // present, takes precedence over the declared type.
    llvm::unique_function<std::optional<const PointerTypeNullability *>(
        const Decl &) const>
        ConcreteNullabilityOverride = [](const Decl &) { return std::nullopt; };
  };

  PointerNullabilityLattice(NonFlowSensitiveState &NFS) : NFS(NFS) {}

  absl::Nullable<const TypeNullability *> getExprNullability(
      absl::Nonnull<const Expr *> E) const {
    auto I = NFS.ExprToNullability.find(&dataflow::ignoreCFGOmittedNodes(*E));
    return I == NFS.ExprToNullability.end() ? nullptr : &I->second;
  }

  // If the `ExprToNullability` map already contains an entry for `E`, does
  // nothing. Otherwise, inserts a new entry with key `E` and value computed by
  // the provided GetNullability.
  // Returns the (cached or computed) nullability.
  const TypeNullability &insertExprNullabilityIfAbsent(
      absl::Nonnull<const Expr *> E,
      const std::function<TypeNullability()> &GetNullability);

  // Gets the PointerValue associated with the RecordStorageLocation and
  // MethodDecl of the CallExpr, creating one if it doesn't yet exist. Requires
  // the CXXMemberCallExpr to have a supported pointer type.
  absl::Nullable<dataflow::PointerValue *> getConstMethodReturnValue(
      const dataflow::RecordStorageLocation &RecordLoc,
      absl::Nonnull<const CallExpr *> CE, dataflow::Environment &Env);

  void clearConstMethodReturnValues(
      const dataflow::RecordStorageLocation &RecordLoc) {
    ConstMethodReturnValues.erase(&RecordLoc);
  }

  // If nullability for the decl D has been overridden, patch N to reflect it.
  // (N is the nullability of an access to D).
  void overrideNullabilityFromDecl(absl::Nullable<const Decl *> D,
                                   TypeNullability &N) const;

  bool operator==(const PointerNullabilityLattice &Other) const { return true; }

  dataflow::LatticeJoinEffect join(const PointerNullabilityLattice &Other);

 private:
  // Owned by the PointerNullabilityAnalysis object, shared by all lattice
  // elements within one analysis run.
  NonFlowSensitiveState &NFS;

  // Maps a record storage location and const method to the value to return
  // from that const method.
  using ConstMethodReturnValuesType = llvm::SmallDenseMap<
      const dataflow::RecordStorageLocation *,
      llvm::SmallDenseMap<const FunctionDecl *, dataflow::PointerValue *>>;
  ConstMethodReturnValuesType ConstMethodReturnValues;
};

inline std::ostream &operator<<(std::ostream &OS,
                                const PointerNullabilityLattice &) {
  return OS << "noop";
}

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_POINTER_NULLABILITY_LATTICE_H_
