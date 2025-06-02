// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/status_bridge.h"

#include "crubit/support/bridge.h"
#include "gtest/gtest.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"

namespace crubit::bridge {
namespace {

TEST(BridgeTest, RoundtripOkStatus) {
  using Abi = StatusAbi;

  absl::Status original = absl::OkStatus();

  unsigned char buf[Abi::kSize];
  internal::Encode<Abi>(buf, original);
  absl::Status value = internal::Decode<Abi>(buf);
  EXPECT_EQ(value, original);
}

TEST(BridgeTest, RoundtripErrStatus) {
  using Abi = StatusAbi;

  absl::Status original = absl::InternalError("test");

  unsigned char buf[Abi::kSize];
  internal::Encode<Abi>(buf, original);
  absl::Status value = internal::Decode<Abi>(buf);
  EXPECT_EQ(value, original);
}

TEST(BridgeTest, RoundtripOkStatusOr) {
  using Abi = StatusOrAbi<TransmuteAbi<int>>;

  absl::StatusOr<int> original = 123;

  unsigned char buf[Abi::kSize];
  internal::Encode<Abi>(buf, original);
  absl::StatusOr<int> value = internal::Decode<Abi>(buf);
  EXPECT_EQ(value, original);
}

TEST(BridgeTest, RoundtripErrStatusOr) {
  using Abi = StatusOrAbi<TransmuteAbi<int>>;

  absl::StatusOr<int> original = absl::InternalError("test");

  unsigned char buf[Abi::kSize];
  internal::Encode<Abi>(buf, original);
  absl::StatusOr<int> value = internal::Decode<Abi>(buf);
  EXPECT_EQ(value, original);
}

}  // namespace
}  // namespace crubit::bridge
