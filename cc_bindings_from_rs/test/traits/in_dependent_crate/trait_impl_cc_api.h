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

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/rs_std/traits.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

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
  ::trait_impl::MyStruct& operator=(MyStruct&&) = default;

  // `trait_impl_golden::MyStruct` doesn't implement the `Clone` trait
  MyStruct(const MyStruct&) = delete;
  MyStruct& operator=(const MyStruct&) = delete;
  MyStruct(::crubit::UnsafeRelocateTag, MyStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/traits/in_dependent_crate/trait_impl.rs;l=12
  static ::trait_impl::MyStruct new_(std::int32_t x);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/traits/in_dependent_crate/trait_impl.rs;l=8
    std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/traits/in_dependent_crate/trait_impl.rs;l=23
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
  ::trait_impl::NotImplemented& operator=(NotImplemented&&) = delete;
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
static_assert(std::is_trivially_move_constructible_v<::trait_impl::MyStruct>);
static_assert(std::is_trivially_move_assignable_v<::trait_impl::MyStruct>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(std::int32_t,
                                   ::trait_impl::MyStruct* __ret_ptr);
}
inline ::trait_impl::MyStruct MyStruct::new_(std::int32_t x) {
  crubit::Slot<::trait_impl::MyStruct> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
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
struct rs_std::impl<::trait_impl::MyStruct, ::trait_definition::MyTrait> {
  static constexpr bool kIsImplemented = true;

  // Generated from:
  // cc_bindings_from_rs/test/traits/in_dependent_crate/trait_impl.rs;l=18
  static std::int32_t do_something(::trait_impl::MyStruct const& self);
};

namespace trait_impl {
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_MyTrait_udo_usomething(
    ::trait_impl::MyStruct const&);
}
}  // namespace trait_impl
inline std::int32_t
rs_std::impl<::trait_impl::MyStruct, ::trait_definition::MyTrait>::do_something(
    ::trait_impl::MyStruct const& self) {
  return trait_impl::__crubit_internal::__crubit_thunk_MyTrait_udo_usomething(
      self);
}

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_IN_DEPENDENT_CRATE_TRAIT_IMPL_GOLDEN
