// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/bridging/composable_bridging_lib.h"

#include <optional>
#include <string_view>
#include <utility>

bool SaysHello(MyStringView view) { return view.view == "Hello"; }

MyStringView MakeHello() { return {.view = "Hello"}; }

Vec3<float> MakeVec3(float x, float y, float z) {
  return {
      .x = x,
      .y = y,
      .z = z,
  };
}

std::optional<Vec3<float>> MakeOptionalVec3(float x, float y, float z,
                                            bool is_present) {
  if (!is_present) {
    return std::nullopt;
  }
  return {{x, y, z}};
}

std::optional<Vec3<float>> MapMultiply(std::optional<Vec3<float>> v,
                                       float factor) {
  if (!v.has_value()) {
    return std::nullopt;
  }
  return {{
      .x = v->x * factor,
      .y = v->y * factor,
      .z = v->z * factor,
  }};
}

std::pair<std::pair<int, float>, bool> MakePair(int a, float b, bool c) {
  return {{a, b}, c};
}

std::pair<std::optional<int>, std::optional<std::pair<float, Vec3<float>>>>
MakeStuff() {
  return {std::nullopt, {{3.14f, {1.0f, 2.0f, 3.0f}}}};
}
