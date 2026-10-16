// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/arrays/arrays.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <new>
#include <utility>

#include "gtest/gtest.h"
#include "support/internal/slot.h"
#include "support/movable.h"

namespace {

using HasDropArray = std::array<arrays::HasDrop, 2>;
using NestedHasDropArray = std::array<HasDropArray, 2>;
using NestedNestedHasDropArray = std::array<NestedHasDropArray, 2>;

HasDropArray MakeHasDropArray(int32_t first) {
  return {arrays::HasDrop::new_(first), arrays::HasDrop::new_(first + 1)};
}

NestedHasDropArray MakeNestedHasDropArray(int32_t first) {
  return {MakeHasDropArray(first), MakeHasDropArray(first + 2)};
}

// Wraps the (non-C++-movable) value returned by `make` in an `rs::Movable`,
// which is how by-value parameters of such types are passed.
template <typename T, typename F>
rs::Movable<T> MakeMovable(F make) {
  crubit::Slot<T> slot;
  new (slot.Get()) T(make());
  return rs::Movable<T>::TakeFromSlot(std::move(slot));
}

TEST(ArraysTest, ConstArrayPtrInOut) {
  std::array<int32_t, 2> array = {1, 2};
  EXPECT_EQ(arrays::function_with_const_array_ptr_id(&array), &array);
}

TEST(ArraysTest, ArrayStructInOut) {
  arrays::ArrayStruct array_struct;
  array_struct.array[0] = 1;
  array_struct.array[1] = 2;
  EXPECT_EQ(arrays::function_with_array_struct_id(array_struct).array,
            array_struct.array);
}

TEST(ArraysTest, ArrayValueInOut) {
  std::array<int32_t, 2> array = {1, 2};
  EXPECT_EQ(arrays::function_with_array_id(array), array);
}

TEST(ArraysTest, TupleOfArraysValueInOut) {
  std::tuple<std::array<int32_t, 2>, std::array<int32_t, 2>> array_tup{{1, 2},
                                                                       {3, 4}};
  EXPECT_EQ(arrays::function_with_array_tuple_id(array_tup), array_tup);
}

TEST(ArraysTest, DropOut) {
  auto out = arrays::function_with_has_drop_ret_only();
  EXPECT_EQ(out[0].x, 1);
  EXPECT_EQ(out[1].x, 2);
}

TEST(ArraysTest, DropInOut) {
  size_t initial_drop_count = arrays::has_drop_drop_count();
  {
    auto out = arrays::function_with_has_drop_array_id(
        MakeMovable<HasDropArray>([] { return MakeHasDropArray(1); }));
    EXPECT_EQ(out[0].x, 1);
    EXPECT_EQ(out[1].x, 2);
    EXPECT_EQ(arrays::has_drop_drop_count(), initial_drop_count);
  }
  // Each element is dropped exactly once.
  EXPECT_EQ(arrays::has_drop_drop_count() - initial_drop_count, 2);
}

TEST(ArraysTest, NestedDropInOut) {
  size_t initial_drop_count = arrays::has_drop_drop_count();
  {
    auto out = arrays::function_with_nested_droponly_arrays(
        MakeMovable<NestedHasDropArray>(
            [] { return MakeNestedHasDropArray(1); }));
    EXPECT_EQ(out[0][0].x, 1);
    EXPECT_EQ(out[0][1].x, 2);
    EXPECT_EQ(out[1][0].x, 3);
    EXPECT_EQ(out[1][1].x, 4);
    EXPECT_EQ(arrays::has_drop_drop_count(), initial_drop_count);
  }
  EXPECT_EQ(arrays::has_drop_drop_count() - initial_drop_count, 4);
}

TEST(ArraysTest, NestedNestedDropInOut) {
  size_t initial_drop_count = arrays::has_drop_drop_count();
  {
    auto out = arrays::function_with_nested_nested_droponly_arrays(
        MakeMovable<NestedNestedHasDropArray>([] {
          return NestedNestedHasDropArray{MakeNestedHasDropArray(1),
                                          MakeNestedHasDropArray(5)};
        }));
    int32_t expected = 1;
    for (const auto& outer : out) {
      for (const auto& inner : outer) {
        for (const auto& element : inner) {
          EXPECT_EQ(element.x, expected++);
        }
      }
    }
    EXPECT_EQ(arrays::has_drop_drop_count(), initial_drop_count);
  }
  EXPECT_EQ(arrays::has_drop_drop_count() - initial_drop_count, 8);
}

TEST(ArraysTest, MovableArrayCanBeMoved) {
  size_t initial_drop_count = arrays::has_drop_drop_count();
  {
    auto array = MakeMovable<NestedHasDropArray>(
        [] { return MakeNestedHasDropArray(1); });
    rs::Movable<NestedHasDropArray> moved = std::move(array);
    EXPECT_TRUE(array.valueless_after_move());
    EXPECT_EQ((*moved)[1][1].x, 4);
  }
  EXPECT_EQ(arrays::has_drop_drop_count() - initial_drop_count, 4);
}

TEST(ArraysTest, DropAndDefaultInOut) {
  arrays::HasDropAndDefault a;
  arrays::HasDropAndDefault b;
  a.x = 1;
  b.x = 2;
  std::array<arrays::HasDropAndDefault, 2> array{std::move(a), std::move(b)};
  EXPECT_EQ(array[0].x, 1);
  EXPECT_EQ(array[1].x, 2);
  size_t initial_drop_count = arrays::has_drop_and_default_nonzero_drop_count();
  {
    auto out =
        arrays::function_with_has_drop_and_default_array_id(std::move(array));
    EXPECT_EQ(out[0].x, 1);
    EXPECT_EQ(out[1].x, 2);
  }
  // Only the default values left behind in `array` remain to be dropped.
  EXPECT_EQ(
      arrays::has_drop_and_default_nonzero_drop_count() - initial_drop_count,
      2);
}

TEST(ArraysTest, EmptyArrayInOut) {
  std::array<int32_t, 0> array;
  EXPECT_EQ(arrays::function_with_empty_array(array), array);
}

TEST(ArraysTest, NestedArraysInOut) {
  std::array<int32_t, 2> a = {1, 2};
  std::array<int32_t, 2> b = {3, 4};
  std::array<std::array<int32_t, 2>, 2> array{std::move(a), std::move(b)};
  EXPECT_EQ(arrays::function_with_nested_arrays(array), array);
}

TEST(ArraysTest, NestedDropDefaultArraysInOut) {
  arrays::HasDropAndDefault a;
  arrays::HasDropAndDefault b;
  arrays::HasDropAndDefault c;
  arrays::HasDropAndDefault d;
  a.x = 1;
  b.x = 2;
  c.x = 3;
  d.x = 4;
  std::array<arrays::HasDropAndDefault, 2> a_b{std::move(a), std::move(b)};
  std::array<arrays::HasDropAndDefault, 2> c_d{std::move(c), std::move(d)};
  std::array<std::array<arrays::HasDropAndDefault, 2>, 2> array{std::move(a_b),
                                                                std::move(c_d)};
  EXPECT_EQ(array[0][0].x, 1);
  EXPECT_EQ(array[0][1].x, 2);
  EXPECT_EQ(array[1][0].x, 3);
  EXPECT_EQ(array[1][1].x, 4);
  auto out = arrays::function_with_nested_drop_default_arrays(std::move(array));
  EXPECT_EQ(out[0][0].x, 1);
  EXPECT_EQ(out[0][1].x, 2);
  EXPECT_EQ(out[1][0].x, 3);
  EXPECT_EQ(out[1][1].x, 4);
}

}  // namespace
