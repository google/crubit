// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/rs_namespace/rs_namespace.h"

#include <cstdint>
#include <optional>
#include <tuple>

#include "gtest/gtest.h"

namespace {

TEST(RsNamespaceTest, StrRef) {
  EXPECT_EQ(test_ns::rs::str_len(::rs::StrRef("hello")), 5);
  EXPECT_EQ(test_ns::rs::identity_str(::rs::StrRef("hi")).to_string_view(),
            "hi");
}

TEST(RsNamespaceTest, SliceRef) {
  int32_t values[] = {1, 2, 3};
  EXPECT_EQ(test_ns::rs::slice_sum(::rs::SliceRef<const int32_t>(values)), 6);
}

TEST(RsNamespaceTest, Tuple) {
  EXPECT_EQ(test_ns::rs::swap_tuple(std::make_tuple(1, 2)),
            std::make_tuple(2, 1));
}

TEST(RsNamespaceTest, Option) {
  EXPECT_EQ(test_ns::rs::double_option(std::optional<int32_t>(21)),
            std::optional<int32_t>(42));
  EXPECT_EQ(test_ns::rs::double_option(std::nullopt), std::nullopt);
}

}  // namespace
