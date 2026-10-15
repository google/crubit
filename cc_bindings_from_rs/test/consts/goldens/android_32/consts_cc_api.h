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
#pragma clang diagnostic ignored "-Wshadow"
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
static constexpr float FLOAT_32_INFINITY = __builtin_inff();
static constexpr float FLOAT_32_MAX_PAYLOAD_NAN = __builtin_nanf("0x3fffff");
static constexpr float FLOAT_32_MAX_SUBNORMAL = 1.1754942e-38f;
static constexpr float FLOAT_32_MIN_SUBNORMAL = 1e-45f;
static constexpr float FLOAT_32_NAN = __builtin_nanf("");
static constexpr float FLOAT_32_NEG_INFINITY = -__builtin_inff();
static constexpr float FLOAT_32_NEG_NAN = -__builtin_nanf("");
static constexpr float FLOAT_32_NEG_PAYLOAD_NAN = -__builtin_nanf("0x42");
static constexpr float FLOAT_32_NEG_SIGNALING_NAN = -__builtin_nansf("0x1");
static constexpr float FLOAT_32_NEG_SUBNORMAL = -1e-45f;
static constexpr float FLOAT_32_NEG_ZERO = -0.0f;
static constexpr float FLOAT_32_PAYLOAD_NAN = __builtin_nanf("0x42");
static constexpr float FLOAT_32_SIGNALING_NAN = __builtin_nansf("0x1");
static constexpr float FLOAT_32_WHOLE = 64.0f;
static constexpr double FLOAT_64 = 0.0078125;
static constexpr ::std::array<double, 2> FLOAT_64_ARRAY_CONST =
    ::std::array<double, 2>{-0.0, __builtin_inf()};
static constexpr double FLOAT_64_INFINITY = __builtin_inf();
static constexpr double FLOAT_64_MAX_PAYLOAD_NAN =
    __builtin_nan("0x7ffffffffffff");
static constexpr double FLOAT_64_MAX_SUBNORMAL = 2.225073858507201e-308;
static constexpr double FLOAT_64_MIN_SUBNORMAL = 5e-324;
static constexpr double FLOAT_64_NAN = __builtin_nan("");
static constexpr double FLOAT_64_NEG_INFINITY = -__builtin_inf();
static constexpr double FLOAT_64_NEG_NAN = -__builtin_nan("");
static constexpr double FLOAT_64_NEG_PAYLOAD_NAN = -__builtin_nan("0x42");
static constexpr double FLOAT_64_NEG_SIGNALING_NAN = -__builtin_nans("0x1");
static constexpr double FLOAT_64_NEG_SUBNORMAL = -5e-324;
static constexpr double FLOAT_64_NEG_ZERO = -0.0;
static constexpr double FLOAT_64_PAYLOAD_NAN = __builtin_nan("0x42");
static constexpr double FLOAT_64_SIGNALING_NAN = __builtin_nans("0x1");
static constexpr double FLOAT_64_WHOLE = 64.0;
static constexpr ::std::array<float, 4> FLOAT_ARRAY_CONST =
    ::std::array<float, 4>{64.0f, -0.0f, -__builtin_inff(), __builtin_nanf("")};
struct CRUBIT_INTERNAL_RUST_TYPE(":: consts_golden :: FloatStruct") alignas(4)
    [[clang::trivial_abi]] FloatStruct final {
 public:
  float x{};
  float y{};

 private:
  static void __crubit_field_offset_assertions();
};
static constexpr ::consts::FloatStruct FLOAT_STRUCT_CONST =
    ::consts::FloatStruct{.x = -0.0f, .y = __builtin_inff()};
