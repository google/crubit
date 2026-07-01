// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_RUST_USE_DECLARATION_EXAMPLE_CRATE_GOLDEN
#define THIRD_PARTY_CRUBIT_EXAMPLES_RUST_USE_DECLARATION_EXAMPLE_CRATE_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>

namespace example_crate {

struct CRUBIT_INTERNAL_RUST_TYPE(":: example_crate_golden :: Type") alignas(4)
    [[clang::trivial_abi]] Type final {
 public:
  // Default::default
  Type();

  // No custom `Drop` impl and no custom "drop glue" required
  ~Type() = default;
  Type(Type&&) = default;
  Type& operator=(Type&&) = default;

  // `example_crate_golden::Type` doesn't implement the `Clone` trait
  Type(const Type&) = delete;
  Type& operator=(const Type&) = delete;
  Type(::crubit::UnsafeRelocateTag, Type&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

void function();

}  // namespace example_crate

namespace example_crate::module {

using Type CRUBIT_INTERNAL_RUST_TYPE(":: example_crate_golden :: Type") =
    ::example_crate::Type;
}

namespace example_crate {

static_assert(
    sizeof(Type) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Type) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_uexample_ucrate_ugolden_x0000003a_x0000003aType(
    ::example_crate::Type* __ret_ptr);
}
inline ::example_crate::Type::Type() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_uexample_ucrate_ugolden_x0000003a_x0000003aType(
          this);
}
static_assert(::std::is_trivially_destructible_v<Type>);
static_assert(::std::is_trivially_move_constructible_v<::example_crate::Type>);
static_assert(::std::is_trivially_move_assignable_v<::example_crate::Type>);
inline void Type::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Type, x));
}
}  // namespace example_crate

namespace example_crate::module {
using ::example_crate::function;
}

namespace example_crate {

namespace __crubit_internal {
extern "C" void __crubit_thunk_function();
}
inline void function() { return __crubit_internal::__crubit_thunk_function(); }

}  // namespace example_crate

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_RUST_USE_DECLARATION_EXAMPLE_CRATE_GOLDEN
