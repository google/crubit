// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// consts_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_CONSTS_CONSTS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_CONSTS_CONSTS_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/rs_std/str_ref.h"
#include "support/rs_std/traits.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>

#include "support/ffi_11/ffi_11.h"
#include "support/rs_std/rs_core.h"

namespace consts {
static constexpr ::std::array<::std::int32_t, 3> ARRAY_CONST =
    ::std::array<::std::int32_t, 3>{INT32_C(10), INT32_C(20), INT32_C(30)};
static constexpr decltype(char(0)) CHAR = 42;
static constexpr float FLOAT_32 = 0.125f;
static constexpr double FLOAT_64 = 0.0078125L;
static constexpr ::std::int32_t INT_NEG = INT32_C(-17);
static constexpr ::std::int32_t INT_POS = INT32_C(42);
static constexpr ::std::intptr_t ISIZE = INT64_C(42);
static constexpr ::std::int64_t LARGE_INT = INT64_C(9223372036854775807);
struct CRUBIT_INTERNAL_RUST_TYPE(":: consts_golden :: Point") alignas(4)
    [[clang::trivial_abi]] Point final {
 public:
  bool operator==(::consts::Point const& other) const;

  ::std::int32_t x = {};
  ::std::int32_t y = {};

 private:
  static void __crubit_field_offset_assertions();
};
static constexpr ::consts::Point POINT_CONST =
    ::consts::Point{.x = INT32_C(10), .y = INT32_C(-20)};
static constexpr float RUST_F32_MAX = 3.40282347E+38f;
static constexpr float RUST_F32_MIN = -3.40282347E+38f;
static constexpr double RUST_F64_MAX = 1.7976931348623157E+308L;
static constexpr double RUST_F64_MIN = -1.7976931348623157E+308L;
static constexpr bool RUST_FALSE = false;
static constexpr ::std::int16_t RUST_INT16_MAX = 32767;
static constexpr ::std::int16_t RUST_INT16_MIN = -32768;
static constexpr ::std::int32_t RUST_INT32_MAX = INT32_C(2147483647);
static constexpr ::std::int32_t RUST_INT32_MIN = INT32_C(-2147483648);
static constexpr ::std::int64_t RUST_INT64_MAX = INT64_C(9223372036854775807);
static constexpr ::std::int64_t RUST_INT64_MIN = INT64_MIN;
static constexpr ::std::int8_t RUST_INT8_MAX = 127;
static constexpr ::std::int8_t RUST_INT8_MIN = -128;
static constexpr ::std::intptr_t RUST_ISIZE_MAX = INT64_C(2147483647);
static constexpr ::std::intptr_t RUST_ISIZE_MIN = INT64_C(-2147483648);
static constexpr bool RUST_TRUE = true;
static constexpr ::std::uint16_t RUST_UINT16_MAX = UINT16_C(65535);
static constexpr ::std::uint16_t RUST_UINT16_MIN = 0;
static constexpr ::std::uint32_t RUST_UINT32_MAX = UINT32_C(4294967295);
static constexpr ::std::uint32_t RUST_UINT32_MIN = 0;
static constexpr ::std::uint64_t RUST_UINT64_MAX =
    UINT64_C(18446744073709551615);
static constexpr ::std::uint64_t RUST_UINT64_MIN = 0;
static constexpr ::std::uint8_t RUST_UINT8_MAX = 255;
static constexpr ::std::uint8_t RUST_UINT8_MIN = 0;
static constexpr ::std::intptr_t RUST_USIZE_MAX = INT64_C(2147483647);
static constexpr ::std::intptr_t RUST_USIZE_MIN = INT64_C(-2147483648);
static constexpr ::std::uintptr_t SLICE_LENGTH = 11;
struct CRUBIT_INTERNAL_RUST_TYPE(":: consts_golden :: StructWithArray") alignas(
    4) [[clang::trivial_abi]] StructWithArray final {
 public:
  bool operator==(::consts::StructWithArray const& other) const;

  ::std::array<::std::int32_t, 2> values = {};

 private:
  static void __crubit_field_offset_assertions();
};
static constexpr ::consts::StructWithArray STRUCT_WITH_ARRAY_CONST =
    ::consts::StructWithArray{
        .values = ::std::array<::std::int32_t, 2>{INT32_C(100), INT32_C(200)}};
struct CRUBIT_INTERNAL_RUST_TYPE(":: consts_golden :: StructWithStr") alignas(4)
    [[clang::trivial_abi]] StructWithStr final {
 public:
  bool operator==(::consts::StructWithStr const& other) const;

  rs_std::StrRef msg = {};
  ::std::int32_t count = {};

 private:
  static void __crubit_field_offset_assertions();
};
static constexpr ::consts::StructWithStr STRUCT_WITH_STR_CONST =
    ::consts::StructWithStr{.msg = rs_std::StrRef("hello world"),
                            .count = INT32_C(42)};
struct CRUBIT_INTERNAL_RUST_TYPE(":: consts_golden :: TupleStruct") alignas(4)
    [[clang::trivial_abi]] TupleStruct final {
 public:
  bool operator==(::consts::TupleStruct const& other) const;

  ::std::int32_t __field0 = {};
  ::std::uint32_t __field1 = {};

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: consts_golden :: NestedStruct") alignas(4)
    [[clang::trivial_abi]] NestedStruct final {
 public:
  bool operator==(::consts::NestedStruct const& other) const;

  ::consts::Point point = {};
  ::consts::TupleStruct tuple = {};

 private:
  static void __crubit_field_offset_assertions();
};
static constexpr ::consts::NestedStruct NESTED_STRUCT_CONST =
    ::consts::NestedStruct{
        .point = ::consts::Point{.x = INT32_C(1), .y = INT32_C(2)},
        .tuple = ::consts::TupleStruct{.__field0 = INT32_C(3), .__field1 = 4}};
static constexpr ::consts::TupleStruct TUPLE_STRUCT_CONST =
    ::consts::TupleStruct{.__field0 = INT32_C(123), .__field1 = 456};
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: consts_golden :: TyWithAssocConsts") alignas(1) [[clang::trivial_abi]]
TyWithAssocConsts final {
 public:
  // `consts_golden::TyWithAssocConsts` doesn't implement the `Default` trait
  TyWithAssocConsts() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~TyWithAssocConsts() = default;
  TyWithAssocConsts(TyWithAssocConsts&&) = default;
  TyWithAssocConsts& operator=(TyWithAssocConsts&&) = default;

  // `consts_golden::TyWithAssocConsts` doesn't implement the `Clone` trait
  TyWithAssocConsts(const TyWithAssocConsts&) = delete;
  TyWithAssocConsts& operator=(const TyWithAssocConsts&) = delete;
  TyWithAssocConsts(::crubit::UnsafeRelocateTag, TyWithAssocConsts&& value);
  static constexpr ::std::int32_t ASSOC_42 = INT32_C(42);
  static constexpr ::consts::Point ASSOC_POINT =
      ::consts::Point{.x = INT32_C(5), .y = INT32_C(6)};

 private:
  union {
    ::std::uint8_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};
static constexpr ::std::uint32_t UNSIGNED_INT = UINT32_C(4294967295);
}  // namespace consts

template <>
struct rs_std::impl<::consts::NestedStruct, ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct rs_std::impl<::consts::NestedStruct, ::rs::core::fmt::Debug> {
  static constexpr bool kIsImplemented = true;

  // Error generating bindings for associated function
  // `<consts_golden::NestedStruct as std::fmt::Debug>::fmt` defined at
  // cc_bindings_from_rs/test/consts/consts.rs;l=72:
  // Error formatting function return type `std::result::Result<(),
  // std::fmt::Error>`: Generic types are not supported yet (b/259749095)
};

template <>
struct rs_std::impl<::consts::Point, ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct rs_std::impl<::consts::Point, ::rs::core::fmt::Debug> {
  static constexpr bool kIsImplemented = true;

  // Error generating bindings for associated function `<consts_golden::Point as
  // std::fmt::Debug>::fmt` defined at
  // cc_bindings_from_rs/test/consts/consts.rs;l=56:
  // Error formatting function return type `std::result::Result<(),
  // std::fmt::Error>`: Generic types are not supported yet (b/259749095)
};

template <>
struct rs_std::impl<::consts::StructWithArray, ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct rs_std::impl<::consts::StructWithArray, ::rs::core::fmt::Debug> {
  static constexpr bool kIsImplemented = true;

  // Error generating bindings for associated function
  // `<consts_golden::StructWithArray as std::fmt::Debug>::fmt` defined at
  // cc_bindings_from_rs/test/consts/consts.rs;l=87:
  // Error formatting function return type `std::result::Result<(),
  // std::fmt::Error>`: Generic types are not supported yet (b/259749095)
};

template <>
struct rs_std::impl<::consts::StructWithStr, ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct rs_std::impl<::consts::StructWithStr, ::rs::core::fmt::Debug> {
  static constexpr bool kIsImplemented = true;

  // Error generating bindings for associated function
  // `<consts_golden::StructWithStr<'a> as std::fmt::Debug>::fmt` defined at
  // cc_bindings_from_rs/test/consts/consts.rs;l=97:
  // Error formatting function return type `std::result::Result<(),
  // std::fmt::Error>`: Generic types are not supported yet (b/259749095)
};

template <>
struct rs_std::impl<::consts::TupleStruct, ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct rs_std::impl<::consts::TupleStruct, ::rs::core::fmt::Debug> {
  static constexpr bool kIsImplemented = true;

  // Error generating bindings for associated function
  // `<consts_golden::TupleStruct as std::fmt::Debug>::fmt` defined at
  // cc_bindings_from_rs/test/consts/consts.rs;l=66:
  // Error formatting function return type `std::result::Result<(),
  // std::fmt::Error>`: Generic types are not supported yet (b/259749095)
};

namespace consts {

static_assert(
    sizeof(NestedStruct) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NestedStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NestedStruct>);
static_assert(::std::is_trivially_move_constructible_v<::consts::NestedStruct>);
static_assert(::std::is_trivially_move_assignable_v<::consts::NestedStruct>);
static_assert(::std::is_trivially_copy_constructible_v<::consts::NestedStruct>);
static_assert(::std::is_trivially_copy_assignable_v<::consts::NestedStruct>);
namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aNestedStruct_uconsts_ugolden_x0000003a_x0000003aNestedStruct(
    ::consts::NestedStruct const&, ::consts::NestedStruct const&);
}
inline bool NestedStruct::operator==(
    ::consts::NestedStruct const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aNestedStruct_uconsts_ugolden_x0000003a_x0000003aNestedStruct(
          self, other);
}
inline void NestedStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NestedStruct, point));
  static_assert(8 == offsetof(NestedStruct, tuple));
}
static_assert(
    sizeof(Point) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Point) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<Point>);
