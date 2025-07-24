// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_SUPPORT_RUST_STD_CHAR_H_
#define CRUBIT_SUPPORT_RUST_STD_CHAR_H_

#include <cstdint>
#include <optional>

#include "absl/base/optimization.h"
#include "support/internal/attribute_macros.h"

namespace rs_std {

// `rs_std::char_` is a C++ representation of the `char` type from Rust.
// `rust_builtin_type_abi_assumptions.md` documents the ABI compatibility of
// these types.
class CRUBIT_INTERNAL_RUST_TYPE("char") CRUBIT_INTERNAL_SAME_ABI char_ final {
 public:
  // Creates a default `char_` - one that represents ASCII NUL character.
  //
  // Providing the default constructor helps to ensure that the `value_` always
  // effectively stores a C++ equivalent of a well-defined Rust's `u32` value
  // (and never has a `MaybeUninit<u32>` value).  See also the P2723R1 proposal
  // for C++ which argues that zero-initialization may mitigate 10% of exploits.
  constexpr char_() = default;

  // Converts a `uint32_t` into a `rs_std::char_`.
  //
  // Note that not all valid `uint32_t`s are valid `rs_std::char_`s.
  // `from_u32` will return `std::nullopt` if the input is not a valid value for
  // a `rs_std::char_`.
  //
  // See also
  // https://doc.rust-lang.org/reference/behavior-considered-undefined.html
  // which documents that undefined behavior may result in presence of "A value
  // in a char which is a surrogate or above char::MAX."
  //
  // This function mimics Rust's `char::from_u32`:
  // https://doc.rust-lang.org/std/primitive.char.html#method.from_u32
  static constexpr std::optional<char_> from_u32(char32_t c) {
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

  constexpr char_(const char_&) = default;
  constexpr char_& operator=(const char_&) = default;
  constexpr char_(char_&&) = default;
  constexpr char_& operator=(char_&&) = default;
  ~char_() = default;

  explicit constexpr operator std::uint32_t() const { return value_; }

  constexpr bool operator==(const char_& other) const {
    return value_ == other.value_;
  }
  constexpr bool operator!=(const char_& other) const {
    return value_ != other.value_;
  }
  constexpr bool operator<=(const char_& other) const {
    return value_ <= other.value_;
  }
  constexpr bool operator<(const char_& other) const {
    return value_ < other.value_;
  }
  constexpr bool operator>=(const char_& other) const {
    return value_ >= other.value_;
  }
  constexpr bool operator>(const char_& other) const {
    return value_ > other.value_;
  }

  // The highest valid code point a char can have, '\u{10FFFF}'.
  //
  // This constant mimics Rust's `char::MAX`:
  // https://doc.rust-lang.org/std/primitive.char.html#associatedconstant.MAX
  static const char_ MAX;

 private:
  // This function mimics Rust's `char::from_u32_unchecked`:
  // https://doc.rust-lang.org/std/primitive.char.html#method.from_u32_unchecked
  //
  // TODO(b/254095482): Figure out how to annotate/expose unsafe functions in
  // C++ and then make this method public.
  static constexpr char_ from_u32_unchecked(std::uint32_t value) {
    return char_(value);
  }

  // Private constructor - intended to only be used from `from_u32_unchecked`.
  explicit constexpr char_(std::uint32_t value) : value_(value) {}

  // See "layout tests" comments in `char_test.cc` for explanation why
  // `char32_t` is not used.
  std::uint32_t value_ = '\0';
};

// Definition of `char_::MAX` - it can't be defined and declared within the
// `class` definition, because before `char_` is fully defined the compiler
// complains that `constexpr` variable cannot have non-literal type
// 'const char_'.
constexpr char_ char_::MAX = char_::from_u32_unchecked(0x10ffff);

}  // namespace rs_std

#endif  // CRUBIT_SUPPORT_RS_STD_CHAR_H_
