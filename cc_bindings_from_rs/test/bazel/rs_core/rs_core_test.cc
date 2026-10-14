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
  static_assert(rs::core::f32::NEG_INFINITY == -__builtin_inff());
  static_assert(rs::core::f64::NEG_INFINITY == -__builtin_inf());
}

}  // namespace
}  // namespace crubit
