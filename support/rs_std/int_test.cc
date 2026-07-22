// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rs_std/int.h"

#include <cstdint>
#include <type_traits>

#include "gtest/gtest.h"

namespace {

static_assert(std::is_nothrow_default_constructible_v<rs_std::usize>);
static_assert(std::is_trivially_destructible_v<rs_std::usize>);
static_assert(std::is_trivially_copyable_v<rs_std::usize>);
static_assert(std::is_trivially_copy_constructible_v<rs_std::usize>);
static_assert(std::is_trivially_copy_assignable_v<rs_std::usize>);
static_assert(std::is_trivially_move_constructible_v<rs_std::usize>);
static_assert(std::is_trivially_move_assignable_v<rs_std::usize>);
static_assert(sizeof(rs_std::usize) == sizeof(std::uintptr_t));
static_assert(alignof(rs_std::usize) == alignof(std::uintptr_t));

static_assert(std::is_nothrow_default_constructible_v<rs_std::isize>);
static_assert(std::is_trivially_destructible_v<rs_std::isize>);
static_assert(std::is_trivially_copyable_v<rs_std::isize>);
static_assert(std::is_trivially_copy_constructible_v<rs_std::isize>);
static_assert(std::is_trivially_copy_assignable_v<rs_std::isize>);
static_assert(std::is_trivially_move_constructible_v<rs_std::isize>);
static_assert(std::is_trivially_move_assignable_v<rs_std::isize>);
static_assert(sizeof(rs_std::isize) == sizeof(std::intptr_t));
static_assert(alignof(rs_std::isize) == alignof(std::intptr_t));

TEST(UsizeTest, DefaultConstructor) {
  rs_std::usize value;
  EXPECT_EQ(value, rs_std::usize(0));
}

TEST(UsizeTest, ImplicitConversions) {
  rs_std::usize val = 42;
  std::uintptr_t raw = val;
  EXPECT_EQ(raw, 42);
}

TEST(UsizeTest, ArithmeticAndComparisons) {
  rs_std::usize a = 10;
  rs_std::usize b = 20;
  EXPECT_LT(a, b);
  EXPECT_EQ(a + b, rs_std::usize(30));
  EXPECT_EQ(b - a, rs_std::usize(10));
}

TEST(IsizeTest, DefaultConstructor) {
  rs_std::isize value;
  EXPECT_EQ(value, rs_std::isize(0));
}

TEST(IsizeTest, ImplicitConversions) {
  rs_std::isize val = -42;
  std::intptr_t raw = val;
  EXPECT_EQ(raw, -42);
}

TEST(IsizeTest, ArithmeticAndComparisons) {
  rs_std::isize a = -10;
  rs_std::isize b = 20;
  EXPECT_LT(a, b);
  EXPECT_EQ(a + b, rs_std::isize(10));
  EXPECT_EQ(b - a, rs_std::isize(30));
}

}  // namespace