static constexpr ::std::int32_t INT_NEG = INT32_C(-17);
static constexpr ::std::int32_t INT_POS = INT32_C(42);
static constexpr ::std::intptr_t ISIZE = INT64_C(42);
static constexpr ::std::int64_t LARGE_INT = INT64_C(9223372036854775807);
struct CRUBIT_INTERNAL_RUST_TYPE(":: consts_golden :: Point") alignas(4)
    [[clang::trivial_abi]] Point final {
 public:
  static const ::consts::Point ZERO;

  bool operator==(::consts::Point const& other) const;

  ::std::int32_t x{};
  ::std::int32_t y{};

 private:
  static void __crubit_field_offset_assertions();
};
static constexpr ::consts::Point POINT_CONST =
    ::consts::Point{.x = INT32_C(10), .y = INT32_C(-20)};
static constexpr float RUST_F32_EPSILON = 1.1920929e-7f;
static constexpr float RUST_F32_MAX = 3.4028235e38f;
static constexpr float RUST_F32_MIN = -3.4028235e38f;
static constexpr float RUST_F32_MIN_POSITIVE = 1.1754944e-38f;
static constexpr double RUST_F64_EPSILON = 2.220446049250313e-16;
static constexpr double RUST_F64_MAX = 1.7976931348623157e308;
static constexpr double RUST_F64_MIN = -1.7976931348623157e308;
static constexpr double RUST_F64_MIN_POSITIVE = 2.2250738585072014e-308;
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

// Error generating bindings for constant `consts_golden::STRUCT_WITH_REF_CONST`
// defined at
// cc_bindings_from_rs/test/consts/consts.rs;l=200:
// const of type `consts_golden::StructWithRef<'static>` cannot be generated as
// it is not a C++ aggregate: Field `r` is not default-constructible in C++

struct CRUBIT_INTERNAL_RUST_TYPE(":: consts_golden :: StructWithArray") alignas(
    4) [[clang::trivial_abi]] StructWithArray final {
 public:
  bool operator==(::consts::StructWithArray const& other) const;

  ::std::array<::std::int32_t, 2> values{};

 private:
  static void __crubit_field_offset_assertions();
};
static constexpr ::consts::StructWithArray STRUCT_WITH_ARRAY_CONST =
    ::consts::StructWithArray{
        .values = ::std::array<::std::int32_t, 2>{INT32_C(100), INT32_C(200)}};
