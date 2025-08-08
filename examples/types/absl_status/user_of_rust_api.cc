// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "examples/types/absl_status/rust_api.h"

namespace {

TEST(UserOfRustApiTest, ReturnsStatus) {
  // Cannot call returns_status() directly for now,
  // and need to call .status() on a `StatusWrapper` return
  // value instead.
  EXPECT_OK(rust_api::ReturnsStatus(true).status());
}

}  // namespace
