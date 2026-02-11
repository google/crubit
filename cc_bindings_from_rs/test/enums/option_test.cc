// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "cc_bindings_from_rs/test/enums/option.h"

#include <optional>

#include "gtest/gtest.h"

namespace {

TEST(OptionTest, OptionIsConvertibleToStd) {
  option::HasOptions has_options = option::HasOptions::new_(1);
  std::optional<option::NonMaxU8> opt_a = has_options.a.take_optional();
  EXPECT_TRUE(opt_a.has_value());
  EXPECT_EQ(opt_a.value().value(), 1);
  std::optional<rs_std::Option<option::NonMaxU8>> opt_b =
      has_options.b.take_optional();
  EXPECT_TRUE(opt_b.has_value());
  EXPECT_FALSE(has_options.b.take_optional().has_value());
  std::optional<option::NonMaxU8> opt_b_inner = opt_b.value().take_optional();
  EXPECT_TRUE(opt_b_inner.has_value());
  EXPECT_EQ(opt_b_inner.value().value(), 1);

  std::optional<std::uint8_t> opt_c = has_options.c.take_optional();
  EXPECT_TRUE(opt_c.has_value());
  EXPECT_EQ(opt_c.value(), 1);
}

TEST(OptionTest, OptionNoneIsNullOpt) {
  option::HasOptions has_options_none = option::HasOptions::with_none();
  EXPECT_FALSE(has_options_none.a.take_optional().has_value());
  EXPECT_FALSE(has_options_none.b.take_optional().has_value());
  EXPECT_FALSE(has_options_none.c.take_optional().has_value());
}

TEST(OptionTest, TakeOptionalEvictsValue) {
  option::HasOptions has_options = option::HasOptions::new_(123);
  std::optional<option::NonMaxU8> opt_a = has_options.a.take_optional();
  EXPECT_TRUE(opt_a.has_value());

  // Our `a` field should be evicted by the call `take_optional`.
  std::optional<option::NonMaxU8> opt_a_retake = has_options.a.take_optional();
  EXPECT_FALSE(opt_a_retake.has_value());
}

TEST(OptionTest, ConstructFromOption) {
  std::optional<std::uint8_t> some_u8 =
      std::make_optional(static_cast<std::uint8_t>(42));
  option::HasOptions has_options = option::HasOptions::with_option(some_u8);
  std::optional<std::uint8_t> some_u8_retaken = has_options.c.take_optional();
  EXPECT_TRUE(some_u8_retaken.has_value());
  EXPECT_EQ(some_u8_retaken.value(), 42);
}

TEST(OptionTest, HasHasOptionTest) {
  option::HasHasOptions has_has_options = option::HasHasOptions::new_(42);
  std::optional<option::HasOptions> has_options =
      has_has_options.me.take_optional();
  EXPECT_TRUE(has_options.has_value());
  EXPECT_TRUE(has_options.value().a.take_optional().has_value());
  EXPECT_TRUE(has_options.value().b.take_optional().has_value());
  EXPECT_TRUE(has_options.value().c.take_optional().has_value());
}

}  // namespace
