// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rs_std/slice_ref.h"

#include <stdint.h>

#include <array>
#include <bit>
#include <concepts>
#include <cstddef>
#include <list>
#include <type_traits>
#include <vector>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "fuzztest/fuzztest.h"
#include "absl/types/span.h"

namespace {
using ::testing::ElementsAre;
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
// Note: `rs_std::SliceRef` is not trivially constructible because its default
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
// checks for this in `format_ty_for_cc` in `generate_bindings/format_type.rs`.
// See the `check_slice_layout` function.
//
// Stabilizing this layout is proposed in
// https://github.com/rust-lang/rfcs/pull/3775
static_assert(sizeof(rs_std::SliceRef<const uint8_t>) ==
              sizeof(const uint8_t*) + sizeof(size_t));
static_assert(alignof(rs_std::SliceRef<const uint8_t>) ==
              alignof(const uint8_t*));
static_assert(std::is_standard_layout_v<rs_std::SliceRef<const uint8_t>>);

static_assert(
    std::is_constructible_v<rs_std::SliceRef<int>, std::vector<int>&>);
static_assert(std::is_constructible_v<rs_std::SliceRef<const int>,
                                      const std::vector<int>&>);
static_assert(!std::is_constructible_v<rs_std::SliceRef<int>, std::list<int>&>);
static_assert(!std::is_constructible_v<rs_std::SliceRef<const int>,
                                       const std::list<int>&>);

TEST(SliceTest, Comparison) {
  static constexpr std::array<uint8_t, 5> kArr = {1, 2, 3, 4, 5};
  static constexpr std::array<uint8_t, 5> kArrCopy = {1, 2, 3, 4, 5};

  static constexpr rs_std::SliceRef<const uint8_t> kSlice = kArr;
  static constexpr rs_std::SliceRef<const uint8_t> kSliceCopy = kSlice;
  static constexpr rs_std::SliceRef<const uint8_t> kSlice2 = kArrCopy;

  EXPECT_EQ(kSlice.to_span(), kSlice.to_span());
  EXPECT_EQ(kSlice.to_span(), kSliceCopy.to_span());
  EXPECT_EQ(kSlice.to_span(), kSlice2.to_span());
  static constexpr rs_std::SliceRef<const uint8_t> kSlicePrefix =
      absl::MakeSpan(kArr.data(), kArr.size() - 1);
  static constexpr rs_std::SliceRef<const uint8_t> kSliceSuffix =
      absl::MakeSpan(kArr.data() + 1, kArr.size() - 1);
  static constexpr rs_std::SliceRef<const uint8_t> kSliceInfix =
      absl::MakeSpan(kArr.data() + 1, kArr.size() - 2);

  EXPECT_NE(kSlice.to_span(), kSlicePrefix.to_span());
  EXPECT_NE(kSlice.to_span(), kSliceSuffix.to_span());
  EXPECT_NE(kSlice.to_span(), kSliceInfix.to_span());
}

TEST(SliceTest, FromAndTo) {
  static constexpr std::array<uint8_t, 5> kArr = {1, 2, 3, 4, 5};
  static constexpr rs_std::SliceRef<const uint8_t> kSlice = kArr;
  EXPECT_EQ(absl::Span<const uint8_t>(kArr), kSlice.to_span());
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
  static constexpr rs_std::SliceRef<const uint8_t> kSlice = kArr;
  const auto fields = std::bit_cast<SliceRefFields>(kSlice);
  EXPECT_EQ(fields.ptr, kArr.data());
  EXPECT_EQ(fields.size, kArr.size());
}

TEST(SliceTest, Empty) {
  static constexpr uint8_t kEmptyArray[] = {};
  static constexpr rs_std::SliceRef<const uint8_t> kEmpty =
      absl::MakeSpan(kEmptyArray, 0);
  static constexpr rs_std::SliceRef<const uint8_t> kDefaultConstructed;
  EXPECT_EQ(kEmpty.to_span(), kDefaultConstructed.to_span());

  const auto fields = std::bit_cast<SliceRefFields>(kEmpty);
  EXPECT_THAT(fields.ptr, Not(IsNull()));
  EXPECT_EQ(fields.size, 0);

  // While `empty.data_` is not null, `data()` converts it to null for
  // compatibility with `std::span`.
  EXPECT_THAT(kEmpty.data(), IsNull());
  EXPECT_EQ(kEmpty.size(), 0);
}

TEST(ImplicitConversionTest, MutableFromVector) {
  std::vector<int> vec = {1, 2, 3};
  // Mirroring `absl::Span`, there is no implicit conversion from mutable
  // containers.
  static_assert(!std::convertible_to<std::vector<int>&, rs_std::SliceRef<int>>);
  // Explicit conversion works.
  const rs_std::SliceRef<int> from_vec(vec);
  EXPECT_THAT(from_vec.to_span(), ElementsAre(1, 2, 3));
}

TEST(ImplicitConversionTest, FromConstVector) {
  const std::vector<int> vec = {1, 2, 3};
  const rs_std::SliceRef<const int> from_vec = vec;
  EXPECT_THAT(from_vec.to_span(), ElementsAre(1, 2, 3));
}

TEST(ImplicitConversionTest, FromArray) {
  std::array<int, 2> arr = {1, 2};
  // Mirroring `absl::Span`, there is no implicit conversion from mutable
  // containers.
  static_assert(
      !std::convertible_to<std::array<int, 2>&, rs_std::SliceRef<int>>);
  // Explicit conversion works.
  const rs_std::SliceRef<int> from_arr(arr);
  EXPECT_THAT(from_arr.to_span(), ElementsAre(1, 2));
}

TEST(ImplicitConversionTest, FromConstArray) {
  static constexpr std::array<int, 2> kArray = {1, 2};
  static constexpr rs_std::SliceRef<const int> kFromArray = kArray;
  EXPECT_THAT(kFromArray.to_span(), ElementsAre(1, 2));
}

void Fuzzer(std::vector<uint8_t> data) {
  const rs_std::SliceRef<const uint8_t> s = data;
  EXPECT_EQ(absl::Span<const uint8_t>(data), s.to_span());
  const std::vector<uint8_t> data_copy(s.data(), s.data() + s.size());
  EXPECT_EQ(data, data_copy);
}

FUZZ_TEST(SliceFuzzTest, Fuzzer);

}  // namespace
