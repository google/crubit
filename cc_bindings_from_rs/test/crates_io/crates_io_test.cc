// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/crates_io/fastrand_cc_api.h"
#include "support/rs_std/rs_char.h"

namespace crubit {
namespace {

TEST(CratesIoTests, FastRand) {
  fastrand::seed(123);
  EXPECT_EQ(123, fastrand::get_seed());
  EXPECT_EQ(uint32_t{'o'}, uint32_t{fastrand::lowercase()});
}

}  // namespace
}  // namespace crubit
