// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate_golden
// Features: supported, unsafe_types

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_RUST_ENUM_EXAMPLE_CRATE_GOLDEN
#define THIRD_PARTY_CRUBIT_EXAMPLES_RUST_ENUM_EXAMPLE_CRATE_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <type_traits>

namespace example_crate {

// Generated from: examples/rust/enum/example.rs;l=6
struct CRUBIT_INTERNAL_RUST_TYPE(":: example_crate_golden :: Color") alignas(1)
    [[clang::trivial_abi]] Color final {
 public:
  // Default::default
  Color();

  // No custom `Drop` impl and no custom "drop glue" required
  ~Color() = default;
  Color(Color&&) = default;
  Color& operator=(Color&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Color(const Color&) = default;
  Color& operator=(const Color&) = default;
  Color(::crubit::UnsafeRelocateTag, Color&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  unsigned char __opaque_blob_of_bytes[1];

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(Color) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Color) == 1,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::example_crate::Color* __ret_ptr);
}
inline Color::Color() { __crubit_internal::__crubit_thunk_default(this); }
static_assert(std::is_trivially_destructible_v<Color>);
static_assert(std::is_trivially_move_constructible_v<Color>);
static_assert(std::is_trivially_move_assignable_v<Color>);
static_assert(std::is_trivially_copy_constructible_v<Color>);
static_assert(std::is_trivially_copy_assignable_v<Color>);
inline void Color::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Color, __opaque_blob_of_bytes));
}
}  // namespace example_crate
#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_RUST_ENUM_EXAMPLE_CRATE_GOLDEN
