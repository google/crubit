// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/rs_std/tuple.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_TUPLE_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_TUPLE_H_

namespace rs {

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

}  // namespace rs

namespace rs_std {
template <typename... Ts>
using Tuple [[deprecated("Use rs::Tuple instead")]] = rs::Tuple<Ts...>;
}  // namespace rs_std

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_TUPLE_H_
