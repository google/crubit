// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/bazel/build_rs/build_rs_user.h"

namespace crubit {
namespace {

// This test reproduces  the following error (that was discovered via
// cc_bindings_from_rs/test/crates_io tests):
//
// ERROR: ... error executing command .../cc_bindings_from_rs --h-out ...
// (remaining 30 arguments skipped) ...
// error: environment variable `OUT_DIR` not defined at compile time
//   -->
//   cc_bindings_from_rs/test/bazel/build_rs_out_dir/build_rs_out_dir.rs:10:18
//    |
// 10 | include!(concat!(env!("OUT_DIR"), "/include_me.rs"));
//    |                  ^^^^^^^^^^^^^^^
TEST(BuildRsTest, OutDir) {
  auto sum = build_rs_user::add_two_integers(123, 456);
  EXPECT_EQ(sum, 123 + 456);
}

// Features set by build.rs should be read by Crubit.
TEST(BuildRsTest, Features) {
  EXPECT_TRUE(build_rs_user::cfg_set_by_build_rs());
}

}  // namespace
}  // namespace crubit
