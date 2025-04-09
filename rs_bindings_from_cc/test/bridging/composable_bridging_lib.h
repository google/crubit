// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_COMPOSABLE_BRIDGING_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_COMPOSABLE_BRIDGING_LIB_H_

#include <cstddef>
#include <optional>
#include <string_view>
#include <utility>

#include "support/bridge.h"

#define CRUBIT_BRIDGE(rust_native_type_path, ...) \
  [[clang::annotate("crubit_bridge_type", rust_native_type_path)]]

// Bridge type without any template type arguments.
struct CRUBIT_BRIDGE("MyStringView") MyStringView {
  std::string_view view;
};

template <>
struct crubit::CrubitAbiTrait<MyStringView, crubit::ByBridge<>> {
  static constexpr size_t kSize = sizeof(std::string_view);
  static void Encode(MyStringView value, crubit::Encoder& encoder) {
    encoder.EncodeTransmute(std::move(value).view);
  }
  static MyStringView Decode(crubit::Decoder& decoder) {
    return {.view = decoder.DecodeTransmute<std::string_view>()};
  }
};

bool SaysHello(MyStringView view);
MyStringView MakeHello();

// Bridge type with a template type argument.
template <typename T>
struct CRUBIT_BRIDGE("Vec3") Vec3 {
  T x;
  T y;
  T z;
};

template <typename T, typename S>
struct crubit::CrubitAbiTrait<Vec3<T>, crubit::ByBridge<S>> {
  static constexpr size_t kSize = crubit::CrubitAbiSize<T, S>() * 3;
  static void Encode(Vec3<T> value, crubit::Encoder& encoder) {
    encoder.Encode<T, S>(std::move(value.x));
    encoder.Encode<T, S>(std::move(value.y));
    encoder.Encode<T, S>(std::move(value.z));
  }
  static Vec3<T> Decode(crubit::Decoder& decoder) {
    return {
        .x = decoder.Decode<T, S>(),
        .y = decoder.Decode<T, S>(),
        .z = decoder.Decode<T, S>(),
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
