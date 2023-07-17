// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/struct/operators/add.h"

#include <cstdint>

UnpinStructByConstRef operator+(const UnpinStructByConstRef& lhs,
                                const UnpinStructByConstRef& rhs) {
  return UnpinStructByConstRef{lhs.i + rhs.i};
}

UnpinStructByMutRef operator+(UnpinStructByMutRef& lhs,
                              UnpinStructByMutRef& rhs) {
  return UnpinStructByMutRef{lhs.i + rhs.i};
}

UnpinStructByValue operator+(UnpinStructByValue lhs, UnpinStructByValue rhs) {
  return UnpinStructByValue{lhs.i + rhs.i};
}

char operator+(AddableOverloaded lhs, std::int16_t rhs) {
  return lhs.int16_char;
}

char operator+(AddableOverloaded lhs, std::int32_t rhs) {
  return lhs.int32_char;
}
