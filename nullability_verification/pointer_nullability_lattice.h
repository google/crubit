// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_LATTICE_H_
#define CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_LATTICE_H_

#include <ostream>

#include "clang/Analysis/FlowSensitive/DataflowLattice.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/DenseSet.h"

namespace clang {
namespace tidy {
namespace nullability {
class PointerNullabilityLattice {
 public:
  PointerNullabilityLattice() = default;

  bool operator==(const PointerNullabilityLattice& Other) const { return true; }

  dataflow::LatticeJoinEffect join(const PointerNullabilityLattice& Other) {
    return dataflow::LatticeJoinEffect::Unchanged;
  }

  bool hasPointerNullability(clang::dataflow::PointerValue* PointerVal) {
    return pointer_nullabilities_.find(PointerVal) !=
           pointer_nullabilities_.end();
  }

  dataflow::BoolValue* getPointerNullability(
      dataflow::PointerValue* PointerVal) {
    auto search = pointer_nullabilities_.find(PointerVal);
    return search == pointer_nullabilities_.end() ? nullptr : search->second;
  }

  void setPointerNullability(dataflow::PointerValue* PointerVal,
                             dataflow::BoolValue* BoolVal) {
    pointer_nullabilities_[PointerVal] = BoolVal;
  }

  bool isSafe() const { return violations_.empty(); }
  void addViolation(const Expr* Violation) { violations_.insert(Violation); }

 private:
  llvm::DenseSet<const Expr*> violations_;
  llvm::DenseMap<dataflow::PointerValue*, dataflow::BoolValue*>
      pointer_nullabilities_;
};

inline std::ostream& operator<<(std::ostream& OS,
                                const PointerNullabilityLattice& L) {
  return OS << "Pointer Nullability Lattice";
}
}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_LATTICE_H_
