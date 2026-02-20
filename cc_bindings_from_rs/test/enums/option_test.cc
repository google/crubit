// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "cc_bindings_from_rs/test/enums/option.h"

#include <optional>

#include "gtest/gtest.h"

namespace {

TEST(OptionTest, OptionWithNicheIsConvertibleToStd) {
  option::HasOptions has_options = option::HasOptions::new_(1);
  std::optional<option::NonMaxU8> opt_niche = std::move(has_options.niche);
  EXPECT_TRUE(opt_niche.has_value());
  EXPECT_EQ(opt_niche.value().value(), 1);
}

TEST(OptionTest, NestedOptionIsConvertibleToStd) {
  option::HasOptions has_options = option::HasOptions::new_(1);
  std::optional<rs_std::Option<option::NonMaxU8>> opt_nested =
      std::move(has_options.nested);
  EXPECT_TRUE(opt_nested.has_value());
  EXPECT_FALSE((static_cast<std::optional<rs_std::Option<option::NonMaxU8>>>(
                    std::move(has_options.nested)))
                   .has_value());
  std::optional<option::NonMaxU8> opt_nested_inner =
      std::move(opt_nested.value());
  EXPECT_TRUE(opt_nested_inner.has_value());
  EXPECT_EQ(opt_nested_inner.value().value(), 1);
}

TEST(OptionTest, OptionWithDirectTagIsConvertibleToStd) {
  option::HasOptions has_options = option::HasOptions::new_(1);
  std::optional<uint8_t> opt_direct = std::move(has_options.direct);
  EXPECT_TRUE(opt_direct.has_value());
  EXPECT_EQ(opt_direct.value(), 1);
}

TEST(OptionTest, OptionNoneIsNullOpt) {
  option::HasOptions has_options_none = option::HasOptions::with_none();
  std::optional<option::NonMaxU8> opt_niche = std::move(has_options_none.niche);
  EXPECT_FALSE(opt_niche.has_value());
  std::optional<rs_std::Option<option::NonMaxU8>> opt_nested =
      std::move(has_options_none.nested);
  EXPECT_FALSE(opt_nested.has_value());
  std::optional<uint8_t> opt_direct = std::move(has_options_none.direct);
  EXPECT_FALSE(opt_direct.has_value());
}

TEST(OptionTest, MoveIntoOptionalSetsOptionToNone) {
  option::HasOptions has_options = option::HasOptions::new_(100);
  std::optional<option::NonMaxU8> opt_niche = std::move(has_options.niche);
  EXPECT_TRUE(opt_niche.has_value());
  EXPECT_EQ(opt_niche.value().value(), 100);
  std::optional<option::NonMaxU8> opt_niche_retake =
      std::move(has_options.niche);
  EXPECT_FALSE(opt_niche_retake.has_value());
}

TEST(OptionTest, ConstructFromOption) {
  std::optional<uint8_t> some_u8 = std::make_optional(uint8_t{42});
  option::HasOptions has_options = option::HasOptions::with_option(some_u8);
  std::optional<uint8_t> some_u8_retaken = std::move(has_options.direct);
  EXPECT_EQ(some_u8_retaken.has_value(), some_u8.has_value());
  EXPECT_EQ(some_u8_retaken.value(), some_u8.value());
}

TEST(OptionTest, StructWithNicheIsConvertibleToStd) {
  option::HasHasOptions has_has_options = option::HasHasOptions::new_(42);
  std::optional<option::HasOptions> has_options = std::move(has_has_options.me);
  EXPECT_TRUE(has_options.has_value());
  std::optional<option::NonMaxU8> opt_niche =
      std::move(has_options.value().niche);
  EXPECT_TRUE(opt_niche.has_value());
  std::optional<rs_std::Option<option::NonMaxU8>> opt_nested =
      std::move(has_options.value().nested);
  EXPECT_TRUE(opt_nested.has_value());
  std::optional<uint8_t> opt_direct = std::move(has_options.value().direct);
  EXPECT_TRUE(opt_direct.has_value());
}

TEST(OptionTest, OptNoDefaultWithDrop) {
  option::OptNoDefaultWithDrop x = option::OptNoDefaultWithDrop::new_("world");
  std::optional<option::HasNoDefault> val = std::move(x.val);
  EXPECT_TRUE(val.has_value());
  EXPECT_EQ(val->a, 1045);
  EXPECT_EQ(val->get_string_inside_option().to_string_view(), "world");
}

TEST(OptionTest, OptionHasNoDefaultDefaultConstruct) {
  rs_std::Option<option::HasNoDefault> val;
  EXPECT_FALSE(val.has_value());
}

TEST(OptionTest, OptionHasNoDefaultNullOptConstruct) {
  rs_std::Option<option::HasNoDefault> val(std::nullopt);
  EXPECT_FALSE(val.has_value());
}

TEST(OptionTest, OptionHasNoDefaultNullOptAssign) {
  rs_std::Option<option::HasNoDefault> val;
  val = std::nullopt;
  EXPECT_FALSE(val.has_value());
}

TEST(OptionTest, OptionHasDefaultValueConstruct) {
  rs_std::Option<option::HasDefault> x(option::HasDefault::new_("has default"));
  EXPECT_TRUE(x.has_value());
  std::optional<option::HasDefault> val = std::move(x);
  EXPECT_TRUE(val.has_value());
  EXPECT_EQ(val->get_string_inside_option().to_string_view(), "has default");
}

TEST(OptionTest, OptionMoveConstructAndAssignFromOption) {
  rs_std::Option<option::HasNoDefault> x(std::nullopt);
  rs_std::Option<option::HasNoDefault> y(std::move(x));
  rs_std::Option<option::HasNoDefault> z;
  z = std::move(y);
  EXPECT_FALSE(z.has_value());
}

TEST(OptionTest, OptionHasDefaultValueAssign) {
  rs_std::Option<option::HasDefault> x;
  x = option::HasDefault::new_("hello");
  EXPECT_TRUE(x.has_value());
  std::optional<option::HasDefault> val = std::move(x);
  EXPECT_TRUE(val.has_value());
  EXPECT_EQ(val->get_string_inside_option().to_string_view(), "hello");
}

TEST(OptionTest, OptionHasNoDefaultOptionConstruct) {
  std::optional<option::HasDefault> some_has_default(
      std::in_place, ::option::HasDefault::new_("hello"));
  rs_std::Option<option::HasDefault> x(std::move(some_has_default));
  EXPECT_TRUE(x.has_value());
  std::optional<option::HasDefault> val = std::move(x);
  EXPECT_TRUE(val.has_value());
  EXPECT_EQ(val->get_string_inside_option().to_string_view(), "hello");
}

TEST(OptionTest, OptionHasNoDefaultOptionAssign) {
  std::optional<option::HasDefault> some_has_default(
      std::in_place, ::option::HasDefault::new_("hello"));
  rs_std::Option<option::HasDefault> x;
  x = std::move(some_has_default);
  EXPECT_TRUE(x.has_value());
  std::optional<option::HasDefault> val = std::move(x);
  EXPECT_TRUE(val.has_value());
  EXPECT_EQ(val->get_string_inside_option().to_string_view(), "hello");
}

TEST(OptionTest, OptDefaultWithDrop) {
  option::OptDefaultWithDrop x = option::OptDefaultWithDrop::new_("berry");
  std::optional<option::HasDefault> val = std::move(x.opt);
  EXPECT_TRUE(val.has_value());
  EXPECT_EQ(val->get_string_inside_option().to_string_view(), "berry");
}

TEST(OptionTest, OptCopyNoDefault) {
  option::OptCopyNoDefault x = option::OptCopyNoDefault::new_(123);
  std::optional<option::CopyNoDefault> val = std::move(x.val);
  EXPECT_TRUE(val.has_value());
  EXPECT_EQ(val->val, 123);
}

TEST(OptionTest, OptCloneNoDefault) {
  option::OptCloneNoDefault x = option::OptCloneNoDefault::new_(74);
  std::optional<option::CloneNoDefault> val = std::move(x.val);
  EXPECT_TRUE(val.has_value());
  EXPECT_EQ(val->val, 74);
}

TEST(OptionTest, PassingOptionAsReferenceArgument) {
  rs_std::Option<option::HasDefault> x = option::HasDefault::new_("hello");
  std::optional<uintptr_t> y = option::stringify_len(x);
  EXPECT_TRUE(y.has_value());
  EXPECT_EQ(y.value(), 5);
}

}  // namespace