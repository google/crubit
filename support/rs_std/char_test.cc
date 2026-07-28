// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rs_std/char.h"

#include <stdint.h>

#include <array>
#include <optional>
#include <type_traits>

#include "gtest/gtest.h"
#include "fuzztest/fuzztest.h"
#include "absl/strings/str_cat.h"
#include "absl/types/span.h"
#include "support/rs_std/internal/is_utf8.h"
#include "support/rs_std/str_ref.h"

namespace {

// Check that `rs_std::char_` is trivially destructible, copyable, and
// moveable.
//
// There are no constructor-related checks, because well-formed-ness checks
// require going through factory methods like `char_::from_u32`.
static_assert(std::is_trivially_destructible_v<rs_std::char_>);
static_assert(std::is_trivially_copy_constructible_v<rs_std::char_>);
static_assert(std::is_trivially_copy_assignable_v<rs_std::char_>);
static_assert(std::is_trivially_move_constructible_v<rs_std::char_>);
static_assert(std::is_trivially_move_assignable_v<rs_std::char_>);

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
// with a single field (i.e. the `char_` struct) has the same ABI
// classification as the field (as long as the field is smaller than "eight
// eightbytes" and the struct is trivial as verified via `static_assert`s
// above).  In other words, under System V ABI we expect `char_` to be of
// INTEGER class - the same as verified by the `layout.abi()` assertion in
// `bindings.rs`.
static_assert(sizeof(rs_std::char_) == 4);
static_assert(alignof(rs_std::char_) == 4);
static_assert(std::is_standard_layout_v<rs_std::char_>);

// This test covers the following case from
// https://en.cppreference.com/w/cpp/language/character_literal:
//
// Ordinary character literal, e.g. 'a' or '\n' or '\13'. Such literal has type
// `char` and the value equal to either:
// - the representation of c-char in the execution character set (until C++23)
// - the corresponding code point from ordinary literal encoding (since C++23).
TEST(CharTest, FromAsciiLiteral) {
  const rs_std::char_ c('x');
  EXPECT_EQ(0x78, uint32_t{c});
}

// This test covers the following case from
// https://en.cppreference.com/w/cpp/language/character_literal:
//
// UTF-8 character literal, e.g. u8'a'. Such literal has type `char` (until
// C++20) or `char8_t` (since C++20) and the value equal to ISO/IEC 10646 code
// point value of c-char, provided that the code point value is representable
// with a single UTF-8 code unit (that is, c-char is in the range 0x0-0x7F,
// inclusive).
TEST(CharTest, FromUtf8Literal) {
  const rs_std::char_ c(u8'x');
  EXPECT_EQ(0x78, uint32_t{c});
}

// This test covers the following case from
// https://en.cppreference.com/w/cpp/language/character_literal:
//
// UTF-16 character literal, e.g. u'猫', but not u'🍌' (u'\U0001f34c'). Such
// literal has type `char16_t` and the value equal to ISO/IEC 10646 code point
// value of c-char, provided that the code point value is representable with a
// single UTF-16 code unit (that is, c-char is in the range 0x0-0xFFFF,
// inclusive).
TEST(CharTest, FromUtf16Literal) {
  const rs_std::char_ c(u'Ł');
  EXPECT_EQ(0x141, uint32_t{c});
}

// This test covers the following case from
// https://en.cppreference.com/w/cpp/language/character_literal:
//
// UTF-32 character literal, e.g. U'猫' or U'🍌'. Such literal has type
// `char32_t` and the value equal to ISO/IEC 10646 code point value of c-char.
TEST(CharTest, FromUtf32Literal) {
  const rs_std::char_ c(U'🦀');
  EXPECT_EQ(0x1F980, uint32_t{c});
}

TEST(CharTest, FromU32ValidityChecks) {
  // Max 32-bit value.
  EXPECT_FALSE(rs_std::char_::from_u32(0xffffffff).has_value());

  // A value just above Rust's `char::MAX`:
  // https://doc.rust-lang.org/std/primitive.char.html#associatedconstant.MAX.
  EXPECT_FALSE(rs_std::char_::from_u32(0x110000).has_value());

  // Smallest/greatest "high"/"low" surrogates.
  EXPECT_FALSE(rs_std::char_::from_u32(0xd800).has_value());
  EXPECT_FALSE(rs_std::char_::from_u32(0xdbff).has_value());
  EXPECT_FALSE(rs_std::char_::from_u32(0xdc00).has_value());
  EXPECT_FALSE(rs_std::char_::from_u32(0xdfff).has_value());

  // Smallest valid value.
  std::optional<rs_std::char_> maybe_c = rs_std::char_::from_u32('\0');
  ASSERT_TRUE(maybe_c.has_value());
  EXPECT_EQ(0x00, uint32_t{*maybe_c});

  // Greatest valid value.  See also Rust's `char::MAX`:
  // https://doc.rust-lang.org/std/primitive.char.html#associatedconstant.MAX.
  maybe_c = rs_std::char_::from_u32(0x10ffff);
  ASSERT_TRUE(maybe_c.has_value());
  EXPECT_EQ(0x10ffff, uint32_t{*maybe_c});

  // Just below surrogates.
  maybe_c = rs_std::char_::from_u32(0xd7ff);
  ASSERT_TRUE(maybe_c.has_value());
  EXPECT_EQ(0xd7ff, uint32_t{*maybe_c});

  // Just above surrogates.
  maybe_c = rs_std::char_::from_u32(0xe000);
  ASSERT_TRUE(maybe_c.has_value());
  EXPECT_EQ(0xe000, uint32_t{*maybe_c});
}

// Test that `rs_std::char_` values can be compared with other
// `rs_std::char_` values.
TEST(CharTest, ComparisonWithAnotherRsChar) {
  const rs_std::char_ a('a');
  const rs_std::char_ b('b');

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

TEST(CharTest, DefaultConstructedValue) {
  rs_std::char_ c;
  EXPECT_EQ(0, uint32_t{c});
}

void ExpectEncodedIsUtf8(uint32_t data) {
  std::optional<rs_std::char_> c = rs_std::char_::from_u32(data);
  if (!c.has_value()) {
    return;
  }
  std::array<uint8_t, 4> buffer;
  rs_std::StrRef str = c->encode_utf8(absl::MakeSpan(buffer));
  EXPECT_TRUE(rs_std::internal::IsUtf8(str.to_string_view()));
}
FUZZ_TEST(ExpectEncodedIsUtf8FuzzTest, ExpectEncodedIsUtf8);

struct EncodeTestCase {
  uint32_t data;
  absl::Span<const char> expected_utf8_bytes;
};

class EncodedIsUtf8BytesTest : public testing::TestWithParam<EncodeTestCase> {};

TEST_P(EncodedIsUtf8BytesTest, EncodedIsUtf8Bytes) {
  auto [data, expected_utf8_bytes] = GetParam();
  std::optional<rs_std::char_> c = rs_std::char_::from_u32(data);
  if (!c.has_value()) {
    return;
  }
  std::array<uint8_t, 4> buffer;
  rs_std::StrRef str = c->encode_utf8(absl::MakeSpan(buffer));
  EXPECT_TRUE(rs_std::internal::IsUtf8(str.to_string_view()));
  EXPECT_EQ(absl::MakeSpan(str), expected_utf8_bytes);
}

INSTANTIATE_TEST_SUITE_P(
    EncodedIsUtf8BytesTests, EncodedIsUtf8BytesTest,
    testing::Values(EncodeTestCase{0x00, {0x00}}, EncodeTestCase{0x01, {0x01}},
                    EncodeTestCase{'x', {0x78}},
                    EncodeTestCase{0xe9, {0xc3, 0xa9}},
                    EncodeTestCase{0xa66e, {0xea, 0x99, 0xae}},
                    EncodeTestCase{0x1f4a9, {0xf0, 0x9f, 0x92, 0xa9}}));

TEST(CharTest, AbslStringify) {
  EXPECT_EQ(absl::StrCat(rs_std::char_('x')), "x");
  std::string expected_emoji = "💩";
  EXPECT_EQ(absl::StrCat(rs_std::char_(U'💩')), expected_emoji);
}

}  // namespace
