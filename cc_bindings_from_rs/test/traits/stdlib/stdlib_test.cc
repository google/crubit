// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/traits/stdlib/stdlib.h"

#include <cstdint>
#include <type_traits>

#include "gtest/gtest.h"
#include "support/rs_std/rs_core.h"

namespace crubit {
namespace {

TEST(StdlibTraitTest, IteratorItem) {
  using impl = rs::core::iter::Iterator::impl<stdlib::MyStruct>;
  static_assert(std::is_same_v<impl::Item, std::int32_t>);
}

TEST(StdlibTraitTest, IteratorNext) {
  using impl = rs::core::iter::Iterator::impl<stdlib::MyStruct>;
  auto s = stdlib::MyStruct::new_(3);
  EXPECT_EQ(std::optional(impl::next(s)), std::make_optional(2));
  EXPECT_EQ(std::optional(impl::next(s)), std::make_optional(1));
  EXPECT_EQ(std::optional(impl::next(s)), std::make_optional(0));
  EXPECT_EQ(std::optional(impl::next(s)), std::nullopt);
}

}  // namespace
}  // namespace crubit
