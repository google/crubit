// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// trait_definition_golden
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector,
// supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_IN_DEPENDENT_CRATE_TRAIT_DEFINITION_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_IN_DEPENDENT_CRATE_TRAIT_DEFINITION_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

namespace trait_definition {

// Generated from:
// cc_bindings_from_rs/test/traits/in_dependent_crate/trait_definition.rs;l=9
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: trait_definition_golden :: MyStruct") alignas(4) [[clang::trivial_abi]]
MyStruct final {
 public:
  // `trait_definition_golden::MyStruct` doesn't implement the `Default` trait
  MyStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyStruct() = default;
  MyStruct(MyStruct&&) = default;
  MyStruct& operator=(MyStruct&&) = default;

  // `trait_definition_golden::MyStruct` doesn't implement the `Clone` trait
  MyStruct(const MyStruct&) = delete;
  MyStruct& operator=(const MyStruct&) = delete;
  MyStruct(::crubit::UnsafeRelocateTag, MyStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/traits/in_dependent_crate/trait_definition.rs;l=10
    std::int32_t y;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/traits/in_dependent_crate/trait_definition.rs;l=5
template <typename Type>
struct CRUBIT_INTERNAL_RUST_TYPE(":: trait_definition_golden :: MyTrait")
    MyTrait {
  static constexpr bool is_implemented = false;
};

static_assert(
    sizeof(MyStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<MyStruct>);
static_assert(std::is_trivially_move_constructible_v<MyStruct>);
static_assert(std::is_trivially_move_assignable_v<MyStruct>);
inline void MyStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyStruct, y));
}
}  // namespace trait_definition

template <>
struct ::trait_definition::MyTrait<::trait_definition::MyStruct> {
  static constexpr bool is_implemented = true;
};

#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_IN_DEPENDENT_CRATE_TRAIT_DEFINITION_GOLDEN
