// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONVERSION_OPERATORS_CONVERSION_OPERATORS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONVERSION_OPERATORS_CONVERSION_OPERATORS_H_

#include "rs_bindings_from_cc/test/conversion_operators/foreign_type.h"

struct ConvertsIntoIntsRef {
  int value;

  int inner_val;
  explicit operator const int&() const { return inner_val; }
  explicit operator int&() { return inner_val; }
};

struct ConvertsIntoIntImplicitly {
  int value;
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator int() const;
};

struct DstLocalMovable {
  int val;
  explicit DstLocalMovable(int v) : val(v) {}
};

struct DstLocalNonMovable {
  int val;
  explicit DstLocalNonMovable(int v) : val(v) {}

  DstLocalNonMovable(const DstLocalNonMovable&) = delete;
  DstLocalNonMovable(DstLocalNonMovable&&) = delete;
};

struct ConvertsIntoVariousTypes {
  int value;

  explicit operator DstLocalMovable() const;
  explicit operator DstLocalNonMovable() const;

  explicit operator double();

  // Value conversion to foreign/primitive, which should implement Into.
  explicit operator int() const;

  // Value conversion to a foreign immovable type, which should be skipped.
  explicit operator ForeignImmovable() const;
};

namespace namespace_b {
struct Dst {
  int val;
};
}  // namespace namespace_b

namespace namespace_a {
struct Src {
  int value;
  explicit operator namespace_b::Dst() const;
};
}  // namespace namespace_a

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONVERSION_OPERATORS_CONVERSION_OPERATORS_H_
