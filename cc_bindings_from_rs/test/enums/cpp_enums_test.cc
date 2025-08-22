// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/enums/cpp_enums.h"

#include <cstdint>
#include <type_traits>

#include "gtest/gtest.h"

namespace {

template <typename T>
class CppEnumTest : public ::testing::Test {};

using MyTypes = ::testing::Types<cpp_enums::classless_enum::Color,
                                 cpp_enums::cpp_enum::Color,
                                 cpp_enums::deprecated_enum::Color>;
TYPED_TEST_SUITE(CppEnumTest, MyTypes);

TYPED_TEST(CppEnumTest, BasicTest) {
  static_assert(std::is_enum_v<TypeParam>);

  TypeParam red = TypeParam::RED;
  TypeParam blue = TypeParam::BLUE;
  EXPECT_EQ(static_cast<int32_t>(red), 0);
  EXPECT_EQ(static_cast<int32_t>(blue), 2);
}

TEST(ClasslessEnumTest, IsClasslessEnum) {
  int32_t red = cpp_enums::classless_enum::RED;
  int32_t blue = cpp_enums::classless_enum::BLUE;
  EXPECT_EQ(red, 0);
  EXPECT_EQ(blue, 2);
}

}  // namespace
