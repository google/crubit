// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "cc_bindings_from_rs/test/enums/option_specialization_diamond/foobar.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace {

TEST(FoobarTest, FooBar) {
  foo::Foo a = foo::Foo::new_(1);
  bar::Bar b = bar::Bar::new_(2);
  // Test that we can use our two option types interchangeably between bindings
  // from our crates.
  const rs_std::Option<int32_t>& b_bar = b.bar;
  a.set_field(b_bar);
  EXPECT_EQ(foobar::foo(foo::Foo::new_(1)), 1);
  EXPECT_EQ(foobar::bar(bar::Bar::new_(2)), 2);
}

}  // namespace
