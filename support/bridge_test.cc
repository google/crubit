// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/bridge.h"

#include <tuple>
#include <type_traits>

#include "gtest/gtest.h"

namespace crubit::bridge {
namespace {

TEST(BridgeTest, RoundtripI32) {
  using Abi = TransmuteAbi<int>;
  static_assert(std::is_default_constructible_v<Abi>);

  int original = 123;

  unsigned char buf[Abi::kSize];
  internal::Encode<Abi>(Abi(), buf, original);
  int value = internal::Decode<Abi>(Abi(), buf);
  EXPECT_EQ(value, original);
}

TEST(BridgeTest, RoundtripOptionalI32) {
  using Abi = OptionAbi<TransmuteAbi<int>>;
  static_assert(std::is_default_constructible_v<Abi>);

  typename Abi::Value original = 123;
  unsigned char buf[Abi::kSize];
  internal::Encode<Abi>(Abi(), buf, original);
  typename Abi::Value value = internal::Decode<Abi>(Abi(), buf);
  EXPECT_EQ(value, original);
}

TEST(BridgeTest, RoundtripTupleI32I32) {
  using Abi = TupleAbi<TransmuteAbi<int>, TransmuteAbi<int>>;
  static_assert(std::is_default_constructible_v<Abi>);

  typename Abi::Value original = std::make_tuple(123, 456);
  unsigned char buf[Abi::kSize];
  internal::Encode<Abi>(Abi(), buf, original);
  typename Abi::Value value = internal::Decode<Abi>(Abi(), buf);
  EXPECT_EQ(value, original);
}

TEST(BridgeTest, RoundtripStuff) {
  using Abi =
      TupleAbi<TransmuteAbi<int>, TransmuteAbi<float>,
               OptionAbi<TupleAbi<TransmuteAbi<int>, TransmuteAbi<bool>>>>;
  static_assert(std::is_default_constructible_v<Abi>);

  typename Abi::Value original =
      std::make_tuple(123, 3.14f, std::make_tuple(456, true));
  unsigned char buf[Abi::kSize];
  internal::Encode<Abi>(Abi(), buf, original);
  typename Abi::Value value = internal::Decode<Abi>(Abi(), buf);
  EXPECT_EQ(value, original);
}

struct [[clang::trivial_abi]] DynamicTrivialAbi {
  explicit DynamicTrivialAbi(int v) : value(v) {}
  DynamicTrivialAbi(DynamicTrivialAbi&&) = default;
  virtual ~DynamicTrivialAbi() = default;
  int value = 0;
};

TEST(BridgeTest, RoundtripDynamicTrivialAbi) {
  using Abi = TransmuteAbi<DynamicTrivialAbi>;
  static_assert(std::is_default_constructible_v<Abi>);

  unsigned char buf[Abi::kSize];
  internal::Encode<Abi>(Abi(), buf, DynamicTrivialAbi(123));
  DynamicTrivialAbi value = internal::Decode<Abi>(Abi(), buf);
  EXPECT_EQ(value.value, 123);

  using MoveOnlyTupleAbi = TupleAbi<Abi, TransmuteAbi<int>>;
  unsigned char tuple_buf[MoveOnlyTupleAbi::kSize];
  internal::Encode<MoveOnlyTupleAbi>(MoveOnlyTupleAbi(), tuple_buf,
                                     {DynamicTrivialAbi(123), 456});
  auto [first, second] =
      internal::Decode<MoveOnlyTupleAbi>(MoveOnlyTupleAbi(), tuple_buf);
  EXPECT_EQ(first.value, 123);
  EXPECT_EQ(second, 456);
}

}  // namespace
}  // namespace crubit::bridge
