// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rs_std/internal/is_utf8.h"

#include <array>
#include <cstdint>

#include "gtest/gtest.h"
#include "absl/strings/string_view.h"

namespace {

static_assert(rs::internal::IsAscii("abc"));
static_assert(rs::internal::IsAscii("abc\0"));
static_assert(!rs::internal::IsAscii("abc\x80"));
static_assert(!rs::internal::IsAscii("abc\xff"));

TEST(IsUtf8Test, IsAscii) {
  EXPECT_TRUE(rs::internal::IsAscii(""));
  EXPECT_TRUE(rs::internal::IsAscii("abc"));
  EXPECT_TRUE(rs::internal::IsAscii(absl::string_view("a\0b\x7f", 4)));
  for (int b = 0; b < 0x80; ++b) {
    const char c = static_cast<char>(b);
    const absl::string_view sv(&c, 1);
    EXPECT_TRUE(rs::internal::IsAscii(sv));
    EXPECT_TRUE(rs::internal::IsUtf8(sv));
  }
  for (int b = 0x80; b <= 0xff; ++b) {
    const char c = static_cast<char>(b);
    const absl::string_view sv(&c, 1);
    EXPECT_FALSE(rs::internal::IsAscii(sv));
  }
}

static_assert(rs::internal::IsUtf8("abc"));
static_assert(rs::internal::IsUtf8("abc\0"));
static_assert(rs::internal::IsUtf8("👁💖🐶"));
static_assert(rs::internal::IsUtf8("abc\xc2\x80"));
static_assert(rs::internal::IsUtf8("abc\xe0\xa0\x80"));

template <int... Bytes>
constexpr bool BytesAreUtf8() {
  const std::array<char, sizeof...(Bytes)> bytes_array = {
      static_cast<char>(Bytes)...};
  return rs::internal::IsUtf8(
      absl::string_view(bytes_array.data(), bytes_array.size()));
}

// Test continuation byte as first byte (first byte is 0b10xxxxxx)
static_assert(!BytesAreUtf8<0b10111111>());
// Test too-short code unit sequence
//   first byte is 0b110yyyyy, second byte begins with either 0b0 or 0b11
static_assert(!BytesAreUtf8<0b11000000, 0b00000000>());
static_assert(!BytesAreUtf8<0b11000000, 0b11000000>());
// Test overlong code unit sequence
//   0b11000000 0b10000000 encodes the codepoint 0 using 2 bytes when it should
//   use one.
static_assert(!BytesAreUtf8<0b11000000, 0b10000000>());
// Test reserved code point
static_assert(!BytesAreUtf8<0b11111111>());
// Test stray continuation bytes
static_assert(!BytesAreUtf8<0b10110000, 0b10111111>());

// Regression test for https://crbug.com/538084342.
//
// Test character signedness across all four special-case RFC 3629 lead bytes
// (`0xE0`, `0xED`, `0xF0`, `0xF4`) and their valid boundaries where `char` is
// treated as `signed char` (e.g. in Chromium).
// We use `static_assert` here so that the failure is caught at compile-time
// during transition-based cross-compilation tests (which only build, not run).
static_assert(!rs::internal::IsUtf8("\xe0\x80\x80"));
static_assert(!rs::internal::IsUtf8("\xe0\x9f\xbf"));
static_assert(rs::internal::IsUtf8("\xe0\xa0\x80"));
static_assert(rs::internal::IsUtf8("\xe0\xbf\xbf"));
static_assert(rs::internal::IsUtf8("\xed\x9f\xbf"));
static_assert(!rs::internal::IsUtf8("\xed\xa0\x80"));
static_assert(!rs::internal::IsUtf8("\xed\xbf\xbf"));
static_assert(rs::internal::IsUtf8("\xee\x80\x80"));
static_assert(!rs::internal::IsUtf8("\xf0\x80\x80\x80"));
static_assert(!rs::internal::IsUtf8("\xf0\x8f\xbf\xbf"));
static_assert(rs::internal::IsUtf8("\xf0\x90\x80\x80"));
static_assert(rs::internal::IsUtf8("\xf0\xbf\xbf\xbf"));
static_assert(rs::internal::IsUtf8("\xf4\x8f\xbf\xbf"));
static_assert(!rs::internal::IsUtf8("\xf4\x90\x80\x80"));
static_assert(!rs::internal::IsUtf8("\xf4\xbf\xbf\xbf"));

TEST(IsUtf8Test, ValidAndInvalidUtf8) {
  EXPECT_TRUE(rs::internal::IsUtf8(""));
  EXPECT_TRUE(rs::internal::IsUtf8("abc"));
  EXPECT_TRUE(rs::internal::IsUtf8(absl::string_view("abc\0def", 7)));
  EXPECT_TRUE(rs::internal::IsUtf8("👁💖🐶"));
  EXPECT_TRUE(rs::internal::IsUtf8("\xc2\x80\xdf\xbf"));
  EXPECT_TRUE(
      rs::internal::IsUtf8("\xe0\xa0\x80\xed\x9f\xbf\xee\x80\x80\xef\xbf\xbf"));
  EXPECT_TRUE(rs::internal::IsUtf8("\xf0\x90\x80\x80\xf4\x8f\xbf\xbf"));

  // Overlong, surrogate, truncated, and out-of-range sequences.
  EXPECT_FALSE(rs::internal::IsUtf8("\x80"));
  EXPECT_FALSE(rs::internal::IsUtf8("\xc0\x80"));
  EXPECT_FALSE(rs::internal::IsUtf8("\xc1\xbf"));
  EXPECT_FALSE(rs::internal::IsUtf8("\xe0\x9f\xbf"));
  EXPECT_FALSE(rs::internal::IsUtf8("\xed\xa0\x80"));
  EXPECT_FALSE(rs::internal::IsUtf8("\xed\xbf\xbf"));
  EXPECT_FALSE(rs::internal::IsUtf8("\xf0\x8f\xbf\xbf"));
  EXPECT_FALSE(rs::internal::IsUtf8("\xf4\x90\x80\x80"));
  EXPECT_FALSE(rs::internal::IsUtf8("\xc2"));
  EXPECT_FALSE(rs::internal::IsUtf8("\xe0\xa0"));
  EXPECT_FALSE(rs::internal::IsUtf8("\xf0\x90\x80"));
}

}  // namespace
