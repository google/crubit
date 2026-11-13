// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/rs/never.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_NEVER_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_NEVER_H_

#include <type_traits>

#include "support/annotations_internal.h"
#include "support/internal/cpp_config.h"

namespace rs {

// A type representing Rust's never type `!`.
struct CRUBIT_INTERNAL_RUST_TYPE("!") CRUBIT_TRIVIAL_ABI never_t final {
  never_t() = delete;
  constexpr never_t(const never_t&) = default;
  constexpr never_t(never_t&&) = default;
  constexpr never_t& operator=(const never_t&) = default;
  constexpr never_t& operator=(never_t&&) = default;
  ~never_t() = default;

  template <typename T>
  operator T() const {
    CRUBIT_UNREACHABLE();
  }

  friend constexpr bool operator==(never_t, never_t) noexcept { return true; }
  friend constexpr bool operator!=(never_t, never_t) noexcept { return false; }
};

template <typename T>
inline constexpr bool is_never_t_v =
    ::std::is_same_v<::std::remove_cvref_t<T>, never_t>;

}  // namespace rs

namespace rs_std {
#ifdef CRUBIT_INTERNAL_RS_NAMESPACE
using never_t [[deprecated("Use rs::never_t instead")]] = ::rs::never_t;
template <typename T>
[[deprecated(
    "Use rs::is_never_t_v instead")]] inline constexpr bool is_never_t_v =
    ::rs::is_never_t_v<T>;
#else
using never_t = ::rs::never_t;
template <typename T>
inline constexpr bool is_never_t_v = ::rs::is_never_t_v<T>;
#endif
}  // namespace rs_std

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_NEVER_H_
