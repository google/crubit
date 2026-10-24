// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Ensure `::std` is root-qualified inside `namespace rs` when `rs::std` exists.
namespace rs::std {}

#include "support/rs_std/slice_box.h"

#include <memory>
#include <span>
#include <type_traits>
#include <utility>

#include "support/internal/slot.h"
#include "support/movable.h"
#include "support/unsafe_relocate_tag.h"
#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "support/rs_std/slice_ref.h"

namespace rs {
namespace {

struct Tracked {
  ::std::shared_ptr<int> tracker;
  Tracked() = default;
  explicit Tracked(::std::shared_ptr<int> t) : tracker(::std::move(t)) {}
};

TEST(SliceBoxTest, StaticAsserts) {
  static_assert(sizeof(::rs::SliceBox<int>) == 2 * sizeof(void*));
  static_assert(!::std::is_copy_constructible_v<SliceBox<int>>);
  static_assert(!::std::is_move_constructible_v<SliceBox<int>>);
  static_assert(!::std::is_copy_assignable_v<SliceBox<int>>);
  static_assert(!::std::is_move_assignable_v<SliceBox<int>>);
  static_assert(
      ::std::is_constructible_v<SliceBox<int>, ::crubit::UnsafeRelocateTag,
                                SliceBox<int>&&>);
}

TEST(SliceBoxTest, Empty) {
  SliceBox<int> b;
  EXPECT_TRUE(b.empty());
  EXPECT_EQ(b.size(), 0);
  EXPECT_EQ(b.data(), nullptr);
}

TEST(SliceBoxTest, FromSpan) {
  int arr[] = {1, 2, 3};
  SliceBox<int> b{::std::span<const int>(arr)};
  EXPECT_FALSE(b.empty());
  EXPECT_EQ(b.size(), 3);
  EXPECT_EQ(b[0], 1);
  EXPECT_EQ(b[1], 2);
  EXPECT_EQ(b[2], 3);
}

TEST(SliceBoxTest, FromInitializerList) {
  SliceBox<int> b{1, 2, 3};
  EXPECT_EQ(b.size(), 3);
  EXPECT_EQ(b[0], 1);
  EXPECT_EQ(b[1], 2);
  EXPECT_EQ(b[2], 3);
}

TEST(SliceBoxTest, FromSizeAndValue) {
  SliceBox<int> b(3, 42);
  EXPECT_EQ(b.size(), 3);
  EXPECT_EQ(b[0], 42);
  EXPECT_EQ(b[1], 42);
  EXPECT_EQ(b[2], 42);
}

TEST(SliceBoxTest, Destructor) {
  auto tracker = ::std::make_shared<int>(42);
  EXPECT_EQ(tracker.use_count(), 1);
  {
    SliceBox<Tracked> b(3, Tracked(tracker));
    EXPECT_EQ(tracker.use_count(), 4);
  }
  EXPECT_EQ(tracker.use_count(), 1);
}

TEST(SliceBoxTest, Relocation) {
  auto tracker = ::std::make_shared<int>(42);
  {
    alignas(SliceBox<Tracked>) char buf[sizeof(SliceBox<Tracked>)];
    SliceBox<Tracked>* b1 = new (buf) SliceBox<Tracked>(3, Tracked(tracker));
    EXPECT_EQ(tracker.use_count(), 4);
    SliceBox<Tracked> b2(::crubit::UnsafeRelocateTag{}, ::std::move(*b1));
    EXPECT_EQ(tracker.use_count(), 4);
  }
  EXPECT_EQ(tracker.use_count(), 1);
}

TEST(SliceBoxTest, AsSlice) {
  SliceBox<int> b{1, 2, 3};
  SliceRef<int> ref = b.as_slice();
  EXPECT_EQ(ref.size(), 3);
  EXPECT_EQ(ref.data()[0], 1);

  SliceRef<const int> const_ref = b;
  EXPECT_EQ(const_ref.size(), 3);
  EXPECT_EQ(const_ref.data()[0], 1);
}

}  // namespace
}  // namespace rs
