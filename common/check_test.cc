// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/check.h"

#include "gtest/gtest.h"

namespace {

TEST(CheckTest, CheckSuccess) {
  CRUBIT_CHECK(true) << "Should not see this";
  CRUBIT_CHECK(1 == 1);
}

TEST(CheckTest, CheckDeath) {
  EXPECT_DEATH(CRUBIT_CHECK(false) << "Death message",
               "Check failed: false Death message");
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

TEST(CheckTest, DieIfNullSuccess) {
  int i = 42;
  int* ptr = &i;
  EXPECT_EQ(CRUBIT_DIE_IF_NULL(ptr), ptr);
}

TEST(CheckTest, DieIfNullDeath) {
  int* ptr = nullptr;
  EXPECT_DEATH(CRUBIT_DIE_IF_NULL(ptr), "Must not be null");
}

}  // namespace
