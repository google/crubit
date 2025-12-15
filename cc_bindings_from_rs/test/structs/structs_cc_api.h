// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// structs_golden
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

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

namespace struct_by_float_passing_with_no_cc_definition {
struct StructFloat;
}

namespace zst_fields {
struct ZstFields;
}

namespace non_cpp_movable {
struct Point;
}

namespace default_repr {
struct Point;
}

namespace keyword_named_fields_and_methods {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=390
struct
    CRUBIT_INTERNAL_RUST_TYPE(
        ":: structs_golden :: keyword_named_fields_and_methods :: "
        "AField") alignas(4) [[clang::trivial_abi]] AField final {
 public:
  // `structs_golden::keyword_named_fields_and_methods::AField` doesn't
  // implement the `Default` trait
  AField() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~AField() = default;
  AField(AField&&) = default;
  AField& operator=(AField&&) = default;

  // `structs_golden::keyword_named_fields_and_methods::AField` doesn't
  // implement the `Clone` trait
  AField(const AField&) = delete;
  AField& operator=(const AField&) = delete;
  AField(::crubit::UnsafeRelocateTag, AField&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=395
  std::int32_t operator_() const;

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=391
    std::int32_t operator__;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace keyword_named_fields_and_methods

namespace zst_fields {

// Error generating bindings for `zst_fields::Zst3` defined at
// cc_bindings_from_rs/test/structs/structs.rs;l=90:
// Zero-sized types (ZSTs) are not supported (b/258259459)

}

namespace struct_by_float_passing_with_no_thunk {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=264
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: struct_by_float_passing_with_no_thunk :: "
    "StructFloat") alignas(8) [[clang::trivial_abi]] StructFloat final {
 public:
  // `structs_golden::struct_by_float_passing_with_no_thunk::StructFloat`
  // doesn't implement the `Default` trait
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
    // cc_bindings_from_rs/test/structs/structs.rs;l=265
    double __field0;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=266
    float __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=295
float thunkless_inspect(
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat s);

}  // namespace struct_by_float_passing_with_no_thunk

namespace struct_by_float_passing_with_no_cc_definition {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=243
float no_mangle_inspect(
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat s);

}  // namespace struct_by_float_passing_with_no_cc_definition

namespace repr_c {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=14
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: repr_c :: Point") alignas(4) [[clang::trivial_abi]]
Point final {
 public:
  // `structs_golden::repr_c::Point` doesn't implement the `Default` trait
  Point() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~Point() = default;
  Point(Point&&) = default;
  Point& operator=(Point&&) = default;

  // `structs_golden::repr_c::Point` doesn't implement the `Clone` trait
  Point(const Point&) = delete;
  Point& operator=(const Point&) = delete;
  Point(::crubit::UnsafeRelocateTag, Point&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=15
    std::int32_t x;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=16
    std::int32_t y;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace repr_c

namespace non_cpp_movable {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=79
std::int32_t get_x(::structs::non_cpp_movable::Point const& p);

}  // namespace non_cpp_movable

namespace default_repr {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=45
::structs::default_repr::Point create(std::int32_t x, std::int32_t y);

}  // namespace default_repr

namespace struct_by_float_passing_with_no_thunk {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=287
::structs::struct_by_float_passing_with_no_thunk::StructFloat
thunkless_multiply(
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat x,
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat y);

}  // namespace struct_by_float_passing_with_no_thunk

namespace unsupported_types {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=373
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

  // `structs_golden::unsupported_types::SomeStruct` doesn't implement the
  // `Clone` trait
  SomeStruct(const SomeStruct&) = delete;
  SomeStruct& operator=(const SomeStruct&) = delete;
  SomeStruct(::crubit::UnsafeRelocateTag, SomeStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=379
  static ::structs::unsupported_types::SomeStruct create(rs_std::char_ x);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=374
    rs_std::char_ unsupported_field;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace unsupported_types

namespace struct_by_float_passing_with_no_cc_definition {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=229
::structs::struct_by_float_passing_with_no_cc_definition::StructFloat
no_mangle_create(float f);

}  // namespace struct_by_float_passing_with_no_cc_definition

namespace dynamically_sized_type {

// Error generating bindings for
// `dynamically_sized_type::DynamicallySizedStruct` defined at
// cc_bindings_from_rs/test/structs/structs.rs;l=309:
// Bindings for dynamically sized types are not supported.

}

namespace zst_fields {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=101
::structs::zst_fields::ZstFields create(std::int32_t value);

}  // namespace zst_fields

namespace struct_by_float_passing_with_no_cc_definition {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=220
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: struct_by_float_passing_with_no_cc_definition :: "
    "StructFloat") alignas(8) [[clang::trivial_abi]] StructFloat final {
 public:
  // `structs_golden::struct_by_float_passing_with_no_cc_definition::StructFloat`
  // doesn't implement the `Default` trait
  StructFloat() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~StructFloat() = default;
  StructFloat(StructFloat&&) = default;
  StructFloat& operator=(StructFloat&&) = default;

  // `structs_golden::struct_by_float_passing_with_no_cc_definition::StructFloat`
  // doesn't implement the `Clone` trait
  StructFloat(const StructFloat&) = delete;
  StructFloat& operator=(const StructFloat&) = delete;
  StructFloat(::crubit::UnsafeRelocateTag, StructFloat&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=221
    double __field0;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=222
    float __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=235
::structs::struct_by_float_passing_with_no_cc_definition::StructFloat
no_mangle_multiply(
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat x,
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat y);

}  // namespace struct_by_float_passing_with_no_cc_definition

namespace abi_classification {

// CRUBIT_ANNOTATE: must_bind=
//
//  Expected ABI classification: integer.  (For indirect confirmation, see
//
//  the disassembly at https://godbolt.org/z/b7eeGcrGn).
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=136
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: abi_classification :: StructInteger") alignas(4)
    [[clang::trivial_abi]] StructInteger final {
 public:
  // `structs_golden::abi_classification::StructInteger` doesn't implement the
  // `Default` trait
  StructInteger() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~StructInteger() = default;
  StructInteger(StructInteger&&) = default;
  StructInteger& operator=(StructInteger&&) = default;

  // `structs_golden::abi_classification::StructInteger` doesn't implement the
  // `Clone` trait
  StructInteger(const StructInteger&) = delete;
  StructInteger& operator=(const StructInteger&) = delete;
  StructInteger(::crubit::UnsafeRelocateTag, StructInteger&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=159
  static ::structs::abi_classification::StructInteger create(std::int32_t i);

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=163
  static ::structs::abi_classification::StructInteger multiply(
      ::structs::abi_classification::StructInteger x,
      ::structs::abi_classification::StructInteger y);

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=167
  static std::int32_t inspect(::structs::abi_classification::StructInteger s);

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=136
    std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace abi_classification

namespace repr_c {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=20
::structs::repr_c::Point create(std::int32_t x, std::int32_t y);

}  // namespace repr_c

namespace default_repr {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=50
std::int32_t get_x(::structs::default_repr::Point p);

}  // namespace default_repr

namespace abi_classification {

// CRUBIT_ANNOTATE: must_bind=
//
//  Expected ABI classification: memory.  (For indirect confirmation, see
//
//  the disassembly at https://godbolt.org/z/b7eeGcrGn).
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=152
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: abi_classification :: StructMemory") alignas(1)
    [[clang::trivial_abi]] __attribute__((packed)) StructMemory final {
 public:
  // `structs_golden::abi_classification::StructMemory` doesn't implement the
  // `Default` trait
  StructMemory() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~StructMemory() = default;
  StructMemory(StructMemory&&) = default;
  StructMemory& operator=(StructMemory&&) = default;

  // `structs_golden::abi_classification::StructMemory` doesn't implement the
  // `Clone` trait
  StructMemory(const StructMemory&) = delete;
  StructMemory& operator=(const StructMemory&) = delete;
  StructMemory(::crubit::UnsafeRelocateTag, StructMemory&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=192
  static ::structs::abi_classification::StructMemory create(std::int32_t i);

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=196
  static ::structs::abi_classification::StructMemory multiply(
      ::structs::abi_classification::StructMemory x,
      ::structs::abi_classification::StructMemory y);

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=200
  static std::int32_t inspect(::structs::abi_classification::StructMemory s);

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=153
    std::uint8_t _padding;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=154
    std::int32_t i;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace abi_classification

namespace nested_ptr_type_mutability_qualifiers {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=321
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

  // `structs_golden::nested_ptr_type_mutability_qualifiers::SomeStruct` doesn't
  // implement the `Clone` trait
  SomeStruct(const SomeStruct&) = delete;
  SomeStruct& operator=(const SomeStruct&) = delete;
  SomeStruct(::crubit::UnsafeRelocateTag, SomeStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=322
    float const** mut_const_ptr;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=323
    float* const* const_mut_ptr;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace nested_ptr_type_mutability_qualifiers

namespace zst_fields {

// Error generating bindings for `zst_fields::Zst2` defined at
// cc_bindings_from_rs/test/structs/structs.rs;l=89:
// Zero-sized types (ZSTs) are not supported (b/258259459)

}

namespace abi_classification {

// CRUBIT_ANNOTATE: must_bind=
//
//  Expected ABI classification: SSE.  (For indirect confirmation, see the
//
//  disassembly at https://godbolt.org/z/b7eeGcrGn).
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=141
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: abi_classification :: StructFloat") alignas(8)
    [[clang::trivial_abi]] StructFloat final {
 public:
  // `structs_golden::abi_classification::StructFloat` doesn't implement the
  // `Default` trait
  StructFloat() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~StructFloat() = default;
  StructFloat(StructFloat&&) = default;
  StructFloat& operator=(StructFloat&&) = default;

  // `structs_golden::abi_classification::StructFloat` doesn't implement the
  // `Clone` trait
  StructFloat(const StructFloat&) = delete;
  StructFloat& operator=(const StructFloat&) = delete;
  StructFloat(::crubit::UnsafeRelocateTag, StructFloat&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=174
  static ::structs::abi_classification::StructFloat create(float f);

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=178
  static ::structs::abi_classification::StructFloat multiply(
      ::structs::abi_classification::StructFloat x,
      ::structs::abi_classification::StructFloat y);

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/structs/structs.rs;l=184
  static float inspect(::structs::abi_classification::StructFloat s);

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=142
    double __field0;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=143
    float __field1;
  };
  unsigned char __padding1[4];

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace abi_classification

namespace interior_mutability {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=360
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

  // `structs_golden::interior_mutability::SomeStruct` doesn't implement the
  // `Clone` trait
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

namespace zst_fields {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=93
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: zst_fields :: ZstFields") alignas(4)
    [[clang::trivial_abi]] ZstFields final {
 public:
  // `structs_golden::zst_fields::ZstFields` doesn't implement the `Default`
  // trait
  ZstFields() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~ZstFields() = default;
  ZstFields(ZstFields&&) = default;
  ZstFields& operator=(ZstFields&&) = default;

  // `structs_golden::zst_fields::ZstFields` doesn't implement the `Clone` trait
  ZstFields(const ZstFields&) = delete;
  ZstFields& operator=(const ZstFields&) = delete;
  ZstFields(::crubit::UnsafeRelocateTag, ZstFields&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=97
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

}  // namespace zst_fields

namespace struct_by_float_passing_with_no_thunk {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=281
::structs::struct_by_float_passing_with_no_thunk::StructFloat thunkless_create(
    float f);

}  // namespace struct_by_float_passing_with_no_thunk

namespace non_cpp_movable {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=60
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: non_cpp_movable :: Point") alignas(4)
    [[clang::trivial_abi]] Point final {
 public:
  // `structs_golden::non_cpp_movable::Point` doesn't implement the `Default`
  // trait
  Point() = delete;

  // Drop::drop
  ~Point();

  // C++ move operations are unavailable for this type. See
  // http://<internal link>/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  Point(Point&&) = delete;
  Point& operator=(Point&&) = delete;
  // `structs_golden::non_cpp_movable::Point` doesn't implement the `Clone`
  // trait
  Point(const Point&) = delete;
  Point& operator=(const Point&) = delete;
  Point(::crubit::UnsafeRelocateTag, Point&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=61
    std::int32_t x;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=62
    std::int32_t y;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace non_cpp_movable

namespace zst_fields {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=106
std::int32_t get_value(::structs::zst_fields::ZstFields x);

// Error generating bindings for `zst_fields::Zst1` defined at
// cc_bindings_from_rs/test/structs/structs.rs;l=88:
// Zero-sized types (ZSTs) are not supported (b/258259459)

}  // namespace zst_fields

namespace repr_c {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=25
std::int32_t get_x(::structs::repr_c::Point p);

}  // namespace repr_c

namespace non_cpp_movable {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=73
::structs::non_cpp_movable::Point create(std::int32_t x, std::int32_t y);

}  // namespace non_cpp_movable

namespace default_repr {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/structs/structs.rs;l=39
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: default_repr :: Point") alignas(4)
    [[clang::trivial_abi]] Point final {
 public:
  // `structs_golden::default_repr::Point` doesn't implement the `Default` trait
  Point() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~Point() = default;
  Point(Point&&) = default;
  Point& operator=(Point&&) = default;

  // `structs_golden::default_repr::Point` doesn't implement the `Clone` trait
  Point(const Point&) = delete;
  Point& operator=(const Point&) = delete;
  Point(::crubit::UnsafeRelocateTag, Point&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=40
    std::int32_t x;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/structs.rs;l=41
    std::int32_t y;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace default_repr

namespace keyword_named_fields_and_methods {

static_assert(
    sizeof(AField) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(AField) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<AField>);
static_assert(std::is_trivially_move_constructible_v<AField>);
static_assert(std::is_trivially_move_assignable_v<AField>);
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_operator(
    ::structs::keyword_named_fields_and_methods::AField const&);
}
inline std::int32_t AField::operator_() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_operator(self);
}
inline void AField::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(AField, operator__));
}
}  // namespace keyword_named_fields_and_methods

namespace zst_fields {}

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

namespace struct_by_float_passing_with_no_cc_definition {

namespace __crubit_internal {
extern "C" float __crubit_thunk_no_umangle_uinspect(
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat*);
}
inline float no_mangle_inspect(
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat s) {
  return __crubit_internal::__crubit_thunk_no_umangle_uinspect(&s);
}

}  // namespace struct_by_float_passing_with_no_cc_definition

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
}  // namespace repr_c

namespace non_cpp_movable {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_get_ux(
    ::structs::non_cpp_movable::Point const&);
}
inline std::int32_t get_x(::structs::non_cpp_movable::Point const& p) {
  return __crubit_internal::__crubit_thunk_get_ux(p);
}

}  // namespace non_cpp_movable

namespace default_repr {

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

}  // namespace default_repr

namespace struct_by_float_passing_with_no_thunk {

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

}  // namespace struct_by_float_passing_with_no_thunk

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

namespace struct_by_float_passing_with_no_cc_definition {

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

}  // namespace struct_by_float_passing_with_no_cc_definition

namespace dynamically_sized_type {}

namespace zst_fields {

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

}  // namespace zst_fields

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

}  // namespace struct_by_float_passing_with_no_cc_definition

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
}  // namespace abi_classification

namespace repr_c {

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

}  // namespace repr_c

namespace default_repr {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_get_ux(::structs::default_repr::Point*);
}
inline std::int32_t get_x(::structs::default_repr::Point p) {
  return __crubit_internal::__crubit_thunk_get_ux(&p);
}

}  // namespace default_repr

namespace abi_classification {

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

namespace zst_fields {}

namespace abi_classification {

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
}  // namespace abi_classification

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
}  // namespace zst_fields

namespace struct_by_float_passing_with_no_thunk {

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

}  // namespace struct_by_float_passing_with_no_thunk

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
}  // namespace non_cpp_movable

namespace zst_fields {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_get_uvalue(
    ::structs::zst_fields::ZstFields*);
}
inline std::int32_t get_value(::structs::zst_fields::ZstFields x) {
  return __crubit_internal::__crubit_thunk_get_uvalue(&x);
}

}  // namespace zst_fields

namespace repr_c {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_get_ux(::structs::repr_c::Point*);
}
inline std::int32_t get_x(::structs::repr_c::Point p) {
  return __crubit_internal::__crubit_thunk_get_ux(&p);
}

}  // namespace repr_c

namespace non_cpp_movable {

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

}  // namespace non_cpp_movable

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
}  // namespace default_repr

}  // namespace structs
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_STRUCTS_GOLDEN
