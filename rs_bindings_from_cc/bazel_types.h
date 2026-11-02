// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_BAZEL_TYPES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_BAZEL_TYPES_H_

#include <string>

#include "common/string_type.h"

namespace crubit {

// Representation of a Bazel label (for example //foo/bar:baz).
CRUBIT_DEFINE_STRING_TYPE(BazelLabel);

}  // namespace crubit

#include "llvm/ADT/DenseMapInfo.h"
#include "llvm/ADT/Hashing.h"

namespace llvm {

template <>
struct DenseMapInfo<crubit::BazelLabel> {
  static inline crubit::BazelLabel getEmptyKey() {
    return crubit::BazelLabel("\x01<empty>");
  }
  static inline crubit::BazelLabel getTombstoneKey() {
    return crubit::BazelLabel("\x01<tombstone>");
  }
  static unsigned getHashValue(const crubit::BazelLabel& Val) {
    return llvm::hash_value(Val.value());
  }
  static bool isEqual(const crubit::BazelLabel& LHS,
                      const crubit::BazelLabel& RHS) {
    return LHS == RHS;
  }
};

}  // namespace llvm

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_BAZEL_TYPES_H_
