// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/string/string.h"

#include <utility>

#include "gtest/gtest.h"
#include "support/rs_std/rs_std.h"

namespace crubit {
namespace {

TEST(RustStringBridging, Basic) {
  rs::std::string::String s("hello world");
  EXPECT_EQ(string::compute_rust_string_length(std::move(s)), 11);

  rs::std::string::String s2("hello world from Rust");
  rs::std::string::String roundtripped =
      string::roundtrip_rust_string(std::move(s2));
  EXPECT_EQ(string::compute_rust_string_length(std::move(roundtripped)), 21);
}

TEST(RustStringBridging, References) {
  rs::std::string::String s("hello");
  EXPECT_EQ(string::compute_rust_string_ref_length(s), 5);

  string::append_to_rust_string(s, " appended");
  EXPECT_EQ(string::compute_rust_string_ref_length(s), 14);
  EXPECT_EQ(s.as_str(), "hello appended");
}

}  // namespace
}  // namespace crubit
