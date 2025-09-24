// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// structs_golden
// Features: supported, unsafe_types

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_STRUCTS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_STRUCTS_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/rs_std/char.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

namespace structs {

namespace repr_c {

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=12
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: repr_c :: Point") alignas(4) [[clang::trivial_abi]]
Point final {
 public:
  // `repr_c::Point` doesn't implement the `Default` trait
  Point() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~Point() = default;
  Point(Point&&) = default;
  Point& operator=(Point&&) = default;

  // `repr_c::Point` doesn't implement the `Clone` trait
  Point(const Point&) = delete;
  Point& operator=(const Point&) = delete;
  Point(::crubit::UnsafeRelocateTag, Point&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=13
    std::int32_t x;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=14
    std::int32_t y;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=17
::structs::repr_c::Point create(std::int32_t x, std::int32_t y);

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=21
std::int32_t get_x(::structs::repr_c::Point p);

}  // namespace repr_c

namespace default_repr {

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=33
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: default_repr :: Point") alignas(4)
    [[clang::trivial_abi]] Point final {
 public:
  // `default_repr::Point` doesn't implement the `Default` trait
  Point() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~Point() = default;
  Point(Point&&) = default;
  Point& operator=(Point&&) = default;

  // `default_repr::Point` doesn't implement the `Clone` trait
  Point(const Point&) = delete;
  Point& operator=(const Point&) = delete;
  Point(::crubit::UnsafeRelocateTag, Point&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=34
    std::int32_t x;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=35
    std::int32_t y;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=38
::structs::default_repr::Point create(std::int32_t x, std::int32_t y);

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=42
std::int32_t get_x(::structs::default_repr::Point p);

}  // namespace default_repr

namespace non_cpp_movable {

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=50
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: non_cpp_movable :: Point") alignas(4)
    [[clang::trivial_abi]] Point final {
 public:
  // `non_cpp_movable::Point` doesn't implement the `Default` trait
  Point() = delete;

  // Drop::drop
  ~Point();

  // C++ moves are deleted because there's no non-destructive implementation
  // available.
  Point(Point&&) = delete;
  Point& operator=(Point&&) = delete;
  // `non_cpp_movable::Point` doesn't implement the `Clone` trait
  Point(const Point&) = delete;
  Point& operator=(const Point&) = delete;
  Point(::crubit::UnsafeRelocateTag, Point&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=51
    std::int32_t x;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=52
    std::int32_t y;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=62
::structs::non_cpp_movable::Point create(std::int32_t x, std::int32_t y);

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=67
std::int32_t get_x(::structs::non_cpp_movable::Point const& p);

}  // namespace non_cpp_movable

namespace zst_fields {

// Error generating bindings for `zst_fields::Zst1` defined at
// cc_bindings_from_rs/test/structs/structs.rs;l=75:
// Zero-sized types (ZSTs) are not supported (b/258259459)

// Error generating bindings for `zst_fields::Zst2` defined at
// cc_bindings_from_rs/test/structs/structs.rs;l=76:
// Zero-sized types (ZSTs) are not supported (b/258259459)

// Error generating bindings for `zst_fields::Zst3` defined at
// cc_bindings_from_rs/test/structs/structs.rs;l=77:
// Zero-sized types (ZSTs) are not supported (b/258259459)

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=79
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: zst_fields :: ZstFields") alignas(4)
    [[clang::trivial_abi]] ZstFields final {
 public:
  // `zst_fields::ZstFields` doesn't implement the `Default` trait
  ZstFields() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~ZstFields() = default;
  ZstFields(ZstFields&&) = default;
  ZstFields& operator=(ZstFields&&) = default;

  // `zst_fields::ZstFields` doesn't implement the `Clone` trait
  ZstFields(const ZstFields&) = delete;
  ZstFields& operator=(const ZstFields&) = delete;
  ZstFields(::crubit::UnsafeRelocateTag, ZstFields&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=83
    std::int32_t value;
  };
  // Skipped bindings for field `zst1`: ZST fields are not supported
  // (b/258259459)

  // Skipped bindings for field `zst2`: ZST fields are not supported
  // (b/258259459)

  // Skipped bindings for field `zst3`: ZST fields are not supported
  // (b/258259459)
 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=86
::structs::zst_fields::ZstFields create(std::int32_t value);

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=90
std::int32_t get_value(::structs::zst_fields::ZstFields x);

}  // namespace zst_fields

namespace abi_classification {

//  Expected ABI classification: integer.  (For indirect confirmation, see
//
//  the disassembly at https://godbolt.org/z/b7eeGcrGn).
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=119
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: abi_classification :: StructInteger") alignas(4)
    [[clang::trivial_abi]] StructInteger final {
 public:
  // `abi_classification::StructInteger` doesn't implement the `Default` trait
  StructInteger() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~StructInteger() = default;
  StructInteger(StructInteger&&) = default;
  StructInteger& operator=(StructInteger&&) = default;

  // `abi_classification::StructInteger` doesn't implement the `Clone` trait
  StructInteger(const StructInteger&) = delete;
  StructInteger& operator=(const StructInteger&) = delete;
  StructInteger(::crubit::UnsafeRelocateTag, StructInteger&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=139
  static ::structs::abi_classification::StructInteger create(std::int32_t i);

  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=142
  static ::structs::abi_classification::StructInteger multiply(
      ::structs::abi_classification::StructInteger x,
      ::structs::abi_classification::StructInteger y);

  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=145
  static std::int32_t inspect(::structs::abi_classification::StructInteger s);

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=119
    std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

//  Expected ABI classification: SSE.  (For indirect confirmation, see the
//
//  disassembly at https://godbolt.org/z/b7eeGcrGn).
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=123
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: abi_classification :: StructFloat") alignas(8)
    [[clang::trivial_abi]] StructFloat final {
 public:
  // `abi_classification::StructFloat` doesn't implement the `Default` trait
  StructFloat() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~StructFloat() = default;
  StructFloat(StructFloat&&) = default;
  StructFloat& operator=(StructFloat&&) = default;

  // `abi_classification::StructFloat` doesn't implement the `Clone` trait
  StructFloat(const StructFloat&) = delete;
  StructFloat& operator=(const StructFloat&) = delete;
  StructFloat(::crubit::UnsafeRelocateTag, StructFloat&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=151
  static ::structs::abi_classification::StructFloat create(float f);

  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=154
  static ::structs::abi_classification::StructFloat multiply(
      ::structs::abi_classification::StructFloat x,
      ::structs::abi_classification::StructFloat y);

  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=159
  static float inspect(::structs::abi_classification::StructFloat s);

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=124
    double __field0;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=125
    float __field1;
  };
  unsigned char __padding1[4];

 private:
  static void __crubit_field_offset_assertions();
};

//  Expected ABI classification: memory.  (For indirect confirmation, see
//
//  the disassembly at https://godbolt.org/z/b7eeGcrGn).
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=133
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: abi_classification :: StructMemory") alignas(1)
    [[clang::trivial_abi]] __attribute__((packed)) StructMemory final {
 public:
  // `abi_classification::StructMemory` doesn't implement the `Default` trait
  StructMemory() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~StructMemory() = default;
  StructMemory(StructMemory&&) = default;
  StructMemory& operator=(StructMemory&&) = default;

  // `abi_classification::StructMemory` doesn't implement the `Clone` trait
  StructMemory(const StructMemory&) = delete;
  StructMemory& operator=(const StructMemory&) = delete;
  StructMemory(::crubit::UnsafeRelocateTag, StructMemory&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=166
  static ::structs::abi_classification::StructMemory create(std::int32_t i);

  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=169
  static ::structs::abi_classification::StructMemory multiply(
      ::structs::abi_classification::StructMemory x,
      ::structs::abi_classification::StructMemory y);

  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=172
  static std::int32_t inspect(::structs::abi_classification::StructMemory s);

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=134
    std::uint8_t _padding;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=135
    std::int32_t i;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace abi_classification

namespace struct_by_float_passing_with_no_cc_definition {

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=191
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: struct_by_float_passing_with_no_cc_definition :: "
    "StructFloat") alignas(8) [[clang::trivial_abi]] StructFloat final {
 public:
  // `struct_by_float_passing_with_no_cc_definition::StructFloat` doesn't
  // implement the `Default` trait
  StructFloat() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~StructFloat() = default;
  StructFloat(StructFloat&&) = default;
  StructFloat& operator=(StructFloat&&) = default;

  // `struct_by_float_passing_with_no_cc_definition::StructFloat` doesn't
  // implement the `Clone` trait
  StructFloat(const StructFloat&) = delete;
  StructFloat& operator=(const StructFloat&) = delete;
  StructFloat(::crubit::UnsafeRelocateTag, StructFloat&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=192
    double __field0;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=193
    float __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=199
::structs::struct_by_float_passing_with_no_cc_definition::StructFloat
no_mangle_create(float f);

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=204
::structs::struct_by_float_passing_with_no_cc_definition::StructFloat
no_mangle_multiply(
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat x,
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat y);

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=211
float no_mangle_inspect(
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat s);

}  // namespace struct_by_float_passing_with_no_cc_definition

namespace struct_by_float_passing_with_no_thunk {

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=231
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: struct_by_float_passing_with_no_thunk :: "
    "StructFloat") alignas(8) [[clang::trivial_abi]] StructFloat final {
 public:
  // `struct_by_float_passing_with_no_thunk::StructFloat` doesn't implement the
  // `Default` trait
  StructFloat() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~StructFloat() = default;
  StructFloat(StructFloat&&) = default;
  StructFloat& operator=(StructFloat&&) = default;

  // Clone::clone
  StructFloat(const StructFloat&);

  // Clone::clone_from
  StructFloat& operator=(const StructFloat&);

  StructFloat(::crubit::UnsafeRelocateTag, StructFloat&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=232
    double __field0;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=233
    float __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=247
::structs::struct_by_float_passing_with_no_thunk::StructFloat thunkless_create(
    float f);

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=252
::structs::struct_by_float_passing_with_no_thunk::StructFloat
thunkless_multiply(
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat x,
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat y);

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=259
float thunkless_inspect(
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat s);

}  // namespace struct_by_float_passing_with_no_thunk

namespace dynamically_sized_type {

// Error generating bindings for
// `dynamically_sized_type::DynamicallySizedStruct` defined at
// cc_bindings_from_rs/test/structs/structs.rs;l=273:
// Bindings for dynamically sized types are not supported.

}

namespace nested_ptr_type_mutability_qualifiers {

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=284
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: nested_ptr_type_mutability_qualifiers :: "
    "SomeStruct") alignas(8) [[clang::trivial_abi]] SomeStruct final {
 public:
  // Default::default
  SomeStruct();

  // No custom `Drop` impl and no custom "drop glue" required
  ~SomeStruct() = default;
  SomeStruct(SomeStruct&&) = default;
  SomeStruct& operator=(SomeStruct&&) = default;

  // `nested_ptr_type_mutability_qualifiers::SomeStruct` doesn't implement the
  // `Clone` trait
  SomeStruct(const SomeStruct&) = delete;
  SomeStruct& operator=(const SomeStruct&) = delete;
  SomeStruct(::crubit::UnsafeRelocateTag, SomeStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=285
    float const** mut_const_ptr;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=286
    float* const* const_mut_ptr;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace nested_ptr_type_mutability_qualifiers

namespace interior_mutability {

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=322
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: interior_mutability :: SomeStruct") alignas(4)
    [[clang::trivial_abi]] SomeStruct final {
 public:
  // Default::default
  SomeStruct();

  // No custom `Drop` impl and no custom "drop glue" required
  ~SomeStruct() = default;
  SomeStruct(SomeStruct&&) = default;
  SomeStruct& operator=(SomeStruct&&) = default;

  // `interior_mutability::SomeStruct` doesn't implement the `Clone` trait
  SomeStruct(const SomeStruct&) = delete;
  SomeStruct& operator=(const SomeStruct&) = delete;
  SomeStruct(::crubit::UnsafeRelocateTag, SomeStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  // Field type has been replaced with a blob of bytes: Generic types are not
  // supported yet (b/259749095)
  unsigned char field[4];

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace interior_mutability

namespace unsupported_types {

// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=332
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: unsupported_types :: SomeStruct") alignas(4)
    [[clang::trivial_abi]] SomeStruct final {
 public:
  // Default::default
  SomeStruct();

  // No custom `Drop` impl and no custom "drop glue" required
  ~SomeStruct() = default;
  SomeStruct(SomeStruct&&) = default;
  SomeStruct& operator=(SomeStruct&&) = default;

  // `unsupported_types::SomeStruct` doesn't implement the `Clone` trait
  SomeStruct(const SomeStruct&) = delete;
  SomeStruct& operator=(const SomeStruct&) = delete;
  SomeStruct(::crubit::UnsafeRelocateTag, SomeStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=337
  static ::structs::unsupported_types::SomeStruct create(rs_std::char_ x);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=333
    rs_std::char_ unsupported_field;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace unsupported_types

namespace repr_c {

static_assert(
    sizeof(Point) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Point) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<Point>);
static_assert(std::is_trivially_move_constructible_v<Point>);
static_assert(std::is_trivially_move_assignable_v<Point>);
inline void Point::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Point, x));
  static_assert(4 == offsetof(Point, y));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(std::int32_t, std::int32_t,
                                      ::structs::repr_c::Point* __ret_ptr);
}
inline ::structs::repr_c::Point create(std::int32_t x, std::int32_t y) {
  crubit::Slot<::structs::repr_c::Point> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(x, y, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_get_ux(::structs::repr_c::Point*);
}
inline std::int32_t get_x(::structs::repr_c::Point p) {
  return __crubit_internal::__crubit_thunk_get_ux(&p);
}

}  // namespace repr_c

namespace default_repr {

static_assert(
    sizeof(Point) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Point) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<Point>);
static_assert(std::is_trivially_move_constructible_v<Point>);
static_assert(std::is_trivially_move_assignable_v<Point>);
inline void Point::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Point, x));
  static_assert(4 == offsetof(Point, y));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    std::int32_t, std::int32_t, ::structs::default_repr::Point* __ret_ptr);
}
inline ::structs::default_repr::Point create(std::int32_t x, std::int32_t y) {
  crubit::Slot<::structs::default_repr::Point> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(x, y, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_get_ux(::structs::default_repr::Point*);
}
inline std::int32_t get_x(::structs::default_repr::Point p) {
  return __crubit_internal::__crubit_thunk_get_ux(&p);
}

}  // namespace default_repr

namespace non_cpp_movable {

static_assert(
    sizeof(Point) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Point) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::structs::non_cpp_movable::Point&);
}
inline Point::~Point() { __crubit_internal::__crubit_thunk_drop(*this); }
inline void Point::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Point, x));
  static_assert(4 == offsetof(Point, y));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    std::int32_t, std::int32_t, ::structs::non_cpp_movable::Point* __ret_ptr);
}
inline ::structs::non_cpp_movable::Point create(std::int32_t x,
                                                std::int32_t y) {
  crubit::Slot<::structs::non_cpp_movable::Point> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(x, y, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_get_ux(
    ::structs::non_cpp_movable::Point const&);
}
inline std::int32_t get_x(::structs::non_cpp_movable::Point const& p) {
  return __crubit_internal::__crubit_thunk_get_ux(p);
}

}  // namespace non_cpp_movable

namespace zst_fields {

static_assert(
    sizeof(ZstFields) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(ZstFields) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<ZstFields>);
static_assert(std::is_trivially_move_constructible_v<ZstFields>);
static_assert(std::is_trivially_move_assignable_v<ZstFields>);
inline void ZstFields::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ZstFields, value));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    std::int32_t, ::structs::zst_fields::ZstFields* __ret_ptr);
}
inline ::structs::zst_fields::ZstFields create(std::int32_t value) {
  crubit::Slot<::structs::zst_fields::ZstFields> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(value, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_get_uvalue(
    ::structs::zst_fields::ZstFields*);
}
inline std::int32_t get_value(::structs::zst_fields::ZstFields x) {
  return __crubit_internal::__crubit_thunk_get_uvalue(&x);
}

}  // namespace zst_fields

namespace abi_classification {

static_assert(
    sizeof(StructInteger) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructInteger) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<StructInteger>);
static_assert(std::is_trivially_move_constructible_v<StructInteger>);
static_assert(std::is_trivially_move_assignable_v<StructInteger>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    std::int32_t, ::structs::abi_classification::StructInteger* __ret_ptr);
}
inline ::structs::abi_classification::StructInteger StructInteger::create(
    std::int32_t i) {
  crubit::Slot<::structs::abi_classification::StructInteger>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(i, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_multiply(
    ::structs::abi_classification::StructInteger*,
    ::structs::abi_classification::StructInteger*,
    ::structs::abi_classification::StructInteger* __ret_ptr);
}
inline ::structs::abi_classification::StructInteger StructInteger::multiply(
    ::structs::abi_classification::StructInteger x,
    ::structs::abi_classification::StructInteger y) {
  crubit::Slot<::structs::abi_classification::StructInteger>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_multiply(&x, &y, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_inspect(
    ::structs::abi_classification::StructInteger*);
}
inline std::int32_t StructInteger::inspect(
    ::structs::abi_classification::StructInteger s) {
  return __crubit_internal::__crubit_thunk_inspect(&s);
}
inline void StructInteger::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructInteger, __field0));
}
static_assert(
    sizeof(StructFloat) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructFloat) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<StructFloat>);
static_assert(std::is_trivially_move_constructible_v<StructFloat>);
static_assert(std::is_trivially_move_assignable_v<StructFloat>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    float, ::structs::abi_classification::StructFloat* __ret_ptr);
}
inline ::structs::abi_classification::StructFloat StructFloat::create(float f) {
  crubit::Slot<::structs::abi_classification::StructFloat>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(f, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_multiply(
    ::structs::abi_classification::StructFloat*,
    ::structs::abi_classification::StructFloat*,
    ::structs::abi_classification::StructFloat* __ret_ptr);
}
inline ::structs::abi_classification::StructFloat StructFloat::multiply(
    ::structs::abi_classification::StructFloat x,
    ::structs::abi_classification::StructFloat y) {
  crubit::Slot<::structs::abi_classification::StructFloat>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_multiply(&x, &y, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" float __crubit_thunk_inspect(
    ::structs::abi_classification::StructFloat*);
}
inline float StructFloat::inspect(
    ::structs::abi_classification::StructFloat s) {
  return __crubit_internal::__crubit_thunk_inspect(&s);
}
inline void StructFloat::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructFloat, __field0));
  static_assert(8 == offsetof(StructFloat, __field1));
}
static_assert(
    sizeof(StructMemory) == 5,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructMemory) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<StructMemory>);
static_assert(std::is_trivially_move_constructible_v<StructMemory>);
static_assert(std::is_trivially_move_assignable_v<StructMemory>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    std::int32_t, ::structs::abi_classification::StructMemory* __ret_ptr);
}
inline ::structs::abi_classification::StructMemory StructMemory::create(
    std::int32_t i) {
  crubit::Slot<::structs::abi_classification::StructMemory>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(i, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_multiply(
    ::structs::abi_classification::StructMemory*,
    ::structs::abi_classification::StructMemory*,
    ::structs::abi_classification::StructMemory* __ret_ptr);
}
inline ::structs::abi_classification::StructMemory StructMemory::multiply(
    ::structs::abi_classification::StructMemory x,
    ::structs::abi_classification::StructMemory y) {
  crubit::Slot<::structs::abi_classification::StructMemory>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_multiply(&x, &y, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_inspect(
    ::structs::abi_classification::StructMemory*);
}
inline std::int32_t StructMemory::inspect(
    ::structs::abi_classification::StructMemory s) {
  return __crubit_internal::__crubit_thunk_inspect(&s);
}
inline void StructMemory::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructMemory, _padding));
  static_assert(1 == offsetof(StructMemory, i));
}
}  // namespace abi_classification

