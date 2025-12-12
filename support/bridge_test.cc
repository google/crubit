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

}  // namespace
}  // namespace crubit::bridge
