// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/status_test_matchers.h"

#include <string>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "common/status.h"
#include "llvm/Support/Error.h"

namespace crubit {
namespace {

using ::testing::HasSubstr;
using ::testing::Not;

TEST(StatusTestMatchersTest, IsOkAbslStatus) {
  absl::Status s = absl::OkStatus();
  EXPECT_OK(s);
  EXPECT_THAT(s, IsOk());

  absl::Status s2 = absl::InvalidArgumentError("err");
  EXPECT_THAT(s2, Not(IsOk()));
}

TEST(StatusTestMatchersTest, IsOkAbslStatusOr) {
  absl::StatusOr<int> s = 42;
  EXPECT_OK(s);
  EXPECT_THAT(s, IsOk());

  absl::StatusOr<int> s2 = absl::InvalidArgumentError("err");
  EXPECT_THAT(s2, Not(IsOk()));
}

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
  absl::StatusOr<int> s = 42;
  EXPECT_THAT(s, IsOkAndHolds(42));

  llvm::Expected<int> e = 42;
  EXPECT_THAT(e, IsOkAndHolds(42));
}

TEST(StatusTestMatchersTest, StatusIsAbsl) {
  absl::Status s = absl::InvalidArgumentError("absl msg");
  EXPECT_THAT(s, StatusIs(absl::StatusCode::kInvalidArgument));
  EXPECT_THAT(s, StatusIs(absl::StatusCode::kInvalidArgument, "absl msg"));
  EXPECT_THAT(s,
              StatusIs(absl::StatusCode::kInvalidArgument, HasSubstr("msg")));
  EXPECT_THAT(s, Not(StatusIs(absl::StatusCode::kNotFound)));
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
  absl::StatusOr<int> s = 42;
  ASSERT_OK_AND_ASSIGN(int x, s);
  EXPECT_EQ(x, 42);

  llvm::Expected<int> e = 100;
  ASSERT_OK_AND_ASSIGN(int y, std::move(e));
  EXPECT_EQ(y, 100);
}

}  // namespace
}  // namespace crubit
