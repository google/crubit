// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "cc_bindings_from_rs/test/traits/traits.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"

TEST(TraitsTest, TraitBinding) {
  EXPECT_EQ(traits::MyTrait<traits::MyStruct>::is_implemented, true);
  EXPECT_EQ(traits::MyTrait<int>::is_implemented, false);
}
