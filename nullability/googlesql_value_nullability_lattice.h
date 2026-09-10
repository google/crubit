// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_GOOGLESQL_VALUE_NULLABILITY_LATTICE_H_
#define CRUBIT_NULLABILITY_GOOGLESQL_VALUE_NULLABILITY_LATTICE_H_

#include "clang/Analysis/FlowSensitive/DataflowLattice.h"

namespace clang {
namespace tidy {
namespace nullability {

// The lattice for GoogleSQL value nullability analysis is empty in terms of
// tracked analysis state fields. We track all specific states (nullability
// logical formulas) in the `dataflow::Environment` using synthetic fields on
// `RecordStorageLocation`.
//
// However, the Clang Dataflow Analysis framework requires a lattice type to be
// provided as a template parameter to manage the state that joins at control
// flow branches. Therefore, this struct acts as a trivial placeholder with
// no-op `==` and `join` hooks to satisfy the API.
struct GoogleSqlValueNullabilityLattice {
  bool operator==(const GoogleSqlValueNullabilityLattice& Other) const {
    return true;
  }
  dataflow::LatticeJoinEffect join(
      const GoogleSqlValueNullabilityLattice& Other) {
    return dataflow::LatticeJoinEffect::Unchanged;
  }
};

}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // THIRD_PARTY_CRUBIT_NULLABILITY_GOOGLESQL_VALUE_NULLABILITY_LATTICE_H_
