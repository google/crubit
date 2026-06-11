// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// stdlib_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_STDLIB_STDLIB_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_STDLIB_STDLIB_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/bridge.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/slice_ref.h"
#include "support/rs_std/traits.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <optional>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_alloc.h"
#include "support/rs_std/rs_core.h"

namespace stdlib {

struct CRUBIT_INTERNAL_RUST_TYPE(":: stdlib_golden :: MyStruct") alignas(4)
    [[clang::trivial_abi]] MyStruct final {
 public:
  // Default::default
  MyStruct();

  // Drop::drop
  ~MyStruct();

  MyStruct(MyStruct&&);
  ::stdlib::MyStruct& operator=(MyStruct&&);

  // Clone::clone
  MyStruct(const MyStruct&);

  // Clone::clone_from
  ::stdlib::MyStruct& operator=(const MyStruct&);

  MyStruct(::crubit::UnsafeRelocateTag, MyStruct&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  static ::stdlib::MyStruct new_(::std::int32_t x);

  explicit MyStruct(::std::int32_t value);

  union {
    ::std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: stdlib_golden :: MyTrait") MyTrait {
  template <typename T>
  using impl = rs_std::impl<T, MyTrait>;
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: stdlib_golden :: NonCloneableIterator") alignas(4)
    [[clang::trivial_abi]] NonCloneableIterator final {
 public:
  // `stdlib_golden::NonCloneableIterator` doesn't implement the `Default` trait
  NonCloneableIterator() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~NonCloneableIterator() = default;
  NonCloneableIterator(NonCloneableIterator&&) = default;
  NonCloneableIterator& operator=(NonCloneableIterator&&) = default;

  // `stdlib_golden::NonCloneableIterator` doesn't implement the `Clone` trait
  NonCloneableIterator(const NonCloneableIterator&) = delete;
  NonCloneableIterator& operator=(const NonCloneableIterator&) = delete;
  NonCloneableIterator(::crubit::UnsafeRelocateTag,
                       NonCloneableIterator&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  static ::stdlib::NonCloneableIterator new_(::std::int32_t x);

  union {
    ::std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: stdlib_golden :: NonCloneableValue") alignas(4) [[clang::trivial_abi]]
NonCloneableValue final {
 public:
  // `stdlib_golden::NonCloneableValue` doesn't implement the `Default` trait
  NonCloneableValue() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~NonCloneableValue() = default;
  NonCloneableValue(NonCloneableValue&&) = default;
  NonCloneableValue& operator=(NonCloneableValue&&) = default;

  // `stdlib_golden::NonCloneableValue` doesn't implement the `Clone` trait
  NonCloneableValue(const NonCloneableValue&) = delete;
  NonCloneableValue& operator=(const NonCloneableValue&) = delete;
  NonCloneableValue(::crubit::UnsafeRelocateTag, NonCloneableValue&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: stdlib_golden :: RefIterator") alignas(8)
    [[clang::trivial_abi]] RefIterator final {
 public:
  // `stdlib_golden::RefIterator` doesn't implement the `Default` trait
  RefIterator() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~RefIterator() = default;
  RefIterator(RefIterator&&) = default;
  RefIterator& operator=(RefIterator&&) = default;

  // `stdlib_golden::RefIterator` doesn't implement the `Clone` trait
  RefIterator(const RefIterator&) = delete;
  RefIterator& operator=(const RefIterator&) = delete;
  RefIterator(::crubit::UnsafeRelocateTag, RefIterator&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  static ::stdlib::RefIterator new_(
      rs_std::SliceRef<const ::std::int32_t> slice);

  union {
    rs_std::SliceRef<const ::std::int32_t> slice;
  };
  union {
    ::std::uintptr_t index;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace stdlib

template <>
struct rs_std::impl<::stdlib::MyStruct, ::rs::core::future::Future> {
  static constexpr bool kIsImplemented = true;
  using Output CRUBIT_INTERNAL_RUST_TYPE(
      "<stdlib_golden::MyStruct as :: core :: future :: Future>::Output") =
      ::std::int32_t;

  // Error generating bindings for associated function `<stdlib_golden::MyStruct
  // as std::future::Future>::poll` defined at
  // cc_bindings_from_rs/test/traits/stdlib/stdlib.rs;l=72:
  // Unsupported `self` type `std::pin::Pin<&'__anon1 mut
  // stdlib_golden::MyStruct>`
};

template <>
struct rs_std::impl<::stdlib::MyStruct, ::rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<stdlib_golden::MyStruct as :: core :: iter :: Iterator>::Item") =
      ::std::int32_t;

  static ::std::optional<::std::int32_t> next(::stdlib::MyStruct& self);
};

template <>
struct rs_std::impl<::stdlib::MyStruct, ::rs::alloc::string::ToString> {
  static constexpr bool kIsImplemented = true;

  static ::rs::alloc::string::String to_string(::stdlib::MyStruct const& self);
};

template <>
struct rs_std::impl<::stdlib::NonCloneableIterator,
                    ::rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<stdlib_golden::NonCloneableIterator as :: core :: iter :: "
      "Iterator>::Item") = ::stdlib::NonCloneableValue;

  static ::std::optional<::stdlib::NonCloneableValue> next(
      ::stdlib::NonCloneableIterator& self);
};

template <>
struct rs_std::impl<::stdlib::RefIterator, ::rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<stdlib_golden::RefIterator<'a> as :: core :: iter :: Iterator>::Item") =
      ::std::int32_t const* $a crubit_nonnull;

  static ::std::optional<::std::int32_t const * $a crubit_nonnull> next(
      ::stdlib::RefIterator& self);
};

namespace stdlib {

static_assert(
    sizeof(MyStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    ::stdlib::MyStruct* crubit_nonnull __ret_ptr);
}
inline ::stdlib::MyStruct::MyStruct() {
  __crubit_internal::__crubit_thunk_default(this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::stdlib::MyStruct&);
}
inline MyStruct::~MyStruct() { __crubit_internal::__crubit_thunk_drop(*this); }
inline ::stdlib::MyStruct::MyStruct(MyStruct&& other) : MyStruct() {
  *this = ::std::move(other);
}
inline ::stdlib::MyStruct& ::stdlib::MyStruct::operator=(MyStruct&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    ::stdlib::MyStruct const&, ::stdlib::MyStruct* crubit_nonnull __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(::stdlib::MyStruct&,
                                           ::stdlib::MyStruct const&);
}
inline ::stdlib::MyStruct::MyStruct(const MyStruct& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline ::stdlib::MyStruct& ::stdlib::MyStruct::operator=(
    const MyStruct& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(
    ::std::int32_t, ::stdlib::MyStruct* crubit_nonnull __ret_ptr);
}
inline ::stdlib::MyStruct MyStruct::new_(::std::int32_t x) {
  crubit::Slot<::stdlib::MyStruct> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_from_ui32(
    ::std::int32_t, ::stdlib::MyStruct* crubit_nonnull __ret_ptr);
}
inline MyStruct::MyStruct(::std::int32_t value) {
  __crubit_internal::__crubit_thunk_from_ui32(value, this);
}
inline void MyStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyStruct, x));
}
static_assert(
    sizeof(NonCloneableIterator) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NonCloneableIterator) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NonCloneableIterator>);
static_assert(
    ::std::is_trivially_move_constructible_v<::stdlib::NonCloneableIterator>);
static_assert(
    ::std::is_trivially_move_assignable_v<::stdlib::NonCloneableIterator>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(
    ::std::int32_t, ::stdlib::NonCloneableIterator* crubit_nonnull __ret_ptr);
}
inline ::stdlib::NonCloneableIterator NonCloneableIterator::new_(
    ::std::int32_t x) {
  crubit::Slot<::stdlib::NonCloneableIterator> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void NonCloneableIterator::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NonCloneableIterator, x));
}
static_assert(
    sizeof(NonCloneableValue) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NonCloneableValue) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NonCloneableValue>);
static_assert(
    ::std::is_trivially_move_constructible_v<::stdlib::NonCloneableValue>);
static_assert(
    ::std::is_trivially_move_assignable_v<::stdlib::NonCloneableValue>);
inline void NonCloneableValue::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NonCloneableValue, x));
}
static_assert(
    sizeof(RefIterator) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(RefIterator) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<RefIterator>);
