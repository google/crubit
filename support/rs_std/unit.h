// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/rs/unit.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_UNIT_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_UNIT_H_

#include <type_traits>

#include "support/internal/cpp_config.h"

namespace rs {

// A type representing Rust's unit type `()`.
//
// This type is not layout-compatible with `()` but acts as a marker for that
// type where it appears in APIs. It should not be stored in a FFI type and only
// used to select APIs out of overload, similar to `::std::nullopt_t`.
struct unit_t final {
  constexpr unit_t() = default;

  friend constexpr bool operator==(unit_t, unit_t) noexcept { return true; }
  friend constexpr bool operator!=(unit_t, unit_t) noexcept { return false; }
};

inline constexpr unit_t unit{};

template <typename T>
inline constexpr bool is_unit_t_v =
    ::std::is_same_v<::std::remove_cvref_t<T>, unit_t>;

}  // namespace rs

namespace rs_std {
#ifdef CRUBIT_INTERNAL_RS_NAMESPACE
using unit_t [[deprecated("Use rs::unit_t instead")]] = ::rs::unit_t;
[[deprecated("Use rs::unit instead")]] inline constexpr unit_t unit{};
template <typename T>
[[deprecated(
    "Use rs::is_unit_t_v instead")]] inline constexpr bool is_unit_t_v =
    ::rs::is_unit_t_v<T>;
#else
using unit_t = ::rs::unit_t;
inline constexpr unit_t unit{};
template <typename T>
inline constexpr bool is_unit_t_v = ::rs::is_unit_t_v<T>;
#endif
}  // namespace rs_std

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_UNIT_H_
