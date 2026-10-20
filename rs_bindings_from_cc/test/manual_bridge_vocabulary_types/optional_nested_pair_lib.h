// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_OPTIONAL_NESTED_PAIR_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_OPTIONAL_NESTED_PAIR_LIB_H_

#include <optional>
#include <utility>

#include "support/annotations.h"

// In a struct field, `optional` is layout-compatible, forcing its template
// argument `pair` to also be layout-compatible (even without
// `layout_compat_tuple`).
struct CRUBIT_MUST_BIND StructWithOptionalPair {
  std::optional<std::pair<int, int>> opt_pair;
};

CRUBIT_MUST_BIND
inline StructWithOptionalPair MakeStructWithOptionalPair(int first,
                                                         int second) {
  return StructWithOptionalPair{.opt_pair = std::make_pair(first, second)};
}

// Outside of a layout-compatible context, `pair` stays bridged to Rust tuple.
CRUBIT_MUST_BIND
inline std::pair<int, int> MakePlainPair(int first, int second) {
  return std::make_pair(first, second);
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_OPTIONAL_NESTED_PAIR_LIB_H_
