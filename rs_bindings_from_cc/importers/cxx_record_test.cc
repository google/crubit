// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/cxx_record.h"

#include <string>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/strings/string_view.h"

namespace crubit {
namespace {

using ::testing::Eq;

TEST(CxxRecordTest, GetProto2MessageRustNameImpl_NoNesting) {
  auto is_parent = [](absl::string_view) { return false; };
  EXPECT_THAT(internal::GetProto2MessageRustNameImpl("MyMessage", is_parent),
              Eq("MyMessage"));
}

TEST(CxxRecordTest, GetProto2MessageRustNameImpl_NestedOnce) {
  auto is_parent = [](absl::string_view prefix) { return prefix == "Outer"; };
  EXPECT_THAT(internal::GetProto2MessageRustNameImpl("Outer_Inner", is_parent),
              Eq("outer::Inner"));
}

TEST(CxxRecordTest, GetProto2MessageRustNameImpl_NestedTwice) {
  auto is_parent = [](absl::string_view prefix) {
    return prefix == "Outer" || prefix == "Outer_Inner";
  };
  EXPECT_THAT(
      internal::GetProto2MessageRustNameImpl("Outer_Inner_Deep", is_parent),
      Eq("outer::inner::Deep"));
}

TEST(CxxRecordTest, GetProto2MessageRustNameImpl_UnderscoreInName) {
  // A_B is a message, and A_B_C_D is a nested message.
  auto is_parent = [](absl::string_view prefix) { return prefix == "A_B"; };
  EXPECT_THAT(internal::GetProto2MessageRustNameImpl("A_B_C_D", is_parent),
              Eq("a_b::C_D"));
}

}  // namespace
}  // namespace crubit
