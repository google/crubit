// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate_golden
// Features: supported, unsafe_types

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_RUST_USE_DECLARATION_EXAMPLE_CRATE_GOLDEN
#define THIRD_PARTY_CRUBIT_EXAMPLES_RUST_USE_DECLARATION_EXAMPLE_CRATE_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"

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
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: example_crate_golden :: module :: Type") alignas(4)
    [[clang::trivial_abi]] Type final {
 public:
  // Default::default
  Type();

  // No custom `Drop` impl and no custom "drop glue" required
  ~Type() = default;
  Type(Type&&) = default;
  Type& operator=(Type&&) = default;

  // `module::Type` doesn't implement the `Clone` trait
  Type(const Type&) = delete;
  Type& operator=(const Type&) = delete;
  Type(::crubit::UnsafeRelocateTag, Type&& value) {
    memcpy(this, &value, sizeof(value));
  }
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
using Type CRUBIT_INTERNAL_RUST_TYPE(
    ":: example_crate_golden :: module :: Type") =
    ::example_crate::module::Type;

namespace module {

namespace __crubit_internal {
extern "C" void __crubit_thunk_function();
}
inline void function() { return __crubit_internal::__crubit_thunk_function(); }

static_assert(
    sizeof(Type) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Type) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    ::example_crate::module::Type* __ret_ptr);
}
inline Type::Type() { __crubit_internal::__crubit_thunk_default(this); }
static_assert(std::is_trivially_destructible_v<Type>);
static_assert(std::is_trivially_move_constructible_v<Type>);
static_assert(std::is_trivially_move_assignable_v<Type>);
inline void Type::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Type, x));
}
}  // namespace module

}  // namespace example_crate
#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_RUST_USE_DECLARATION_EXAMPLE_CRATE_GOLDEN
