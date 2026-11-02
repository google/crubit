// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/status_macros.h"

#include <string>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "common/status.h"
#include "llvm/Support/Error.h"

namespace crubit {
namespace {

llvm::Error ReturnsLlvmError() {
  return MakeStatusError(StatusCode::kInternal, "llvm error");
}

llvm::Expected<int> ReturnsLlvmExpectedError() {
  return MakeStatusError(StatusCode::kFailedPrecondition,
                         "llvm expected error");
}

llvm::Error TestReturnIfError_Llvm_To_Llvm() {
  CRUBIT_RETURN_IF_ERROR(ReturnsLlvmError());
  return llvm::Error::success();
}

llvm::Expected<int> TestAssignOrReturn_Llvm_To_Llvm() {
  CRUBIT_ASSIGN_OR_RETURN(int x, ReturnsLlvmExpectedError());
  return x;
}

TEST(StatusMacrosTest, ReturnIfError) {
  llvm::Error e2 = TestReturnIfError_Llvm_To_Llvm();
  EXPECT_EQ(GetStatusCode(e2), StatusCode::kInternal);
  EXPECT_EQ(GetStatusMessage(e2), "llvm error");
  llvm::consumeError(std::move(e2));
}

TEST(StatusMacrosTest, AssignOrReturn) {
  llvm::Expected<int> e2 = TestAssignOrReturn_Llvm_To_Llvm();
  EXPECT_EQ(GetStatusCode(e2.takeError()), StatusCode::kFailedPrecondition);
}

}  // namespace
}  // namespace crubit
