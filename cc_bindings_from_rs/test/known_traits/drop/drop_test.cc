// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <type_traits>
#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/known_traits/drop/drop_cc_api.h"

namespace crubit {
namespace {

template <typename T>
class CustomDropWithDefaultTest : public testing::Test {};

using MyTypes =
    ::testing::Types<drop::drop_impl_with_default::DropImplWithDefault,
                     drop::drop_glue_with_default::DropGlueWithDefault>;
TYPED_TEST_SUITE(CustomDropWithDefaultTest, MyTypes);

TYPED_TEST(CustomDropWithDefaultTest, StaticAsserts) {
  using TypeUnderTest = TypeParam;
  static_assert(std::is_default_constructible_v<TypeUnderTest>);
  static_assert(!std::is_trivially_default_constructible_v<TypeUnderTest>);
  static_assert(std::is_move_constructible_v<TypeUnderTest>);
  static_assert(!std::is_trivially_move_constructible_v<TypeUnderTest>);
  static_assert(std::is_move_assignable_v<TypeUnderTest>);
  static_assert(!std::is_trivially_move_assignable_v<TypeUnderTest>);
  static_assert(!std::is_trivially_destructible_v<TypeUnderTest>);
  static_assert(!std::is_copy_constructible_v<TypeUnderTest>);
  static_assert(!std::is_copy_assignable_v<TypeUnderTest>);
}

TYPED_TEST(CustomDropWithDefaultTest, Destructor) {
  using TypeUnderTest = TypeParam;
  drop::counters::reset_counts();
  {
    TypeUnderTest s;
    EXPECT_EQ(0, s.get_int());
    EXPECT_EQ(1, drop::counters::get_default_count());
    EXPECT_EQ(0, drop::counters::get_drop_count());
  }  // `TypeUnderTest`'s destructor runs when `s` goes out of scope.
  EXPECT_EQ(1, drop::counters::get_default_count());
  EXPECT_EQ(1, drop::counters::get_drop_count());
}

TYPED_TEST(CustomDropWithDefaultTest, MoveConstructor) {
  using TypeUnderTest = TypeParam;
  drop::counters::reset_counts();
  {
    TypeUnderTest s;
    EXPECT_EQ(1, drop::counters::get_default_count());
    EXPECT_EQ(0, drop::counters::get_drop_count());
    s.set_int(123);

    // After move construction, we expect the "moved from" object to be in the
    // `Default` state (i.e., have a value of 0).
    TypeUnderTest s2(std::move(s));
    EXPECT_EQ(123, s2.get_int());
    EXPECT_EQ(0, s.get_int());
    EXPECT_EQ(2, drop::counters::get_default_count());
    EXPECT_EQ(0, drop::counters::get_drop_count());
  }  // `TypeUnderTest`'s destructor runs when `s` goes out of scope.
  EXPECT_EQ(2, drop::counters::get_default_count());
  EXPECT_EQ(2, drop::counters::get_drop_count());
}

TYPED_TEST(CustomDropWithDefaultTest, MoveAssignmentOperator) {
  using TypeUnderTest = TypeParam;
  drop::counters::reset_counts();
  {
    TypeUnderTest s1;
    TypeUnderTest s2;
    EXPECT_EQ(2, drop::counters::get_default_count());
    EXPECT_EQ(0, drop::counters::get_drop_count());
    s1.set_int(123);
    s2.set_int(456);

    // After move assignment we expect the values to be swapped.
    s2 = std::move(s1);
    EXPECT_EQ(456, s1.get_int());
    EXPECT_EQ(123, s2.get_int());
    EXPECT_EQ(2, drop::counters::get_default_count());
    EXPECT_EQ(0, drop::counters::get_drop_count());

    // Okay to assign a value to itself.
    s2 = std::move(s2);
    EXPECT_EQ(456, s1.get_int());
    EXPECT_EQ(123, s2.get_int());
    EXPECT_EQ(2, drop::counters::get_default_count());
    EXPECT_EQ(0, drop::counters::get_drop_count());
  }  // `TypeUnderTest`'s destructor runs when `s1` and `s2` go out of scope.
  EXPECT_EQ(2, drop::counters::get_default_count());
  EXPECT_EQ(2, drop::counters::get_drop_count());
}

}  // namespace
}  // namespace crubit
