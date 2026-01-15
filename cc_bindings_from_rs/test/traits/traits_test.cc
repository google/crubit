// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "cc_bindings_from_rs/test/traits/traits.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"

TEST(TraitsTest, TraitIsImplemented) {
  EXPECT_EQ(traits::MyTrait<traits::MyStruct>::is_implemented, true);
  EXPECT_EQ(traits::MyTrait<traits::MyStruct2>::is_implemented, true);
  EXPECT_EQ(traits::MyTrait<int>::is_implemented, false);
}

TEST(TraitsTest, MyStructMethods) {
  traits::MyStruct s = traits::MyStruct::new_(42);
  EXPECT_EQ(traits::MyTrait<traits::MyStruct>::do_something(s), 42);
  EXPECT_EQ(traits::MyTrait<traits::MyStruct>::consume_self(s), 42);
  traits::Foo foo = traits::Foo::new_(1, 2);
  std::tuple<int, int> bar =
      traits::MyTrait<traits::MyStruct>::take_and_return_other_types(s, foo);
  EXPECT_EQ(bar, std::make_tuple(1, 2));
}
