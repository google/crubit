// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/structs/structs_cc_api.h"

namespace crubit {
namespace {

TEST(StructsTest, ReprCPointReturnedOrTakenByValue) {
  structs::repr_c::Point p = structs::repr_c::create(123, 456);
  EXPECT_EQ(123, structs::repr_c::get_x(std::move(p)));
}

TEST(StructsTest, DefaultReprPointReturnedOrTakenByValue) {
  structs::default_repr::Point p = structs::default_repr::create(123, 456);
  EXPECT_EQ(123, structs::default_repr::get_x(std::move(p)));
}

TEST(StructsTest, StructInteger) {
  namespace test = structs::abi_classification;
  test::StructInteger x = test::StructInteger::create(123);
  test::StructInteger y = test::StructInteger::create(456);
  test::StructInteger product =
      test::StructInteger::multiply(std::move(x), std::move(y));
  EXPECT_EQ(123 * 456, test::StructInteger::inspect(std::move(product)));
}

TEST(StructsTest, StructFloat) {
  namespace test = structs::abi_classification;
  test::StructFloat x = test::StructFloat::create(456.0);
  test::StructFloat y = test::StructFloat::create(789.0);
  test::StructFloat product =
      test::StructFloat::multiply(std::move(x), std::move(y));
  EXPECT_EQ(456.0 * 789.0, test::StructFloat::inspect(std::move(product)));
}

TEST(StructsTest, StructMemory) {
  namespace test = structs::abi_classification;
  test::StructMemory x = test::StructMemory::create(321);
  test::StructMemory y = test::StructMemory::create(654);
  test::StructMemory product =
      test::StructMemory::multiply(std::move(x), std::move(y));
  EXPECT_EQ(321 * 654, test::StructMemory::inspect(std::move(product)));
}

TEST(StructsTest, DefinitionlessStructFloat) {
  namespace test = structs::struct_by_float_passing_with_no_cc_definition;
  test::StructFloat x = test::no_mangle_create(111.0);
  test::StructFloat y = test::no_mangle_create(222.0);
  test::StructFloat product =
      test::no_mangle_multiply(std::move(x), std::move(y));
  EXPECT_EQ(111.0 * 222.0, test::no_mangle_inspect(std::move(product)));
}

TEST(StructsTest, ThunklessStructFloat) {
  namespace test = structs::struct_by_float_passing_with_no_thunk;
  test::StructFloat x = test::thunkless_create(111.0);
  test::StructFloat y = test::thunkless_create(222.0);
  test::StructFloat product =
      test::thunkless_multiply(std::move(x), std::move(y));
  EXPECT_EQ(111.0 * 222.0, test::thunkless_inspect(std::move(product)));
}

}  // namespace
}  // namespace crubit
