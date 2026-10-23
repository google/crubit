// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "cc_bindings_from_rs/test/enums/option.h"

#include <cstdint>
#include <optional>
#include <utility>

#include "gtest/gtest.h"
#include "support/rs_std/rs_std.h"

namespace {

TEST(OptionTest, OptionWithNicheIsConvertibleToStd) {
  option::HasOptions has_options = option::HasOptions::new_(1);
  ASSERT_TRUE(has_options.niche.has_value());
  EXPECT_EQ(has_options.niche->value(), 1);
  std::optional<option::LessThan20U8> opt_niche = std::move(has_options.niche);
  ASSERT_TRUE(opt_niche.has_value());
  EXPECT_EQ(opt_niche->value(), 1);
}

TEST(OptionTest, NestedOptionIsConvertibleToStd) {
  option::HasOptions has_options = option::HasOptions::new_(1);
  std::optional<rs::Option<option::LessThan20U8>> opt_nested =
      std::move(has_options.nested);
  ASSERT_TRUE(opt_nested.has_value());
  EXPECT_EQ((static_cast<std::optional<rs::Option<option::LessThan20U8>>>(
                std::move(has_options.nested))),
            std::nullopt);
  ASSERT_TRUE(opt_nested.value().has_value());
  std::optional<option::LessThan20U8> opt_nested_inner =
      std::move(opt_nested.value());
  ASSERT_TRUE(opt_nested_inner.has_value());
  EXPECT_EQ(opt_nested_inner.value().value(), 1);
}

TEST(OptionTest, OptionWithDirectTagIsConvertibleToStd) {
  option::HasOptions has_options = option::HasOptions::new_(1);
  std::optional<uint8_t> opt_direct = std::move(has_options.direct);
  ASSERT_TRUE(opt_direct.has_value());
  EXPECT_EQ(*opt_direct, 1);
}

TEST(OptionTest, OptionNoneIsNullOpt) {
  option::HasOptions has_options_none = option::HasOptions::with_none();
  std::optional<option::LessThan20U8> opt_niche =
      std::move(has_options_none.niche);
  EXPECT_EQ(opt_niche, std::nullopt);
  std::optional<rs::Option<option::LessThan20U8>> opt_nested =
      std::move(has_options_none.nested);
  EXPECT_EQ(opt_nested, std::nullopt);
  std::optional<uint8_t> opt_direct = std::move(has_options_none.direct);
  EXPECT_EQ(opt_direct, std::nullopt);
}

TEST(OptionTest, MoveIntoOptionalSetsOptionToNone) {
  option::HasOptions has_options = option::HasOptions::new_(19);
  std::optional<option::LessThan20U8> opt_niche = std::move(has_options.niche);
  ASSERT_TRUE(opt_niche.has_value());
  EXPECT_EQ(opt_niche->value(), 19);
  std::optional<option::LessThan20U8> opt_niche_retake =
      std::move(has_options.niche);
  EXPECT_EQ(opt_niche_retake, std::nullopt);
}

TEST(OptionTest, ConstructFromOption) {
  std::optional<uint8_t> some_u8 = std::make_optional(uint8_t{42});
  option::HasOptions has_options =
      option::HasOptions::with_option(rs::Option<uint8_t>(std::move(some_u8)));
  std::optional<uint8_t> some_u8_retaken = std::move(has_options.direct);
  ASSERT_TRUE(some_u8_retaken.has_value());
  EXPECT_EQ(*some_u8_retaken, 42);
}

TEST(OptionTest, StructWithNicheIsConvertibleToStd) {
  option::HasHasOptions has_has_options = option::HasHasOptions::new_(19);
  std::optional<option::HasOptions> has_options = std::move(has_has_options.me);
  ASSERT_TRUE(has_options.has_value());
  std::optional<option::LessThan20U8> opt_niche =
      std::move(has_options.value().niche);
  ASSERT_TRUE(opt_niche.has_value());
  std::optional<rs::Option<option::LessThan20U8>> opt_nested =
      std::move(has_options.value().nested);
  ASSERT_TRUE(opt_nested.has_value());
  std::optional<uint8_t> opt_direct = std::move(has_options.value().direct);
  ASSERT_TRUE(opt_direct.has_value());
}

TEST(OptionTest, OptNoDefaultWithDrop) {
  option::OptNoDefaultWithDrop x = option::OptNoDefaultWithDrop::new_("world");
  std::optional<option::HasNoDefault> val = std::move(x.val);
  ASSERT_TRUE(val.has_value());
  EXPECT_EQ(val->a, 1045);
  EXPECT_EQ(val->get_string_inside_option().to_string_view(), "world");
}

TEST(OptionTest, OptionHasNoDefaultDefaultConstruct) {
  rs::Option<option::HasNoDefault> val;
  EXPECT_FALSE(val.has_value());
}

TEST(OptionTest, OptionHasNoDefaultNullOptConstruct) {
  rs::Option<option::HasNoDefault> val(std::nullopt);
  EXPECT_FALSE(val.has_value());
}

TEST(OptionTest, OptionHasNoDefaultNullOptAssign) {
  rs::Option<option::HasNoDefault> val;
  val = std::nullopt;
  EXPECT_FALSE(val.has_value());
}

TEST(OptionTest, OptionHasDefaultValueConstruct) {
  rs::Option<option::HasDefault> x(option::HasDefault::new_("has default"));
  ASSERT_TRUE(x.has_value());
  std::optional<option::HasDefault> val = std::move(x);
  ASSERT_TRUE(val.has_value());
  EXPECT_EQ(val->get_string_inside_option().to_string_view(), "has default");
}

TEST(OptionTest, OptionMoveConstructAndAssignFromOption) {
  rs::Option<option::HasNoDefault> x(std::nullopt);
  rs::Option<option::HasNoDefault> y(std::move(x));
  rs::Option<option::HasNoDefault> z;
  z = std::move(y);
  EXPECT_FALSE(z.has_value());
}

TEST(OptionTest, OptionArrowProvidesReferenceToValue) {
  rs::Option<option::HasDefault> opt(option::HasDefault::new_("pls move me"));
  ASSERT_TRUE(opt.has_value());
  EXPECT_EQ(std::move(opt->get_string_inside_option()), "pls move me");
  std::optional<option::HasDefault> val = std::move(opt);
  ASSERT_TRUE(val.has_value());
  EXPECT_EQ(val->get_string_inside_option().to_string_view(), "pls move me");
  EXPECT_FALSE(opt.has_value());
}

TEST(OptionTest, OptionHasDefaultValueAssign) {
  rs::Option<option::HasDefault> x;
  x = option::HasDefault::new_("hello");
  ASSERT_TRUE(x.has_value());
  std::optional<option::HasDefault> val = std::move(x);
  ASSERT_TRUE(val.has_value());
  EXPECT_EQ(val->get_string_inside_option().to_string_view(), "hello");
}

TEST(OptionTest, OptionHasNoDefaultOptionConstruct) {
  std::optional<option::HasDefault> some_has_default(
      std::in_place, ::option::HasDefault::new_("hello"));
  rs::Option<option::HasDefault> x(std::move(some_has_default));
  ASSERT_TRUE(x.has_value());
  std::optional<option::HasDefault> val = std::move(x);
  ASSERT_TRUE(val.has_value());
  EXPECT_EQ(val->get_string_inside_option().to_string_view(), "hello");
}

TEST(OptionTest, OptionHasNoDefaultOptionAssign) {
  std::optional<option::HasDefault> some_has_default(
      std::in_place, ::option::HasDefault::new_("hello"));
  rs::Option<option::HasDefault> x;
  x = std::move(some_has_default);
  ASSERT_TRUE(x.has_value());
  std::optional<option::HasDefault> val = std::move(x);
  ASSERT_TRUE(val.has_value());
  EXPECT_EQ(val->get_string_inside_option().to_string_view(), "hello");
}

TEST(OptionTest, OptDefaultWithDrop) {
  option::OptDefaultWithDrop x = option::OptDefaultWithDrop::new_("berry");
  std::optional<option::HasDefault> val = std::move(x.opt);
  ASSERT_TRUE(val.has_value());
  EXPECT_EQ(val->get_string_inside_option().to_string_view(), "berry");
}

TEST(OptionTest, OptCopyNoDefault) {
  option::OptCopyNoDefault x = option::OptCopyNoDefault::new_(123);
  std::optional<option::CopyNoDefault> val = std::move(x.val);
  ASSERT_TRUE(val.has_value());
  EXPECT_EQ(val->val, 123);
}

TEST(OptionTest, OptCloneNoDefault) {
  option::OptCloneNoDefault x = option::OptCloneNoDefault::new_(74);
  std::optional<option::CloneNoDefault> val = std::move(x.val);
  ASSERT_TRUE(val.has_value());
  EXPECT_EQ(val->val, 74);
}

TEST(OptionTest, PassingOptionAsReferenceArgument) {
  rs::Option<option::HasDefault> x = option::HasDefault::new_("hello");
  std::optional<uint32_t> y = std::move(option::stringify_len(x));
  ASSERT_TRUE(y.has_value());
  EXPECT_EQ(y.value(), 5);
}

TEST(OptionTest, ReturnOptionResult) {
  std::optional<rs::Result<int32_t, rs::std::string::String>> result =
      option::return_option_result();
  ASSERT_TRUE(result.has_value());
  ASSERT_TRUE(result.value().has_value());
  EXPECT_EQ(result.value().value(), 1);
}

TEST(OptionTest, ReturnNestedOptionResult) {
  auto opt_result = option::stress_testing_nested_types();
  EXPECT_FALSE(opt_result.has_value());
}

TEST(OptionTest, OptionWithSizeTypes) {
  decltype(option::OptionWithSizeTypes::uval) uval(10);
  decltype(option::OptionWithSizeTypes::ival) ival(-10);
  option::OptionWithSizeTypes s = option::OptionWithSizeTypes::new_(uval, ival);
  ASSERT_TRUE(s.uval.has_value());
  EXPECT_EQ(*s.uval, 10);
  ASSERT_TRUE(s.ival.has_value());
  EXPECT_EQ(*s.ival, -10);
}

TEST(OptionTest, OptionUnitNone) {
  option::UnitOptionField unit_option_field;
  rs::Option<rs::unit_t> none_unit = unit_option_field.unit;
  EXPECT_FALSE(none_unit.has_value());
}

template <typename T>
concept HasValue = requires(T& t) { t.value(); };
template <typename T>
concept HasDeref = requires(T& t) { *t; };
template <typename T, typename U>
concept HasValueOr =
    requires(T t, U u) { std::forward<T>(t).value_or(std::forward<U>(u)); };

TEST(OptionTest, OptionUnitSome) {
  option::UnitOptionField unit_option_field =
      option::UnitOptionField::new_with_some();
  rs::Option<rs::unit_t> some_unit = unit_option_field.unit;
  EXPECT_TRUE(some_unit.has_value());
  static_assert(!HasDeref<rs::Option<rs::unit_t>>);
  static_assert(!HasValue<rs::Option<rs::unit_t>>);
  static_assert(!HasValueOr<rs::Option<rs::unit_t>&, rs::unit_t>);
  static_assert(!HasValueOr<rs::Option<rs::unit_t>&&, rs::unit_t>);
}

TEST(OptionTest, OptionUnitEquality) {
  const rs::Option<rs::unit_t> some_unit =
      option::UnitOptionField::new_with_some().unit;
  const rs::Option<rs::unit_t> other_some_unit =
      option::UnitOptionField::new_with_some().unit;
  const rs::Option<rs::unit_t> none_unit = option::UnitOptionField().unit;
  EXPECT_EQ(some_unit, other_some_unit);
  EXPECT_NE(some_unit, none_unit);
  EXPECT_EQ(none_unit, option::UnitOptionField().unit);
  EXPECT_EQ(none_unit, std::nullopt);
  EXPECT_NE(some_unit, std::nullopt);
}

TEST(OptionTest, ValueOrLvalue) {
  const rs::Option<uint8_t> some_u8(uint8_t{42});
  const rs::Option<uint8_t> none_u8(std::nullopt);
  EXPECT_EQ(some_u8.value_or(99), 42);
  EXPECT_EQ(none_u8.value_or(99), 99);
  EXPECT_EQ(none_u8.value_or({}), 0);
}

TEST(OptionTest, Value) {
  rs::Option<uint8_t> some_u8(uint8_t{42});
  const rs::Option<uint8_t>& const_some_u8 = some_u8;
  EXPECT_EQ(some_u8.value(), 42);
  EXPECT_EQ(const_some_u8.value(), 42);
  EXPECT_EQ(std::move(some_u8).value(), 42);
}

TEST(OptionTest, ValueOnNoneDies) {
  const rs::Option<uint8_t> none_u8(std::nullopt);
  EXPECT_DEATH_IF_SUPPORTED(none_u8.value(), "");
}

TEST(OptionTest, ValueOrRvalue) {
  static_assert(
      HasValueOr<rs::Option<option::HasDefault>&&, option::HasDefault>);

  rs::Option<option::HasDefault> some_opt(
      option::HasDefault::new_("from some"));
  option::HasDefault unwrapped_some =
      std::move(some_opt).value_or(option::HasDefault::new_("default"));
  EXPECT_EQ(unwrapped_some.get_string_inside_option().to_string_view(),
            "from some");
  EXPECT_FALSE(some_opt.has_value());

  rs::Option<option::HasDefault> none_opt(std::nullopt);
  option::HasDefault unwrapped_none =
      std::move(none_opt).value_or(option::HasDefault::new_("default"));
  EXPECT_EQ(unwrapped_none.get_string_inside_option().to_string_view(),
            "default");
  EXPECT_FALSE(none_opt.has_value());
}

TEST(OptionTest, EqualityComparison) {
  const rs::Option<uint8_t> some_42(uint8_t{42});
  const rs::Option<uint8_t> some_42_b(uint8_t{42});
  const rs::Option<uint8_t> some_99(uint8_t{99});
  const rs::Option<uint8_t> none_u8(std::nullopt);

  EXPECT_EQ(some_42, some_42_b);
  EXPECT_NE(some_42, some_99);
  EXPECT_NE(some_42, none_u8);
  EXPECT_EQ(none_u8, std::nullopt);
  EXPECT_EQ(std::nullopt, none_u8);
  EXPECT_NE(some_42, std::nullopt);
  EXPECT_EQ(some_42, 42);
  EXPECT_EQ(42, some_42);
  EXPECT_NE(some_42, 99);
  EXPECT_NE(none_u8, 42);
}

TEST(OptionTest, ExplicitOperatorBoolAndImplicitNullopt) {
  const rs::Option<uint8_t> some_42(uint8_t{42});
  const rs::Option<uint8_t> none_u8 = std::nullopt;

  EXPECT_TRUE(static_cast<bool>(some_42));
  EXPECT_FALSE(static_cast<bool>(none_u8));
  if (some_42) {
    // Expected.
  } else {
    FAIL() << "some_42 should be truthy";
  }
  if (none_u8) {
    FAIL() << "none_u8 should be falsy";
  }

  bool cond = false;
  rs::Option<uint8_t> ternary_opt = cond ? some_42 : std::nullopt;
  EXPECT_FALSE(ternary_opt.has_value());
}

}  // namespace
