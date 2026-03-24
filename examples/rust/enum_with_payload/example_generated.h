// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate_golden
// Features: supported, types

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_RUST_ENUM_WITH_PAYLOAD_EXAMPLE_CRATE_GOLDEN
#define THIRD_PARTY_CRUBIT_EXAMPLES_RUST_ENUM_WITH_PAYLOAD_EXAMPLE_CRATE_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

namespace example_crate {

// Generated from:
// examples/rust/enum_with_payload/example.rs;l=6
struct CRUBIT_INTERNAL_RUST_TYPE(":: example_crate_golden :: Color") alignas(1)
    [[clang::trivial_abi]] Color final {
 public:
  // `example_crate_golden::Color` doesn't implement the `Default` trait
  Color() = delete;

  //  A completely transparent color (no payload)
  //
  // Generated from:
  // examples/rust/enum_with_payload/example.rs;l=8
  static constexpr Color MakeTransparent();

  //  A grayscale value from 0 to 255
  //
  // Generated from:
  // examples/rust/enum_with_payload/example.rs;l=10
  static ::example_crate::Color MakeGrayscale(std::uint8_t __param_0);

  //  Red, Green, and Blue values from 0 to 255
  //
  // Generated from:
  // examples/rust/enum_with_payload/example.rs;l=12
  static ::example_crate::Color MakeRgb(std::uint8_t __param_0,
                                        std::uint8_t __param_1,
                                        std::uint8_t __param_2);

  // No custom `Drop` impl and no custom "drop glue" required
  ~Color() = default;
  Color(Color&&) = default;
  ::example_crate::Color& operator=(Color&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Color(const Color&) = default;
  ::example_crate::Color& operator=(const Color&) = default;
  Color(::crubit::UnsafeRelocateTag, Color&& value) {
    std::memcpy(this, &value, sizeof(value));
  }

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  std::array<unsigned char, 4> __opaque_blob_of_bytes;

 private:
  struct PrivateBytesTag {};
  constexpr Color(PrivateBytesTag, std::array<unsigned char, 4> bytes)
      : __opaque_blob_of_bytes(bytes) {}

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(Color) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Color) == 1,
    "Verify that ADT layout didn't change since this header got generated");

// `static` constructor
inline constexpr Color Color::MakeTransparent() {
  return Color(PrivateBytesTag{}, {0, 0, 0, 0});
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_Grayscale(std::uint8_t,
                                         ::example_crate::Color* __ret_ptr);
}
inline ::example_crate::Color Color::MakeGrayscale(std::uint8_t __param_0) {
  crubit::Slot<::example_crate::Color> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_Grayscale(__param_0,
                                              __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_Rgb(std::uint8_t, std::uint8_t, std::uint8_t,
                                   ::example_crate::Color* __ret_ptr);
}
inline ::example_crate::Color Color::MakeRgb(std::uint8_t __param_0,
                                             std::uint8_t __param_1,
                                             std::uint8_t __param_2) {
  crubit::Slot<::example_crate::Color> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_Rgb(__param_0, __param_1, __param_2,
                                        __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
static_assert(std::is_trivially_destructible_v<Color>);
static_assert(std::is_trivially_move_constructible_v<::example_crate::Color>);
static_assert(std::is_trivially_move_assignable_v<::example_crate::Color>);
static_assert(std::is_trivially_copy_constructible_v<::example_crate::Color>);
static_assert(std::is_trivially_copy_assignable_v<::example_crate::Color>);
inline void Color::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Color, __opaque_blob_of_bytes));
}
}  // namespace example_crate

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_RUST_ENUM_WITH_PAYLOAD_EXAMPLE_CRATE_GOLDEN