static_assert(::std::is_trivially_move_constructible_v<::stdlib::RefIterator>);
static_assert(::std::is_trivially_move_assignable_v<::stdlib::RefIterator>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(rs_std::SliceRef<const ::std::int32_t>,
                                   ::stdlib::RefIterator* crubit_nonnull
                                       __ret_ptr);
}
inline ::stdlib::RefIterator RefIterator::new_(
    rs_std::SliceRef<const ::std::int32_t> slice) {
  crubit::Slot<::stdlib::RefIterator> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(slice, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void RefIterator::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(RefIterator, slice));
  static_assert(16 == offsetof(RefIterator, index));
}
}  // namespace stdlib

namespace stdlib {
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Iterator_unext_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
    ::stdlib::MyStruct&, unsigned char* crubit_nonnull __ret_ptr);
}
}  // namespace stdlib
inline ::std::optional<::std::int32_t>
rs_std::impl<::stdlib::MyStruct, ::rs::core::iter::Iterator>::next(
    ::stdlib::MyStruct& self) {
  unsigned char __return_value_storage
      [::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>::kSize];
  stdlib::__crubit_internal::
      __crubit_thunk_Iterator_unext_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
          self, __return_value_storage);
  return ::crubit::internal::Decode<
      ::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>>(
      ::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>(
          ::crubit::TransmuteAbi<::std::int32_t>()),
      __return_value_storage);
}

