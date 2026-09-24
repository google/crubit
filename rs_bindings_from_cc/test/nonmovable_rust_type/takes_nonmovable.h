// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_NONMOVABLE_RUST_TYPE_TAKES_NONMOVABLE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_NONMOVABLE_RUST_TYPE_TAKES_NONMOVABLE_H_

#include <cstdint>

#include "rs_bindings_from_cc/test/nonmovable_rust_type/nonmovable.h"

namespace crubit_test {

// Takes a Rust-native, non-C++-movable type by value.
inline std::uint8_t TakesByValue(::nonmovable::NonMovable x) {
  return x.read_byte();
}

}  // namespace crubit_test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_NONMOVABLE_RUST_TYPE_TAKES_NONMOVABLE_H_
