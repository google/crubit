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

static_assert(std::equality_comparable<rs_std::SliceRef<int>>);
static_assert(std::equality_comparable<rs_std::SliceRef<const int>>);
static_assert(std::equality_comparable_with<rs_std::SliceRef<const int>,
                                            rs_std::SliceRef<int>>);
// Ensure that slices can be compared with types that are convertible to slices.
static_assert(std::equality_comparable_with<rs_std::SliceRef<const int>,
                                            const std::vector<int>&>);

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

// Regression test for b/558688351: A range type with a converting constructor
// from SliceRef should not trigger a circular constraint check when testing
// whether it is copy-constructible.
struct RangeWithSliceRefConstructor {
  struct Iterator {
    using value_type = uint8_t;
    using difference_type = std::ptrdiff_t;
    const uint8_t& operator*() const;
    Iterator& operator++() { return *this; }
    void operator++(int) {}
    bool operator==(const Iterator&) const { return true; }
  };

  RangeWithSliceRefConstructor() = default;
  RangeWithSliceRefConstructor(const RangeWithSliceRefConstructor&) = default;
  RangeWithSliceRefConstructor(RangeWithSliceRefConstructor&&) = default;
  explicit RangeWithSliceRefConstructor(rs_std::SliceRef<const uint8_t>);

  Iterator begin() const { return {}; }
  Iterator end() const { return {}; }
};

static_assert(std::is_copy_constructible_v<RangeWithSliceRefConstructor>);
static_assert(std::is_move_constructible_v<RangeWithSliceRefConstructor>);

TEST(SliceTest, RangeWithSliceRefConstructorIsCopyConstructible) {
  EXPECT_TRUE(std::is_copy_constructible_v<RangeWithSliceRefConstructor>);
  EXPECT_TRUE(std::is_move_constructible_v<RangeWithSliceRefConstructor>);
}

TEST(SliceTest, Comparison) {
  static constexpr std::array<uint8_t, 5> kArr = {1, 2, 3, 4, 5};
  static constexpr std::array<uint8_t, 5> kArrCopy = {1, 2, 3, 4, 5};

  static constexpr rs_std::SliceRef<const uint8_t> kSlice = kArr;
  static constexpr rs_std::SliceRef<const uint8_t> kSliceCopy = kSlice;
  static constexpr rs_std::SliceRef<const uint8_t> kSlice2 = kArrCopy;

  EXPECT_EQ(kSlice, kSlice);
  EXPECT_EQ(kSlice, kSliceCopy);
  EXPECT_EQ(kSlice, kSlice2);
  static constexpr rs_std::SliceRef<const uint8_t> kSlicePrefix =
      absl::MakeSpan(kArr).first(kArr.size() - 1);
  static constexpr rs_std::SliceRef<const uint8_t> kSliceSuffix =
      absl::MakeSpan(kArr).subspan(1);
  static constexpr rs_std::SliceRef<const uint8_t> kSliceInfix =
      absl::MakeSpan(kArr).subspan(1, kArr.size() - 2);

  EXPECT_NE(kSlice, kSlicePrefix);
  EXPECT_NE(kSlice, kSliceSuffix);
  EXPECT_NE(kSlice, kSliceInfix);
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
  EXPECT_EQ(kEmpty, kDefaultConstructed);

  const auto fields = std::bit_cast<SliceRefFields>(kEmpty);
  EXPECT_THAT(fields.ptr, Not(IsNull()));
  EXPECT_EQ(fields.size, 0);

  // While `empty.data_` is not null, `data()` converts it to null for
  // compatibility with `std::span`.
  EXPECT_THAT(kEmpty.data(), IsNull());
  EXPECT_EQ(kEmpty.size(), 0);
}

