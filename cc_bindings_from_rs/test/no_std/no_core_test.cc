// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "cc_bindings_from_rs/test/no_std/no_core.h"

#include "gtest/gtest.h"

namespace {

TEST(NoCoreTest, TestNoCoreStruct) {
  auto s = no_core::Test::new_();
  EXPECT_EQ(s.s().to_string_view(), "");
}

}  // namespace
