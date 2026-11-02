// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/logging.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(LoggingTest, MinLogSeverity) {
  SetMinLogSeverity(LogSeverity::kWarning);
  EXPECT_EQ(MinLogSeverity(), LogSeverity::kWarning);
  EXPECT_TRUE(ShouldLog(LogSeverity::kWarning));
  EXPECT_TRUE(ShouldLog(LogSeverity::kError));
  EXPECT_FALSE(ShouldLog(LogSeverity::kInfo));
  SetMinLogSeverity(LogSeverity::kError);  // Restore
}

}  // namespace
}  // namespace crubit
