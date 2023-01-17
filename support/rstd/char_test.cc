// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rstd/char.h"

#include <stdint.h>

#include <type_traits>

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace {

// Check that `rstd::Char` is trivially destructible, copyable, and moveable.
//
// There are no constructor-related checks, because well-formed-ness checks
// will make construction non-trivial.  The FromAsciiLiteral, FromUtf32Literal,
// etc. tests ensure that `rstd::Char` provide test coverage for certain
// construction-related scenarios.
static_assert(std::is_trivially_destructible_v<rstd::Char>);
static_assert(std::is_trivially_copy_constructible_v<rstd::Char>);
static_assert(std::is_trivially_copy_assignable_v<rstd::Char>);
static_assert(std::is_trivially_move_constructible_v<rstd::Char>);
static_assert(std::is_trivially_move_assignable_v<rstd::Char>);

// Layout tests.
//
// https://rust-lang.github.io/unsafe-code-guidelines/layout/scalars.html#char
// documents that "Rust char is 32-bit wide and represents an unicode scalar
// value".
//
// We don't map Rust's `char` to C++ `char32_t` because
// - It may be wider than 32 bits - <internal link>/c/string/multibyte/char32_t says
//   that "char32_t is an unsigned integer type used for 32-bit wide characters
//   and is the same type as uint_least32_t. uint_least32_t is the smallest
//   unsigned integer type with width of at least 32 bits"
// - It is problematic on MacOS - https://github.com/eqrion/cbindgen/issues/423
//   points out that `uchar.h` is missing on that platform.
static_assert(sizeof(rstd::Char) == 4);
static_assert(alignof(rstd::Char) == 4);
static_assert(std::is_standard_layout_v<rstd::Char>);

// This test covers the following case from
// https://en.cppreference.com/w/cpp/language/character_literal:
//
// Ordinary character literal, e.g. 'a' or '\n' or '\13'. Such literal has type
// `char` and the value equal to either:
// - the representation of c-char in the execution character set (until C++23)
// - the corresponding code point from ordinary literal encoding (since C++23).
TEST(RsCharTest, FromAsciiLiteral) {
  rstd::Char c = 'x';
  EXPECT_EQ(0x78, static_cast<uint32_t>(c));
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
  rstd::Char c = u8'x';
  EXPECT_EQ(0x78, static_cast<uint32_t>(c));
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
  // Not testing `is_trivially_constructible`, because UTF-16 literals may
  // fail Rust's well-formed-ness checks (e.g. they may represent only one
  // part of a surrogate pair).
  rstd::Char c = u'Ł';
  EXPECT_EQ(0x141, static_cast<uint32_t>(c));
}

// This test covers the following case from
// https://en.cppreference.com/w/cpp/language/character_literal:
//
// UTF-32 character literal, e.g. U'猫' or U'🍌'. Such literal has type
// `char32_t` and the value equal to ISO/IEC 10646 code point value of c-char.
TEST(RsCharTest, FromUtf32Literal) {
  // Not testing `is_trivially_constructible`, because UTF-32 literals may fail
  // Rust's well-formed-ness checks (e.g. they may exceed the value of Rust's
  // `std::char::MAX`).
  rstd::Char c = U'🦀';
  EXPECT_EQ(0x1F980, static_cast<uint32_t>(c));
}

// Test that `rstd::Char` values can be compared with other `rstd::Char` values.
TEST(RsCharTest, ComparisonWithAnotherRsChar) {
  const rstd::Char a = 'a';
  const rstd::Char b = 'b';

  EXPECT_TRUE(a == a);
  EXPECT_FALSE(a != a);
  EXPECT_TRUE(a <= a);
  EXPECT_FALSE(a < a);
  EXPECT_TRUE(a >= a);
  EXPECT_FALSE(a > a);

  EXPECT_FALSE(a == b);
  EXPECT_TRUE(a != b);
  EXPECT_TRUE(a <= b);
  EXPECT_TRUE(a < b);
  EXPECT_FALSE(a >= b);
  EXPECT_FALSE(a > b);

  EXPECT_FALSE(b == a);
  EXPECT_TRUE(b != a);
  EXPECT_FALSE(b <= a);
  EXPECT_FALSE(b < a);
  EXPECT_TRUE(b >= a);
  EXPECT_TRUE(b > a);
}

}  // namespace