struct CRUBIT_INTERNAL_RUST_TYPE(":: consts_golden :: StructWithRef") alignas(4)
    [[clang::trivial_abi]] StructWithRef final {
 public:
  // Type is not a C++ aggregate: Field `r` is not default-constructible in C++

  // `consts_golden::StructWithRef` doesn't implement the `Default` trait
  StructWithRef() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~StructWithRef() = default;
  StructWithRef(StructWithRef&&) = default;
  StructWithRef& operator=(StructWithRef&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  StructWithRef(const StructWithRef&) = default;
  StructWithRef& operator=(const StructWithRef&) = default;
  StructWithRef(::crubit::UnsafeRelocateTag, StructWithRef&& value);

  bool operator==(::consts::StructWithRef const& other) const;

  union {
    ::std::int32_t const* crubit_nonnull r;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: consts_golden :: StructWithStr") alignas(4)
    [[clang::trivial_abi]] StructWithStr final {
 public:
  bool operator==(::consts::StructWithStr const& other) const;

  rs_std::StrRef msg{};
  ::std::int32_t count{};

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

  ::std::int32_t __field0{};
  ::std::uint32_t __field1{};

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: consts_golden :: NestedStruct") alignas(4)
    [[clang::trivial_abi]] NestedStruct final {
 public:
  bool operator==(::consts::NestedStruct const& other) const;

  ::consts::Point point{};
  ::consts::TupleStruct tuple{};

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
  // Field type has been replaced with a blob of bytes: Field is private
  ::std::array<unsigned char, 1> __field0;

 private:
  static void __crubit_field_offset_assertions();
};
static constexpr ::std::uint32_t UNSIGNED_INT = UINT32_C(4294967295);

// CRUBIT_ANNOTATE: must_bind=
::std::uint32_t f32_to_bits(float x);

// CRUBIT_ANNOTATE: must_bind=
::std::uint64_t f64_to_bits(double x);

// Error generating bindings for function
// `consts_golden::fn_with_unreturnable_const` defined at
// cc_bindings_from_rs/test/consts/consts.rs;l=188:
// Function contains a const block that cannot return

// CRUBIT_ANNOTATE: must_bind=
float roundtrip_f32(float x);

// CRUBIT_ANNOTATE: must_bind=
double roundtrip_f64(double x);

}  // namespace consts

template <>
struct rs_std::impl<::consts::FloatStruct, ::rs::core::fmt::Debug> {
  static constexpr bool kIsImplemented = true;

  // Error generating bindings for associated function
  // `<consts_golden::FloatStruct as std::fmt::Debug>::fmt` defined at
  // cc_bindings_from_rs/test/consts/consts.rs;l=124:
  // Error formatting function return type `std::result::Result<(),
  // std::fmt::Error>`: Failed to format type for the definition of
  // `std::fmt::Error`: Zero-sized types (ZSTs) are not supported (b/258259459)
};

template <>
struct rs_std::impl<::consts::NestedStruct, ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct rs_std::impl<::consts::NestedStruct, ::rs::core::fmt::Debug> {
  static constexpr bool kIsImplemented = true;

  // Error generating bindings for associated function
  // `<consts_golden::NestedStruct as std::fmt::Debug>::fmt` defined at
  // cc_bindings_from_rs/test/consts/consts.rs;l=109:
  // Error formatting function return type `std::result::Result<(),
  // std::fmt::Error>`: Failed to format type for the definition of
  // `std::fmt::Error`: Zero-sized types (ZSTs) are not supported (b/258259459)
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
  // cc_bindings_from_rs/test/consts/consts.rs;l=88:
  // Error formatting function return type `std::result::Result<(),
  // std::fmt::Error>`: Failed to format type for the definition of
  // `std::fmt::Error`: Zero-sized types (ZSTs) are not supported (b/258259459)
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
  // cc_bindings_from_rs/test/consts/consts.rs;l=163:
  // Error formatting function return type `std::result::Result<(),
  // std::fmt::Error>`: Failed to format type for the definition of
  // `std::fmt::Error`: Zero-sized types (ZSTs) are not supported (b/258259459)
};

template <>
struct rs_std::impl<::consts::StructWithRef, ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct rs_std::impl<::consts::StructWithRef, ::rs::core::fmt::Debug> {
  static constexpr bool kIsImplemented = true;

  // Error generating bindings for associated function
  // `<consts_golden::StructWithRef<'a> as std::fmt::Debug>::fmt` defined at
  // cc_bindings_from_rs/test/consts/consts.rs;l=194:
  // Error formatting function return type `std::result::Result<(),
  // std::fmt::Error>`: Failed to format type for the definition of
  // `std::fmt::Error`: Zero-sized types (ZSTs) are not supported (b/258259459)
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
  // cc_bindings_from_rs/test/consts/consts.rs;l=173:
  // Error formatting function return type `std::result::Result<(),
  // std::fmt::Error>`: Failed to format type for the definition of
  // `std::fmt::Error`: Zero-sized types (ZSTs) are not supported (b/258259459)
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
  // cc_bindings_from_rs/test/consts/consts.rs;l=103:
  // Error formatting function return type `std::result::Result<(),
  // std::fmt::Error>`: Failed to format type for the definition of
  // `std::fmt::Error`: Zero-sized types (ZSTs) are not supported (b/258259459)
};

namespace consts {

static_assert(
    sizeof(::consts::FloatStruct) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::consts::FloatStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::consts::FloatStruct>);
static_assert(::std::is_trivially_move_constructible_v<::consts::FloatStruct>);
static_assert(::std::is_trivially_move_assignable_v<::consts::FloatStruct>);
static_assert(::std::is_trivially_copy_constructible_v<::consts::FloatStruct>);
static_assert(::std::is_trivially_copy_assignable_v<::consts::FloatStruct>);
inline void ::consts::FloatStruct::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::consts::FloatStruct;
  static_assert(0 == offsetof(__crubit_assert_type, x));
  static_assert(4 == offsetof(__crubit_assert_type, y));
}
static_assert(
    sizeof(::consts::NestedStruct) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::consts::NestedStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::consts::NestedStruct>);
static_assert(::std::is_trivially_move_constructible_v<::consts::NestedStruct>);
static_assert(::std::is_trivially_move_assignable_v<::consts::NestedStruct>);
static_assert(::std::is_trivially_copy_constructible_v<::consts::NestedStruct>);
static_assert(::std::is_trivially_copy_assignable_v<::consts::NestedStruct>);
namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aNestedStruct_uconsts_ugolden_x0000003a_x0000003aNestedStruct(
    ::consts::NestedStruct const&, ::consts::NestedStruct const&);
}
inline bool(NestedStruct::operator==)(
    ::consts::NestedStruct const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aNestedStruct_uconsts_ugolden_x0000003a_x0000003aNestedStruct(
          self, other);
}
inline void ::consts::NestedStruct::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::consts::NestedStruct;
  static_assert(0 == offsetof(__crubit_assert_type, point));
  static_assert(8 == offsetof(__crubit_assert_type, tuple));
}
static_assert(
    sizeof(::consts::Point) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::consts::Point) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::consts::Point>);
