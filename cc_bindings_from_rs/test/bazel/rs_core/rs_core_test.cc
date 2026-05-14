// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rs_std/rs_core.h"

#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(RsCoreTest, Duration) {
  rs::core::time::Duration d = rs::core::time::Duration::from_secs(10);
  EXPECT_EQ(d.as_secs(), 10);
}

}  // namespace
}  // namespace crubit
