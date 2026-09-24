// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_NONMOVABLE_RUST_TYPE_TAKES_NONMOVABLE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_NONMOVABLE_RUST_TYPE_TAKES_NONMOVABLE_H_

#include <cstdint>

#include "rs_bindings_from_cc/test/nonmovable_rust_type/nonmovable.h"

// C++ functions which take a Rust-native type by value.
//
// `::nonmovable::NonMovable` has no C++ move constructor, only a relocating
// `(crubit::UnsafeRelocateTag, NonMovable&&)` constructor. So the thunks that
// Crubit generates for these functions can't initialize the parameter with
// `std::move(*x)`. Instead they call `crubit::UnsafeTakeValue(x)` (see
// `support/internal/slot.h`), which relocates the Rust value into the
// parameter.

namespace crubit_test {

inline std::uint8_t TakesByValue(::nonmovable::NonMovable x) {
  return x.read_byte();
}

inline std::uint8_t TakesTwoByValue(::nonmovable::NonMovable a,
                                    ::nonmovable::NonMovable b) {
  return a.read_byte() + b.read_byte();
}

struct Receiver final {
  std::uint8_t TakesByValue(::nonmovable::NonMovable x) const {
    return x.read_byte();
  }
};

}  // namespace crubit_test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_NONMOVABLE_RUST_TYPE_TAKES_NONMOVABLE_H_
