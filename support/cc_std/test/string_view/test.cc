// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gtest/gtest.h"
#include "support/cc_std/test/string_view/string_view_rs_apis.h"

namespace {

TEST(StringViewTest, ConsumeStringView) {
  // Asserts are done on the Rust side.
  string_view_rs_apis::consume_raw_string_view("Hello World");
  string_view_rs_apis::consume_string_view("Hello World");
}

TEST(StringViewTest, ReturnStringView) {
  EXPECT_EQ(string_view_rs_apis::return_raw_string_view(), "Hello World");
  EXPECT_EQ(string_view_rs_apis::return_string_view(), "Hello World");
}

}  // namespace
