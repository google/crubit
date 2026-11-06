// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "cc_bindings_from_rs/test/enums/option_references.h"

#include <cstdint>
#include <optional>
#include <type_traits>
#include <utility>
#include <vector>

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace {

using ::option_references::Value;
using ::option_references::Values;
using ::testing::ElementsAre;

Value MakeValue(std::int32_t x) {
  Value value;
  value.x = x;
  return value;
}

// `Option<&Value>` and `Option<*const Value>` are distinct C++ types, with
// the layouts of the corresponding Rust types (b/575916406).
static_assert(std::is_same_v<decltype(option_references::if_positive(
                                 std::declval<const Value*>())),
                             rs::Option<const Value&>>);
static_assert(std::is_same_v<decltype(option_references::if_positive_mut(
                                 std::declval<Value*>())),
                             rs::Option<Value&>>);
static_assert(std::is_same_v<
              decltype(option_references::as_ptr(std::declval<const Value&>())),
              rs::Option<const Value*>>);
static_assert(sizeof(rs::Option<const Value&>) == sizeof(const Value*));
static_assert(sizeof(rs::Option<Value&>) == sizeof(Value*));
static_assert(sizeof(rs::Option<const Value*>) > sizeof(const Value*));

// Like `std::optional<T&>`, `rs::Option<T&>` can only bind to lvalues that
// `T&` can refer to.
static_assert(std::is_constructible_v<rs::Option<const Value&>, Value&>);
static_assert(std::is_constructible_v<rs::Option<const Value&>, const Value&>);
static_assert(!std::is_constructible_v<rs::Option<const Value&>, Value>);
static_assert(!std::is_constructible_v<rs::Option<const Value&>, Value&&>);
static_assert(!std::is_constructible_v<rs::Option<Value&>, const Value&>);
static_assert(
    !std::is_constructible_v<rs::Option<const Value&>, const std::int32_t&>);

// `Option<&T>` is `Copy`, but `Option<&mut T>` is not.
static_assert(std::is_trivially_copyable_v<rs::Option<const Value&>>);
static_assert(!std::is_copy_constructible_v<rs::Option<Value&>>);
static_assert(std::is_trivially_move_constructible_v<rs::Option<Value&>>);

TEST(OptionReferencesTest, ReturnSharedReference) {
  Value value = MakeValue(1);
  rs::Option<const Value&> opt = option_references::if_positive(&value);
  ASSERT_TRUE(opt.has_value());
  EXPECT_EQ(&*opt, &value);
  EXPECT_EQ(opt->x, 1);

  value.x = 0;
  EXPECT_FALSE(option_references::if_positive(&value).has_value());
}

TEST(OptionReferencesTest, ReturnMutableReference) {
  Value value = MakeValue(1);
  rs::Option<Value&> opt = option_references::if_positive_mut(&value);
  ASSERT_TRUE(opt.has_value());
  EXPECT_EQ(&*opt, &value);
  opt->x = 42;
  EXPECT_EQ(value.x, 42);

  value.x = 0;
  EXPECT_FALSE(option_references::if_positive_mut(&value).has_value());
}

TEST(OptionReferencesTest, ReturnPointer) {
  Value value = MakeValue(1);
  rs::Option<const Value*> opt = option_references::as_ptr(value);
  ASSERT_TRUE(opt.has_value());
  EXPECT_EQ(*opt, &value);
}

TEST(OptionReferencesTest, PassSharedReference) {
  Value value = MakeValue(7);
  EXPECT_EQ(option_references::x_or(value, -1), 7);
  EXPECT_EQ(option_references::x_or(std::nullopt, -1), -1);
  EXPECT_EQ(option_references::x_or(rs::Option<const Value&>(), -1), -1);
  EXPECT_EQ(option_references::x_or(option_references::if_positive(&value), -1),
            7);
}

TEST(OptionReferencesTest, PassMutableReference) {
  Value value = MakeValue(7);
  option_references::increment(value);
  EXPECT_EQ(value.x, 8);
  option_references::increment(option_references::if_positive_mut(&value));
  EXPECT_EQ(value.x, 9);
  option_references::increment(std::nullopt);
}

TEST(OptionReferencesTest, RoundTripPreservesAddress) {
  Value a = MakeValue(1);
  Value b = MakeValue(1);
  EXPECT_TRUE(option_references::same_value(a, a));
  EXPECT_FALSE(option_references::same_value(a, b));
  EXPECT_FALSE(option_references::same_value(a, std::nullopt));
  EXPECT_TRUE(option_references::same_value(std::nullopt, std::nullopt));
}

TEST(OptionReferencesTest, AssignmentRebinds) {
  Value a = MakeValue(1);
  Value b = MakeValue(2);
  rs::Option<const Value&> opt = a;
  opt = b;
  EXPECT_EQ(a.x, 1);
  EXPECT_EQ(&*opt, &b);

  rs::Option<const Value&> copy = opt;
  EXPECT_EQ(&*copy, &b);

  opt = std::nullopt;
  EXPECT_FALSE(opt.has_value());
  EXPECT_TRUE(copy.has_value());
}

TEST(OptionReferencesTest, Comparisons) {
  Values values = Values::new_(1, 2, 3);
  EXPECT_EQ(values.get(0), 1);
  EXPECT_NE(values.get(0), 2);
  EXPECT_EQ(values.get(3), std::nullopt);
  EXPECT_EQ(values.get(1), values.get(1));
  EXPECT_NE(values.get(1), values.get(3));
}

TEST(OptionReferencesTest, Methods) {
  Values values = Values::new_(1, 2, 3);
  rs::Option<const std::int32_t&> second = values.get(1);
  ASSERT_TRUE(second.has_value());
  EXPECT_EQ(*second, 2);
  EXPECT_FALSE(values.get(3).has_value());

  rs::Option<std::int32_t&> third = values.get_mut(2);
  ASSERT_TRUE(third.has_value());
  *third = 30;
  EXPECT_EQ(*values.get(2), 30);
}

TEST(OptionReferencesTest, IteratorYieldsReferences) {
  Values values = Values::new_(1, 2, 3);
  std::vector<const std::int32_t*> addresses;
  for (const std::int32_t& x : values.iter()) {
    addresses.push_back(&x);
  }
  EXPECT_THAT(addresses,
              ElementsAre(&*values.get(0), &*values.get(1), &*values.get(2)));
}

}  // namespace
