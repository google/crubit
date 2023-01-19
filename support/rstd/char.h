// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_SUPPORT_RSTD_CHAR_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_SUPPORT_RSTD_CHAR_H_

#include <cstdint>

namespace rstd {

// `rstd::Char` is a C++ representation of the `char` type from Rust.
class Char final {
 public:
  // Creates a default `Char` - one that represents ASCII NUL character.
  //
  // Providing the default constructor helps to ensure that the `value_` always
  // effectively stores a C++ equivalent of a well-defined Rust's `u32` value
  // (and never has a `MaybeUninit<u32>` value).  See also the P2723R1 proposal
  // for C++ which argues that zero-initialization may mitigate 10% of exploits.
  constexpr Char() = default;

  // TODO(b/265338802): Reject `char` values that may represent a part of a
  // UTF-8 character (i.e. only the first 0-127 ASCII characters should be
  // accepted).
  constexpr explicit Char(char c) : value_(c) {}

  // TODO(b/265338802): Reject `char` values with invalid bit patterns
  // (retaining the `constexpr` aspect if possible).
  constexpr explicit Char(char16_t c) : value_(c) {}
  constexpr explicit Char(char32_t c) : value_(c) {}

  constexpr Char(const Char&) = default;
  constexpr Char& operator=(const Char&) = default;
  constexpr Char(Char&&) = default;
  constexpr Char& operator=(Char&&) = default;
  ~Char() = default;

  explicit constexpr operator std::uint32_t() const { return value_; }

  constexpr bool operator==(const Char& other) const {
    return value_ == other.value_;
  }
  constexpr bool operator!=(const Char& other) const {
    return value_ != other.value_;
  }
  constexpr bool operator<=(const Char& other) const {
    return value_ <= other.value_;
  }
  constexpr bool operator<(const Char& other) const {
    return value_ < other.value_;
  }
  constexpr bool operator>=(const Char& other) const {
    return value_ >= other.value_;
  }
  constexpr bool operator>(const Char& other) const {
    return value_ > other.value_;
  }

 private:
  // See "layout tests" comments in `char_test.cc` for explanation why
  // `char32_t` is not used.
  std::uint32_t value_ = '\0';
};

}  // namespace rstd

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_SUPPORT_RSTD_CHAR_H_
