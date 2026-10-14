// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/bridging/string/string.h"

#include <optional>
#include <string>

#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(StdStringBridging, Basic) {
  std::string s = "hello world";
  auto expected = string::roundtrip_string(s);
  static_assert(std::is_same_v<std::string, decltype(expected)>);
  EXPECT_EQ(expected, s);

  EXPECT_EQ(string::create_from_rust(), std::string("hello world from Rust"));
  EXPECT_EQ(string::compute_string_length("hello world"), 11);
}

TEST(StdStringBridging, LayoutCompatStringInOptionalIsCppMovable) {
  std::optional<std::string> opt =
      string::roundtrip_optional_layout_compat_string("hello world");
  ASSERT_TRUE(opt.has_value());
  EXPECT_EQ(*opt, "hello world");
  EXPECT_EQ(string::compute_optional_layout_compat_string_length(opt), 11);
  EXPECT_EQ(string::compute_optional_layout_compat_string_length(std::nullopt),
            0);
}

}  // namespace
}  // namespace crubit