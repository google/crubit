// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rs_std/str_ref.h"

#include <bit>
#include <cstddef>
#include <cstdint>
#include <optional>
#include <string>
#include <type_traits>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "fuzztest/fuzztest.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"

namespace {
using ::rs_std::StrRef;
using ::testing::IsNull;
using ::testing::Not;

// Check that `StrRef` is trivially destructible, copyable, and
// moveable. It is not trivial because it has a non-trivial constructor, and
// that's because its default constructor needs to be user-defined to make sure
// that `ptr_` is never null. The latter is needed so that interpreting the data
// as a slice in Rust won't result in a (in Rust) forbidden null pointer.
static_assert(std::is_nothrow_constructible_v<StrRef>);
static_assert(std::is_trivially_destructible_v<StrRef>);
static_assert(std::is_trivially_copyable_v<StrRef>);
static_assert(std::is_trivially_copy_constructible_v<StrRef>);
static_assert(std::is_trivially_copy_assignable_v<StrRef>);
static_assert(std::is_trivially_move_constructible_v<StrRef>);
static_assert(std::is_trivially_move_assignable_v<StrRef>);
// Note: `StrRef` is not trivially constructible because its default
// constructor ensures that the data pointer is not null.

// Verify that the layout of `StrRef` is as expected and described in
// `rust_builtin_type_abi_assumptions.md`.

// `sizeof(uintptr_t)` is guaranteed to be the size of `usize` in Rust.
//
// StrRef assumes that the Rust slice layout is first the pointer and then the
// size. This does not appear to be standardised, so instead there are runtime
// checks for this in `format_ty_for_cc` in `generate_bindings/format_type.rs`.
// See the `check_slice_layout` function.
//
// Stabilizing this layout is proposed in
// https://github.com/rust-lang/rfcs/pull/3775
static_assert(sizeof(StrRef) == sizeof(uintptr_t) * 2);
static_assert(alignof(StrRef) == alignof(uintptr_t));

static_assert(std::is_standard_layout_v<StrRef>);

static_assert(std::is_constructible_v<StrRef, absl::string_view>);
static_assert(std::is_constructible_v<StrRef, std::string_view>);
static_assert(std::is_constructible_v<StrRef, const char*>);
static_assert(std::is_constructible_v<StrRef, std::string&>);
static_assert(std::is_constructible_v<StrRef, const std::string&>);

TEST(StrTest, Comparison) {
  static constexpr absl::string_view kStr = "12345";
  static constexpr absl::string_view kStrCopy = "12345";

  static constexpr StrRef kStrRef = StrRef(kStr);
  static constexpr StrRef kStrRefCopy = kStrRef;
  static constexpr StrRef kStrRef2 = StrRef(kStrCopy);

  static_assert(kStrRef == kStrRef);
  static_assert(kStrRef == kStrRefCopy);
  static_assert(kStrRef == kStrRef2);

  static constexpr StrRef kStrRef_prefix =
      StrRef(absl::string_view(kStr.data(), kStr.size() - 1));
  static constexpr StrRef kStrRef_suffix =
      StrRef(absl::string_view(kStr.data() + 1, kStr.size() - 1));
  static constexpr StrRef kStrRef_infix =
      StrRef(absl::string_view(kStr.data() + 1, kStr.size() - 2));

  EXPECT_GT(kStrRef, kStrRef_prefix);
  EXPECT_LT(kStrRef, kStrRef_suffix);
  EXPECT_LT(kStrRef, kStrRef_infix);
}

TEST(StrTest, FromAndTo) {
  static constexpr absl::string_view kStr = "12345";
  static constexpr StrRef kStrRef = StrRef(kStr);
  EXPECT_EQ(kStr, kStrRef);
}

// To test the value of `data_`, the test includes the knowledge of
// `StrRef`'s layout to extract it via a peeker struct.
struct StrRefFields {
  const void* ptr;
  size_t size;
};

// This test checks that the assumption of Peeker having the same layout as
// `StrRef` is correct.
TEST(StrTest, Layout) {
  static constexpr absl::string_view kStr = "foo";
  const StrRef s = StrRef(kStr);
  const auto fields = std::bit_cast<StrRefFields>(s);
  EXPECT_EQ(fields.ptr, kStr.data());
  EXPECT_EQ(fields.size, kStr.size());
}

TEST(StrTest, Empty) {
  static constexpr const char* kEmpty = "";
  const StrRef empty = StrRef(absl::string_view(kEmpty, 0));
  static constexpr StrRef default_constructed;
  EXPECT_EQ(empty, default_constructed);

  const auto fields = std::bit_cast<StrRefFields>(empty);
  EXPECT_THAT(fields.ptr, Not(IsNull()));
  EXPECT_EQ(fields.size, 0);

  // While `empty.data_` is not null, `data()` converts it to null for
  // compatibility with `std::span`.
  EXPECT_THAT(empty.data(), IsNull());
  EXPECT_EQ(empty.size(), 0);
}

TEST(StrTest, StrCat) {
  static constexpr absl::string_view kStr = "12345";
  static constexpr StrRef kStrRef = StrRef("12345");
  EXPECT_EQ(absl::StrCat(kStrRef), kStr);
}

TEST(StrTest, FromUtf8OnNonUtf8ReturnsNullopt) {
  // Uncomment to see compiler error.
  // static constexpr StrRef kStrRef = "a\x80";
  EXPECT_FALSE(StrRef::FromUtf8("a\x80").has_value());
}

TEST(ImplicitConversionTest, FromConstString) {
  const std::string string = "123";
  const std::optional<StrRef> from_string = StrRef::FromUtf8(string);
  ASSERT_TRUE(from_string.has_value());
  EXPECT_EQ(from_string, "123");
}

TEST(ImplicitConversionTest, FromConstCharPtr) {
  static constexpr const char* kConstCharPtr = "12";
  static constexpr StrRef kStrRef = StrRef(kConstCharPtr);
  EXPECT_EQ(kStrRef, "12");
}

void Fuzzer(std::string data) {
  const std::optional<StrRef> s = StrRef::FromUtf8(data);
  ASSERT_TRUE(s.has_value());
  EXPECT_EQ(data, *s);
  const std::string data_copy(s->data(), s->data() + s->size());
  EXPECT_EQ(data, data_copy);
}

FUZZ_TEST(StrFuzzTest, Fuzzer).WithDomains(fuzztest::Utf8String());

}  // namespace
