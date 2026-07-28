// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/rs_std/int.h"

#ifndef CRUBIT_SUPPORT_RS_STD_INT_H_
#define CRUBIT_SUPPORT_RS_STD_INT_H_

#include <compare>
#include <cstdint>
#include <type_traits>

#include "support/annotations.h"

namespace rs_std {

// `rs_std::usize` is a C++ representation of the `usize` type from Rust when
// used as a template argument inside template specializations (such as
// `rs_std::Option<T>`). This prevents C++ explicit template specialization
// redefinition errors or overload collisions between `usize`, `u64`, and `u32`.
class CRUBIT_INTERNAL_RUST_TYPE("usize") usize final {
 public:
  constexpr usize() noexcept = default;

  // Implicit constructing conversion from standard integer types.
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr usize(::std::uintptr_t value) noexcept : value_(value) {}

  constexpr usize(const usize&) noexcept = default;
  constexpr usize& operator=(const usize&) noexcept = default;

  // Implicit conversion back to `::std::uintptr_t` for seamless indexing and
  // arithmetic in C++.
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr operator ::std::uintptr_t() const noexcept { return value_; }

  friend constexpr bool operator==(usize lhs, usize rhs) noexcept = default;
  friend constexpr std::strong_ordering operator<=>(
      usize lhs, usize rhs) noexcept = default;

  friend constexpr usize operator+(usize lhs, usize rhs) noexcept {
    return usize(lhs.value_ + rhs.value_);
  }
  friend constexpr usize operator-(usize lhs, usize rhs) noexcept {
    return usize(lhs.value_ - rhs.value_);
  }
  constexpr usize& operator+=(usize rhs) noexcept {
    value_ += rhs.value_;
    return *this;
  }
  constexpr usize& operator-=(usize rhs) noexcept {
    value_ -= rhs.value_;
    return *this;
  }

 private:
  ::std::uintptr_t value_ = 0;
};

static_assert(sizeof(usize) == sizeof(::std::uintptr_t));
static_assert(alignof(usize) == alignof(::std::uintptr_t));
static_assert(::std::is_trivially_copyable_v<usize>);
static_assert(::std::is_trivially_destructible_v<usize>);

// `rs_std::isize` is a C++ representation of the `isize` type from Rust when
// used as a template argument inside template specializations (such as
// `rs_std::Option<T>`). This prevents C++ explicit template specialization
// redefinition errors or overload collisions between `isize`, `i64`, and `i32`.
class CRUBIT_INTERNAL_RUST_TYPE("isize") isize final {
 public:
  constexpr isize() noexcept = default;

  // Implicit constructing conversion from standard integer types.
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr isize(::std::intptr_t value) noexcept : value_(value) {}

  constexpr isize(const isize&) noexcept = default;
  constexpr isize& operator=(const isize&) noexcept = default;

  // Implicit conversion back to `::std::intptr_t` for seamless arithmetic and
  // conversion in C++.
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr operator ::std::intptr_t() const noexcept { return value_; }

  friend constexpr bool operator==(isize lhs, isize rhs) noexcept = default;
  friend constexpr auto operator<=>(isize lhs, isize rhs) noexcept = default;

  friend constexpr isize operator+(isize lhs, isize rhs) noexcept {
    return isize(lhs.value_ + rhs.value_);
  }
  friend constexpr isize operator-(isize lhs, isize rhs) noexcept {
    return isize(lhs.value_ - rhs.value_);
  }
  constexpr isize& operator+=(isize rhs) noexcept {
    value_ += rhs.value_;
    return *this;
  }
  constexpr isize& operator-=(isize rhs) noexcept {
    value_ -= rhs.value_;
    return *this;
  }

 private:
  ::std::intptr_t value_ = 0;
};

static_assert(sizeof(isize) == sizeof(::std::intptr_t));
static_assert(alignof(isize) == alignof(::std::intptr_t));
static_assert(::std::is_trivially_copyable_v<isize>);
static_assert(::std::is_trivially_destructible_v<isize>);

}  // namespace rs_std

#endif  // CRUBIT_SUPPORT_RS_STD_INT_H_
