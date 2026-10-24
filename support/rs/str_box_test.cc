// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Ensure `::std` is root-qualified inside `namespace rs` when `rs::std` exists.
namespace rs::std {}

#include "support/rs/str_box.h"

#include <optional>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>

#include "support/movable.h"
#include "support/unsafe_relocate_tag.h"
#include "gtest/gtest.h"
#include "support/rs_std/str_ref.h"

namespace rs {
namespace {

TEST(StrBoxTest, StaticAsserts) {
  static_assert(sizeof(::rs::StrBox) == 2 * sizeof(void*));
  static_assert(!::std::is_copy_constructible_v<StrBox>);
  static_assert(!::std::is_move_constructible_v<StrBox>);
  static_assert(!::std::is_copy_assignable_v<StrBox>);
  static_assert(!::std::is_move_assignable_v<StrBox>);
  static_assert(
      ::std::is_constructible_v<StrBox, ::crubit::UnsafeRelocateTag, StrBox&&>);
  // `std::string` is not trivially copyable, so it cannot use the `consteval`
  // constructor and must go through `StrBox::FromUtf8`.
  static_assert(!::std::is_constructible_v<StrBox, ::std::string>);
}

TEST(StrBoxTest, Empty) {
  StrBox b;
  EXPECT_TRUE(b.empty());
  EXPECT_EQ(b.size(), 0);
  EXPECT_EQ(b.data(), nullptr);
}

TEST(StrBoxTest, ConstevalCheckedConstructor) {
  // Uncomment to see compile-time UTF-8 error:
  // StrBox bad("a\x80");
  StrBox from_literal("hello");
  EXPECT_FALSE(from_literal.empty());
  EXPECT_EQ(from_literal.size(), 5);
  EXPECT_EQ(::std::string_view(from_literal), "hello");

  static constexpr ::std::string_view kSv = "constexpr utf8";
  StrBox from_constexpr_sv(kSv);
  EXPECT_EQ(::std::string_view(from_constexpr_sv), "constexpr utf8");

  StrRef ref = "from ref";
  StrBox from_ref(ref);
  EXPECT_EQ(::std::string_view(from_ref), "from ref");
}

TEST(StrBoxTest, FromUtf8ValidAndInvalid) {
  const ::std::string runtime_str = "hello runtime";
  ::std::optional<::rs::Movable<StrBox>> valid = StrBox::FromUtf8(runtime_str);
  ASSERT_TRUE(valid.has_value());
  StrBox unwrapped = *::std::move(valid);
  EXPECT_EQ(::std::string_view(unwrapped), "hello runtime");

  ::std::optional<::rs::Movable<StrBox>> empty = StrBox::FromUtf8("");
  ASSERT_TRUE(empty.has_value());
  StrBox unwrapped_empty = *::std::move(empty);
  EXPECT_TRUE(unwrapped_empty.empty());
  EXPECT_EQ(unwrapped_empty.data(), nullptr);

  EXPECT_FALSE(StrBox::FromUtf8("a\x80").has_value());
}

TEST(StrBoxTest, FromUtf8Unchecked) {
  StrBox b = StrBox::FromUtf8Unchecked("unchecked");
  EXPECT_EQ(::std::string_view(b), "unchecked");
}

TEST(StrBoxTest, Relocation) {
  alignas(StrBox) char buf[sizeof(StrBox)];
  StrBox* b1 = new (buf) StrBox("hello");
  const char* old_data = b1->data();
  StrBox b2(::crubit::UnsafeRelocateTag{}, ::std::move(*b1));
  EXPECT_EQ(b2.data(), old_data);
  EXPECT_EQ(b2.size(), 5);
}

TEST(StrBoxTest, AsStr) {
  StrBox b("hello");
  StrRef ref = b.as_str();
  EXPECT_EQ(ref.size(), 5);
  EXPECT_EQ(ref, "hello");

  StrRef implicit_ref = b;
  EXPECT_EQ(implicit_ref.size(), 5);

  ::std::string_view sv = b;
  EXPECT_EQ(sv, "hello");
}

}  // namespace
}  // namespace rs
