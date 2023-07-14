// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rs_std/rs_char.h"

#include <stdint.h>

#include <optional>
#include <type_traits>

#include "gtest/gtest.h"

namespace {

// Check that `rs_std::rs_char` is trivially destructible, copyable, and
// moveable.
//
// There are no constructor-related checks, because well-formed-ness checks
// require going through factory methods like `rs_char::from_u32`.
static_assert(std::is_trivially_destructible_v<rs_std::rs_char>);
static_assert(std::is_trivially_copy_constructible_v<rs_std::rs_char>);
static_assert(std::is_trivially_copy_assignable_v<rs_std::rs_char>);
static_assert(std::is_trivially_move_constructible_v<rs_std::rs_char>);
static_assert(std::is_trivially_move_assignable_v<rs_std::rs_char>);

// ABI^H^H^HLayout assertions.
//
// https://rust-lang.github.io/unsafe-code-guidelines/layout/scalars.html#char
// documents that "Rust char is 32-bit wide and represents an unicode scalar
// value".
//
// We don't map Rust's `char` to C++ `char32_t` because
// https://en.cppreference.com/w/cpp/language/types#char32_t points out that the
// builtin `char32_t` type "has the same size, signedness, and alignment as
// std::uint_least32_t" (and therefore it is not guaranteed to be exactly
// 32-bits wide as required for ABI-compatibility with Rust).
//
// Equivalent layout and ABI assertion are also checked on Rust side in
// `format_ty_for_cc` in `cc_bindings_from_rs/bindings.rs` via `layout.align()`
// and `layout.size()`.  It seems that there is no way to check `layout.abi()`
// on C++ side, but we can at least say that under the System V ABI a struct
// with a single field (i.e. the `rs_char` struct) has the same ABI
// classification as the field (as long as the field is smaller than "eight
// eightbytes" and the struct is trivial as verified via `static_assert`s
// above).  In other words, under System V ABI we expect `rs_char` to be of
// INTEGER class - the same as verified by the `layout.abi()` assertion in
// `bindings.rs`.
static_assert(sizeof(rs_std::rs_char) == 4);
static_assert(alignof(rs_std::rs_char) == 4);
static_assert(std::is_standard_layout_v<rs_std::rs_char>);

// This test covers the following case from
// https://en.cppreference.com/w/cpp/language/character_literal:
//
// Ordinary character literal, e.g. 'a' or '\n' or '\13'. Such literal has type
// `char` and the value equal to either:
// - the representation of c-char in the execution character set (until C++23)
// - the corresponding code point from ordinary literal encoding (since C++23).
TEST(RsCharTest, FromAsciiLiteral) {
  std::optional<const rs_std::rs_char> c = rs_std::rs_char::from_u32('x');
  ASSERT_TRUE(c.has_value());
  EXPECT_EQ(0x78, uint32_t{*c});
}

// This test covers the following case from
// https://en.cppreference.com/w/cpp/language/character_literal:
//
// UTF-8 character literal, e.g. u8'a'. Such literal has type `char` (until
// C++20) or `char8_t` (since C++20) and the value equal to ISO/IEC 10646 code
// point value of c-char, provided that the code point value is representable
// with a single UTF-8 code unit (that is, c-char is in the range 0x0-0x7F,
// inclusive).
TEST(RsCharTest, FromUtf8Literal) {
  std::optional<const rs_std::rs_char> c = rs_std::rs_char::from_u32(u8'x');
  ASSERT_TRUE(c.has_value());
  EXPECT_EQ(0x78, uint32_t{*c});
}

// This test covers the following case from
// https://en.cppreference.com/w/cpp/language/character_literal:
//
// UTF-16 character literal, e.g. u'猫', but not u'🍌' (u'\U0001f34c'). Such
// literal has type `char16_t` and the value equal to ISO/IEC 10646 code point
// value of c-char, provided that the code point value is representable with a
// single UTF-16 code unit (that is, c-char is in the range 0x0-0xFFFF,
// inclusive).
TEST(RsCharTest, FromUtf16Literal) {
  std::optional<const rs_std::rs_char> c = rs_std::rs_char::from_u32(u'Ł');
  ASSERT_TRUE(c.has_value());
  EXPECT_EQ(0x141, uint32_t{*c});
}

// This test covers the following case from
// https://en.cppreference.com/w/cpp/language/character_literal:
//
// UTF-32 character literal, e.g. U'猫' or U'🍌'. Such literal has type
// `char32_t` and the value equal to ISO/IEC 10646 code point value of c-char.
TEST(RsCharTest, FromUtf32Literal) {
  std::optional<const rs_std::rs_char> c = rs_std::rs_char::from_u32(U'🦀');
  ASSERT_TRUE(c.has_value());
  EXPECT_EQ(0x1F980, uint32_t{*c});
}

TEST(RsCharTest, FromU32ValidityChecks) {
  // Max 32-bit value.
  EXPECT_FALSE(rs_std::rs_char::from_u32(0xffffffff).has_value());

  // A value just above Rust's `char::MAX`:
  // https://doc.rust-lang.org/std/primitive.char.html#associatedconstant.MAX.
  EXPECT_FALSE(rs_std::rs_char::from_u32(0x110000).has_value());

  // Smallest/greatest "high"/"low" surrogates.
  EXPECT_FALSE(rs_std::rs_char::from_u32(0xd800).has_value());
  EXPECT_FALSE(rs_std::rs_char::from_u32(0xdbff).has_value());
  EXPECT_FALSE(rs_std::rs_char::from_u32(0xdc00).has_value());
  EXPECT_FALSE(rs_std::rs_char::from_u32(0xdfff).has_value());

  // Smallest valid value.
  std::optional<rs_std::rs_char> maybe_c = rs_std::rs_char::from_u32('\0');
  ASSERT_TRUE(maybe_c.has_value());
  EXPECT_EQ(0x00, uint32_t{*maybe_c});

  // Greatest valid value.  See also Rust's `char::MAX`:
  // https://doc.rust-lang.org/std/primitive.char.html#associatedconstant.MAX.
  maybe_c = rs_std::rs_char::from_u32(0x10ffff);
  ASSERT_TRUE(maybe_c.has_value());
  EXPECT_EQ(0x10ffff, uint32_t{*maybe_c});

  // Just below surrogates.
  maybe_c = rs_std::rs_char::from_u32(0xd7ff);
  ASSERT_TRUE(maybe_c.has_value());
  EXPECT_EQ(0xd7ff, uint32_t{*maybe_c});

  // Just above surrogates.
  maybe_c = rs_std::rs_char::from_u32(0xe000);
  ASSERT_TRUE(maybe_c.has_value());
  EXPECT_EQ(0xe000, uint32_t{*maybe_c});
}

// Test that `rs_std::rs_char` values can be compared with other
// `rs_std::rs_char` values.
TEST(RsCharTest, ComparisonWithAnotherRsChar) {
  std::optional<const rs_std::rs_char> a = rs_std::rs_char::from_u32('a');
  std::optional<const rs_std::rs_char> b = rs_std::rs_char::from_u32('b');
  ASSERT_TRUE(a.has_value());
  ASSERT_TRUE(b.has_value());

  EXPECT_TRUE(*a == *a);
  EXPECT_FALSE(*a != *a);
  EXPECT_TRUE(*a <= *a);
  EXPECT_FALSE(a < *a);
  EXPECT_TRUE(*a >= *a);
  EXPECT_FALSE(*a > *a);

  EXPECT_FALSE(*a == *b);
  EXPECT_TRUE(*a != *b);
  EXPECT_TRUE(*a <= *b);
  EXPECT_TRUE(*a < *b);
  EXPECT_FALSE(*a >= *b);
  EXPECT_FALSE(*a > *b);

  EXPECT_FALSE(*b == *a);
  EXPECT_TRUE(*b != *a);
  EXPECT_FALSE(*b <= *a);
  EXPECT_FALSE(*b < *a);
  EXPECT_TRUE(*b >= *a);
  EXPECT_TRUE(*b > *a);
}

TEST(RsCharTest, DefaultConstructedValue) {
  rs_std::rs_char c;
  EXPECT_EQ(0, uint32_t{c});
}

}  // namespace
