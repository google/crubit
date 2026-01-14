// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "cc_bindings_from_rs/test/traits/in_dependent_crate/trait_impl.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace {

TEST(TraitImplTest, Test) {
  EXPECT_TRUE(trait_definition::MyTrait<trait_impl::MyStruct>::is_implemented);
  EXPECT_TRUE(
      trait_definition::MyTrait<trait_definition::MyStruct>::is_implemented);
  EXPECT_FALSE(
      trait_definition::MyTrait<trait_impl::NotImplemented>::is_implemented);
}

}  // namespace
