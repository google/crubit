// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "examples/types/absl_status/rust_api.h"

namespace {

TEST(UserOfRustApiTest, ReturnsStatus) {
  EXPECT_OK(rust_api::returns_status(true));
  EXPECT_FALSE(rust_api::returns_status(false).ok());
}

TEST(UserOfRustApiTest, ReturnsStatusOrInt) {
  auto status_or_int = rust_api::returns_status_or_int(true);
  ASSERT_OK(status_or_int);
  EXPECT_EQ(*status_or_int, 42);
  EXPECT_FALSE(rust_api::returns_status_or_int(false).ok());
}

}  // namespace
