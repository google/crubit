// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_TUPLE_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_TUPLE_H_

namespace rs_std {

/**
 * A layout-compatible type for Rust tuples.
 *
 * This type should only be used via a generated specialization.
 */
template <typename... Ts>
struct Tuple final {
  static_assert(false,
                "This type should only be used via a generated specialization");
};

}  // namespace rs_std

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_TUPLE_H_
