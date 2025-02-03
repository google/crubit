// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/bridging/string/string.h"

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

}  // namespace
}  // namespace crubit