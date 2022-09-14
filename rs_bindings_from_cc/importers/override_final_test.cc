// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include <string_view>
#include <type_traits>

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace {

using testing::Ne;

template <typename T>
class OverrideFinalTest : public testing::Test {};
TYPED_TEST_SUITE_P(OverrideFinalTest);

TYPED_TEST_P(OverrideFinalTest, NoTailPaddingInBase) {
  struct Derived : public TypeParam {
    char extra;
  };

  EXPECT_THAT(sizeof(Derived), Ne(sizeof(TypeParam)));
}

TYPED_TEST_P(OverrideFinalTest, NotPolymorphic) {
  EXPECT_FALSE(std::is_polymorphic_v<TypeParam>);
}

TYPED_TEST_P(OverrideFinalTest, TriviallyDestructible) {
  EXPECT_TRUE(std::is_trivially_destructible_v<TypeParam>);
}

REGISTER_TYPED_TEST_SUITE_P(OverrideFinalTest, NoTailPaddingInBase,
                            NotPolymorphic, TriviallyDestructible);
INSTANTIATE_TYPED_TEST_SUITE_P(BuiltinTypes, OverrideFinalTest,
                               ::testing::Types<std::string_view>);

}  // namespace
