// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/bazel/build_rs_out_dir/build_rs_out_dir_cc_api.h"

namespace crubit {
namespace {

TEST(BuildRsOutDirTests, MainTest) {
  auto sum = build_rs_out_dir::add_two_integers(123, 456);
  EXPECT_EQ(sum, 123 + 456);
}

}  // namespace
}  // namespace crubit
