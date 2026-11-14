// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_BAZEL_TYPES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_BAZEL_TYPES_H_

#include <string>
#include <utility>

#include "common/string_type.h"

namespace crubit {

// Representation of a Bazel label (for example //foo/bar:baz).
class BazelLabel : public StringType<BazelLabel> {
 public:
  BazelLabel() = default;
  explicit BazelLabel(std::string value)
      : StringType(value.starts_with("@@//") ? value.substr(2)
                                             : std::move(value)) {}
};

}  // namespace crubit

#include "llvm/ADT/DenseMapInfo.h"
#include "llvm/ADT/Hashing.h"

namespace llvm {

template <>
struct DenseMapInfo<crubit::BazelLabel> {
  static crubit::BazelLabel getEmptyKey() {
    return crubit::BazelLabel("\x01<empty>");
  }
  static crubit::BazelLabel getTombstoneKey() {
    return crubit::BazelLabel("\x01<tombstone>");
  }
  static unsigned getHashValue(const crubit::BazelLabel& Val) {
    return static_cast<unsigned>(llvm::hash_value(Val.value()));
  }
  static bool isEqual(const crubit::BazelLabel& LHS,
                      const crubit::BazelLabel& RHS) {
    return LHS == RHS;
  }
};

}  // namespace llvm

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_BAZEL_TYPES_H_
