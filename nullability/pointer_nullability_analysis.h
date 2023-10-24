// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_POINTER_NULLABILITY_ANALYSIS_H_
#define CRUBIT_NULLABILITY_POINTER_NULLABILITY_ANALYSIS_H_

#include <optional>
#include <utility>

#include "nullability/pointer_nullability_lattice.h"
#include "nullability/type_nullability.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Type.h"
#include "clang/Analysis/FlowSensitive/Arena.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "llvm/ADT/FunctionExtras.h"

namespace clang {
namespace tidy {
namespace nullability {

/// Analyses constructs in the source code to collect nullability information
/// about pointers at each program point.
class PointerNullabilityAnalysis
    : public dataflow::DataflowAnalysis<PointerNullabilityAnalysis,
                                        PointerNullabilityLattice> {
 private:
  PointerNullabilityLattice::NonFlowSensitiveState NFS;

 public:
  explicit PointerNullabilityAnalysis(ASTContext &context);

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
  PointerTypeNullability assignNullabilityVariable(const ValueDecl *D,
                                                   dataflow::Arena &);

  void assignNullabilityOverride(
      llvm::unique_function<
          std::optional<const PointerTypeNullability *>(const Decl &) const>
          Override) {
    NFS.ConcreteNullabilityOverride = std::move(Override);
  }

  void transfer(const CFGElement &Elt, PointerNullabilityLattice &Lattice,
                dataflow::Environment &Env);

  bool merge(QualType Type, const dataflow::Value &Val1,
             const dataflow::Environment &Env1, const dataflow::Value &Val2,
             const dataflow::Environment &Env2, dataflow::Value &MergedVal,
             dataflow::Environment &MergedEnv) override;

 private:
  // Applies non-flow-sensitive transfer functions on statements
  dataflow::CFGMatchSwitch<dataflow::TransferState<PointerNullabilityLattice>>
      NonFlowSensitiveTransferer;

  // Applies flow-sensitive transfer functions on statements
  dataflow::CFGMatchSwitch<dataflow::TransferState<PointerNullabilityLattice>>
      FlowSensitiveTransferer;
};
}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_NULLABILITY_POINTER_NULLABILITY_ANALYSIS_H_
