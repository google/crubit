// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rs_std/slice_ref.h"

#include <stdint.h>

#include <array>
#include <bit>
#include <concepts>
#include <cstddef>
#include <type_traits>
#include <vector>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "testing/fuzzing/fuzztest.h"
#include "absl/types/span.h"

namespace {
using ::testing::IsNull;
using ::testing::Not;

// Check that `rs_std::SliceRef` is trivially destructible, copyable, and
// moveable. It is not trivial because it has a non-trivial constructor, and
// that's because its default constructor needs to be user-defined to make sure
// that `ptr_` is never null. The latter is needed so that interpreting the data
// as a slice in Rust won't result in a (in Rust) forbidden null pointer.
static_assert(std::is_nothrow_constructible_v<rs_std::SliceRef<const uint8_t>>);
static_assert(
    std::is_trivially_destructible_v<rs_std::SliceRef<const uint8_t>>);
static_assert(std::is_trivially_copyable_v<rs_std::SliceRef<const uint8_t>>);
static_assert(
    std::is_trivially_copy_constructible_v<rs_std::SliceRef<const uint8_t>>);
static_assert(
    std::is_trivially_copy_assignable_v<rs_std::SliceRef<const uint8_t>>);
static_assert(
    std::is_trivially_move_constructible_v<rs_std::SliceRef<const uint8_t>>);
static_assert(
    std::is_trivially_move_assignable_v<rs_std::SliceRef<const uint8_t>>);
// Note: `rs_std::SliceRef` is not trivially constructible because it's default
// constructor ensures that the data pointer is not null.

// `SliceRef` does on purpose not have `operator==`, because <internal link>
// did not specify that `SliceRef` should be comparable.
static_assert(!std::equality_comparable<rs_std::SliceRef<int>>);

// Verify that the layout of `rs_std::SliceRef` is as expected and described in
// `rust_builtin_type_abi_assumptions.md`. Sample a few wrapped types to make
// sure that the layout is the same for all of them.
// `sizeof(uintptr_t)` is guaranteed to be the size of `usize` in Rust.
constexpr size_t kSliceRefSize = sizeof(uintptr_t) * 2;
constexpr size_t kSliceRefAlign = alignof(uintptr_t);
static_assert(sizeof(rs_std::SliceRef<const uint8_t>) == kSliceRefSize);
static_assert(alignof(rs_std::SliceRef<const uint8_t>) == kSliceRefAlign);
static_assert(std::is_standard_layout_v<rs_std::SliceRef<const uint8_t>>);
static_assert(sizeof(rs_std::SliceRef<char>) == kSliceRefSize);
static_assert(alignof(rs_std::SliceRef<char>) == kSliceRefAlign);
static_assert(std::is_standard_layout_v<rs_std::SliceRef<char>>);
static_assert(sizeof(rs_std::SliceRef<int64_t>) == kSliceRefSize);
static_assert(alignof(rs_std::SliceRef<int64_t>) == kSliceRefAlign);
static_assert(std::is_standard_layout_v<rs_std::SliceRef<int64_t>>);

// Slice assumes that the Rust slice layout is first the pointer and then the
// size. This does not appear to be standardised, so instead there are runtime
// checks for this in `format_ty_for_cc` in `cc_bindings_from_rs/bindings.rs`.
static_assert(sizeof(rs_std::SliceRef<const uint8_t>) ==
              sizeof(const uint8_t*) + sizeof(size_t));
static_assert(alignof(rs_std::SliceRef<const uint8_t>) ==
              alignof(const uint8_t*));
static_assert(std::is_standard_layout_v<rs_std::SliceRef<const uint8_t>>);

TEST(SliceTest, Comparison) {
  static constexpr std::array<uint8_t, 5> kArr = {1, 2, 3, 4, 5};
  static constexpr std::array<uint8_t, 5> kArrCopy = {1, 2, 3, 4, 5};

  const rs_std::SliceRef<const uint8_t> s1(kArr);
  const rs_std::SliceRef<const uint8_t> s1_copy = s1;
  const rs_std::SliceRef<const uint8_t> s2(kArrCopy);

  EXPECT_EQ(s1.to_span(), s1.to_span());
  EXPECT_EQ(s1.to_span(), s1_copy.to_span());
  EXPECT_EQ(s1.to_span(), s2.to_span());
  const rs_std::SliceRef<const uint8_t> s1_prefix(
      absl::MakeSpan(kArr.data(), kArr.size() - 1));
  const rs_std::SliceRef<const uint8_t> s1_suffix(
      absl::MakeSpan(kArr.data() + 1, kArr.size() - 1));
  const rs_std::SliceRef<const uint8_t> s1_infix(
      absl::MakeSpan(kArr.data() + 1, kArr.size() - 2));

  EXPECT_NE(s1.to_span(), s1_prefix.to_span());
  EXPECT_NE(s1.to_span(), s1_suffix.to_span());
  EXPECT_NE(s1.to_span(), s1_infix.to_span());
}

TEST(SliceTest, FromAndTo) {
  static constexpr std::array<uint8_t, 5> kArr = {1, 2, 3, 4, 5};
  const rs_std::SliceRef<const uint8_t> s(kArr);
  EXPECT_EQ(absl::Span<const uint8_t>(kArr), s.to_span());
}

// To test the value of `data_`, the test includes the knowledge of
// `SliceRef`'s layout to extract it via a peeker struct.
struct SliceRefFields {
  const void* ptr;
  size_t size;
};

// This test checks that the assumption of Peeker having the same layout as
// `SliceRef` is correct.
TEST(SliceTest, Layout) {
  static constexpr std::array<uint8_t, 5> kArr = {2, 3, 5};
  const rs_std::SliceRef<const uint8_t> s(kArr);
  const auto fields = std::bit_cast<SliceRefFields>(s);
  EXPECT_EQ(fields.ptr, kArr.data());
  EXPECT_EQ(fields.size, kArr.size());
}

TEST(SliceTest, Empty) {
  static constexpr uint8_t kEmpty[] = {};
  const rs_std::SliceRef<const uint8_t> empty(absl::MakeSpan(kEmpty, 0));
  static constexpr rs_std::SliceRef<const uint8_t> default_constructed;
  EXPECT_EQ(empty.to_span(), default_constructed.to_span());

  const auto fields = std::bit_cast<SliceRefFields>(empty);
  EXPECT_THAT(fields.ptr, Not(IsNull()));
  EXPECT_EQ(fields.size, 0);

  // While `empty.data_` is not null, `data()` converts it to null for
  // compatibility with `std::span`.
  EXPECT_THAT(empty.data(), IsNull());
  EXPECT_EQ(empty.size(), 0);
}

void Fuzzer(std::vector<uint8_t> data) {
  const rs_std::SliceRef<const uint8_t> s(data);
  EXPECT_EQ(absl::Span<const uint8_t>(data), s.to_span());
  const std::vector<uint8_t> data_copy(s.data(), s.data() + s.size());
  EXPECT_EQ(data, data_copy);
}

FUZZ_TEST(SliceFuzzTest, Fuzzer);

}  // namespace
