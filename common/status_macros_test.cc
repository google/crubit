// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/status_macros.h"

#include <memory>
#include <type_traits>
#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "common/status.h"
#include "llvm/Support/Error.h"

namespace crubit {
namespace {

template <typename Ret, typename Expr>
Ret PropagateError(Expr expr) {
  CRUBIT_RETURN_IF_ERROR(std::move(expr));
  if constexpr (std::is_same_v<Ret, absl::Status>) {
    return absl::OkStatus();
  } else {
    return llvm::Error::success();
  }
}

template <typename Ret, typename Expr>
Ret AssignAndReturn(Expr expr) {
  CRUBIT_ASSIGN_OR_RETURN(auto x, std::move(expr));
  return x;
}

TEST(StatusMacrosTest, ReturnIfError) {
  absl::Status s1 =
      PropagateError<absl::Status>(absl::InvalidArgumentError("absl error"));
  EXPECT_EQ(s1.code(), absl::StatusCode::kInvalidArgument);
  EXPECT_EQ(s1.message(), "absl error");

  absl::Status s2 = PropagateError<absl::Status>(
      MakeStatusError(StatusCode::kInternal, "llvm error"));
  EXPECT_EQ(s2.code(), absl::StatusCode::kInternal);
  EXPECT_EQ(s2.message(), "llvm error");

  llvm::Error e1 =
      PropagateError<llvm::Error>(absl::InvalidArgumentError("absl error"));
  EXPECT_EQ(GetStatusCode(e1), StatusCode::kInvalidArgument);
  EXPECT_EQ(GetStatusMessage(e1), "absl error");
  llvm::consumeError(std::move(e1));

  llvm::Error e2 = PropagateError<llvm::Error>(
      MakeStatusError(StatusCode::kInternal, "llvm error"));
  EXPECT_EQ(GetStatusCode(e2), StatusCode::kInternal);
  EXPECT_EQ(GetStatusMessage(e2), "llvm error");
  llvm::consumeError(std::move(e2));
}

TEST(StatusMacrosTest, AssignOrReturn) {
  auto s1 = AssignAndReturn<absl::StatusOr<int>>(
      absl::StatusOr<int>(absl::NotFoundError("absl or error")));
  EXPECT_EQ(s1.status().code(), absl::StatusCode::kNotFound);
  EXPECT_EQ(s1.status().message(), "absl or error");

  auto s2 = AssignAndReturn<absl::StatusOr<int>>(llvm::Expected<int>(
      MakeStatusError(StatusCode::kFailedPrecondition, "llvm expected error")));
  EXPECT_EQ(s2.status().code(), absl::StatusCode::kFailedPrecondition);
  EXPECT_EQ(s2.status().message(), "llvm expected error");

  auto e1 = AssignAndReturn<llvm::Expected<int>>(
      absl::StatusOr<int>(absl::NotFoundError("absl or error")));
  EXPECT_EQ(GetStatusCode(e1.takeError()), StatusCode::kNotFound);

  auto e2 = AssignAndReturn<llvm::Expected<int>>(llvm::Expected<int>(
      MakeStatusError(StatusCode::kFailedPrecondition, "llvm expected error")));
  EXPECT_EQ(GetStatusCode(e2.takeError()), StatusCode::kFailedPrecondition);
}

TEST(StatusMacrosTest, ReturnIfErrorSuccess) {
  EXPECT_TRUE(PropagateError<absl::Status>(absl::OkStatus()).ok());

  llvm::Error e = PropagateError<llvm::Error>(llvm::Error::success());
  EXPECT_FALSE(e);
  llvm::consumeError(std::move(e));
}

TEST(StatusMacrosTest, AssignOrReturnSuccess) {
  absl::StatusOr<int> s =
      AssignAndReturn<absl::StatusOr<int>>(absl::StatusOr<int>(42));
  ASSERT_TRUE(s.ok());
  EXPECT_EQ(*s, 42);

  llvm::Expected<int> e =
      AssignAndReturn<llvm::Expected<int>>(llvm::Expected<int>(42));
  ASSERT_TRUE(static_cast<bool>(e));
  EXPECT_EQ(*e, 42);

  llvm::Expected<std::unique_ptr<int>> m =
      AssignAndReturn<llvm::Expected<std::unique_ptr<int>>>(
          llvm::Expected<std::unique_ptr<int>>(std::make_unique<int>(42)));
  ASSERT_TRUE(static_cast<bool>(m));
  EXPECT_EQ(**m, 42);
}

TEST(StatusMacrosTest, ReturnIfErrorNonStatusError) {
  absl::Status s = PropagateError<absl::Status>(llvm::createStringError(
      llvm::inconvertibleErrorCode(), "raw llvm error"));
  EXPECT_FALSE(s.ok());
  EXPECT_EQ(s.code(), absl::StatusCode::kUnknown);
  EXPECT_THAT(s.message(), testing::HasSubstr("raw llvm error"));
}

}  // namespace
}  // namespace crubit
