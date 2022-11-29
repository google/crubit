// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_ANALYSIS_H_
#define CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_ANALYSIS_H_

#include "nullability_verification/pointer_nullability_lattice.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Type.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Value.h"

namespace clang {
namespace tidy {
namespace nullability {

/// Analyses constructs in the source code to collect nullability information
/// about pointers at each program point.
class PointerNullabilityAnalysis
    : public dataflow::DataflowAnalysis<PointerNullabilityAnalysis,
                                        PointerNullabilityLattice> {
 public:
  explicit PointerNullabilityAnalysis(ASTContext& context);

  static PointerNullabilityLattice initialElement() { return {}; }

  void transfer(const CFGElement* Elt, PointerNullabilityLattice& Lattice,
                dataflow::Environment& Env);

  bool merge(QualType Type, const dataflow::Value& Val1,
             const dataflow::Environment& Env1, const dataflow::Value& Val2,
             const dataflow::Environment& Env2, dataflow::Value& MergedVal,
             dataflow::Environment& MergedEnv) override;

 private:
  // Applies transfer functions on statements
  dataflow::CFGMatchSwitch<dataflow::TransferState<PointerNullabilityLattice>>
      Transferer;
};
}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_ANALYSIS_H_
