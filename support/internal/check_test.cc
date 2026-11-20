// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/internal/check.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace {

using ::testing::AllOf;
using ::testing::HasSubstr;
using ::testing::Not;

TEST(CheckTest, CheckSuccess) {
  CRUBIT_CHECK(true) << "Should not see this";
  CRUBIT_CHECK(1 == 1);
}

TEST(CheckTest, CheckDeath) {
#ifdef __FILE_NAME__
  EXPECT_DEATH_IF_SUPPORTED(
      CRUBIT_CHECK(false) << "Death message",
      AllOf(HasSubstr("check_test.cc:"),
            HasSubstr("Check failed: false Death message"),
            Not(HasSubstr("/check_test.cc:")),
            Not(HasSubstr("\\check_test.cc:"))));
#else
  EXPECT_DEATH_IF_SUPPORTED(CRUBIT_CHECK(false) << "Death message",
                            HasSubstr("Check failed: false Death message"));
#endif
}

TEST(CheckTest, DcheckEvaluation) {
  int i = 0;
  CRUBIT_DCHECK(++i == 1);
#ifndef NDEBUG
  EXPECT_EQ(i, 1);
#else
  EXPECT_EQ(i, 0);
#endif
}

}  // namespace
