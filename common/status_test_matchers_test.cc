// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/status_test_matchers.h"

#include <string>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "common/status.h"
#include "llvm/Support/Error.h"

namespace crubit {
namespace {

using ::testing::HasSubstr;
using ::testing::Not;

TEST(StatusTestMatchersTest, IsOkLlvmError) {
  llvm::Error e = llvm::Error::success();
  EXPECT_OK(e);

  llvm::Error e2 = InvalidArgumentError("err");
  EXPECT_THAT(e2, Not(IsOk()));
}

TEST(StatusTestMatchersTest, IsOkLlvmExpected) {
  llvm::Expected<int> e = 42;
  EXPECT_OK(e);

  llvm::Expected<int> e2 = InvalidArgumentError("err");
  EXPECT_THAT(e2, Not(IsOk()));
}

TEST(StatusTestMatchersTest, IsOkAndHolds) {
  llvm::Expected<int> e = 42;
  EXPECT_THAT(e, IsOkAndHolds(42));
}

TEST(StatusTestMatchersTest, StatusIsLlvm) {
  llvm::Error e = InvalidArgumentError("llvm msg");
  EXPECT_THAT(e, StatusIs(StatusCode::kInvalidArgument));

  llvm::Error e2 = InvalidArgumentError("llvm msg");
  EXPECT_THAT(e2, StatusIs(StatusCode::kInvalidArgument, "llvm msg"));

  llvm::Error e3 = InvalidArgumentError("llvm msg");
  EXPECT_THAT(e3, StatusIs(StatusCode::kInvalidArgument, HasSubstr("msg")));

  llvm::Error e4 = InvalidArgumentError("llvm msg");
  EXPECT_THAT(e4, Not(StatusIs(StatusCode::kNotFound)));
}

TEST(StatusTestMatchersTest, AssertOkAndAssign) {
  llvm::Expected<int> e = 100;
  ASSERT_OK_AND_ASSIGN(int y, std::move(e));
  EXPECT_EQ(y, 100);
}

}  // namespace
}  // namespace crubit
