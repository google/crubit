// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// rs_index_golden
// Features: callables, supported, types

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_INDEX_RS_INDEX_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_INDEX_RS_INDEX_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/str_ref.h"
#include "support/rs_std/traits.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <tuple>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_core.h"

namespace rs_index {

// Generated from:
// cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=67
struct CRUBIT_INTERNAL_RUST_TYPE(":: rs_index_golden :: CustomIndex") alignas(8)
    [[clang::trivial_abi]] CustomIndex final {
 public:
  // `rs_index_golden::CustomIndex` doesn't implement the `Default` trait
  CustomIndex() = delete;

  // Synthesized tuple constructor
  explicit CustomIndex(::std::uintptr_t __field0)
      : __field0(::std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~CustomIndex() = default;
  CustomIndex(CustomIndex&&) = default;
  CustomIndex& operator=(CustomIndex&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  CustomIndex(const CustomIndex&) = default;
  CustomIndex& operator=(const CustomIndex&) = default;
  CustomIndex(::crubit::UnsafeRelocateTag, CustomIndex&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=69
  static ::rs_index::CustomIndex new_(::std::uintptr_t index);

  template <typename TOther>
    requires(rs_std::where_v<CustomIndex, ::rs::core::cmp::PartialEq<TOther>>)
  friend bool operator==(const CustomIndex& lhs, const TOther& rhs) {
    using impl = rs_std::impl<CustomIndex, ::rs::core::cmp::PartialEq<TOther>>;
    return impl::eq(lhs, rhs);
  }

  union {
    // Generated from:
    // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=67
    ::std::uintptr_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=127
struct CRUBIT_INTERNAL_RUST_TYPE(":: rs_index_golden :: Id") alignas(4)
    [[clang::trivial_abi]] Id final {
 public:
  // `rs_index_golden::Id` doesn't implement the `Default` trait
  Id() = delete;

  // Synthesized tuple constructor
  explicit Id(::std::int32_t __field0) : __field0(::std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~Id() = default;
  Id(Id&&) = default;
  Id& operator=(Id&&) = default;

  // `rs_index_golden::Id` doesn't implement the `Clone` trait
  Id(const Id&) = delete;
  Id& operator=(const Id&) = delete;
  Id(::crubit::UnsafeRelocateTag, Id&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=129
  static ::rs_index::Id new_(::std::int32_t id);

  template <typename TOther>
    requires(rs_std::where_v<Id, ::rs::core::cmp::PartialEq<TOther>>)
  friend bool operator==(const Id& lhs, const TOther& rhs) {
    using impl = rs_std::impl<Id, ::rs::core::cmp::PartialEq<TOther>>;
    return impl::eq(lhs, rhs);
  }

  union {
    // Generated from:
    // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=127
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=10
struct CRUBIT_INTERNAL_RUST_TYPE(":: rs_index_golden :: IntPair") alignas(4)
    [[clang::trivial_abi]] IntPair final {
 public:
  // `rs_index_golden::IntPair` doesn't implement the `Default` trait
  IntPair() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~IntPair() = default;
  IntPair(IntPair&&) = default;
  IntPair& operator=(IntPair&&) = default;

  // `rs_index_golden::IntPair` doesn't implement the `Clone` trait
  IntPair(const IntPair&) = delete;
  IntPair& operator=(const IntPair&) = delete;
  IntPair(::crubit::UnsafeRelocateTag, IntPair&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=16
  static ::rs_index::IntPair new_(::std::int32_t x, ::std::int32_t y);

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=22
  ::std::int32_t const& $(__anon1) operator[](::std::uintptr_t index) const&;

  // Error generating bindings for implementation `<rs_index_golden::IntPair as
  // std::ops::Index<u64>>` defined at
  // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=44:
  // Index implementation for `u64` is not supported when `Index<usize>` is
  // implemented as it may overlap.

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=75
  ::std::int32_t const& $(__anon1) operator[](
      ::rs_index::CustomIndex index) const&;

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=34
  ::std::int32_t& $(__anon1) operator[](::std::uintptr_t index) &;

  // Error generating bindings for implementation `<rs_index_golden::IntPair as
  // std::ops::IndexMut<u64>>` defined at
  // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=56:
  // Index implementation for `u64` is not supported when `Index<usize>` is
  // implemented as it may overlap.

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=87
  ::std::int32_t& $(__anon1) operator[](::rs_index::CustomIndex index) &;

  template <typename TOther>
    requires(rs_std::where_v<IntPair, ::rs::core::cmp::PartialEq<TOther>>)
  friend bool operator==(const IntPair& lhs, const TOther& rhs) {
    using impl = rs_std::impl<IntPair, ::rs::core::cmp::PartialEq<TOther>>;
    return impl::eq(lhs, rhs);
  }

  union {
    // Generated from:
    // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=11
    ::std::int32_t x;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=12
    ::std::int32_t y;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=97
struct CRUBIT_INTERNAL_RUST_TYPE(":: rs_index_golden :: Map") alignas(8)
    [[clang::trivial_abi]] Map final {
 public:
  // `rs_index_golden::Map` doesn't implement the `Default` trait
  Map() = delete;

  // Drop::drop
  ~Map();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  Map(Map&&) = delete;
  ::rs_index::Map& operator=(Map&&) = delete;
  // `rs_index_golden::Map` doesn't implement the `Clone` trait
  Map(const Map&) = delete;
  Map& operator=(const Map&) = delete;
  Map(::crubit::UnsafeRelocateTag, Map&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=103
  static ::rs_index::Map new_(::std::uintptr_t row_size,
                              ::std::uintptr_t col_size);

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=113
  rs_std::StrRef operator[](
      ::std::tuple<::std::uintptr_t, ::std::uintptr_t> index) const&;

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=134
  rs_std::StrRef operator[](::rs_index::Id const* crubit_nonnull index) const&;

  // Error generating bindings for implementation `<rs_index_golden::Map as
  // std::ops::IndexMut<(usize, usize)>>` defined at
  // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=121:
  // Mutable references to `str` are not yet supported.

  template <typename TOther>
    requires(rs_std::where_v<Map, ::rs::core::cmp::PartialEq<TOther>>)
  friend bool operator==(const Map& lhs, const TOther& rhs) {
    using impl = rs_std::impl<Map, ::rs::core::cmp::PartialEq<TOther>>;
    return impl::eq(lhs, rhs);
  }

 private:
  // Field type has been replaced with a blob of bytes: Generic types are not
  // supported yet (b/259749095)
  ::std::array<unsigned char, 24> data;
  union {
    // Generated from:
    // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=98
    ::std::uintptr_t row_size;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(CustomIndex) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CustomIndex) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CustomIndex>);
static_assert(
    ::std::is_trivially_move_constructible_v<::rs_index::CustomIndex>);
static_assert(::std::is_trivially_move_assignable_v<::rs_index::CustomIndex>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::rs_index::CustomIndex>);
static_assert(::std::is_trivially_copy_assignable_v<::rs_index::CustomIndex>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uintptr_t,
                                   ::rs_index::CustomIndex* __ret_ptr);
}
inline ::rs_index::CustomIndex CustomIndex::new_(::std::uintptr_t index) {
  crubit::Slot<::rs_index::CustomIndex> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(index, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void CustomIndex::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CustomIndex, __field0));
}
static_assert(
    sizeof(Id) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Id) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<Id>);
static_assert(::std::is_trivially_move_constructible_v<::rs_index::Id>);
static_assert(::std::is_trivially_move_assignable_v<::rs_index::Id>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::int32_t, ::rs_index::Id* __ret_ptr);
}
inline ::rs_index::Id Id::new_(::std::int32_t id) {
  crubit::Slot<::rs_index::Id> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(id, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void Id::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Id, __field0));
}
static_assert(
    sizeof(IntPair) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(IntPair) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<IntPair>);
static_assert(::std::is_trivially_move_constructible_v<::rs_index::IntPair>);
static_assert(::std::is_trivially_move_assignable_v<::rs_index::IntPair>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::int32_t, ::std::int32_t,
                                   ::rs_index::IntPair* __ret_ptr);
}
inline ::rs_index::IntPair IntPair::new_(::std::int32_t x, ::std::int32_t y) {
  crubit::Slot<::rs_index::IntPair> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, y, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
namespace __crubit_internal {
extern "C" ::std::int32_t const& $(__anon1)
    __crubit_thunk_index_uusize(::rs_index::IntPair const&, ::std::uintptr_t);
}
inline ::std::int32_t const& $(__anon1) IntPair::operator[](
    ::std::uintptr_t index) const& {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_index_uusize(self, index);
}
namespace __crubit_internal {
extern "C" ::std::int32_t const& $(__anon1)
    __crubit_thunk_index_urs_uindex_ugolden_x0000003a_x0000003aCustomIndex(
        ::rs_index::IntPair const&, ::rs_index::CustomIndex*);
}
inline ::std::int32_t const& $(__anon1) IntPair::operator[](
    ::rs_index::CustomIndex index) const& {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::
      __crubit_thunk_index_urs_uindex_ugolden_x0000003a_x0000003aCustomIndex(
          self, &index);
}
namespace __crubit_internal {
extern "C" ::std::int32_t& $(__anon1)
    __crubit_thunk_index_umut_uusize(::rs_index::IntPair&, ::std::uintptr_t);
}
inline ::std::int32_t& $(__anon1) IntPair::operator[](
    ::std::uintptr_t index) & {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_index_umut_uusize(self, index);
}
namespace __crubit_internal {
extern "C" ::std::int32_t& $(__anon1)
    __crubit_thunk_index_umut_urs_uindex_ugolden_x0000003a_x0000003aCustomIndex(
        ::rs_index::IntPair&, ::rs_index::CustomIndex*);
}
inline ::std::int32_t& $(__anon1) IntPair::operator[](
    ::rs_index::CustomIndex index) & {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_index_umut_urs_uindex_ugolden_x0000003a_x0000003aCustomIndex(
          self, &index);
}
inline void IntPair::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(IntPair, x));
  static_assert(4 == offsetof(IntPair, y));
}
static_assert(
    sizeof(Map) == 32,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Map) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::rs_index::Map&);
}
inline Map::~Map() { __crubit_internal::__crubit_thunk_drop(*this); }
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uintptr_t, ::std::uintptr_t,
                                   ::rs_index::Map* __ret_ptr);
}
inline ::rs_index::Map Map::new_(::std::uintptr_t row_size,
                                 ::std::uintptr_t col_size) {
  crubit::Slot<::rs_index::Map> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(row_size, col_size,
                                        __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
namespace __crubit_internal {
extern "C" rs_std::StrRef
__crubit_thunk_index_u_x00000028usize_x0000002c_x00000020usize_x00000029(
    ::rs_index::Map const&, void**);
}
inline rs_std::StrRef Map::operator[](
    ::std::tuple<::std::uintptr_t, ::std::uintptr_t> index) const& {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  auto&& index_0 = ::std::get<0>(index);
  auto&& index_cabi_0 = index_0;
  auto&& index_1 = ::std::get<1>(index);
  auto&& index_cabi_1 = index_1;
  void* index_cabi[] = {&index_cabi_0, &index_cabi_1};
  return __crubit_internal::
      __crubit_thunk_index_u_x00000028usize_x0000002c_x00000020usize_x00000029(
          self, index_cabi);
}
namespace __crubit_internal {
extern "C" rs_std::StrRef
__crubit_thunk_index_u_x00000026rs_uindex_ugolden_x0000003a_x0000003aId(
    ::rs_index::Map const&, ::rs_index::Id const* crubit_nonnull);
}
inline rs_std::StrRef Map::operator[](
    ::rs_index::Id const* crubit_nonnull index) const& {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::
      __crubit_thunk_index_u_x00000026rs_uindex_ugolden_x0000003a_x0000003aId(
          self, index);
}
inline void Map::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Map, data));
  static_assert(24 == offsetof(Map, row_size));
}
}  // namespace rs_index

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_INDEX_RS_INDEX_GOLDEN
