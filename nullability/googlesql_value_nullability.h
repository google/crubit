// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_GOOGLESQL_VALUE_NULLABILITY_H_
#define CRUBIT_NULLABILITY_GOOGLESQL_VALUE_NULLABILITY_H_

#include "absl/base/nullability.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/DataflowLattice.h"
#include "clang/Analysis/FlowSensitive/Formula.h"

namespace clang {
namespace dataflow {
class Arena;
class RecordStorageLocation;
}  // namespace dataflow

namespace tidy {
namespace nullability {

// Represents the nullability state of a `googlesql::Value` object.
//
// This state is tracked via a boolean formula stored as a synthetic field on
// the `RecordStorageLocation` representing the `googlesql::Value` instance.
struct GoogleSqlValueNullState {
  // A boolean formula representing whether the value is null.
  // If null, the state is unknown (equivalent to Top).
  const dataflow::Formula* absl_nullable IsNull = nullptr;

  bool operator==(const GoogleSqlValueNullState& Other) const {
    return IsNull == Other.IsNull;
  }
  bool operator!=(const GoogleSqlValueNullState& Other) const {
    return !(*this == Other);
  }

  // Returns an unknown state (Top).
  static GoogleSqlValueNullState getTop() { return {}; }

  // Joins this state with another state.
  // If the states differ, the result is unknown (Top).
  dataflow::LatticeJoinEffect join(const GoogleSqlValueNullState& Other);
};

// Returns true if the given location has a mapped GoogleSQL value null state
// in the environment.
bool hasGoogleSqlValueNullState(const dataflow::RecordStorageLocation& Loc,
                                const dataflow::Environment& Env);

// Retrieves the GoogleSQL value null state for the given location.
// Returns an unknown state (Top) if not found.
GoogleSqlValueNullState getGoogleSqlValueNullState(
    const dataflow::RecordStorageLocation& Loc,
    const dataflow::Environment& Env);

// Initializes the GoogleSQL value null state for the given location.
// This should be called when a `googlesql::Value` object is newly created
// and we need to establish its initial nullability state.
void initGoogleSqlValueNullState(dataflow::RecordStorageLocation& Loc,
                                 dataflow::Environment& Env,
                                 const dataflow::Formula* absl_nullable IsNull);

// Sets the GoogleSQL value null state for the given location.
// This can be used to update the state of an existing object, e.g., after
// an assignment.
void setGoogleSqlValueNullState(dataflow::RecordStorageLocation& Loc,
                                dataflow::Environment& Env,
                                const dataflow::Formula* absl_nullable IsNull);

}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_NULLABILITY_GOOGLESQL_VALUE_NULLABILITY_H_
