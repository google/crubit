// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "cc_bindings_from_rs/test/no_std/no_std.h"

#include "gtest/gtest.h"

namespace {

TEST(NoStdTest, TestNoStdStruct) {
  auto s = no_std::NoStdStruct::new_(1, 2.0);
  EXPECT_EQ(s.display().to_string_view(), "(1, 2)");
}

}  // namespace
