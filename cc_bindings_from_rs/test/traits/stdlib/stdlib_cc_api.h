// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// stdlib_golden
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector,
// supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_STDLIB_STDLIB_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_STDLIB_STDLIB_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/rs_std/traits.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

namespace stdlib {

// Generated from:
// cc_bindings_from_rs/test/traits/stdlib/stdlib.rs;l=16
struct CRUBIT_INTERNAL_RUST_TYPE(":: stdlib_golden :: MyStruct") alignas(4)
    [[clang::trivial_abi]] MyStruct final {
 public:
  // Default::default
  MyStruct();

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyStruct() = default;
  MyStruct(MyStruct&&) = default;
  MyStruct& operator=(MyStruct&&) = default;

  // `stdlib_golden::MyStruct` doesn't implement the `Clone` trait
  MyStruct(const MyStruct&) = delete;
  MyStruct& operator=(const MyStruct&) = delete;
  MyStruct(::crubit::UnsafeRelocateTag, MyStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/traits/stdlib/stdlib.rs;l=17
    std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/traits/stdlib/stdlib.rs;l=5
struct CRUBIT_INTERNAL_RUST_TYPE(":: stdlib_golden :: MyTrait") MyTrait {
  template <typename T>
  using impl = rs_std::impl<T, MyTrait>;
};

static_assert(
    sizeof(MyStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::stdlib::MyStruct* __ret_ptr);
}
inline MyStruct::MyStruct() { __crubit_internal::__crubit_thunk_default(this); }
static_assert(std::is_trivially_destructible_v<MyStruct>);
static_assert(std::is_trivially_move_constructible_v<MyStruct>);
static_assert(std::is_trivially_move_assignable_v<MyStruct>);
inline void MyStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyStruct, x));
}
}  // namespace stdlib

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_STDLIB_STDLIB_GOLDEN
