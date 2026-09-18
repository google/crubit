// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// rs_index_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_INDEX_RS_INDEX_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_INDEX_RS_INDEX_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/check.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/str_ref.h"
#include "support/rs_std/vec.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <memory>
#include <new>
#include <tuple>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_alloc.h"

namespace rs_index {

struct CRUBIT_INTERNAL_RUST_TYPE(":: rs_index_golden :: CustomIndex") alignas(4)
    [[clang::trivial_abi]] CustomIndex final {
 public:
  static ::rs_index::CustomIndex new_(::std::uintptr_t index);

  ::std::uintptr_t __field0{};

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: rs_index_golden :: Id") alignas(4)
    [[clang::trivial_abi]] Id final {
 public:
  static ::rs_index::Id new_(::std::int32_t id);

  ::std::int32_t __field0{};

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: rs_index_golden :: IntPair") alignas(4)
    [[clang::trivial_abi]] IntPair final {
 public:
  static ::rs_index::IntPair new_(::std::int32_t x, ::std::int32_t y);

  ::std::int32_t const& $(__anon1) operator[](::std::uintptr_t index) const& $(
      __anon1) CRUBIT_LIFETIME_BOUND;

  // Error generating bindings for implementation `<rs_index_golden::IntPair as
  // std::ops::Index<u64>>` defined at
  // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=44:
  // Index implementation for `u64` is not supported when `Index<usize>` is
  // implemented as it may overlap.

  ::std::int32_t const& $(__anon1) operator[](
      ::rs_index::CustomIndex index) const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  ::std::int32_t& $(__anon1) operator[](::std::uintptr_t index) &
      $(__anon1) CRUBIT_LIFETIME_BOUND;

  // Error generating bindings for implementation `<rs_index_golden::IntPair as
  // std::ops::IndexMut<u64>>` defined at
  // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=56:
  // IndexMut implementation for `u64` is not supported when `IndexMut<usize>`
  // is implemented as it may overlap.

  ::std::int32_t& $(__anon1) operator[](::rs_index::CustomIndex index) &
      $(__anon1) CRUBIT_LIFETIME_BOUND;

  ::std::int32_t x{};
  ::std::int32_t y{};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace rs_index

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    ":: alloc :: vec :: Vec < :: alloc :: string :: String >")
    rs_std::Vec<::rs::alloc::string::String>
    : public rs_std::VecBase<::rs::alloc::string::String> {
 public:
  // Default::default
  Vec() noexcept;

  // Clone::clone
  Vec(const Vec&) noexcept;

  // Clone::clone_from
  rs_std::Vec<::rs::alloc::string::String>& operator=(const Vec&) noexcept;

  Vec(Vec&&) noexcept;
  rs_std::Vec<::rs::alloc::string::String>& operator=(Vec&&) noexcept;
  Vec(::crubit::UnsafeRelocateTag, Vec&& value);

  ~Vec() noexcept;

 private:
  friend class rs_std::VecBase<::rs::alloc::string::String>;
  static constexpr std::size_t kPtrOffset = 4;
  static constexpr std::size_t kCapOffset = 0;
  static constexpr std::size_t kLenOffset = 8;
  unsigned char storage_[12];
};
#endif

namespace rs_index {

struct CRUBIT_INTERNAL_RUST_TYPE(":: rs_index_golden :: Map") alignas(4)
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
  Map(::crubit::UnsafeRelocateTag, Map&& value);

  static ::rs_index::Map new_(::std::uintptr_t row_size,
                              ::std::uintptr_t col_size);

  rs_std::StrRef operator[](::std::tuple<::std::uintptr_t, ::std::uintptr_t>
                                index) const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  rs_std::StrRef operator[](::rs_index::Id const* $(__anon2)
                                crubit_nonnull _index CRUBIT_LIFETIME_BOUND)
      const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  // Error generating bindings for associated function `<rs_index_golden::Map as
  // std::ops::IndexMut<(usize, usize)>>::index_mut` defined at
  // cc_bindings_from_rs/test/known_traits/index/rs_index.rs;l=122:
  // Error formatting function return type `&'__anon1 mut str`: Mutable
  // references to `str` are not yet supported.

 private:
  union {
    rs_std::Vec<::rs::alloc::string::String> data;
  };
  union {
    ::std::uintptr_t row_size;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(::rs_index::CustomIndex) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::rs_index::CustomIndex) == 4,
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
inline ::rs_index::CustomIndex(CustomIndex::new_)(::std::uintptr_t index) {
  crubit::Slot<::rs_index::CustomIndex> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(index, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::rs_index::CustomIndex::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::rs_index::CustomIndex;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
static_assert(
    sizeof(::rs_index::Id) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::rs_index::Id) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<Id>);
static_assert(::std::is_trivially_move_constructible_v<::rs_index::Id>);
static_assert(::std::is_trivially_move_assignable_v<::rs_index::Id>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::int32_t, ::rs_index::Id* __ret_ptr);
}
inline ::rs_index::Id(Id::new_)(::std::int32_t id) {
  crubit::Slot<::rs_index::Id> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(id, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::rs_index::Id::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::rs_index::Id;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
static_assert(
    sizeof(::rs_index::IntPair) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::rs_index::IntPair) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<IntPair>);
static_assert(::std::is_trivially_move_constructible_v<::rs_index::IntPair>);
static_assert(::std::is_trivially_move_assignable_v<::rs_index::IntPair>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::int32_t, ::std::int32_t,
                                   ::rs_index::IntPair* __ret_ptr);
}
inline ::rs_index::IntPair(IntPair::new_)(::std::int32_t x, ::std::int32_t y) {
  crubit::Slot<::rs_index::IntPair> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, y, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t const& $(__anon1)
    __crubit_thunk_Index_uindex_urs_uindex_ugolden_x0000003a_x0000003aIntPair_uusize(
        ::rs_index::IntPair const&, ::std::uintptr_t);
}
inline ::std::int32_t const& $(__anon1)(IntPair::operator[])(
    ::std::uintptr_t index) const& $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_Index_uindex_urs_uindex_ugolden_x0000003a_x0000003aIntPair_uusize(
          self, index);
}

namespace __crubit_internal {
extern "C" ::std::int32_t const& $(__anon1)
    __crubit_thunk_Index_uindex_urs_uindex_ugolden_x0000003a_x0000003aIntPair_urs_uindex_ugolden_x0000003a_x0000003aCustomIndex(
        ::rs_index::IntPair const&, ::rs_index::CustomIndex*);
}
inline ::std::int32_t const& $(__anon1)(IntPair::operator[])(
    ::rs_index::CustomIndex index) const& $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_Index_uindex_urs_uindex_ugolden_x0000003a_x0000003aIntPair_urs_uindex_ugolden_x0000003a_x0000003aCustomIndex(
          self, &index);
}

namespace __crubit_internal {
extern "C" ::std::int32_t& $(__anon1)
    __crubit_thunk_IndexMut_uindex_umut_urs_uindex_ugolden_x0000003a_x0000003aIntPair_uusize(
        ::rs_index::IntPair&, ::std::uintptr_t);
}
inline ::std::int32_t& $(__anon1)(IntPair::operator[])(
    ::std::uintptr_t index)&$(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_IndexMut_uindex_umut_urs_uindex_ugolden_x0000003a_x0000003aIntPair_uusize(
          self, index);
}

namespace __crubit_internal {
extern "C" ::std::int32_t& $(__anon1)
    __crubit_thunk_IndexMut_uindex_umut_urs_uindex_ugolden_x0000003a_x0000003aIntPair_urs_uindex_ugolden_x0000003a_x0000003aCustomIndex(
        ::rs_index::IntPair&, ::rs_index::CustomIndex*);
}
inline ::std::int32_t& $(__anon1)(IntPair::operator[])(
    ::rs_index::CustomIndex index)&$(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_IndexMut_uindex_umut_urs_uindex_ugolden_x0000003a_x0000003aIntPair_urs_uindex_ugolden_x0000003a_x0000003aCustomIndex(
          self, &index);
}
inline void ::rs_index::IntPair::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::rs_index::IntPair;
  static_assert(0 == offsetof(__crubit_assert_type, x));
  static_assert(4 == offsetof(__crubit_assert_type, y));
}
static_assert(
    sizeof(::rs_index::Map) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::rs_index::Map) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_urs_uindex_ugolden_x0000003a_x0000003aMap(
    ::rs_index::Map&);
}
inline ::rs_index::Map::~Map() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_urs_uindex_ugolden_x0000003a_x0000003aMap(
          *this);
}
inline ::rs_index::Map::Map(::crubit::UnsafeRelocateTag, Map&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uintptr_t, ::std::uintptr_t,
                                   ::rs_index::Map* __ret_ptr);
}
inline ::rs_index::Map(Map::new_)(::std::uintptr_t row_size,
                                  ::std::uintptr_t col_size) {
  crubit::Slot<::rs_index::Map> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(row_size, col_size,
                                        __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_Index_uindex_urs_uindex_ugolden_x0000003a_x0000003aMap_u_x00000028usize_x0000002c_x00000020usize_x00000029(
    ::rs_index::Map const&, void**, rs_std::StrRef* __ret_ptr);
}
inline rs_std::StrRef(Map::operator[])(
    ::std::tuple<::std::uintptr_t, ::std::uintptr_t> index) const& $(__anon1)
    CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  auto&& index_0 = ::std::get<0>(index);
  auto&& index_cabi_0 = index_0;
  auto&& index_1 = ::std::get<1>(index);
  auto&& index_cabi_1 = index_1;
  void* index_cabi[] = {&index_cabi_0, &index_cabi_1};
  crubit::Slot<rs_std::StrRef> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Index_uindex_urs_uindex_ugolden_x0000003a_x0000003aMap_u_x00000028usize_x0000002c_x00000020usize_x00000029(
          self, index_cabi, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_Index_uindex_urs_uindex_ugolden_x0000003a_x0000003aMap_u_x00000026rs_uindex_ugolden_x0000003a_x0000003aId_u_x00000027_u(
    ::rs_index::Map const&, ::rs_index::Id const* $(__anon2) crubit_nonnull,
    rs_std::StrRef* __ret_ptr);
}
inline rs_std::StrRef(Map::operator[])(
    ::rs_index::Id const* $(__anon2) crubit_nonnull _index
        CRUBIT_LIFETIME_BOUND) const& $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  crubit::Slot<rs_std::StrRef> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Index_uindex_urs_uindex_ugolden_x0000003a_x0000003aMap_u_x00000026rs_uindex_ugolden_x0000003a_x0000003aId_u_x00000027_u(
          self, _index, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::rs_index::Map::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::rs_index::Map;
  static_assert(0 == offsetof(__crubit_assert_type, data));
  static_assert(12 == offsetof(__crubit_assert_type, row_size));
}
}  // namespace rs_index

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cstd_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e(
    rs_std::Vec<::rs::alloc::string::String> const&,
    rs_std::Vec<::rs::alloc::string::String>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cstd_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e(
    rs_std::Vec<::rs::alloc::string::String>&,
    rs_std::Vec<::rs::alloc::string::String> const&);
}
inline rs_std::Vec<::rs::alloc::string::String>::Vec(
    const Vec& other) noexcept {
  ::__crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cstd_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e(
          other, this);
}
inline rs_std::Vec<::rs::alloc::string::String>&
rs_std::Vec<::rs::alloc::string::String>::operator=(const Vec& other) noexcept {
  if (this != &other) {
    ::__crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cstd_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e(
            *this, other);
  }
  return *this;
}
inline rs_std::Vec<::rs::alloc::string::String>::Vec(Vec&& other) noexcept
    : storage_{} {
  ::std::memcpy(storage_, other.storage_, sizeof(storage_));
  other.init_empty();
}
inline rs_std::Vec<::rs::alloc::string::String>&
rs_std::Vec<::rs::alloc::string::String>::operator=(Vec&& other) noexcept {
  if (this != &other) {
    destroy();
    crubit::MemSwap(*this, other);
  }
  return *this;
}
inline rs_std::Vec<::rs::alloc::string::String>::Vec(
    ::crubit::UnsafeRelocateTag, Vec&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

inline rs_std::Vec<::rs::alloc::string::String>::Vec() noexcept : storage_{} {
  init_empty();
}
inline rs_std::Vec<::rs::alloc::string::String>::~Vec() noexcept { destroy(); }
#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_INDEX_RS_INDEX_GOLDEN
