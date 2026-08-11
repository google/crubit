// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_POINTER_NULLABILITY_LATTICE_H_
#define CRUBIT_NULLABILITY_POINTER_NULLABILITY_LATTICE_H_

#include <functional>
#include <optional>
#include <ostream>

#include "absl/base/nullability.h"
#include "nullability/type_nullability.h"
#include "clang/AST/Decl.h"
#include "clang/AST/Expr.h"
#include "clang/Analysis/FlowSensitive/ASTOps.h"
#include "clang/Analysis/FlowSensitive/CachedConstAccessorsLattice.h"
#include "clang/Analysis/FlowSensitive/DataflowLattice.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/DenseSet.h"

namespace clang::tidy::nullability {
class PointerNullabilityLatticeBase {
 public:
  struct NonFlowSensitiveState {
    // Nullability interpretation of types as set e.g. by per-file #pragmas.
    TypeNullabilityDefaults Defaults;

    llvm::DenseMap<const Expr*, TypeNullability> ExprToNullability;

    // Overridden symbolic nullability for pointer-typed decls.
    // These are set by PointerNullabilityAnalysis::assignNullabilityVariable,
    // and take precedence over the declared type.
    llvm::DenseMap<const ValueDecl* absl_nonnull, PointerTypeNullability>
        DeclTopLevelNullability;

    // Nonnull pointer fields (raw and smart) of the class whose destructor is
    // being analyzed that must be modeled as nullable at destructor entry,
    // because a move operation or `&&`-qualified method may have nulled them
    // out before destruction. Empty unless analyzing such a destructor.
    llvm::DenseSet<const FieldDecl*> FieldsToTreatAsNullableAtDestructorEntry;

    // Flow-sensitive nullability of by-value captured pointers, recorded at the
    // capture site. Empty unless analyzing a lambda call operator.
    llvm::DenseMap<const ValueDecl*, NullabilityKind> CapturedVarNullability;
  };

  PointerNullabilityLatticeBase(NonFlowSensitiveState &NFS) : NFS(NFS) {}

  const TypeNullability *absl_nullable getTypeNullability(
      const Expr *absl_nonnull E) const {
    auto I = NFS.ExprToNullability.find(&dataflow::ignoreCFGOmittedNodes(*E));
    return I == NFS.ExprToNullability.end() ? nullptr : &I->second;
  }

  // Nonnull pointer fields to be modeled as nullable at destructor entry.
  const llvm::DenseSet<const FieldDecl*>&
  fieldsToTreatAsNullableAtDestructorEntry() const {
    return NFS.FieldsToTreatAsNullableAtDestructorEntry;
  }

  // Flow-narrowed nullability of by-value captures to model at lambda
  // call-operator entry.
  const llvm::DenseMap<const ValueDecl*, NullabilityKind>&
  capturedVarNullability() const {
    return NFS.CapturedVarNullability;
  }

  /// Extract the nullability of the type of `D`.
  ///
  /// The file where the type is written affects the interpretation of
  /// unannotated pointer types. If the nullability for `D` has been overridden,
  /// the returned nullability will contain these overrides.
  TypeNullability getTypeNullabilityWithOverrides(
      const ValueDecl &D,
      llvm::function_ref<GetTypeParamNullability> SubstituteTypeParam =
          nullptr) {
    TypeNullability Nullability = clang::tidy::nullability::getTypeNullability(
        D, defaults(), SubstituteTypeParam);
    overrideNullabilityFromDecl(&D, Nullability);
    return Nullability;
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

// If `E` accesses, through `*this`, a nonnull pointer field that is in the
// lattice's "treat as nullable at destructor entry" set, returns that field;
// otherwise returns nullptr.
const clang::FieldDecl* absl_nullable fieldTreatedAsNullableAtDestructorEntry(
    const clang::Expr& E, const PointerNullabilityLatticeBase& Lattice);

// Returns true if `E` accesses, through `*this`, a nonnull pointer field that
// is in the lattice's "treat as nullable at destructor entry" set.
bool shouldTreatFieldAsNullableAtDestructorEntry(
    const clang::Expr& E, const PointerNullabilityLatticeBase& Lattice);

// Flow-sensitive nullability of the by-value captured pointer read by `E`, if
// recorded at the capture site; otherwise std::nullopt.
std::optional<NullabilityKind> getCapturedVarNullability(
    const clang::Expr& E, const PointerNullabilityLatticeBase& Lattice);

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_POINTER_NULLABILITY_LATTICE_H_
