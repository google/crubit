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

}  // namespace
