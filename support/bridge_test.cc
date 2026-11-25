// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/bridge.h"

#include <tuple>
#include <type_traits>

#include "gtest/gtest.h"
#include "support/rs_std/fn_ref.h"

namespace crubit::bridge {
namespace {

TEST(BridgeTest, CallableRefAbi) {
  using Abi = crubit::CallableRefAbi<rs::FnRef<int(int)>>;
  static_assert(crubit::is_crubit_abi<Abi>);

  // Test Decode
  // A fat pointer in Rust is just two void* (data, invoker).
  void* fat_ptr[2] = {nullptr, nullptr};
  unsigned char buf[Abi::kSize];
  std::memcpy(buf, fat_ptr, sizeof(fat_ptr));

  auto make_callable = [](const void* state) {
    int (*invoker)(void*, int) = nullptr;
    return rs::internal::MakeFnRef<int(int)>(const_cast<void*>(state), invoker);
  };
  auto decoded = internal::Decode<Abi>(Abi(make_callable), buf);
  EXPECT_EQ(decoded.data(), buf);
}

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
}

}  // namespace
}  // namespace crubit::bridge
