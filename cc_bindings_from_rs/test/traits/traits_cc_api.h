// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// traits_golden
// Features: assume_lifetimes, custom_ffi_types, experimental, non_unpin_ctor,
// std_unique_ptr, std_vector, supported, wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_TRAITS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_TRAITS_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

namespace traits {

// Error generating bindings for `GenericTrait` defined at
// cc_bindings_from_rs/test/traits/traits.rs;l=17:
// Trait is not yet supported

// Generated from:
// cc_bindings_from_rs/test/traits/traits.rs;l=49
struct CRUBIT_INTERNAL_RUST_TYPE(":: traits_golden :: LifetimeStruct") alignas(
    8) [[clang::trivial_abi]] LifetimeStruct final {
 public:
  // `traits_golden::LifetimeStruct` doesn't implement the `Default` trait
  LifetimeStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~LifetimeStruct() = default;
  LifetimeStruct(LifetimeStruct&&) = default;
  LifetimeStruct& operator=(LifetimeStruct&&) = default;

  // `traits_golden::LifetimeStruct` doesn't implement the `Clone` trait
  LifetimeStruct(const LifetimeStruct&) = delete;
  LifetimeStruct& operator=(const LifetimeStruct&) = delete;
  LifetimeStruct(::crubit::UnsafeRelocateTag, LifetimeStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  // Field type has been replaced with a blob of bytes: Can't format `&i32`,
  // because references are only supported in function parameter types, return
  // types, and consts (b/286256327)
  unsigned char x[8];

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/traits/traits.rs;l=21
template <typename Type>
struct CRUBIT_INTERNAL_RUST_TYPE(":: traits_golden :: LifetimeTrait")
    LifetimeTrait {
  static constexpr bool is_implemented = false;
};

// Generated from:
// cc_bindings_from_rs/test/traits/traits.rs;l=27
struct CRUBIT_INTERNAL_RUST_TYPE(":: traits_golden :: MyStruct") alignas(4)
    [[clang::trivial_abi]] MyStruct final {
 public:
  // `traits_golden::MyStruct` doesn't implement the `Default` trait
  MyStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyStruct() = default;
  MyStruct(MyStruct&&) = default;
  MyStruct& operator=(MyStruct&&) = default;

  // `traits_golden::MyStruct` doesn't implement the `Clone` trait
  MyStruct(const MyStruct&) = delete;
  MyStruct& operator=(const MyStruct&) = delete;
  MyStruct(::crubit::UnsafeRelocateTag, MyStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/traits/traits.rs;l=28
    std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/traits/traits.rs;l=8
template <typename Type>
struct CRUBIT_INTERNAL_RUST_TYPE(":: traits_golden :: MyTrait") MyTrait {
  static constexpr bool is_implemented = false;
};

static_assert(
    sizeof(LifetimeStruct) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(LifetimeStruct) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<LifetimeStruct>);
static_assert(std::is_trivially_move_constructible_v<LifetimeStruct>);
static_assert(std::is_trivially_move_assignable_v<LifetimeStruct>);
inline void LifetimeStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(LifetimeStruct, x));
}
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
  static_assert(0 == offsetof(MyStruct, x));
}
}  // namespace traits
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_TRAITS_GOLDEN