TEST(SliceTest, FromStringView) {
  std::string_view sv = "hello";
  rs_std::SliceRef<const uint8_t> slice(sv);
  EXPECT_EQ(slice.size(), sv.size());
  EXPECT_EQ(reinterpret_cast<const char*>(slice.data()), sv.data());
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

// Generates a compiler error when uncommented:
//
// std::array<int, 3> TempArray() { return {1, 2, 3}; };
//
// TEST(DemoTest, SliceRef) {
//   rs_std::SliceRef<const int> slice_ref(TempArray());
//   // ^ error: temporary whose address is used as the value of local...
//   ASSERT_EQ(slice_ref.size(), 3);
//   EXPECT_EQ(slice_ref.to_span()[2], 3);  // BOOM
// }

TEST(ImplicitConversionTest, ToAbslSpan) {
  static constexpr std::array<int, 2> kArray = {1, 2};
  static constexpr rs_std::SliceRef<const int> kSlice = kArray;
  absl::Span<const int> span = kSlice;
  EXPECT_THAT(span, ElementsAre(1, 2));
}

TEST(ImplicitConversionTest, FromStringViewUint8) {
  constexpr std::string_view kStr = "hello";
  // Implicit conversion enabled by the new constructor's reinterpret_cast:
  const rs_std::SliceRef<const uint8_t> kSlice = kStr;
  EXPECT_EQ(kSlice.size(), 5);
  EXPECT_EQ(kSlice.data(), reinterpret_cast<const uint8_t*>(kStr.data()));
}

TEST(ImplicitConversionTest, FromStringViewChar) {
  constexpr std::string_view kStr = "hello";
  constexpr rs_std::SliceRef<const char> kSlice(kStr);
  EXPECT_EQ(kSlice.size(), 5);
  EXPECT_EQ(kSlice.data(), kStr.data());
}

TEST(ImplicitConversionTest, FromConstUint8Array) {
  static constexpr std::array<uint8_t, 5> kArr = {'h', 'e', 'l', 'l', 'o'};
  static constexpr rs_std::SliceRef<const uint8_t> kSlice = kArr;
  EXPECT_EQ(kSlice.size(), 5);
  EXPECT_EQ(kSlice.to_span()[0], 'h');
}

void TakingSliceRefUint8(rs_std::SliceRef<const uint8_t> slice) {
  EXPECT_EQ(slice.size(), 5);
  EXPECT_EQ(slice.to_span()[0], 'h');
  EXPECT_EQ(slice.to_span()[4], 'o');
}
void TakingSliceRefChar(rs_std::SliceRef<const char> slice) {
  EXPECT_EQ(slice.size(), 6);
  EXPECT_EQ(slice.to_span()[0], 'h');
  EXPECT_EQ(slice.to_span()[4], 'o');
  EXPECT_EQ(slice.to_span()[5], '\0');
}

TEST(ImplicitConversionTest, CallWithLiteral) {
  TakingSliceRefUint8("hello");
  TakingSliceRefChar("hello");
}

// `SliceRef<T>` can be a member of a class before `T` is complete, as in
// generated bindings for
// `struct Dict { values: &'static [(&'static str, Value)] }` when `Value`
// refers back to `Dict`.
struct Incomplete;
struct HoldsSliceOfIncomplete {
  rs_std::SliceRef<const Incomplete> values{
      crubit::internal::SliceRefDanglingTag{}, 8};
};
// Copying and moving `SliceRef<T>` (via the implicit special members of the
// class) doesn't need `T` to be complete.
static_assert(std::is_trivially_copyable_v<HoldsSliceOfIncomplete>);
HoldsSliceOfIncomplete CopyThenMove(const HoldsSliceOfIncomplete& holder) {
  HoldsSliceOfIncomplete copy = holder;
  return copy;
}
struct alignas(8) Incomplete {
  int64_t value;
};

TEST(SliceRefTest, MemberWithIncompleteElementType) {
  HoldsSliceOfIncomplete holder;
  EXPECT_EQ(CopyThenMove(holder).values.size(), 0);
  EXPECT_THAT(CopyThenMove(holder).values.data(), IsNull());
}

TEST(SliceRefTest, DanglingTagConstructorMatchesDefault) {
  using Slice = rs_std::SliceRef<const int64_t>;
  constexpr Slice from_alignment(crubit::internal::SliceRefDanglingTag{},
                                 alignof(int64_t));
  EXPECT_EQ(from_alignment.size(), 0);
  EXPECT_THAT(from_alignment.data(), IsNull());
  // Rust sees the dangling data pointer too, so compare the whole
  // representation, not just `data()` and `size()`.
  using Bytes = std::array<std::byte, sizeof(Slice)>;
  EXPECT_EQ(std::bit_cast<Bytes>(from_alignment),
            std::bit_cast<Bytes>(Slice()));
}

TEST(SliceTest, RoundTripVectors) {
  const std::vector<std::vector<uint8_t>> test_cases = {
      {}, {0}, {255}, {0, 1, 2, 0, 254, 255}, std::vector<uint8_t>(1024, 0xab),
  };
  for (const std::vector<uint8_t>& data : test_cases) {
    const rs_std::SliceRef<const uint8_t> s = data;
    EXPECT_EQ(absl::Span<const uint8_t>(data), s.to_span());
    const std::vector<uint8_t> data_copy(s.data(), s.data() + s.size());
    EXPECT_EQ(data, data_copy);
  }
}

}  // namespace
