// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.h"

#include <type_traits>
#include <utility>

#include "gtest/gtest.h"

namespace crubit {
namespace {

using ::tuple_structs::CloneNoDefault;
using ::tuple_structs::CopyNoDefault;
using ::tuple_structs::DefaultAndCloneNoUnpin;
using ::tuple_structs::DefaultNoCopyNoClone;
using ::tuple_structs::TupleStructOnePrivateArg;
using ::tuple_structs::TupleStructOnePublicArg;
using ::tuple_structs::TupleStructOnePublicArgOnePrivateArg;
using ::tuple_structs::TupleStructTwoPrivateArgs;
using ::tuple_structs::TupleStructTwoPublicArgs;
using ::tuple_structs::TupleStructWithCloneNoDefault;
using ::tuple_structs::TupleStructWithCppImmovableType;
using ::tuple_structs::TupleStructWithDefaultAndCloneNoUnpin;
using ::tuple_structs::TupleStructWithDefaultNoCopyNoClone;
using ::tuple_structs::TupleStructWithInvalidArgumentType;
using ::tuple_structs::TupleStructWithNoDefault;
using ::tuple_structs::TupleStructWithNonExhaustiveCtor;

template <typename T>
concept HasFieldZero = requires(T t) { t.__field0; };

template <typename T>
concept HasFieldOne = requires(T t) { t.__field1; };

TEST(TupleStructsTest, TupleStructOnePublicArgIsConstructibleFromArg) {
  TupleStructOnePublicArg arg(5);
  EXPECT_EQ(arg.__field0, 5);
  static_assert(HasFieldZero<TupleStructOnePublicArg>);
}

TEST(TupleStructsTest, TupleStructOnePrivateArg) {
  static_assert(!std::is_constructible_v<TupleStructOnePrivateArg, int32_t>);
  TupleStructOnePrivateArg arg = TupleStructOnePrivateArg::create(5);
  EXPECT_EQ(arg.get_arg(), 5);
  static_assert(!HasFieldZero<TupleStructOnePrivateArg>);
}

TEST(TupleStructsTest, TupleStructTwoPublicArgsIsConstructibleFromArgs) {
  TupleStructTwoPublicArgs arg(5, 7);
  EXPECT_EQ(arg.__field0, 5);
  EXPECT_EQ(arg.__field1, 7);
  static_assert(HasFieldZero<TupleStructTwoPublicArgs>);
  static_assert(HasFieldOne<TupleStructTwoPublicArgs>);
}

TEST(TupleStructsTest, TupleStructTwoPrivateArgs) {
  static_assert(
      !std::is_constructible_v<TupleStructTwoPrivateArgs, int32_t, int32_t>);
  TupleStructTwoPrivateArgs arg = TupleStructTwoPrivateArgs::create(5, 7);
  EXPECT_EQ(arg.get_first_arg(), 5);
  EXPECT_EQ(arg.get_second_arg(), 7);
  static_assert(!HasFieldZero<TupleStructTwoPrivateArgs>);
  static_assert(!HasFieldOne<TupleStructTwoPrivateArgs>);
}

TEST(TupleStructsTest,
     TupleStructWithInvalidArgumentTypeIsNotConstructableFromArg) {
  static_assert(!std::is_constructible_v<TupleStructWithInvalidArgumentType,
                                         std::tuple<int32_t, int32_t>>);
  std::tuple<int32_t, int32_t> tuple(49, 144);
  TupleStructWithInvalidArgumentType arg =
      TupleStructWithInvalidArgumentType::create(tuple);
  std::tuple<int32_t, int32_t> tuple_from_arg = arg.get_arg();
  EXPECT_EQ(std::get<0>(tuple_from_arg), 49);
  EXPECT_EQ(std::get<1>(tuple_from_arg), 144);
}

TEST(TupleStructsTest, TupleStructWithNonExhaustiveCtorIsNotConstructible) {
  static_assert(!std::is_constructible_v<TupleStructWithNonExhaustiveCtor,
                                         int32_t, int32_t>);
  TupleStructWithNonExhaustiveCtor arg =
      TupleStructWithNonExhaustiveCtor::create(5, 7);
  EXPECT_EQ(arg.__field0, 5);
  EXPECT_EQ(arg.__field1, 7);
}

TEST(TupleStructsTest, TupleStructOnePublicArgOnePrivateArgIsNotConstructible) {
  static_assert(!std::is_constructible_v<TupleStructOnePublicArgOnePrivateArg,
                                         int32_t, int32_t>);
  TupleStructOnePublicArgOnePrivateArg arg =
      TupleStructOnePublicArgOnePrivateArg::create(5, 7);
  EXPECT_EQ(arg.__field0, 5);
  EXPECT_EQ(arg.get_second_arg(), 7);
  static_assert(!HasFieldOne<TupleStructOnePublicArgOnePrivateArg>);
}

TEST(TupleStructsTest, TupleStructWithCppImmovableTypeIsNotConstructible) {
  static_assert(!std::is_constructible_v<TupleStructWithCppImmovableType,
                                         int32_t, int32_t>);
  TupleStructWithCppImmovableType arg =
      TupleStructWithCppImmovableType::create(5, 7);
  EXPECT_EQ(arg.get_first_arg(), 5);
  EXPECT_EQ(arg.get_second_arg(), 7);
}

TEST(TupleStructsTest, TupleStructWithNoDefaultIsConstructible) {
  CopyNoDefault copy_no_default = CopyNoDefault::create(732);
  TupleStructWithNoDefault arg(copy_no_default);
  EXPECT_EQ(arg.__field0.value, 732);
}

TEST(TupleStructsTest, TupleStructWithDefaultNoCopyNoCloneIsConstructible) {
  DefaultNoCopyNoClone default_no_copy_no_clone;
  TupleStructWithDefaultNoCopyNoClone arg(DefaultNoCopyNoClone{});
  EXPECT_EQ(arg.__field0.value, 0);
}

TEST(TupleStructsTest, TupleStructWithCloneNoDefaultIsNotConstructible) {
  static_assert(
      !std::is_constructible_v<TupleStructWithCloneNoDefault, CloneNoDefault>);
  TupleStructWithCloneNoDefault arg =
      TupleStructWithCloneNoDefault::create(891);
  EXPECT_EQ(arg.get_value(), 891);
}

TEST(TupleStructsTest,
     TupleStructWithDefaultAndCloneNoUnpinIsNotConstructible) {
  static_assert(!std::is_constructible_v<TupleStructWithDefaultAndCloneNoUnpin,
                                         DefaultAndCloneNoUnpin>);
  TupleStructWithDefaultAndCloneNoUnpin arg =
      TupleStructWithDefaultAndCloneNoUnpin::create();
  EXPECT_EQ(arg.__field0.value, 0);
}

}  // namespace
}  // namespace crubit
