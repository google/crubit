// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_LATTICE_H_
#define THIRD_PARTY_CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_LATTICE_H_

#include <ostream>

#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/DataflowLattice.h"

namespace clang {
namespace tidy {
namespace nullability {

class PointerNullabilityLattice {
 public:
  bool operator==(const PointerNullabilityLattice &Other) const { return true; }

  dataflow::LatticeJoinEffect join(const PointerNullabilityLattice &Other) {
    return dataflow::LatticeJoinEffect::Unchanged;
  }
};

inline std::ostream &operator<<(std::ostream &OS,
                                const PointerNullabilityLattice &) {
  return OS << "noop";
}

}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // THIRD_PARTY_CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_LATTICE_H_
