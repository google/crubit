// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/bazel/include_str/include_str.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(IncludeStrTests, MainTest) {
  EXPECT_EQ(42, include_str::get_the_answer());
}

}  // namespace
}  // namespace crubit