static_assert(::std::is_trivially_move_constructible_v<::consts::Point>);
static_assert(::std::is_trivially_move_assignable_v<::consts::Point>);
static_assert(::std::is_trivially_copy_constructible_v<::consts::Point>);
static_assert(::std::is_trivially_copy_assignable_v<::consts::Point>);
constexpr ::consts::Point(::consts::Point::ZERO) = ::consts::Point{
    .x = INT32_C(0), .y = INT32_C(0)};
namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aPoint_uconsts_ugolden_x0000003a_x0000003aPoint(
    ::consts::Point const&, ::consts::Point const&);
}
inline bool(Point::operator==)(::consts::Point const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aPoint_uconsts_ugolden_x0000003a_x0000003aPoint(
          self, other);
}
inline void ::consts::Point::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::consts::Point;
  static_assert(0 == offsetof(__crubit_assert_type, x));
  static_assert(4 == offsetof(__crubit_assert_type, y));
}
static_assert(
    sizeof(::consts::StructWithArray) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::consts::StructWithArray) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::consts::StructWithArray>);
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
inline bool(StructWithArray::operator==)(
    ::consts::StructWithArray const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aStructWithArray_uconsts_ugolden_x0000003a_x0000003aStructWithArray(
          self, other);
}
inline void ::consts::StructWithArray::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::consts::StructWithArray;
  static_assert(0 == offsetof(__crubit_assert_type, values));
}
static_assert(
    sizeof(::consts::StructWithRef) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::consts::StructWithRef) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::consts::StructWithRef>);
static_assert(
    ::std::is_trivially_move_constructible_v<::consts::StructWithRef>);
static_assert(::std::is_trivially_move_assignable_v<::consts::StructWithRef>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::consts::StructWithRef>);
static_assert(::std::is_trivially_copy_assignable_v<::consts::StructWithRef>);
inline ::consts::StructWithRef::StructWithRef(::crubit::UnsafeRelocateTag,
                                              StructWithRef&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aStructWithRef_x0000003c_x00000027a_x0000003e_uconsts_ugolden_x0000003a_x0000003aStructWithRef_x0000003c_x00000027a_x0000003e_u_x00000027a(
    ::consts::StructWithRef const&, ::consts::StructWithRef const&);
}
inline bool(StructWithRef::operator==)(
    ::consts::StructWithRef const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aStructWithRef_x0000003c_x00000027a_x0000003e_uconsts_ugolden_x0000003a_x0000003aStructWithRef_x0000003c_x00000027a_x0000003e_u_x00000027a(
          self, other);
}
inline void ::consts::StructWithRef::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::consts::StructWithRef;
  static_assert(0 == offsetof(__crubit_assert_type, r));
}
static_assert(
    sizeof(::consts::StructWithStr) == 12,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::consts::StructWithStr) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::consts::StructWithStr>);
