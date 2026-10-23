// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Declare `namespace rs::std` before the generated bindings headers (as
// `rs_std.h` does when Rust `std` types are bound) so `namespace rs`
// specializations and declarations are checked with `rs::std` in scope,
// ensuring C++ standard library types are root-qualified as `::std`.
namespace rs::std {}

#include "cc_bindings_from_rs/test/bridging/string/string.h"

#include <optional>
#include <string>
#include <string_view>
#include <type_traits>

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/bridging/string/rs_namespace_string.h"
#include "support/rs_std/option.h"

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

TEST(StdStringBridging, OptionalStringViewCoexistsWithRsStdNamespace) {
  rs::Option<std::string_view> present =
      string::roundtrip_optional_string_view("hello view");
  ASSERT_TRUE(present.has_value());
  EXPECT_EQ(*present, "hello view");

  rs::Option<std::string_view> absent =
      string::roundtrip_optional_string_view(std::nullopt);
  EXPECT_FALSE(absent.has_value());
}

TEST(StdStringBridging, StringWrapperInRsNamespaceCoexistsWithRsStdNamespace) {
  std::string result =
      rs::string_bridging_test::roundtrip_string_in_rs_namespace(
          "hello from rs namespace");
  EXPECT_EQ(result, "hello from rs namespace");
}

}  // namespace
}  // namespace crubit
