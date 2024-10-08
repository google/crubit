// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate
// Features: <none>

// clang-format off
#pragma once

#include "support/internal/attribute_macros.h"

#include <cstddef>
#include <type_traits>

namespace example_crate {

// Generated from: examples/rust/enum/example.rs;l=6
struct CRUBIT_INTERNAL_RUST_TYPE(":: example_crate :: Color") alignas(1)
    [[clang::trivial_abi]] Color final {
 public:
  // Default::default
  Color();

  // No custom `Drop` impl and no custom \"drop glue\" required
  ~Color() = default;
  Color(Color&&) = default;
  Color& operator=(Color&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Color(const Color&) = default;
  Color& operator=(const Color&) = default;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual `enum` fields
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
extern "C" void
__crubit_thunk__uRNvYNtCsh2mUQSogkZl_u13example_ucrate5ColorNtNtCs8sGNUgcxoFi_u4core7default7Default7defaultB4_u(
    ::example_crate::Color* __ret_ptr);
}
inline Color::Color() {
  __crubit_internal::
      __crubit_thunk__uRNvYNtCsh2mUQSogkZl_u13example_ucrate5ColorNtNtCs8sGNUgcxoFi_u4core7default7Default7defaultB4_u(
          this);
}
static_assert(std::is_trivially_destructible_v<Color>);
static_assert(std::is_trivially_move_constructible_v<Color>);
static_assert(std::is_trivially_move_assignable_v<Color>);
static_assert(std::is_trivially_copy_constructible_v<Color>);
static_assert(std::is_trivially_copy_assignable_v<Color>);
inline void Color::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Color, __opaque_blob_of_bytes));
}
}  // namespace example_crate