static_assert(
    ::std::is_trivially_move_constructible_v<::consts::StructWithStr>);
static_assert(::std::is_trivially_move_assignable_v<::consts::StructWithStr>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::consts::StructWithStr>);
static_assert(::std::is_trivially_copy_assignable_v<::consts::StructWithStr>);
namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aStructWithStr_x0000003c_x00000027a_x0000003e_uconsts_ugolden_x0000003a_x0000003aStructWithStr_x0000003c_x00000027a_x0000003e_u_x00000027a(
    ::consts::StructWithStr const&, ::consts::StructWithStr const&);
}
inline bool(StructWithStr::operator==)(
    ::consts::StructWithStr const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aStructWithStr_x0000003c_x00000027a_x0000003e_uconsts_ugolden_x0000003a_x0000003aStructWithStr_x0000003c_x00000027a_x0000003e_u_x00000027a(
          self, other);
}
inline void ::consts::StructWithStr::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::consts::StructWithStr;
  static_assert(0 == offsetof(__crubit_assert_type, msg));
  static_assert(8 == offsetof(__crubit_assert_type, count));
}
static_assert(
    sizeof(::consts::TupleStruct) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::consts::TupleStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::consts::TupleStruct>);
static_assert(::std::is_trivially_move_constructible_v<::consts::TupleStruct>);
static_assert(::std::is_trivially_move_assignable_v<::consts::TupleStruct>);
static_assert(::std::is_trivially_copy_constructible_v<::consts::TupleStruct>);
static_assert(::std::is_trivially_copy_assignable_v<::consts::TupleStruct>);
namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aTupleStruct_uconsts_ugolden_x0000003a_x0000003aTupleStruct(
    ::consts::TupleStruct const&, ::consts::TupleStruct const&);
}
inline bool(TupleStruct::operator==)(::consts::TupleStruct const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aTupleStruct_uconsts_ugolden_x0000003a_x0000003aTupleStruct(
          self, other);
}
inline void ::consts::TupleStruct::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::consts::TupleStruct;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
  static_assert(4 == offsetof(__crubit_assert_type, __field1));
}
static_assert(
    sizeof(::consts::TyWithAssocConsts) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::consts::TyWithAssocConsts) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::consts::TyWithAssocConsts>);
static_assert(
    ::std::is_trivially_move_constructible_v<::consts::TyWithAssocConsts>);
static_assert(
    ::std::is_trivially_move_assignable_v<::consts::TyWithAssocConsts>);
inline ::consts::TyWithAssocConsts::TyWithAssocConsts(
    ::crubit::UnsafeRelocateTag, TyWithAssocConsts&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::consts::TyWithAssocConsts::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::consts::TyWithAssocConsts;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
namespace __crubit_internal {
extern "C" ::std::uint32_t __crubit_thunk_f32_uto_ubits(float);
}
inline ::std::uint32_t f32_to_bits(float x) {
  return __crubit_internal::__crubit_thunk_f32_uto_ubits(x);
}

namespace __crubit_internal {
extern "C" ::std::uint64_t __crubit_thunk_f64_uto_ubits(double);
}
inline ::std::uint64_t f64_to_bits(double x) {
  return __crubit_internal::__crubit_thunk_f64_uto_ubits(x);
}

namespace __crubit_internal {
extern "C" float __crubit_thunk_roundtrip_uf32(float);
}
inline float roundtrip_f32(float x) {
  return __crubit_internal::__crubit_thunk_roundtrip_uf32(x);
}

namespace __crubit_internal {
extern "C" double __crubit_thunk_roundtrip_uf64(double);
}
inline double roundtrip_f64(double x) {
  return __crubit_internal::__crubit_thunk_roundtrip_uf64(x);
}

}  // namespace consts

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_CONSTS_CONSTS_GOLDEN
