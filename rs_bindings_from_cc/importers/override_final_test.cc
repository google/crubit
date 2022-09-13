// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include <string_view>

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace {

using testing::Ne;

template <typename T>
class TailPaddingTest : public testing::Test {};
TYPED_TEST_SUITE_P(TailPaddingTest);

TYPED_TEST_P(TailPaddingTest, NoTailPaddingInBase) {
  struct Derived : public TypeParam {
    char extra;
  };

  EXPECT_THAT(sizeof(Derived), Ne(sizeof(TypeParam)));
}

REGISTER_TYPED_TEST_SUITE_P(TailPaddingTest, NoTailPaddingInBase);
INSTANTIATE_TYPED_TEST_SUITE_P(BuiltinTypes, TailPaddingTest,
                               ::testing::Types<std::string_view>);

}  // namespace
