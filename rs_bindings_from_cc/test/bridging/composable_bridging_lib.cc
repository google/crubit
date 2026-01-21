// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/bridging/composable_bridging_lib.h"

#include <optional>
#include <string>
#include <string_view>
#include <utility>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "support/rs_std/slice_ref.h"

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

Vec3<Stuff> MakeVec3OfStructs(Stuff x, Stuff y, Stuff z) { return {x, y, z}; }

std::string ReturnProperGreeting() { return "Hello, world!"; }

bool IsProperGreeting(std::string greeting) {
  return greeting == "Hello, world!";
}

std::pair<std::string, Stuff> ProperlyGreetStuff(Stuff stuff) {
  return {"Hello, world!", stuff};
}

std::string_view StringViewByValue(std::string_view sv) { return sv; }

std::optional<std::string_view> ReturnOptionalStringView(bool is_present,
                                                         std::string_view sv) {
  if (!is_present) {
    return std::nullopt;
  }
  return sv;
}

rs_std::SliceRef<const std::string_view> ReturnSliceRefStringView(
    rs_std::SliceRef<const std::string_view> slice) {
  return slice;
}

absl::StatusOr<void*> AcceptsVoidPtrAndReturnsStatusErrorIfNull(void* ptr) {
  if (ptr == nullptr) {
    return absl::InvalidArgumentError("ptr is null");
  }
  return ptr;
}

absl::StatusOr<rs_std::SliceRef<const int>>
AcceptsSliceAndReturnsStatusErrorIfEmpty(rs_std::SliceRef<const int> slice) {
  if (slice.size() == 0) {
    return absl::InvalidArgumentError("slice is empty");
  }
  return slice;
}

std::optional<MyEnum> ValidateMyEnum(MyEnum value) {
  switch (value) {
    case MyEnum::kFoo:
    case MyEnum::kBar:
      return value;
    default:
      return std::nullopt;
  }
}
