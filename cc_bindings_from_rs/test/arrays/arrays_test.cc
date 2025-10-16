// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/arrays/arrays.h"

#include <cstdint>

#include "gtest/gtest.h"

namespace {

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
  auto out = arrays::function_with_has_drop_array_id(
      {arrays::HasDrop::new_(1), arrays::HasDrop::new_(2)});
  EXPECT_EQ(out[0].x, 1);
  EXPECT_EQ(out[1].x, 2);
}

TEST(ArraysTest, DropAndDefaultInOut) {
  arrays::HasDropAndDefault a;
  arrays::HasDropAndDefault b;
  a.x = 1;
  b.x = 2;
  std::array<arrays::HasDropAndDefault, 2> array{std::move(a), std::move(b)};
  EXPECT_EQ(array[0].x, 1);
  EXPECT_EQ(array[1].x, 2);
  auto out =
      arrays::function_with_has_drop_and_default_array_id(std::move(array));
  EXPECT_EQ(out[0].x, 1);
  EXPECT_EQ(out[1].x, 2);
}

TEST(ArraysTest, EmptyArrayInOut) {
  std::array<int32_t, 0> array;
  EXPECT_EQ(arrays::function_with_empty_array(array), array);
}
}  // namespace
