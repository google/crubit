// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/struct/operators/operators.h"

bool OperandForOutOfLineDefinition::operator==(
    const OperandForOutOfLineDefinition& other) const {
  return (i % 10) == (other.i % 10);
}

bool operator==(const OperandForFreeFunc& lhs, const OperandForFreeFunc& rhs) {
  return (lhs.i % 10) == (rhs.i % 10);
}

namespace some_other_namespace {

bool operator==(const OperandForFreeFuncInDifferentNamespace& lhs,
                const OperandForFreeFuncInDifferentNamespace& rhs) {
  return (lhs.i % 10) == (rhs.i % 10);
}

}  // namespace some_other_namespace

//////////////////////////////////////////////////////////////////////

UnpinStruct operator+(const UnpinStruct& lhs, const UnpinStruct& rhs) {
  return UnpinStruct{lhs.i + rhs.i};
}

UnpinStruct operator+(UnpinStruct& lhs, UnpinStruct& rhs) {
  return UnpinStruct{lhs.i + rhs.i};
}

UnpinStruct operator+(UnpinStruct lhs, UnpinStruct rhs) {
  return UnpinStruct{lhs.i + rhs.i};
}

char operator+(AddableOverloaded lhs, std::int16_t rhs) {
  return lhs.int16_char;
}

char operator+(AddableOverloaded lhs, std::int32_t rhs) {
  return lhs.int32_char;
}
