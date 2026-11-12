// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/string/string.h"

#include <optional>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>

#include "gtest/gtest.h"
#include "absl/strings/string_view.h"
#include "support/rs/str_box.h"
#include "support/rs_std/rs_std.h"
#include "support/rs_std/str_ref.h"

namespace crubit {
namespace {

TEST(RustStringBridging, Basic) {
  rs::std::string::String s("hello world");
  EXPECT_EQ(string::compute_rust_string_length(std::move(s)), 11);

  rs::std::string::String s2("hello world from Rust");
  rs::std::string::String roundtripped =
      string::roundtrip_rust_string(std::move(s2));
  EXPECT_EQ(string::compute_rust_string_length(std::move(roundtripped)), 21);
}

TEST(RustStringBridging, References) {
  rs::std::string::String s("hello");
  EXPECT_EQ(string::compute_rust_string_ref_length(s), 5);

  string::append_to_rust_string(s, " appended");
  EXPECT_EQ(string::compute_rust_string_ref_length(s), 14);
  EXPECT_EQ(s.as_str(), "hello appended");
}

TEST(RustStringBridging, ShorthandAlias) {
  static_assert(std::is_same_v<rs::String, rs::alloc::string::String>);
  static_assert(std::is_same_v<rs::String, rs::std::string::String>);

  rs::String s("hello");
  EXPECT_EQ(string::compute_rust_string_ref_length(s), 5);
  EXPECT_EQ(string::compute_rust_string_length(std::move(s)), 5);
}

// The tests below show the supported ways to construct a Rust `String` from
// C++ strings. `String` only has explicit converting constructors from
// `rs::StrRef`, `rs::Movable<rs::StrBox>`, and `rs::char_` (from Rust's
// `From<&str>`, `From<Box<str>>`, and `From<char>` impls). Other C++ strings
// must first be converted to a `StrRef`.

TEST(RustStringConstruction, FromStringLiteral) {
  // String literals implicitly convert to `StrRef` at compile time, which also
  // checks at compile time that they are valid UTF-8.
  rs::String s("hello");
  EXPECT_EQ(s.as_str(), "hello");
}

TEST(RustStringConstruction, FromStrRef) {
  rs::StrRef str_ref = "hello";
  rs::String s(str_ref);
  EXPECT_EQ(s.as_str(), "hello");
}

TEST(RustStringConstruction, FromStringView) {
  // Runtime strings must be checked for valid UTF-8 using
  // `StrRef::FromUtf8`, which returns `std::nullopt` for invalid UTF-8.
  std::string_view view = "hello";
  std::optional<rs::StrRef> str_ref = rs::StrRef::FromUtf8(view);
  ASSERT_TRUE(str_ref.has_value());
  rs::String s(*str_ref);
  EXPECT_EQ(s.as_str(), "hello");

  absl::string_view absl_view = "world";
  rs::String s2(rs::StrRef::FromUtf8(absl_view).value());
  EXPECT_EQ(s2.as_str(), "world");
}

TEST(RustStringConstruction, FromStdString) {
  // `std::string` converts to `std::string_view`, so it can be passed to
  // `StrRef::FromUtf8` directly. Using `.value()` crashes on invalid UTF-8.
  std::string cpp_string = "hello";
  rs::String s(rs::StrRef::FromUtf8(cpp_string).value());
  EXPECT_EQ(s.as_str(), "hello");
}

TEST(RustStringConstruction, FromNonUtf8StringFails) {
  std::string invalid = "\xff";
  EXPECT_FALSE(rs::StrRef::FromUtf8(invalid).has_value());
}

TEST(RustStringConstruction, FromStringViewUnchecked) {
  // If the caller already knows that the string is valid UTF-8,
  // `StrRef::FromUtf8Unchecked` skips the check. Passing invalid UTF-8 is
  // undefined behavior.
  std::string cpp_string = "hello";
  rs::String s(rs::StrRef::FromUtf8Unchecked(cpp_string));
  EXPECT_EQ(s.as_str(), "hello");
}

TEST(RustStringConstruction, FromStrBox) {
  rs::String s(rs::StrBox::FromUtf8Unchecked("hello"));
  EXPECT_EQ(s.as_str(), "hello");
}

TEST(RustStringConstruction, FromChar) {
  rs::String s(rs::char_('x'));
  EXPECT_EQ(s.as_str(), "x");

  rs::String s2(rs::char_(U'\u00e9'));
  EXPECT_EQ(s2.as_str(), "\u00e9");
}

TEST(RustStringConversion, ToCppStrings) {
  rs::String s("hello");
  // `as_str()` returns a `StrRef`, which implicitly converts to
  // `std::string_view`.
  std::string_view view = s.as_str();
  EXPECT_EQ(view, "hello");
  std::string cpp_string(s.as_str());
  EXPECT_EQ(cpp_string, "hello");
}

}  // namespace
}  // namespace crubit
