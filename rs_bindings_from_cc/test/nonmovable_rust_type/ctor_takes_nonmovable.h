// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_NONMOVABLE_RUST_TYPE_CTOR_TAKES_NONMOVABLE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_NONMOVABLE_RUST_TYPE_CTOR_TAKES_NONMOVABLE_H_

#include "rs_bindings_from_cc/test/nonmovable_rust_type/nonmovable.h"

namespace crubit_test {

// A constructor which takes a Rust-native, non-C++-movable type by value.
struct CtorTakesByValue final {
  explicit CtorTakesByValue(::nonmovable::NonMovable x) {}
};

// Not trivially relocatable, so it is `!Unpin` in Rust, and its constructor is
// a deferred `Ctor` which holds the Rust value until the `Ctor` runs.
class NonUnpinCtorTakesByValue final {
 public:
  explicit NonUnpinCtorTakesByValue(::nonmovable::NonMovable x) {}
  // NOLINTNEXTLINE(modernize-use-equals-default)
  ~NonUnpinCtorTakesByValue() {}
};

}  // namespace crubit_test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_NONMOVABLE_RUST_TYPE_CTOR_TAKES_NONMOVABLE_H_
