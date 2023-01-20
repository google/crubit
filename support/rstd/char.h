// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_SUPPORT_RSTD_CHAR_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_SUPPORT_RSTD_CHAR_H_

#include <cstdint>
#include <optional>

#include "absl/base/optimization.h"

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

  // Converts a `uint32_t` into a `rstd::Char`.
  //
  // Note that not all valid `uint32_t`s are valid `rstd::Char`s. `from_u32`
  // will return `std::nullopt` if the input is not a valid value for a
  // `rstd::Char`.
  //
  // See also
  // https://doc.rust-lang.org/reference/behavior-considered-undefined.html
  // which documents that undefined behavior may result in presence of "A value
  // in a char which is a surrogate or above char::MAX."
  //
  // This function mimics Rust's `char::from_u32`:
  // https://doc.rust-lang.org/std/primitive.char.html#method.from_u32
  static constexpr std::optional<Char> from_u32(char32_t c) {
    // TODO(lukasza): Consider using slightly more efficient checks similarly
    // to how `char_try_from_u32` is implemented in Rust standard library.
    if (ABSL_PREDICT_FALSE(c > 0x10ffff)) {
      // Value greater than Rust's `char::MAX`:
      // https://doc.rust-lang.org/std/primitive.char.html#associatedconstant.MAX
      return std::nullopt;
    }

    if (ABSL_PREDICT_FALSE(c >= 0xd800 && c <= 0xdfff)) {
      // Surrogate characters.
      return std::nullopt;
    }

    return from_u32_unchecked(c);
  }

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

  // The highest valid code point a char can have, '\u{10FFFF}'.
  //
  // This constant mimics Rust's `char::MAX`:
  // https://doc.rust-lang.org/std/primitive.char.html#associatedconstant.MAX
  static const Char MAX;

 private:
  // This function mimics Rust's `char::from_u32_unchecked`:
  // https://doc.rust-lang.org/std/primitive.char.html#method.from_u32_unchecked
  //
  // TODO(b/254095482): Figure out how to annotate/expose unsafe functions in
  // C++ and then make this method public.
  static constexpr Char from_u32_unchecked(std::uint32_t value) {
    return Char(value);
  }

  // Private constructor - intended to only be used from `from_u32_unchecked`.
  explicit constexpr Char(std::uint32_t value) : value_(value) {}

  // See "layout tests" comments in `char_test.cc` for explanation why
  // `char32_t` is not used.
  std::uint32_t value_ = '\0';
};

// Definition of `Char::MAX` - it can't be defined and declared within the
// `class` definition, because before `Char` is fully defined the compiler
// complains that `constexpr` variable cannot have non-literal type
// 'const Char'.
constexpr Char Char::MAX = Char::from_u32_unchecked(0x10ffff);

}  // namespace rstd

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_SUPPORT_RSTD_CHAR_H_
