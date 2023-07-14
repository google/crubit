// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/internal/memswap.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace {

TEST(MemSwapTest, Basic) {
  int a = 123;
  int b = 456;
  crubit::MemSwap(a, b);
  EXPECT_EQ(a, 456);
  EXPECT_EQ(b, 123);
}

TEST(MemSwapTest, Aliasing) {
  int a = 123;
  crubit::MemSwap(a, a);
  EXPECT_EQ(a, 123);
}

}  // namespace
