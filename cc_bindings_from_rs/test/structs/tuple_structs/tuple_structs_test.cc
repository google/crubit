// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.h"

#include <type_traits>
#include <utility>

#include "gtest/gtest.h"

namespace crubit {
namespace {

using ::tuple_structs::TupleStructOnePrivateArg;
using ::tuple_structs::TupleStructOnePublicArg;
using ::tuple_structs::TupleStructTwoPrivateArgs;
using ::tuple_structs::TupleStructTwoPublicArgs;

template <typename T>
concept HasFieldZero = requires(T t) { t.__field0; };

template <typename T>
concept HasFieldOne = requires(T t) { t.__field1; };

TEST(TupleStructsTest, TupleStructOnePublicArgIsConstructibleFromArg) {
  // TODO: 438752078 - Add support for tuple struct ctors, then replace this
  // assertion with a usage of the constructor.
  static_assert(!std::is_constructible_v<TupleStructOnePublicArg, int32_t>);
  TupleStructOnePublicArg arg = TupleStructOnePublicArg::create(5);
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
  // TODO: 438752078 - Add support for tuple struct ctors, then replace this
  // assertion with a usage of the constructor.
  static_assert(
      !std::is_constructible_v<TupleStructTwoPublicArgs, int32_t, int32_t>);
  TupleStructTwoPublicArgs arg = TupleStructTwoPublicArgs::create(5, 7);
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

}  // namespace
}  // namespace crubit