namespace struct_by_float_passing_with_no_cc_definition {

static_assert(
    sizeof(StructFloat) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructFloat) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<StructFloat>);
static_assert(std::is_trivially_move_constructible_v<StructFloat>);
static_assert(std::is_trivially_move_assignable_v<StructFloat>);
inline void StructFloat::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructFloat, __field0));
  static_assert(8 == offsetof(StructFloat, __field1));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_no_umangle_ucreate(
    float,
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat*
        __ret_ptr);
}
inline ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat
no_mangle_create(float f) {
  crubit::Slot<
      ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_no_umangle_ucreate(f,
                                                       __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_no_umangle_umultiply(
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat*,
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat*,
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat*
        __ret_ptr);
}
inline ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat
no_mangle_multiply(
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat x,
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat y) {
  crubit::Slot<
      ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_no_umangle_umultiply(
      &x, &y, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" float __crubit_thunk_no_umangle_uinspect(
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat*);
}
inline float no_mangle_inspect(
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat s) {
  return __crubit_internal::__crubit_thunk_no_umangle_uinspect(&s);
}

}  // namespace struct_by_float_passing_with_no_cc_definition

namespace struct_by_float_passing_with_no_thunk {

static_assert(
    sizeof(StructFloat) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructFloat) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<StructFloat>);
