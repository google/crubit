// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstdint>
#include <string>
#include <variant>

#include "gtest/gtest.h"
#include "support/cc_std_impl/test/variant/rust_variant_lib.h"
#include "support/rs_std/char.h"

namespace {

TEST(VariantTest, TrivialVariant) {
  std::variant<int, float, bool> v = rust_variant_lib::get_trivial_variant();
  EXPECT_TRUE(std::holds_alternative<float>(v));
  EXPECT_EQ(std::get<float>(v), 3.5f);
  EXPECT_TRUE(rust_variant_lib::check_trivial_variant(v));
}

TEST(VariantTest, SmallAlignmentVariant) {
  std::variant<uint8_t, bool> v = rust_variant_lib::get_small_variant();
  EXPECT_TRUE(std::holds_alternative<bool>(v));
  EXPECT_TRUE(std::get<bool>(v));
  EXPECT_TRUE(rust_variant_lib::check_small_variant(v));
}

TEST(VariantTest, NonTrivialVariant) {
  std::variant<int, std::string> v = std::string("hello");
  EXPECT_TRUE(rust_variant_lib::check_non_trivial_variant(v));
}

TEST(VariantTest, SeventeenVariant) {
  std::variant<uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t, int8_t,
               int16_t, int32_t, int64_t, intptr_t, float, double, bool,
               rs::char_, const uint8_t*, uint8_t*, int32_t>
      v = rust_variant_lib::get_seventeen_variant();
  EXPECT_EQ(v.index(), 16);
  EXPECT_EQ(std::get<16>(v), 42);
  EXPECT_TRUE(rust_variant_lib::check_seventeen_variant(v));
}

}  // namespace
