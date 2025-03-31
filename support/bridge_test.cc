// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/bridge.h"

#include <cstdint>
#include <optional>
#include <tuple>

#include "gtest/gtest.h"

namespace crubit::bridge {
namespace {

TEST(BridgeTest, RoundtripI32) {
  using T = int32_t;
  using S = ByTransmute;

  T original = 123;

  unsigned char buf[CrubitAbiSize<T, S>()];
  internal::Encode<T, S>(buf, original);
  T value = internal::Decode<T, S>(buf);
  EXPECT_EQ(value, original);
}

TEST(BridgeTest, RoundtripOptionalI32) {
  using T = std::optional<int32_t>;
  using S = ByBridge<ByTransmute>;

  T original = 123;
  unsigned char buf[CrubitAbiSize<T, S>()];
  internal::Encode<T, S>(buf, original);
  T value = internal::Decode<T, S>(buf);
  EXPECT_EQ(value, original);
}

TEST(BridgeTest, RoundtripTupleI32I32) {
  using T = std::tuple<int, int>;
  using S = ByBridge<ByTransmute, ByTransmute>;

  T original = std::make_tuple(123, 456);
  unsigned char buf[CrubitAbiSize<T, S>()];
  internal::Encode<T, S>(buf, original);
  T value = internal::Decode<T, S>(buf);
  EXPECT_EQ(value, original);
}

TEST(BridgeTest, RoundtripStuff) {
  using T = std::tuple<int, float, std::optional<std::tuple<int, bool>>>;
  using S = ByBridge<ByTransmute, ByTransmute,
                     ByBridge<ByBridge<ByTransmute, ByTransmute>>>;

  T original = std::make_tuple(123, 3.14f, std::make_tuple(456, true));
  unsigned char buf[CrubitAbiSize<T, S>()];
  internal::Encode<T, S>(buf, original);
  T value = internal::Decode<T, S>(buf);
  EXPECT_EQ(value, original);
}

}  // namespace
}  // namespace crubit::bridge
