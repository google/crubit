// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/known_traits/compare/compare.h"

#include <cmath>
#include <compare>
#include <limits>
#include <optional>

#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(CompareTest, TestMyPartialOrd) {
  auto a = compare::MyPartialOrd{1};
  auto b = compare::MyPartialOrd{2};
  auto c = compare::MyPartialOrd{1};
  auto missing = compare::MyPartialOrd{std::nullopt};

  EXPECT_EQ(a <=> b, std::partial_ordering::less);
  EXPECT_EQ(b <=> a, std::partial_ordering::greater);
  EXPECT_EQ(a <=> c, std::partial_ordering::equivalent);
  EXPECT_EQ(a <=> missing, std::partial_ordering::unordered);

  EXPECT_TRUE(a < b);
  EXPECT_TRUE(b > a);
  EXPECT_TRUE(a <= b);
  EXPECT_TRUE(b >= a);
  EXPECT_TRUE(a == c);
  EXPECT_TRUE(a != b);
  EXPECT_FALSE(a == missing);
}

TEST(CompareTest, TestMyOrd) {
  auto a = compare::MyOrd{1};
  auto b = compare::MyOrd{2};
  auto c = compare::MyOrd{1};

  EXPECT_EQ(a <=> b, std::strong_ordering::less);
  EXPECT_EQ(b <=> a, std::strong_ordering::greater);
  EXPECT_EQ(a <=> c, std::strong_ordering::equivalent);

  EXPECT_TRUE(a < b);
  EXPECT_TRUE(b > a);
  EXPECT_TRUE(a <= b);
  EXPECT_TRUE(b >= a);
  EXPECT_TRUE(a == c);
  EXPECT_TRUE(a != b);
}

TEST(CompareTest, TestMyUnordered) {
  auto a = compare::MyUnordered{1.0f};
  auto b = compare::MyUnordered{std::numeric_limits<float>::quiet_NaN()};

  EXPECT_EQ(a <=> b, std::partial_ordering::unordered);
  EXPECT_EQ(b <=> a, std::partial_ordering::unordered);

  EXPECT_FALSE(a < b);
  EXPECT_FALSE(a > b);
  EXPECT_FALSE(a <= b);
  EXPECT_FALSE(a >= b);
  EXPECT_FALSE(a == b);
  EXPECT_TRUE(a != b);
}

}  // namespace
}  // namespace crubit
