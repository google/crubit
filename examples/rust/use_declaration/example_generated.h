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
#include <cstdint>
#include <type_traits>

namespace example_crate {

namespace module {

// Generated from:
// examples/rust/use_declaration/example.rs;l=6
void function();

// Generated from:
// examples/rust/use_declaration/example.rs;l=8
struct CRUBIT_INTERNAL_RUST_TYPE(":: example_crate :: module :: Type") alignas(
    4) [[clang::trivial_abi]] Type final {
 public:
  // Default::default
  Type();

  // No custom `Drop` impl and no custom \"drop glue\" required
  ~Type() = default;
  Type(Type&&) = default;
  Type& operator=(Type&&) = default;

  // `module::Type` doesn't implement the `Clone` trait
  Type(const Type&) = delete;
  Type& operator=(const Type&) = delete;

 public:
  union {
    // Generated from:
    // examples/rust/use_declaration/example.rs;l=9
    std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace module

using ::example_crate::module::function;
using Type = ::example_crate::module::Type;

namespace module {

namespace __crubit_internal {
extern "C" void
__crubit_thunk__uRNvNtCsh2mUQSogkZl_u13example_ucrate6module8function();
}
inline void function() {
  return __crubit_internal::
      __crubit_thunk__uRNvNtCsh2mUQSogkZl_u13example_ucrate6module8function();
}

static_assert(
    sizeof(Type) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Type) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk__uRNvYNtNtCsh2mUQSogkZl_u13example_ucrate6module4TypeNtNtCs8sGNUgcxoFi_u4core7default7Default7defaultB6_u(
    ::example_crate::module::Type* __ret_ptr);
}
inline Type::Type() {
  __crubit_internal::
      __crubit_thunk__uRNvYNtNtCsh2mUQSogkZl_u13example_ucrate6module4TypeNtNtCs8sGNUgcxoFi_u4core7default7Default7defaultB6_u(
          this);
}
static_assert(std::is_trivially_destructible_v<Type>);
static_assert(std::is_trivially_move_constructible_v<Type>);
static_assert(std::is_trivially_move_assignable_v<Type>);
inline void Type::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Type, x));
}
}  // namespace module

}  // namespace example_crate
