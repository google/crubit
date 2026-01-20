// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "cc_bindings_from_rs/test/traits/in_dependent_crate/trait_impl.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace {

TEST(TraitImplTest, TestIsImplemented) {
  EXPECT_TRUE(
      trait_definition::MyTrait::impl<trait_impl::MyStruct>::kIsImplemented);
  EXPECT_TRUE(trait_definition::MyTrait::impl<
              trait_definition::MyStruct>::kIsImplemented);
  EXPECT_FALSE(trait_definition::MyTrait::impl<
               trait_impl::NotImplemented>::kIsImplemented);
}

TEST(TraitImplTest, TestMethods) {
  ::trait_impl::MyStruct s = ::trait_impl::MyStruct::new_(42);
  EXPECT_EQ(
      ::trait_definition::MyTrait::impl<::trait_impl::MyStruct>::do_something(
          s),
      42);
}

}  // namespace
