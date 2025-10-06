// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_POINTER_NULLABILITY_ANALYSIS_H_
#define CRUBIT_NULLABILITY_POINTER_NULLABILITY_ANALYSIS_H_

#include <functional>
#include <memory>
#include <optional>
#include <utility>

#include "absl/base/nullability.h"
#include "nullability/pointer_nullability_lattice.h"
#include "nullability/pragma.h"
#include "nullability/type_nullability.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/TypeBase.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/Arena.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/MatchSwitch.h"
#include "clang/Analysis/FlowSensitive/Solver.h"
#include "clang/Analysis/FlowSensitive/StorageLocation.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/FunctionExtras.h"

namespace clang {
namespace tidy {
namespace nullability {

/// Factory function for creating a solver implementation.
using SolverFactory = std::function<std::unique_ptr<dataflow::Solver>()>;

/// Analyzes constructs in the source code to collect nullability information
/// about pointers at each program point. This analysis and the corresponding
/// lattice were based on the gradual analysis in 'Estep, Sam, Jenna Wise,
/// Jonathan Aldrich, Éric Tanter, Johannes Bader, and Joshua Sunshine. "Gradual
/// Program Analysis for Null Pointers." In 35th European Conference on
/// Object-Oriented Programming (ECOOP). 2021.'
class PointerNullabilityAnalysis
    : public dataflow::DataflowAnalysis<PointerNullabilityAnalysis,
                                        PointerNullabilityLattice> {
 private:
  PointerNullabilityLattice::NonFlowSensitiveState NFS;

 public:
  explicit PointerNullabilityAnalysis(ASTContext &Context,
                                      dataflow::Environment &Env,
                                      const NullabilityPragmas &Pragmas);

  PointerNullabilityLattice initialElement() {
    return PointerNullabilityLattice(NFS);
  }

  // Instead of fixing D's nullability invariants from its annotations,
  // bind them to symbolic variables, and return those variables.
  // This is useful to infer the annotations that should be present on D.
  //
  // For example, given the following program:
  //   void target(int* p) {
  //     int* q = p;
  //     *q;
  //   }
  //
  // By default, p is treated as having unspecified nullability.
  // When we reach the dereference, our flow condition will say:
  //   from_nullable = false
  //
  // However, if we bind p's nullability to a variable:
  //   pn = assignNullabilityVariable(p)
  // Then the flow condition at dereference includes:
  //   from_nullable = pn.Nullable
  //   pn.Nonnull => !is_null
  // Logically connecting dereferenced values and possible invariants on p
  // allows us to infer p's proper annotations (here: Nonnull).
  //
  // For now, only the top-level nullability is assigned, and the returned
  // variables are only associated with direct reads of pointer values from D.
  //
  // The returned nullability is guaranteed to be symbolic.
  PointerTypeNullability assignNullabilityVariable(
      const ValueDecl *absl_nonnull D, dataflow::Arena &);

  void assignNullabilityOverride(
      llvm::unique_function<
          std::optional<const PointerTypeNullability *>(const Decl &) const>
          Override) {
    NFS.ConcreteNullabilityOverride = std::move(Override);
  }

  void transfer(const CFGElement &Elt, PointerNullabilityLattice &Lattice,
                dataflow::Environment &Env);

  void join(QualType Type, const dataflow::Value &Val1,
            const dataflow::Environment &Env1, const dataflow::Value &Val2,
            const dataflow::Environment &Env2, dataflow::Value &MergedVal,
            dataflow::Environment &MergedEnv) override;

  dataflow::ComparisonResult compare(
      QualType Type, const dataflow::Value &Val1,
      const dataflow::Environment &Env1, const dataflow::Value &Val2,
      const dataflow::Environment &Env2) override;

  std::optional<dataflow::WidenResult> widen(
      QualType Type, dataflow::Value &Prev,
      const dataflow::Environment &PrevEnv, dataflow::Value &Current,
      dataflow::Environment &CurrentEnv) override;

 private:
  // Returns a storage location representing "top", i.e. a storage location of
  // type `Ty` about which nothing else is known.
  // Known limitation: We can't prevent a "top" storage location from being
  // associated with a value. This is somewhat strange but does not appear to
  // have any ill effects in practice. To disallow this, we may at some point
  // want to move the concept of "top" storage locations to the framework.
  dataflow::StorageLocation &getTopStorageLocation(
      dataflow::DataflowAnalysisContext &DACtx, QualType Ty);

  // Transfers (non-flow-sensitive) type properties through statements.
  dataflow::CFGMatchSwitch<dataflow::TransferState<PointerNullabilityLattice>>
      TypeTransferer;

  // Transfers (flow-sensitive) value properties through statements.
  dataflow::CFGMatchSwitch<dataflow::TransferState<PointerNullabilityLattice>>
      ValueTransferer;

  // Storage locations that represent "top" for each given type.
  llvm::DenseMap<QualType, dataflow::StorageLocation *> TopStorageLocations;
};
}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_NULLABILITY_POINTER_NULLABILITY_ANALYSIS_H_
