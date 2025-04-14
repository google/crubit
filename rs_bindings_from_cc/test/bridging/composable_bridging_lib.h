// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_COMPOSABLE_BRIDGING_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_COMPOSABLE_BRIDGING_LIB_H_

#include <cstddef>
#include <utility>

#include "support/bridge_cpp.h"

// clang-format off
#define CRUBIT_BRIDGE(rust_name, abi_rust, abi_cpp)         \
  [[clang::annotate("crubit_bridge_rust_name", rust_name)]] \
  [[clang::annotate("crubit_bridge_abi_rust", abi_rust)]]   \
  [[clang::annotate("crubit_bridge_abi_cpp", abi_cpp)]]
// clang-format on

// Bridge type with a template type argument.
template <typename T>
struct CRUBIT_BRIDGE("Vec3", "Vec3Abi", "Vec3Abi") Vec3 {
  T x;
  T y;
  T z;
};

template <typename Abi>
  requires(crubit::is_crubit_abi<Abi>)
struct Vec3Abi {
  using Value = Vec3<typename Abi::Value>;
  static constexpr size_t kSize = Abi::kSize * 3;
  static void Encode(Value value, crubit::Encoder& encoder) {
    encoder.Encode<Abi>(std::move(value.x));
    encoder.Encode<Abi>(std::move(value.y));
    encoder.Encode<Abi>(std::move(value.z));
  }
  static Value Decode(crubit::Decoder& decoder) {
    return {
        .x = decoder.Decode<Abi>(),
        .y = decoder.Decode<Abi>(),
        .z = decoder.Decode<Abi>(),
    };
  }
};

Vec3<float> MakeVec3(float x, float y, float z);

std::optional<Vec3<float>> MakeOptionalVec3(float x, float y, float z,
                                            bool is_present);

std::optional<Vec3<float>> MapMultiply(std::optional<Vec3<float>> v,
                                       float factor);

std::pair<std::pair<int, float>, bool> MakePair(int a, float b, bool c);

std::pair<std::optional<int>, std::optional<std::pair<float, Vec3<float>>>>
MakeStuff();

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_COMPOSABLE_BRIDGING_LIB_H_
