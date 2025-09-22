// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_NULLABILITY_TYPE_TRANSFERER_H_
#define THIRD_PARTY_CRUBIT_NULLABILITY_TYPE_TRANSFERER_H_

#include "nullability/pointer_nullability_lattice.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/MatchSwitch.h"

namespace clang::tidy::nullability {
// Returns a switch over CFG elements that can be used to transfer
// (non-flow-sensitive) type properties, i.e. to update the analysis state to
// account for the impact of the CFG element.
dataflow::CFGMatchSwitch<dataflow::TransferState<PointerNullabilityLattice>>
buildTypeTransferer();

}  // namespace clang::tidy::nullability

#endif  // THIRD_PARTY_CRUBIT_NULLABILITY_TYPE_TRANSFERER_H_
