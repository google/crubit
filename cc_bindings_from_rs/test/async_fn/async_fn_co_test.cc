// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstdint>

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/async_fn/async_fn.h"
#include "util/c9/testing/co_test.h"

namespace {

CO_TEST(AsyncFnsTest, Add) {
  std::int32_t sum = co_await async_fn::add(12, 34);
  EXPECT_EQ(46, sum);
}

}  // namespace
