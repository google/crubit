// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_COMPOSABLE_BRIDGING_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_COMPOSABLE_BRIDGING_LIB_H_

#include <cstddef>
#include <optional>
#include <string>
#include <string_view>
#include <utility>

#include "absl/status/statusor.h"
#include "rs_bindings_from_cc/test/bridging/rust_library.h"
#include "support/annotations.h"
#include "support/bridge.h"
#include "support/rs_std/slice_ref.h"

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

namespace inner {
class Foo {
 public:
  int Bar() { return 42; }
};
}  // namespace inner
inline std::optional<inner::Foo> MakeOptionalFoo() { return inner::Foo(); }

std::pair<std::pair<int, float>, bool> MakePair(int a, float b, bool c);

std::pair<std::optional<int>, std::optional<std::pair<float, Vec3<float>>>>
MakeStuff();

// Not a bridge type!
struct [[clang::annotate("crubit_internal_trait_derive", "Debug", "PartialEq")]]
Stuff {
  int i;
  float f;
};

Vec3<Stuff> MakeVec3OfStructs(Stuff x, Stuff y, Stuff z);

std::string ReturnProperGreeting();

bool IsProperGreeting(std::string greeting);

std::pair<std::string, Stuff> ProperlyGreetStuff(Stuff stuff);

std::string_view StringViewByValue(std::string_view sv);

std::optional<std::string_view> ReturnOptionalStringView(bool is_present,
                                                         std::string_view sv);

rs_std::SliceRef<const std::string_view> ReturnSliceRefStringView(
    rs_std::SliceRef<const std::string_view> slice);

CRUBIT_UNSAFE_MARK_SAFE
absl::StatusOr<void*> AcceptsVoidPtrAndReturnsStatusErrorIfNull(void* ptr);

CRUBIT_UNSAFE_MARK_SAFE
absl::StatusOr<rs_std::SliceRef<const int>>
AcceptsSliceAndReturnsStatusErrorIfEmpty(rs_std::SliceRef<const int> slice);

CRUBIT_MUST_BIND
inline std::optional<rust_library::MyStruct> ReturnOptionalMyStruct() {
  return rust_library::MyStruct();
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_COMPOSABLE_BRIDGING_LIB_H_
