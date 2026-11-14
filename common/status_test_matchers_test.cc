// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/status_test_matchers.h"

#include <sstream>
#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "common/status.h"
#include "llvm/Support/Error.h"

namespace crubit {
namespace {

using ::testing::HasSubstr;
using ::testing::Not;

TEST(StatusTestMatchersTest, IsOk) {
  EXPECT_OK(llvm::Error::success());
  EXPECT_THAT(InvalidArgumentError("err"), Not(IsOk()));

  EXPECT_OK(llvm::Expected<int>(42));
  EXPECT_THAT(llvm::Expected<int>(InvalidArgumentError("err")), Not(IsOk()));
}

TEST(StatusTestMatchersTest, IsOkAndHolds) {
  EXPECT_THAT(llvm::Expected<int>(42), IsOkAndHolds(42));
}

TEST(StatusTestMatchersTest, StatusIsLlvm) {
  auto check = [](auto matcher) {
    EXPECT_THAT(InvalidArgumentError("llvm msg"), matcher);
    EXPECT_THAT(llvm::Expected<int>(InvalidArgumentError("llvm msg")), matcher);
  };
  check(StatusIs(StatusCode::kInvalidArgument));
  check(StatusIs(StatusCode::kInvalidArgument, "llvm msg"));
  check(StatusIs(StatusCode::kInvalidArgument, HasSubstr("msg")));
  check(Not(StatusIs(StatusCode::kNotFound)));
}

TEST(StatusTestMatchersTest, AssertOkAndAssign) {
  llvm::Expected<int> e = 100;
  ASSERT_OK_AND_ASSIGN(int y, std::move(e));
  EXPECT_EQ(y, 100);
}

TEST(StatusTestMatchersTest, StatusIsDescription) {
  ::testing::Matcher<llvm::Error> matcher =
      StatusIs(StatusCode::kInvalidArgument, HasSubstr("bad"));

  std::ostringstream desc;
  matcher.DescribeTo(&desc);
  EXPECT_EQ(desc.str(),
            "has status code INVALID_ARGUMENT and a message that has substring "
            "\"bad\"");

  std::ostringstream neg_desc;
  matcher.DescribeNegationTo(&neg_desc);
  EXPECT_EQ(neg_desc.str(),
            "does not have status code INVALID_ARGUMENT and a message that has "
            "substring \"bad\"");

  ::testing::StringMatchResultListener code_listener;
  EXPECT_FALSE(matcher.MatchAndExplain(NotFoundError("bad"), &code_listener));
  EXPECT_THAT(
      code_listener.str(),
      HasSubstr(
          "whose canonical status code is not equal to INVALID_ARGUMENT (was "
          "NOT_FOUND)"));

  ::testing::StringMatchResultListener msg_listener;
  EXPECT_FALSE(
      matcher.MatchAndExplain(InvalidArgumentError("other"), &msg_listener));
  EXPECT_THAT(msg_listener.str(),
              HasSubstr("whose error message is wrong (was: \"other\")"));
}
}  // namespace
}  // namespace crubit
