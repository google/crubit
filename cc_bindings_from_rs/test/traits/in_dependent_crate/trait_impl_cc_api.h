// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// trait_impl_golden
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector,
// supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_IN_DEPENDENT_CRATE_TRAIT_IMPL_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_IN_DEPENDENT_CRATE_TRAIT_IMPL_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

#include "cc_bindings_from_rs/test/traits/in_dependent_crate/trait_definition.h"

namespace trait_impl {

// Generated from:
// cc_bindings_from_rs/test/traits/in_dependent_crate/trait_impl.rs;l=7
struct CRUBIT_INTERNAL_RUST_TYPE(":: trait_impl_golden :: MyStruct") alignas(4)
    [[clang::trivial_abi]] MyStruct final {
 public:
  // `trait_impl_golden::MyStruct` doesn't implement the `Default` trait
  MyStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyStruct() = default;
  MyStruct(MyStruct&&) = default;
  MyStruct& operator=(MyStruct&&) = default;

  // `trait_impl_golden::MyStruct` doesn't implement the `Clone` trait
  MyStruct(const MyStruct&) = delete;
  MyStruct& operator=(const MyStruct&) = delete;
  MyStruct(::crubit::UnsafeRelocateTag, MyStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/traits/in_dependent_crate/trait_impl.rs;l=8
    std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/traits/in_dependent_crate/trait_impl.rs;l=17
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: trait_impl_golden :: NotImplemented") alignas(8) [[clang::trivial_abi]]
NotImplemented final {
 public:
  // `trait_impl_golden::NotImplemented` doesn't implement the `Default` trait
  NotImplemented() = delete;

  // Drop::drop
  ~NotImplemented();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  NotImplemented(NotImplemented&&) = delete;
  NotImplemented& operator=(NotImplemented&&) = delete;
  // `trait_impl_golden::NotImplemented` doesn't implement the `Clone` trait
  NotImplemented(const NotImplemented&) = delete;
  NotImplemented& operator=(const NotImplemented&) = delete;
  NotImplemented(::crubit::UnsafeRelocateTag, NotImplemented&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  // Field type has been replaced with a blob of bytes: Type
  // `std::string::String` comes from the `alloc` crate, but no `--crate-header`
  // was specified for this crate
  unsigned char foo[24];

 private:
  static void __crubit_field_offset_assertions();
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
  static_assert(0 == offsetof(MyStruct, x));
}
static_assert(
    sizeof(NotImplemented) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NotImplemented) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::trait_impl::NotImplemented&);
}
inline NotImplemented::~NotImplemented() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
inline void NotImplemented::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NotImplemented, foo));
}
}  // namespace trait_impl

template <>
struct ::trait_definition::MyTrait<::trait_impl::MyStruct> {
  static constexpr bool is_implemented = true;
};

#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_IN_DEPENDENT_CRATE_TRAIT_IMPL_GOLDEN
