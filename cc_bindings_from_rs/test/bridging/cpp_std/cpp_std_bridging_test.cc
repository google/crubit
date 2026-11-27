// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/bridging/cpp_std/cpp_std_bridging.h"

#include <memory>
#include <optional>
#include <string_view>
#include <vector>

#include "gtest/gtest.h"

namespace {

TEST(CppStdBridgingTest, TakeCallback) {
  bool called = false;
  std::shared_ptr<int> res = cpp_std_bridging::take_callback(
      [&](std::string_view sv) -> std::shared_ptr<int> {
        called = true;
        EXPECT_EQ(sv, "hello");
        return std::make_shared<int>(42);
      },
      "hello");

  EXPECT_TRUE(called);
  EXPECT_NE(res, nullptr);
  EXPECT_EQ(*res, 42);
}

TEST(CppStdBridgingTest, OptionStringView) {
  EXPECT_EQ(cpp_std_bridging::take_option_string_view(std::string_view("foo")),
            "some");
  EXPECT_EQ(cpp_std_bridging::take_option_string_view(std::nullopt), "none");
}

TEST(CppStdBridgingTest, StringWrapper) {
  EXPECT_EQ(cpp_std_bridging::return_string_wrapper(), "hello");
}

TEST(CppStdBridgingTest, Vector) {
  std::vector<int> v = {1, 2, 3};
  std::vector<int> res = cpp_std_bridging::roundtrip_vector(v);
  EXPECT_EQ(res, v);
}

TEST(CppStdBridgingTest, Optional) {
  std::optional<int> v = 42;
  std::optional<int> res = cpp_std_bridging::roundtrip_optional(v);
  EXPECT_EQ(res, v);
}

}  // namespace
