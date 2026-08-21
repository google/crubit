// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// rs_hash_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_HASH_RS_HASH_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_HASH_RS_HASH_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/rs_std/traits.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <functional>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_core.h"

namespace rs_hash::derived_enum {
struct Color;
}

namespace rs_hash::derived_struct {
struct Point;
}

namespace rs_hash::derived_tuple_struct {
struct TupleStruct;
}

namespace rs_hash::explicit_struct {
struct CustomHashStruct;
}

namespace rs_hash::derived_enum {

::rs_hash::derived_enum::Color create_blue();

::rs_hash::derived_enum::Color create_green();

::rs_hash::derived_enum::Color create_red();

}  // namespace rs_hash::derived_enum

namespace rs_hash::derived_struct {

::rs_hash::derived_struct::Point create_point(::std::int32_t x,
                                              ::std::int32_t y);

}

namespace rs_hash::derived_tuple_struct {

::rs_hash::derived_tuple_struct::TupleStruct create_tuple(::std::int32_t x,
                                                          ::std::int32_t y);

}

namespace rs_hash::explicit_struct {

::rs_hash::explicit_struct::CustomHashStruct create_custom(
    ::std::int32_t value);

}

template <>
struct rs_std::impl<::rs_hash::derived_enum::Color, ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct rs_std::impl<::rs_hash::derived_struct::Point, ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct rs_std::impl<::rs_hash::derived_tuple_struct::TupleStruct,
                    ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct rs_std::impl<::rs_hash::explicit_struct::CustomHashStruct,
                    ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};
namespace __crubit_internal {
extern "C" ::std::uint64_t
__crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aderived_uenum_x0000003a_x0000003aColor(
    const ::rs_hash::derived_enum::Color&);
}
namespace std {
template <>
struct hash<::rs_hash::derived_enum::Color> {
  ::std::size_t operator()(const ::rs_hash::derived_enum::Color& self) const {
    return static_cast<::std::size_t>(
        __crubit_internal::
            __crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aderived_uenum_x0000003a_x0000003aColor(
                self));
  }
};
}  // namespace std

