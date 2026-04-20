// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/structs/const_field/const_field.h"

namespace {

TEST(ConstFieldTest, StructWithConstFieldCanBeReturnedByValueInsideGenerics) {
  auto result =
      const_field::return_struct_with_const_field_by_value_in_result();
  EXPECT_TRUE(result.has_value());
  auto option =
      const_field::return_struct_with_const_field_by_value_in_option();
  EXPECT_TRUE(option.has_value());
}

}  // namespace