static_assert(std::is_trivially_move_constructible_v<StructFloat>);
static_assert(std::is_trivially_move_assignable_v<StructFloat>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat const&,
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat&,
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat const&);
}
inline StructFloat::StructFloat(const StructFloat& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline StructFloat& StructFloat::operator=(const StructFloat& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
inline void StructFloat::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructFloat, __field0));
  static_assert(8 == offsetof(StructFloat, __field1));
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_struct_uby_ufloat_upassing_uwith_uno_uthunk_u_uthunkless_ucreate(
    float,
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat* __ret_ptr);
}
inline ::structs::struct_by_float_passing_with_no_thunk::StructFloat
thunkless_create(float f) {
  crubit::Slot<::structs::struct_by_float_passing_with_no_thunk::StructFloat>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_struct_uby_ufloat_upassing_uwith_uno_uthunk_u_uthunkless_ucreate(
          f, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_struct_uby_ufloat_upassing_uwith_uno_uthunk_u_uthunkless_umultiply(
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat*,
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat*,
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat* __ret_ptr);
}
inline ::structs::struct_by_float_passing_with_no_thunk::StructFloat
thunkless_multiply(
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat x,
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat y) {
  crubit::Slot<::structs::struct_by_float_passing_with_no_thunk::StructFloat>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_struct_uby_ufloat_upassing_uwith_uno_uthunk_u_uthunkless_umultiply(
          &x, &y, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" float
__crubit_thunk_struct_uby_ufloat_upassing_uwith_uno_uthunk_u_uthunkless_uinspect(
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat*);
}
inline float thunkless_inspect(
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat s) {
  return __crubit_internal::
      __crubit_thunk_struct_uby_ufloat_upassing_uwith_uno_uthunk_u_uthunkless_uinspect(
          &s);
}

}  // namespace struct_by_float_passing_with_no_thunk

namespace dynamically_sized_type {}

namespace nested_ptr_type_mutability_qualifiers {

static_assert(
    sizeof(SomeStruct) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(SomeStruct) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    ::structs::nested_ptr_type_mutability_qualifiers::SomeStruct* __ret_ptr);
}
inline SomeStruct::SomeStruct() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<SomeStruct>);
static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
inline void SomeStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(SomeStruct, mut_const_ptr));
  static_assert(8 == offsetof(SomeStruct, const_mut_ptr));
}
}  // namespace nested_ptr_type_mutability_qualifiers

namespace interior_mutability {

static_assert(
    sizeof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    ::structs::interior_mutability::SomeStruct* __ret_ptr);
}
inline SomeStruct::SomeStruct() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<SomeStruct>);
static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
inline void SomeStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(SomeStruct, field));
}
}  // namespace interior_mutability

namespace unsupported_types {

static_assert(
    sizeof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    ::structs::unsupported_types::SomeStruct* __ret_ptr);
}
inline SomeStruct::SomeStruct() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<SomeStruct>);
static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    rs_std::char_, ::structs::unsupported_types::SomeStruct* __ret_ptr);
}
inline ::structs::unsupported_types::SomeStruct SomeStruct::create(
    rs_std::char_ x) {
  crubit::Slot<::structs::unsupported_types::SomeStruct>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(x, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void SomeStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(SomeStruct, unsupported_field));
}
}  // namespace unsupported_types

}  // namespace structs
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_STRUCTS_GOLDEN
