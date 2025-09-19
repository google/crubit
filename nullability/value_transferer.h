// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_NULLABILITY_VALUE_TRANSFERER_H_
#define THIRD_PARTY_CRUBIT_NULLABILITY_VALUE_TRANSFERER_H_

#include "absl/base/nullability.h"
#include "nullability/pointer_nullability_lattice.h"
#include "clang/AST/Expr.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/MatchSwitch.h"
#include "clang/Analysis/FlowSensitive/Value.h"

namespace clang::tidy::nullability {
// Returns a switch over CFG elements that can be used to transfer
// (flow-sensitive) value properties, i.e. to update the analysis state to
// account for the impact of the CFG element.
dataflow::CFGMatchSwitch<dataflow::TransferState<PointerNullabilityLattice>>
buildValueTransferer();

// If `E` is already associated with a `PointerValue`, returns it.
// Otherwise, associates a newly created `PointerValue` with `E` and returns it.
// Returns null iff `E` is not a raw pointer expression.
dataflow::PointerValue* absl_nullable ensureRawPointerHasValue(
    const Expr* absl_nonnull E, dataflow::Environment& Env);

// Initialize the null state of `PointerVal` based on the nullability of the
// type of `E`.
void initPointerFromTypeNullability(
    dataflow::PointerValue& PointerVal, const Expr* absl_nonnull E,
    dataflow::TransferState<PointerNullabilityLattice>& State);

// Ensure that all expressions of smart pointer type have an underlying
// raw pointer initialized from the type nullability.
void ensureSmartPointerInitialized(
    const CFGElement& Elt,
    dataflow::TransferState<PointerNullabilityLattice>& State);
}  // namespace clang::tidy::nullability

#endif  // THIRD_PARTY_CRUBIT_NULLABILITY_VALUE_TRANSFERER_H_