static_assert(::std::is_trivially_move_constructible_v<::consts::Point>);
static_assert(::std::is_trivially_move_assignable_v<::consts::Point>);
static_assert(::std::is_trivially_copy_constructible_v<::consts::Point>);
static_assert(::std::is_trivially_copy_assignable_v<::consts::Point>);
namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aPoint_uconsts_ugolden_x0000003a_x0000003aPoint(
    ::consts::Point const&, ::consts::Point const&);
}
inline bool Point::operator==(::consts::Point const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aPoint_uconsts_ugolden_x0000003a_x0000003aPoint(
          self, other);
}
inline void Point::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Point, x));
  static_assert(4 == offsetof(Point, y));
}
static_assert(
    sizeof(StructWithArray) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructWithArray) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<StructWithArray>);
static_assert(
    ::std::is_trivially_move_constructible_v<::consts::StructWithArray>);
static_assert(::std::is_trivially_move_assignable_v<::consts::StructWithArray>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::consts::StructWithArray>);
static_assert(::std::is_trivially_copy_assignable_v<::consts::StructWithArray>);
namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aStructWithArray_uconsts_ugolden_x0000003a_x0000003aStructWithArray(
    ::consts::StructWithArray const&, ::consts::StructWithArray const&);
}
inline bool StructWithArray::operator==(
    ::consts::StructWithArray const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aStructWithArray_uconsts_ugolden_x0000003a_x0000003aStructWithArray(
          self, other);
}
inline void StructWithArray::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructWithArray, values));
}
static_assert(
    sizeof(StructWithStr) == 12,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructWithStr) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<StructWithStr>);