namespace stdlib {
namespace __crubit_internal {
extern "C" void
__crubit_thunk_ToString_uto_ustring_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
    ::stdlib::MyStruct const&,
    ::rs::alloc::string::String* crubit_nonnull __ret_ptr);
}
}  // namespace stdlib
inline ::rs::alloc::string::String
rs_std::impl<::stdlib::MyStruct, ::rs::alloc::string::ToString>::to_string(
    ::stdlib::MyStruct const& self) {
  crubit::Slot<::rs::alloc::string::String> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  stdlib::__crubit_internal::
      __crubit_thunk_ToString_uto_ustring_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
          self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace stdlib {
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Iterator_unext_ustdlib_ugolden_x0000003a_x0000003aNonCloneableIterator(
    ::stdlib::NonCloneableIterator&, unsigned char* crubit_nonnull __ret_ptr);
}
}  // namespace stdlib
inline ::std::optional<::stdlib::NonCloneableValue>
rs_std::impl<::stdlib::NonCloneableIterator, ::rs::core::iter::Iterator>::next(
    ::stdlib::NonCloneableIterator& self) {
  unsigned char __return_value_storage[::crubit::OptionAbi<
      ::crubit::TransmuteAbi<::stdlib::NonCloneableValue>>::kSize];
  stdlib::__crubit_internal::
      __crubit_thunk_Iterator_unext_ustdlib_ugolden_x0000003a_x0000003aNonCloneableIterator(
          self, __return_value_storage);
  return ::crubit::internal::Decode<
      ::crubit::OptionAbi<::crubit::TransmuteAbi<::stdlib::NonCloneableValue>>>(
      ::crubit::OptionAbi<::crubit::TransmuteAbi<::stdlib::NonCloneableValue>>(
          ::crubit::TransmuteAbi<::stdlib::NonCloneableValue>()),
      __return_value_storage);
}

namespace stdlib {
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Iterator_unext_ustdlib_ugolden_x0000003a_x0000003aRefIterator_x0000003c_x00000027a_x0000003e(
    ::stdlib::RefIterator&, unsigned char* crubit_nonnull __ret_ptr);
}
}  // namespace stdlib
inline ::std::optional<::std::int32_t const * $a crubit_nonnull>
rs_std::impl<::stdlib::RefIterator, ::rs::core::iter::Iterator>::next(
    ::stdlib::RefIterator& self) {
  unsigned char
      __return_value_storage[::crubit::OptionAbi<::crubit::TransmuteAbi<
          ::std::int32_t const * $static crubit_nonnull>>::kSize];
  stdlib::__crubit_internal::
      __crubit_thunk_Iterator_unext_ustdlib_ugolden_x0000003a_x0000003aRefIterator_x0000003c_x00000027a_x0000003e(
          self, __return_value_storage);
  return ::crubit::internal::Decode<::crubit::OptionAbi<
      ::crubit::TransmuteAbi<::std::int32_t const * $static crubit_nonnull>>>(
      ::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t const *
                                                 $static crubit_nonnull>>(
          ::crubit::TransmuteAbi<::std::int32_t const *
                                 $static crubit_nonnull>()),
      __return_value_storage);
}

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_STDLIB_STDLIB_GOLDEN
