// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/rs_std/unit.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_UNIT_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_UNIT_H_

#include <type_traits>

namespace rs_std {

// A type representing Rust's unit type `()`.
//
// This type is not layout-compatible with `()` but acts as a marker for that
// type where it appears in APIs. It should not be stored in a FFI type and only
// used to select APIs out of overload, similar to `std::nullopt_t`.
struct unit_t final {
  constexpr unit_t() = default;

  friend constexpr bool operator==(unit_t, unit_t) noexcept { return true; }
  friend constexpr bool operator!=(unit_t, unit_t) noexcept { return false; }
};

inline constexpr unit_t unit{};

template <typename T>
inline constexpr bool is_unit_t_v =
    std::is_same_v<std::remove_cvref_t<T>, unit_t>;

}  // namespace rs_std

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_UNIT_H_
