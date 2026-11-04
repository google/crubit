// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/logging.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(LoggingTest, StderrThreshold) {
  SetStderrThreshold(LogSeverity::kWarning);
  EXPECT_EQ(StderrThreshold(), LogSeverity::kWarning);
  EXPECT_TRUE(ShouldLog(LogSeverity::kWarning));
  EXPECT_TRUE(ShouldLog(LogSeverity::kError));
  EXPECT_FALSE(ShouldLog(LogSeverity::kInfo));
  SetStderrThreshold(LogSeverity::kError);  // Restore
}

}  // namespace
}  // namespace crubit
