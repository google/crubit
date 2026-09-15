// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_OPTIONAL_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_OPTIONAL_LIB_H_

#include <optional>
#include <vector>

#include "support/annotations.h"

// With the `layout_compat_optional` feature, `std::optional<T>` is
// `cc_std::std::optional<T>` everywhere, including by value.
CRUBIT_MUST_BIND
inline std::optional<int> MakeOptional(int value) { return value; }
CRUBIT_MUST_BIND
inline int UseOptionalByValue(std::optional<int> o) { return o.value_or(-1); }

struct CRUBIT_MUST_BIND StructWithOptionalField {
  std::optional<int> optional_int;
};

CRUBIT_MUST_BIND
inline StructWithOptionalField MakeStructWithOptionalField(int value) {
  return StructWithOptionalField{.optional_int = value};
}

CRUBIT_MUST_BIND
inline int UseOptionalByRef(const std::optional<int>& o) {
  return o.value_or(-1);
}

// Trivially copyable in C++, and `Copy` in Rust too: `int` is `Copy`, so the
// field is a `cc_std::std::trivial_optional`, which has no destructor.
struct CRUBIT_MUST_BIND StructWithNestedOptionalField {
  StructWithOptionalField inner;
};

CRUBIT_MUST_BIND
inline StructWithNestedOptionalField MakeStructWithNestedOptionalField(
    int value) {
  return StructWithNestedOptionalField{.inner =
                                           MakeStructWithOptionalField(value)};
}

// `std::vector<int>` is not `Copy` in Rust, so this is the `Drop`-carrying
// `cc_std::std::optional`, not `cc_std::std::trivial_optional`.
CRUBIT_MUST_BIND
inline std::optional<std::vector<int>> MakeOptionalVector(int value) {
  return std::vector<int>{value};
}

// As an argument to another layout-compatible generic.
CRUBIT_MUST_BIND
inline std::vector<std::optional<int>> MakeVectorOfOptional(int value) {
  return {std::optional<int>(value), std::nullopt};
}

// `std::optional<T>` nests, because both `cc_std::std::optional<T>` and
// `cc_std::std::trivial_optional<T>` are themselves layout-compatible.
CRUBIT_MUST_BIND
inline std::optional<std::optional<int>> MakeNestedOptional(int value) {
  return std::optional<int>(value);
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_OPTIONAL_LIB_H_