namespace rs_hash::derived_enum {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: rs_hash_golden :: derived_enum :: Color") alignas(1)
    [[clang::trivial_abi]] Color final {
 public:
  // `rs_hash_golden::derived_enum::Color` doesn't implement the `Default` trait
  Color() = delete;

  static constexpr Color MakeRed();

  static constexpr Color MakeGreen();

  static constexpr Color MakeBlue();

  // No custom `Drop` impl and no custom "drop glue" required
  ~Color() = default;
  Color(Color&&) = default;
  Color& operator=(Color&&) = default;

  // Clone::clone
  Color(const Color&);

  // Clone::clone_from
  ::rs_hash::derived_enum::Color& operator=(const Color&);

  Color(::crubit::UnsafeRelocateTag, Color&& value);

  bool operator==(::rs_hash::derived_enum::Color const& other) const;

  // AbslHashValue and std::hash support via core::hash::Hash
  template <typename H>
  friend H AbslHashValue(H h, const Color& self) {
    return H::combine(
        ::std::move(h),
        __crubit_internal::
            __crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aderived_uenum_x0000003a_x0000003aColor(
                self));
  }

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  ::std::array<unsigned char, 1> __opaque_blob_of_bytes;

 private:
  struct PrivateBytesTag {};
  constexpr Color(PrivateBytesTag, ::std::array<unsigned char, 1> bytes)
      : __opaque_blob_of_bytes(bytes) {}

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace rs_hash::derived_enum

namespace __crubit_internal {
extern "C" ::std::uint64_t
__crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aderived_ustruct_x0000003a_x0000003aPoint(
    const ::rs_hash::derived_struct::Point&);
}
namespace std {
template <>
struct hash<::rs_hash::derived_struct::Point> {
  ::std::size_t operator()(const ::rs_hash::derived_struct::Point& self) const {
    return static_cast<::std::size_t>(
        __crubit_internal::
            __crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aderived_ustruct_x0000003a_x0000003aPoint(
                self));
  }
};
}  // namespace std

namespace rs_hash::derived_struct {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: rs_hash_golden :: derived_struct :: Point") alignas(4)
    [[clang::trivial_abi]] Point final {
 public:
  // `rs_hash_golden::derived_struct::Point` doesn't implement the `Default`
  // trait
  Point() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~Point() = default;
  Point(Point&&) = default;
  Point& operator=(Point&&) = default;

  // Clone::clone
  Point(const Point&);

  // Clone::clone_from
  ::rs_hash::derived_struct::Point& operator=(const Point&);

  Point(::crubit::UnsafeRelocateTag, Point&& value);

  bool operator==(::rs_hash::derived_struct::Point const& other) const;

  // AbslHashValue and std::hash support via core::hash::Hash
  template <typename H>
  friend H AbslHashValue(H h, const Point& self) {
    return H::combine(
        ::std::move(h),
        __crubit_internal::
            __crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aderived_ustruct_x0000003a_x0000003aPoint(
                self));
  }

  union {
    ::std::int32_t x;
  };
  union {
    ::std::int32_t y;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace rs_hash::derived_struct

namespace __crubit_internal {
extern "C" ::std::uint64_t
__crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aderived_utuple_ustruct_x0000003a_x0000003aTupleStruct(
    const ::rs_hash::derived_tuple_struct::TupleStruct&);
}
namespace std {
template <>
struct hash<::rs_hash::derived_tuple_struct::TupleStruct> {
  ::std::size_t operator()(
      const ::rs_hash::derived_tuple_struct::TupleStruct& self) const {
    return static_cast<::std::size_t>(
        __crubit_internal::
            __crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aderived_utuple_ustruct_x0000003a_x0000003aTupleStruct(
                self));
  }
};
}  // namespace std

namespace rs_hash::derived_tuple_struct {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: rs_hash_golden :: derived_tuple_struct :: TupleStruct") alignas(4)
    [[clang::trivial_abi]] TupleStruct final {
 public:
  // `rs_hash_golden::derived_tuple_struct::TupleStruct` doesn't implement the
  // `Default` trait
  TupleStruct() = delete;

  // Synthesized tuple constructor
  TupleStruct(::std::int32_t __field0, ::std::int32_t __field1)
      : __field0(::std::move(__field0)), __field1(::std::move(__field1)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~TupleStruct() = default;
  TupleStruct(TupleStruct&&) = default;
  TupleStruct& operator=(TupleStruct&&) = default;

  // Clone::clone
  TupleStruct(const TupleStruct&);

  // Clone::clone_from
  ::rs_hash::derived_tuple_struct::TupleStruct& operator=(const TupleStruct&);

  TupleStruct(::crubit::UnsafeRelocateTag, TupleStruct&& value);

  bool operator==(
      ::rs_hash::derived_tuple_struct::TupleStruct const& other) const;

  // AbslHashValue and std::hash support via core::hash::Hash
  template <typename H>
  friend H AbslHashValue(H h, const TupleStruct& self) {
    return H::combine(
        ::std::move(h),
        __crubit_internal::
            __crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aderived_utuple_ustruct_x0000003a_x0000003aTupleStruct(
                self));
  }

  union {
    ::std::int32_t __field0;
  };
  union {
    ::std::int32_t __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace rs_hash::derived_tuple_struct

namespace __crubit_internal {
extern "C" ::std::uint64_t
__crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aexplicit_ustruct_x0000003a_x0000003aCustomHashStruct(
    const ::rs_hash::explicit_struct::CustomHashStruct&);
}
namespace std {
template <>
struct hash<::rs_hash::explicit_struct::CustomHashStruct> {
  ::std::size_t operator()(
      const ::rs_hash::explicit_struct::CustomHashStruct& self) const {
    return static_cast<::std::size_t>(
        __crubit_internal::
            __crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aexplicit_ustruct_x0000003a_x0000003aCustomHashStruct(
                self));
  }
};
}  // namespace std

namespace rs_hash::explicit_struct {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: rs_hash_golden :: explicit_struct :: CustomHashStruct") alignas(4)
    [[clang::trivial_abi]] CustomHashStruct final {
 public:
  // `rs_hash_golden::explicit_struct::CustomHashStruct` doesn't implement the
  // `Default` trait
  CustomHashStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~CustomHashStruct() = default;
  CustomHashStruct(CustomHashStruct&&) = default;
  CustomHashStruct& operator=(CustomHashStruct&&) = default;

  // Clone::clone
  CustomHashStruct(const CustomHashStruct&);

  // Clone::clone_from
  ::rs_hash::explicit_struct::CustomHashStruct& operator=(
      const CustomHashStruct&);

  CustomHashStruct(::crubit::UnsafeRelocateTag, CustomHashStruct&& value);

  bool operator==(
      ::rs_hash::explicit_struct::CustomHashStruct const& other) const;

  // AbslHashValue and std::hash support via core::hash::Hash
  template <typename H>
  friend H AbslHashValue(H h, const CustomHashStruct& self) {
    return H::combine(
        ::std::move(h),
        __crubit_internal::
            __crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aexplicit_ustruct_x0000003a_x0000003aCustomHashStruct(
                self));
  }

  union {
    ::std::int32_t value;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace rs_hash::explicit_struct

namespace rs_hash::derived_enum {

static_assert(
    sizeof(Color) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Color) == 1,
    "Verify that ADT layout didn't change since this header got generated");

// `static` constructor
inline constexpr Color Color::MakeRed() {
  return Color(PrivateBytesTag{}, {0});
}

// `static` constructor
inline constexpr Color Color::MakeGreen() {
  return Color(PrivateBytesTag{}, {1});
}

// `static` constructor
inline constexpr Color Color::MakeBlue() {
  return Color(PrivateBytesTag{}, {2});
}
static_assert(::std::is_trivially_destructible_v<Color>);
static_assert(
    ::std::is_trivially_move_constructible_v<::rs_hash::derived_enum::Color>);
static_assert(
    ::std::is_trivially_move_assignable_v<::rs_hash::derived_enum::Color>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_urs_uhash_ugolden_x0000003a_x0000003aderived_uenum_x0000003a_x0000003aColor(
    ::rs_hash::derived_enum::Color const&,
    ::rs_hash::derived_enum::Color* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_urs_uhash_ugolden_x0000003a_x0000003aderived_uenum_x0000003a_x0000003aColor(
    ::rs_hash::derived_enum::Color&, ::rs_hash::derived_enum::Color const&);
}
inline ::rs_hash::derived_enum::Color::Color(const Color& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_urs_uhash_ugolden_x0000003a_x0000003aderived_uenum_x0000003a_x0000003aColor(
          other, this);
}
inline ::rs_hash::derived_enum::Color& ::rs_hash::derived_enum::Color::
operator=(const Color& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_urs_uhash_ugolden_x0000003a_x0000003aderived_uenum_x0000003a_x0000003aColor(
            *this, other);
  }
  return *this;
}
inline ::rs_hash::derived_enum::Color::Color(::crubit::UnsafeRelocateTag,
                                             Color&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_urs_uhash_ugolden_x0000003a_x0000003aderived_uenum_x0000003a_x0000003aColor_urs_uhash_ugolden_x0000003a_x0000003aderived_uenum_x0000003a_x0000003aColor(
    ::rs_hash::derived_enum::Color const&,
    ::rs_hash::derived_enum::Color const&);
}
inline bool Color::operator==(
    ::rs_hash::derived_enum::Color const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_urs_uhash_ugolden_x0000003a_x0000003aderived_uenum_x0000003a_x0000003aColor_urs_uhash_ugolden_x0000003a_x0000003aderived_uenum_x0000003a_x0000003aColor(
          self, other);
}
namespace __crubit_internal {
extern "C" ::std::uint64_t
__crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aderived_uenum_x0000003a_x0000003aColor(
    ::rs_hash::derived_enum::Color const&);
}
inline void Color::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Color, __opaque_blob_of_bytes));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_create_ublue(
    ::rs_hash::derived_enum::Color* __ret_ptr);
}
inline ::rs_hash::derived_enum::Color create_blue() {
  crubit::Slot<::rs_hash::derived_enum::Color> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create_ublue(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_create_ugreen(
    ::rs_hash::derived_enum::Color* __ret_ptr);
}
inline ::rs_hash::derived_enum::Color create_green() {
  crubit::Slot<::rs_hash::derived_enum::Color> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create_ugreen(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_create_ured(
    ::rs_hash::derived_enum::Color* __ret_ptr);
}
inline ::rs_hash::derived_enum::Color create_red() {
  crubit::Slot<::rs_hash::derived_enum::Color> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create_ured(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace rs_hash::derived_enum

namespace rs_hash::derived_struct {

static_assert(
    sizeof(Point) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Point) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<Point>);
static_assert(
    ::std::is_trivially_move_constructible_v<::rs_hash::derived_struct::Point>);
static_assert(
    ::std::is_trivially_move_assignable_v<::rs_hash::derived_struct::Point>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_urs_uhash_ugolden_x0000003a_x0000003aderived_ustruct_x0000003a_x0000003aPoint(
    ::rs_hash::derived_struct::Point const&,
    ::rs_hash::derived_struct::Point* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_urs_uhash_ugolden_x0000003a_x0000003aderived_ustruct_x0000003a_x0000003aPoint(
    ::rs_hash::derived_struct::Point&, ::rs_hash::derived_struct::Point const&);
}
inline ::rs_hash::derived_struct::Point::Point(const Point& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_urs_uhash_ugolden_x0000003a_x0000003aderived_ustruct_x0000003a_x0000003aPoint(
          other, this);
}
inline ::rs_hash::derived_struct::Point& ::rs_hash::derived_struct::Point::
operator=(const Point& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_urs_uhash_ugolden_x0000003a_x0000003aderived_ustruct_x0000003a_x0000003aPoint(
            *this, other);
  }
  return *this;
}
inline ::rs_hash::derived_struct::Point::Point(::crubit::UnsafeRelocateTag,
                                               Point&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_urs_uhash_ugolden_x0000003a_x0000003aderived_ustruct_x0000003a_x0000003aPoint_urs_uhash_ugolden_x0000003a_x0000003aderived_ustruct_x0000003a_x0000003aPoint(
    ::rs_hash::derived_struct::Point const&,
    ::rs_hash::derived_struct::Point const&);
}
inline bool Point::operator==(
    ::rs_hash::derived_struct::Point const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_urs_uhash_ugolden_x0000003a_x0000003aderived_ustruct_x0000003a_x0000003aPoint_urs_uhash_ugolden_x0000003a_x0000003aderived_ustruct_x0000003a_x0000003aPoint(
          self, other);
}
namespace __crubit_internal {
extern "C" ::std::uint64_t
__crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aderived_ustruct_x0000003a_x0000003aPoint(
    ::rs_hash::derived_struct::Point const&);
}
inline void Point::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Point, x));
  static_assert(4 == offsetof(Point, y));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_create_upoint(
    ::std::int32_t, ::std::int32_t,
    ::rs_hash::derived_struct::Point* __ret_ptr);
}
inline ::rs_hash::derived_struct::Point create_point(::std::int32_t x,
                                                     ::std::int32_t y) {
  crubit::Slot<::rs_hash::derived_struct::Point> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create_upoint(x, y, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace rs_hash::derived_struct

namespace rs_hash::derived_tuple_struct {

static_assert(
    sizeof(TupleStruct) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<TupleStruct>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_hash::derived_tuple_struct::TupleStruct>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_hash::derived_tuple_struct::TupleStruct>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_urs_uhash_ugolden_x0000003a_x0000003aderived_utuple_ustruct_x0000003a_x0000003aTupleStruct(
    ::rs_hash::derived_tuple_struct::TupleStruct const&,
    ::rs_hash::derived_tuple_struct::TupleStruct* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_urs_uhash_ugolden_x0000003a_x0000003aderived_utuple_ustruct_x0000003a_x0000003aTupleStruct(
    ::rs_hash::derived_tuple_struct::TupleStruct&,
    ::rs_hash::derived_tuple_struct::TupleStruct const&);
}
inline ::rs_hash::derived_tuple_struct::TupleStruct::TupleStruct(
    const TupleStruct& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_urs_uhash_ugolden_x0000003a_x0000003aderived_utuple_ustruct_x0000003a_x0000003aTupleStruct(
          other, this);
}
inline ::rs_hash::derived_tuple_struct::TupleStruct& ::rs_hash::
    derived_tuple_struct::TupleStruct::operator=(const TupleStruct& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_urs_uhash_ugolden_x0000003a_x0000003aderived_utuple_ustruct_x0000003a_x0000003aTupleStruct(
            *this, other);
  }
  return *this;
}
inline ::rs_hash::derived_tuple_struct::TupleStruct::TupleStruct(
    ::crubit::UnsafeRelocateTag, TupleStruct&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_urs_uhash_ugolden_x0000003a_x0000003aderived_utuple_ustruct_x0000003a_x0000003aTupleStruct_urs_uhash_ugolden_x0000003a_x0000003aderived_utuple_ustruct_x0000003a_x0000003aTupleStruct(
    ::rs_hash::derived_tuple_struct::TupleStruct const&,
    ::rs_hash::derived_tuple_struct::TupleStruct const&);
}
inline bool TupleStruct::operator==(
    ::rs_hash::derived_tuple_struct::TupleStruct const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_urs_uhash_ugolden_x0000003a_x0000003aderived_utuple_ustruct_x0000003a_x0000003aTupleStruct_urs_uhash_ugolden_x0000003a_x0000003aderived_utuple_ustruct_x0000003a_x0000003aTupleStruct(
          self, other);
}
namespace __crubit_internal {
extern "C" ::std::uint64_t
__crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aderived_utuple_ustruct_x0000003a_x0000003aTupleStruct(
    ::rs_hash::derived_tuple_struct::TupleStruct const&);
}
inline void TupleStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStruct, __field0));
  static_assert(4 == offsetof(TupleStruct, __field1));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_create_utuple(
    ::std::int32_t, ::std::int32_t,
    ::rs_hash::derived_tuple_struct::TupleStruct* __ret_ptr);
}
inline ::rs_hash::derived_tuple_struct::TupleStruct create_tuple(
    ::std::int32_t x, ::std::int32_t y) {
  crubit::Slot<::rs_hash::derived_tuple_struct::TupleStruct>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create_utuple(x, y, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace rs_hash::derived_tuple_struct

namespace rs_hash::explicit_struct {

static_assert(
    sizeof(CustomHashStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CustomHashStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CustomHashStruct>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_hash::explicit_struct::CustomHashStruct>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_hash::explicit_struct::CustomHashStruct>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_urs_uhash_ugolden_x0000003a_x0000003aexplicit_ustruct_x0000003a_x0000003aCustomHashStruct(
    ::rs_hash::explicit_struct::CustomHashStruct const&,
    ::rs_hash::explicit_struct::CustomHashStruct* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_urs_uhash_ugolden_x0000003a_x0000003aexplicit_ustruct_x0000003a_x0000003aCustomHashStruct(
    ::rs_hash::explicit_struct::CustomHashStruct&,
    ::rs_hash::explicit_struct::CustomHashStruct const&);
}
inline ::rs_hash::explicit_struct::CustomHashStruct::CustomHashStruct(
    const CustomHashStruct& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_urs_uhash_ugolden_x0000003a_x0000003aexplicit_ustruct_x0000003a_x0000003aCustomHashStruct(
          other, this);
}
inline ::rs_hash::explicit_struct::CustomHashStruct& ::rs_hash::
    explicit_struct::CustomHashStruct::operator=(
        const CustomHashStruct& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_urs_uhash_ugolden_x0000003a_x0000003aexplicit_ustruct_x0000003a_x0000003aCustomHashStruct(
            *this, other);
  }
  return *this;
}
inline ::rs_hash::explicit_struct::CustomHashStruct::CustomHashStruct(
    ::crubit::UnsafeRelocateTag, CustomHashStruct&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_urs_uhash_ugolden_x0000003a_x0000003aexplicit_ustruct_x0000003a_x0000003aCustomHashStruct_urs_uhash_ugolden_x0000003a_x0000003aexplicit_ustruct_x0000003a_x0000003aCustomHashStruct(
    ::rs_hash::explicit_struct::CustomHashStruct const&,
    ::rs_hash::explicit_struct::CustomHashStruct const&);
}
inline bool CustomHashStruct::operator==(
    ::rs_hash::explicit_struct::CustomHashStruct const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_urs_uhash_ugolden_x0000003a_x0000003aexplicit_ustruct_x0000003a_x0000003aCustomHashStruct_urs_uhash_ugolden_x0000003a_x0000003aexplicit_ustruct_x0000003a_x0000003aCustomHashStruct(
          self, other);
}
namespace __crubit_internal {
extern "C" ::std::uint64_t
__crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aexplicit_ustruct_x0000003a_x0000003aCustomHashStruct(
    ::rs_hash::explicit_struct::CustomHashStruct const&);
}
inline void CustomHashStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CustomHashStruct, value));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_create_ucustom(
    ::std::int32_t, ::rs_hash::explicit_struct::CustomHashStruct* __ret_ptr);
}
inline ::rs_hash::explicit_struct::CustomHashStruct create_custom(
    ::std::int32_t value) {
  crubit::Slot<::rs_hash::explicit_struct::CustomHashStruct>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create_ucustom(value,
                                                   __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace rs_hash::explicit_struct

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_HASH_RS_HASH_GOLDEN
