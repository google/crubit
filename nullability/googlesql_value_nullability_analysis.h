#include "clang/AST/Stmt.h"
#include "clang/Analysis/CFG.h"
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_GOOGLESQL_VALUE_NULLABILITY_ANALYSIS_H_
#define CRUBIT_NULLABILITY_GOOGLESQL_VALUE_NULLABILITY_ANALYSIS_H_

#include "nullability/googlesql_value_nullability_lattice.h"
#include "clang/AST/ASTContext.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "llvm/ADT/SmallVector.h"

namespace clang {
namespace tidy {
namespace nullability {

// Custom dataflow analysis to track the nullability of `googlesql::Value`
// objects.
//
// This analysis ensures that accessor methods on `googlesql::Value` are only
// called after a successful `is_null()` check, preventing runtime crashes.
// It uses synthetic fields on `RecordStorageLocation` to track the nullability
// state across control flow merges.
class GoogleSqlValueNullabilityAnalysis
    : public dataflow::DataflowAnalysis<GoogleSqlValueNullabilityAnalysis,
                                        GoogleSqlValueNullabilityLattice> {
 public:
  explicit GoogleSqlValueNullabilityAnalysis(ASTContext& Context)
      : DataflowAnalysis<GoogleSqlValueNullabilityAnalysis,
                         GoogleSqlValueNullabilityLattice>(Context) {}

  // Returns the initial lattice element for the analysis.
  GoogleSqlValueNullabilityLattice initialElement() { return {}; }

  // Dispatches to specific transfer functions based on the CFG element type.
  // This method is called by the Clang Dataflow framework for each element in
  // the CFG.
  void transfer(const CFGElement& Elt,
                GoogleSqlValueNullabilityLattice& Lattice,
                dataflow::Environment& Env);
};

// Diagnoses unsafe accesses to `googlesql::Value` objects.
//
// Returns a list of statements where an accessor method is called on a
// `googlesql::Value` object that is not proven to be non-null. These statements
// represent potential runtime crashes.
llvm::SmallVector<const Stmt*, 1> diagnoseGoogleSqlValueNullability(
    const CFGElement& Elt, ASTContext& ASTCtx,
    const dataflow::Environment& Env);

}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_NULLABILITY_GOOGLESQL_VALUE_NULLABILITY_ANALYSIS_H_
