// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/conversion_operators/conversion_operators.h"

#include "rs_bindings_from_cc/test/conversion_operators/foreign_type.h"

ConvertsIntoIntImplicitly::operator int() const { return value; }

ConvertsIntoVariousTypes::operator DstLocalMovable() const {
  return DstLocalMovable(value);
}

ConvertsIntoVariousTypes::operator DstLocalNonMovable() const {
  return DstLocalNonMovable(value);
}

ConvertsIntoVariousTypes::operator int() const { return value; }

ConvertsIntoVariousTypes::operator double() { return value; }

ConvertsIntoVariousTypes::operator ForeignImmovable() const {
  return ForeignImmovable(value);
}

namespace namespace_a {
Src::operator namespace_b::Dst() const { return namespace_b::Dst{value}; }
}  // namespace namespace_a