static_assert(
    ::std::is_trivially_move_constructible_v<::consts::StructWithStr>);
static_assert(::std::is_trivially_move_assignable_v<::consts::StructWithStr>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::consts::StructWithStr>);
static_assert(::std::is_trivially_copy_assignable_v<::consts::StructWithStr>);
namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aStructWithStr_x0000003c_x00000027a_x0000003e_uconsts_ugolden_x0000003a_x0000003aStructWithStr_x0000003c_x00000027a_x0000003e(
    ::consts::StructWithStr const&, ::consts::StructWithStr const&);
}
inline bool StructWithStr::operator==(
    ::consts::StructWithStr const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aStructWithStr_x0000003c_x00000027a_x0000003e_uconsts_ugolden_x0000003a_x0000003aStructWithStr_x0000003c_x00000027a_x0000003e(
          self, other);
}
inline void StructWithStr::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructWithStr, msg));
  static_assert(8 == offsetof(StructWithStr, count));
}
static_assert(
    sizeof(TupleStruct) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<TupleStruct>);
static_assert(::std::is_trivially_move_constructible_v<::consts::TupleStruct>);
static_assert(::std::is_trivially_move_assignable_v<::consts::TupleStruct>);
static_assert(::std::is_trivially_copy_constructible_v<::consts::TupleStruct>);
static_assert(::std::is_trivially_copy_assignable_v<::consts::TupleStruct>);
namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aTupleStruct_uconsts_ugolden_x0000003a_x0000003aTupleStruct(
    ::consts::TupleStruct const&, ::consts::TupleStruct const&);
}
inline bool TupleStruct::operator==(::consts::TupleStruct const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aTupleStruct_uconsts_ugolden_x0000003a_x0000003aTupleStruct(
          self, other);
}
inline void TupleStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStruct, __field0));
  static_assert(4 == offsetof(TupleStruct, __field1));
}
static_assert(
    sizeof(TyWithAssocConsts) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TyWithAssocConsts) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<TyWithAssocConsts>);
static_assert(
    ::std::is_trivially_move_constructible_v<::consts::TyWithAssocConsts>);
static_assert(
    ::std::is_trivially_move_assignable_v<::consts::TyWithAssocConsts>);
inline ::consts::TyWithAssocConsts::TyWithAssocConsts(
    ::crubit::UnsafeRelocateTag, TyWithAssocConsts&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void TyWithAssocConsts::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TyWithAssocConsts, __field0));
}
}  // namespace consts

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_CONSTS_CONSTS_GOLDEN
