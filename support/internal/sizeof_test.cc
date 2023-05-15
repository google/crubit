// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/internal/sizeof.h"

#include "gtest/gtest.h"

namespace {

TEST(SizeofTest, Char) { EXPECT_EQ(CRUBIT_SIZEOF(char), sizeof(char)); }

TEST(SizeofTest, Int) { EXPECT_EQ(CRUBIT_SIZEOF(int), sizeof(int)); }

TEST(SizeofTest, AlignedAlias) {
  typedef __attribute__((__aligned__(64))) struct {
  } AlignedAlias;
  EXPECT_EQ(CRUBIT_SIZEOF(AlignedAlias), 64);
  EXPECT_EQ(sizeof(AlignedAlias), 1);
}

}  // namespace
