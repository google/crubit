// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/non_trivially_destructible/non_trivially_destructible.h"

#include <type_traits>
#include <utility>

#include "gtest/gtest.h"

namespace {

TEST(NonTriviallyDestructibleTest, TestDestructible) {
  using namespace non_trivially_destructible;
  static_assert(!std::is_trivially_destructible_v<NonTriviallyDestructable>);

  NonTriviallyDestructable x = return_by_value();
  EXPECT_EQ(x.field, 123);
  take_by_value(std::move(x));
}

}  // namespace
