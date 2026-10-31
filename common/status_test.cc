// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/status.h"

#include <string>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "llvm/Support/Error.h"

namespace crubit {
namespace {

TEST(StatusTest, StatusCodeToString) {
  EXPECT_EQ(StatusCodeToString(StatusCode::kOk), "OK");
  EXPECT_EQ(StatusCodeToString(StatusCode::kUnknown), "UNKNOWN");
  EXPECT_EQ(StatusCodeToString(StatusCode::kInvalidArgument),
            "INVALID_ARGUMENT");
}

TEST(StatusTest, StatusErrorConstruction) {
  llvm::Error err = MakeStatusError(StatusCode::kNotFound, "not found msg");
  EXPECT_EQ(GetStatusCode(err), StatusCode::kNotFound);
  EXPECT_EQ(GetStatusMessage(err), "not found msg");

  std::string str = llvm::toString(std::move(err));
  EXPECT_EQ(str, "not found msg");
}

TEST(StatusTest, GetStatusCodePlainStringError) {
  llvm::Error err =
      llvm::createStringError(llvm::inconvertibleErrorCode(), "plain string");
  EXPECT_EQ(GetStatusCode(err), StatusCode::kUnknown);
  EXPECT_EQ(GetStatusMessage(err), "plain string");
  llvm::consumeError(std::move(err));
}

TEST(StatusTest, Helpers) {
  llvm::Error err = InvalidArgumentError("invalid");
  EXPECT_EQ(GetStatusCode(err), StatusCode::kInvalidArgument);
  llvm::consumeError(std::move(err));
}

}  // namespace
}  // namespace crubit
