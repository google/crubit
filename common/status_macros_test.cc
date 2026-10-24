// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/status_macros.h"

#include <string>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "common/status.h"
#include "llvm/Support/Error.h"

namespace crubit {
namespace {

absl::Status ReturnsAbslStatusError() {
  return absl::InvalidArgumentError("absl error");
}

absl::StatusOr<int> ReturnsAbslStatusOrError() {
  return absl::NotFoundError("absl or error");
}

llvm::Error ReturnsLlvmError() {
  return MakeStatusError(StatusCode::kInternal, "llvm error");
}

llvm::Expected<int> ReturnsLlvmExpectedError() {
  return MakeStatusError(StatusCode::kFailedPrecondition,
                         "llvm expected error");
}

absl::Status TestReturnIfError_Absl_To_Absl() {
  CRUBIT_RETURN_IF_ERROR(ReturnsAbslStatusError());
  return absl::OkStatus();
}

absl::Status TestReturnIfError_Llvm_To_Absl() {
  CRUBIT_RETURN_IF_ERROR(ReturnsLlvmError());
  return absl::OkStatus();
}

llvm::Error TestReturnIfError_Absl_To_Llvm() {
  CRUBIT_RETURN_IF_ERROR(ReturnsAbslStatusError());
  return llvm::Error::success();
}

llvm::Error TestReturnIfError_Llvm_To_Llvm() {
  CRUBIT_RETURN_IF_ERROR(ReturnsLlvmError());
  return llvm::Error::success();
}

absl::StatusOr<int> TestAssignOrReturn_Absl_To_Absl() {
  CRUBIT_ASSIGN_OR_RETURN(int x, ReturnsAbslStatusOrError());
  return x;
}

absl::StatusOr<int> TestAssignOrReturn_Llvm_To_Absl() {
  CRUBIT_ASSIGN_OR_RETURN(int x, ReturnsLlvmExpectedError());
  return x;
}

llvm::Expected<int> TestAssignOrReturn_Absl_To_Llvm() {
  CRUBIT_ASSIGN_OR_RETURN(int x, ReturnsAbslStatusOrError());
  return x;
}

llvm::Expected<int> TestAssignOrReturn_Llvm_To_Llvm() {
  CRUBIT_ASSIGN_OR_RETURN(int x, ReturnsLlvmExpectedError());
  return x;
}

TEST(StatusMacrosTest, ReturnIfError) {
  absl::Status s1 = TestReturnIfError_Absl_To_Absl();
  EXPECT_EQ(s1.code(), absl::StatusCode::kInvalidArgument);
  EXPECT_EQ(s1.message(), "absl error");

  absl::Status s2 = TestReturnIfError_Llvm_To_Absl();
  EXPECT_EQ(s2.code(), absl::StatusCode::kInternal);
  EXPECT_EQ(s2.message(), "llvm error");

  llvm::Error e1 = TestReturnIfError_Absl_To_Llvm();
  EXPECT_EQ(GetStatusCode(e1), StatusCode::kInvalidArgument);
  EXPECT_EQ(GetStatusMessage(e1), "absl error");
  llvm::consumeError(std::move(e1));

  llvm::Error e2 = TestReturnIfError_Llvm_To_Llvm();
  EXPECT_EQ(GetStatusCode(e2), StatusCode::kInternal);
  EXPECT_EQ(GetStatusMessage(e2), "llvm error");
  llvm::consumeError(std::move(e2));
}

TEST(StatusMacrosTest, AssignOrReturn) {
  absl::StatusOr<int> s1 = TestAssignOrReturn_Absl_To_Absl();
  EXPECT_EQ(s1.status().code(), absl::StatusCode::kNotFound);
  EXPECT_EQ(s1.status().message(), "absl or error");

  absl::StatusOr<int> s2 = TestAssignOrReturn_Llvm_To_Absl();
  EXPECT_EQ(s2.status().code(), absl::StatusCode::kFailedPrecondition);
  EXPECT_EQ(s2.status().message(), "llvm expected error");

  llvm::Expected<int> e1 = TestAssignOrReturn_Absl_To_Llvm();
  EXPECT_EQ(GetStatusCode(e1.takeError()), StatusCode::kNotFound);

  llvm::Expected<int> e2 = TestAssignOrReturn_Llvm_To_Llvm();
  EXPECT_EQ(GetStatusCode(e2.takeError()), StatusCode::kFailedPrecondition);
}

}  // namespace
}  // namespace crubit
