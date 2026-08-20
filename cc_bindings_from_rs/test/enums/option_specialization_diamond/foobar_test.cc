// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/enums/option_specialization_diamond/foobar.h"

#include <concepts>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/enums/option_specialization_diamond/bar.h"
#include "cc_bindings_from_rs/test/enums/option_specialization_diamond/foo.h"
#include "support/rs_std/option.h"

namespace {

TEST(FoobarTest, FooBar) {
  foo::Foo a = foo::Foo::new_(1);
  bar::Bar b = bar::Bar::new_(2);
  // Test that we can use our two option types interchangeably between bindings
  // from our crates.
  const rs_std::Option<int32_t>& b_bar = b.bar;
  a.set_field(b_bar);
  EXPECT_TRUE(foobar::foo(foo::Foo::new_(1)).has_value());
  EXPECT_EQ(*foobar::foo(foo::Foo::new_(1)), 1);
  EXPECT_TRUE(foobar::bar(bar::Bar::new_(2)).has_value());
  EXPECT_EQ(*foobar::bar(bar::Bar::new_(2)), 2);
}

// Test that re-exported types across crates produce the exact same template
// specialization type.
static_assert(
    std::same_as<decltype(foobar::foo_opt()), decltype(foobar::bar_opt())>);
static_assert(std::same_as<rs_std::Option<foo::SomeStruct>,
                           rs_std::Option<bar::BarStruct>>);

}  // namespace
