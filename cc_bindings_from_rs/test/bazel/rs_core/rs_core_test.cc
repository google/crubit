// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Verify that <cmath> macros (INFINITY, NAN) do not collide with rs_core.h.
// clang-format off
#include <cmath>
#include "support/rs_std/rs_core.h"
// clang-format on

#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(RsCoreTest, Duration) {
  rs::core::time::Duration d = rs::core::time::Duration::from_secs(10);
  EXPECT_EQ(d.as_secs(), 10);
}

TEST(RsCoreTest, FloatConstants) {
  static_assert(rs::core::f32::INFINITY_ == __builtin_inff());
  static_assert(rs::core::f32::NEG_INFINITY == -__builtin_inff());
  static_assert(rs::core::f32::NAN_ != rs::core::f32::NAN_);
  static_assert(rs::core::f64::INFINITY_ == __builtin_inf());
  static_assert(rs::core::f64::NEG_INFINITY == -__builtin_inf());
  static_assert(rs::core::f64::NAN_ != rs::core::f64::NAN_);

  EXPECT_TRUE(std::isinf(rs::core::f32::INFINITY_));
  EXPECT_GT(rs::core::f32::INFINITY_, 0.0f);
  EXPECT_TRUE(std::isinf(rs::core::f32::NEG_INFINITY));
  EXPECT_LT(rs::core::f32::NEG_INFINITY, 0.0f);
  EXPECT_TRUE(std::isnan(rs::core::f32::NAN_));

  EXPECT_TRUE(std::isinf(rs::core::f64::INFINITY_));
  EXPECT_GT(rs::core::f64::INFINITY_, 0.0);
  EXPECT_TRUE(std::isinf(rs::core::f64::NEG_INFINITY));
  EXPECT_LT(rs::core::f64::NEG_INFINITY, 0.0);
  EXPECT_TRUE(std::isnan(rs::core::f64::NAN_));
}

}  // namespace
}  // namespace crubit
