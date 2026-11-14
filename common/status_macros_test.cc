// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/status_macros.h"

#include <memory>
#include <utility>

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

llvm::Error TestReturnIfError_Llvm_Success() {
  CRUBIT_RETURN_IF_ERROR(llvm::Error::success());
  return llvm::Error::success();
}

llvm::Expected<int> TestAssignOrReturn_Llvm_Success() {
  CRUBIT_ASSIGN_OR_RETURN(int x, llvm::Expected<int>(42));
  return x;
}

llvm::Expected<std::unique_ptr<int>> TestAssignOrReturn_MoveOnly() {
  CRUBIT_ASSIGN_OR_RETURN(
      std::unique_ptr<int> x,
      llvm::Expected<std::unique_ptr<int>>(std::make_unique<int>(42)));
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

TEST(StatusMacrosTest, ReturnIfErrorSuccess) {
  llvm::Error e = TestReturnIfError_Llvm_Success();
  EXPECT_FALSE(e);
  llvm::consumeError(std::move(e));
}

TEST(StatusMacrosTest, AssignOrReturnSuccess) {
  llvm::Expected<int> e = TestAssignOrReturn_Llvm_Success();
  ASSERT_TRUE(static_cast<bool>(e));
  EXPECT_EQ(*e, 42);

  llvm::Expected<std::unique_ptr<int>> m = TestAssignOrReturn_MoveOnly();
  ASSERT_TRUE(static_cast<bool>(m));
  EXPECT_EQ(**m, 42);
}

}  // namespace
}  // namespace crubit
