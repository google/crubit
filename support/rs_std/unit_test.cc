// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rs_std/unit.h"

#include <type_traits>

#include "gtest/gtest.h"

namespace rs_std {
namespace {

static_assert(std::is_trivially_destructible_v<unit_t>);
static_assert(std::is_trivially_copy_constructible_v<unit_t>);
static_assert(std::is_trivially_copy_assignable_v<unit_t>);
static_assert(std::is_trivially_move_constructible_v<unit_t>);
static_assert(std::is_trivially_move_assignable_v<unit_t>);
static_assert(std::is_standard_layout_v<unit_t>);

TEST(UnitTest, Equality) {
  unit_t u1 = unit;
  unit_t u2{};
  EXPECT_EQ(u1, u2);
  EXPECT_FALSE(u1 != u2);
}

}  // namespace
}  // namespace rs_std
