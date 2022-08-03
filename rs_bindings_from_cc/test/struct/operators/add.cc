// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/struct/operators/add.h"

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
