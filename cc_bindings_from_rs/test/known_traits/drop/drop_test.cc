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
  static_assert(std::is_destructible_v<TypeUnderTest>);
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
    EXPECT_EQ(0, s.get_int());  // NOLINT(bugprone-use-after-move)
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
    EXPECT_EQ(456, s1.get_int());  // NOLINT(bugprone-use-after-move)
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

TEST(DropTest, DropImplWithClone) {
  using TypeUnderTest = drop::drop_impl_with_clone::DropImplWithClone;
  static_assert(!std::is_default_constructible_v<TypeUnderTest>);
  static_assert(std::is_move_constructible_v<TypeUnderTest>);
  static_assert(!std::is_trivially_move_constructible_v<TypeUnderTest>);
  static_assert(std::is_move_assignable_v<TypeUnderTest>);
  static_assert(!std::is_trivially_move_assignable_v<TypeUnderTest>);
  static_assert(std::is_destructible_v<TypeUnderTest>);
  static_assert(!std::is_trivially_destructible_v<TypeUnderTest>);
  static_assert(std::is_copy_constructible_v<TypeUnderTest>);
  static_assert(std::is_copy_assignable_v<TypeUnderTest>);

  // Test the destructor.
  drop::counters::reset_counts();
  {
    // Non-zero clone and drop count come from the temporary created by
    // `create_from_int`.
    TypeUnderTest s = TypeUnderTest::create_from_int(123);
    EXPECT_EQ(123, s.get_int());
    EXPECT_EQ(1, drop::counters::get_clone_count());
    EXPECT_EQ(1, drop::counters::get_drop_count());
  }  // `TypeUnderTest`'s destructor runs when `s` goes out of scope.
  EXPECT_EQ(1, drop::counters::get_clone_count());
  EXPECT_EQ(2, drop::counters::get_drop_count());

  // Testing the move constructor.
  drop::counters::reset_counts();
  {
    TypeUnderTest s = TypeUnderTest::create_from_int(123);
    EXPECT_EQ(123, s.get_int());
    EXPECT_EQ(1, drop::counters::get_clone_count());
    EXPECT_EQ(1, drop::counters::get_drop_count());

    // We expect the move to be implemented in terms of copy, so we expect
    // a corresponding increase in clone counters.
    TypeUnderTest s2(std::move(s));
    EXPECT_EQ(123, s2.get_int());
    EXPECT_EQ(123, s.get_int());  // NOLINT(bugprone-use-after-move)
    EXPECT_EQ(2, drop::counters::get_clone_count());
    EXPECT_EQ(1, drop::counters::get_drop_count());
  }
  EXPECT_EQ(2, drop::counters::get_clone_count());
  EXPECT_EQ(3, drop::counters::get_drop_count());

  // Testing the move assignment operator.
  drop::counters::reset_counts();
  {
    TypeUnderTest s1 = TypeUnderTest::create_from_int(123);
    TypeUnderTest s2 = TypeUnderTest::create_from_int(456);
    EXPECT_EQ(2, drop::counters::get_clone_count());
    EXPECT_EQ(0, drop::counters::get_clone_from_count());
    EXPECT_EQ(2, drop::counters::get_drop_count());

    // We expect the move to be implemented in terms of copy, so we expect
    // a corresponding increase in clone counters.
    s2 = std::move(s1);
    EXPECT_EQ(123, s1.get_int());  // NOLINT(bugprone-use-after-move)
    EXPECT_EQ(123, s2.get_int());
    EXPECT_EQ(2, drop::counters::get_clone_count());
    EXPECT_EQ(1, drop::counters::get_clone_from_count());
    EXPECT_EQ(2, drop::counters::get_drop_count());

    // Okay to assign a value to itself.  `clone_from` should *not* be called
    // (it would lead to aliasing-related UB in Rust when `self` and `source`
    // point to the same object).
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wself-move"
    s2 = std::move(s2);
#pragma clang diagnostic pop
    EXPECT_EQ(123, s1.get_int());
    EXPECT_EQ(123, s2.get_int());
    EXPECT_EQ(2, drop::counters::get_clone_count());
    EXPECT_EQ(1, drop::counters::get_clone_from_count());
    EXPECT_EQ(2, drop::counters::get_drop_count());
  }
  EXPECT_EQ(2, drop::counters::get_clone_count());
  EXPECT_EQ(1, drop::counters::get_clone_from_count());
  EXPECT_EQ(4, drop::counters::get_drop_count());
}

TEST(DropTest, DropImplWithNothingElse) {
  using TypeUnderTest =
      drop::drop_impl_with_nothing_else::DropImplWithNothingElse;
  static_assert(!std::is_default_constructible_v<TypeUnderTest>);
  static_assert(!std::is_move_constructible_v<TypeUnderTest>);
  static_assert(!std::is_trivially_move_constructible_v<TypeUnderTest>);
  static_assert(!std::is_move_assignable_v<TypeUnderTest>);
  static_assert(!std::is_trivially_move_assignable_v<TypeUnderTest>);
  static_assert(std::is_destructible_v<TypeUnderTest>);
  static_assert(!std::is_trivially_destructible_v<TypeUnderTest>);
  static_assert(!std::is_copy_constructible_v<TypeUnderTest>);
  static_assert(!std::is_copy_assignable_v<TypeUnderTest>);

  using WrappedTypeUnderTest =
      drop::drop_impl_with_nothing_else::WrappedDropImplWithNothingElse;

  // Test the destructor of `WrappedTypeUnderTest`.  Test that instance methods
  // of `TypeUnderTest` work (e.g. `get_int`).
  //
  // TODO(lukasza): Figure out how to test the destructor of `TypeUnderTest`.
  // Maybe this needs to wait until `TypeUnderTest` can be constructed without
  // having a `Default` or `Clone` impl (e.g. once `From<T>` is mapped to a
  // C++ constructor - see b/286941486).
  drop::counters::reset_counts();
  {
    WrappedTypeUnderTest wrapper;
    EXPECT_EQ(0, drop::counters::get_drop_count());
    EXPECT_EQ(123, wrapper.field.get_int());
  }
  EXPECT_EQ(1, drop::counters::get_drop_count());
}

}  // namespace
}  // namespace crubit
