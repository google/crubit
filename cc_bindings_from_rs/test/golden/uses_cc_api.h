// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// uses_rust

// clang-format off
#pragma once

#include "support/internal/attribute_macros.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

namespace uses_rust {

namespace test_use_glob {

// Generated from:
// cc_bindings_from_rs/test/golden/uses.rs;l=8
std::int32_t f1();

// Generated from:
// cc_bindings_from_rs/test/golden/uses.rs;l=12
std::int32_t f2();

// Generated from:
// cc_bindings_from_rs/test/golden/uses.rs;l=20
struct CRUBIT_INTERNAL_RUST_TYPE(":: uses_rust :: test_use_glob :: X1") alignas(
    4) [[clang::trivial_abi]] X1 final {
 public:
  // `test_use_glob::X1` doesn't implement the `Default` trait
  X1() = delete;

  // No custom `Drop` impl and no custom \"drop glue\" required
  ~X1() = default;
  X1(X1&&) = default;
  X1& operator=(X1&&) = default;

  // `test_use_glob::X1` doesn't implement the `Clone` trait
  X1(const X1&) = delete;
  X1& operator=(const X1&) = delete;

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/golden/uses.rs;l=21
    std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace test_use_glob

using ::uses_rust::test_use_glob::f1;
using ::uses_rust::test_use_glob::f2;
using X1 = ::uses_rust::test_use_glob::X1;

namespace test_use_glob {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_f1();
}
inline std::int32_t f1() { return __crubit_internal::__crubit_thunk_f1(); }

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_f2();
}
inline std::int32_t f2() { return __crubit_internal::__crubit_thunk_f2(); }

static_assert(
    sizeof(X1) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(X1) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<X1>);
static_assert(std::is_trivially_move_constructible_v<X1>);
static_assert(std::is_trivially_move_assignable_v<X1>);
inline void X1::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(X1, x));
}
}  // namespace test_use_glob

}  // namespace uses_rust
