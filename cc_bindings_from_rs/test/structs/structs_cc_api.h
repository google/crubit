// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// structs_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_STRUCTS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_STRUCTS_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/rs_std/char.h"
#include "support/rs_std/traits.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <ostream>
#include <string_view>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_alloc.h"
#include "support/rs_std/rs_core.h"

namespace structs::abi_classification {

// CRUBIT_ANNOTATE: must_bind=
//  Expected ABI classification: SSE.  (For indirect confirmation, see the
//  disassembly at https://godbolt.org/z/b7eeGcrGn).
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
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::structs::abi_classification::StructFloat create(float f);

  // CRUBIT_ANNOTATE: must_bind=
  static ::structs::abi_classification::StructFloat multiply(
      ::structs::abi_classification::StructFloat x,
      ::structs::abi_classification::StructFloat y);

  // CRUBIT_ANNOTATE: must_bind=
  static float inspect(::structs::abi_classification::StructFloat s);

 private:
  union {
    double __field0;
  };
  union {
    float __field1;
  };
  unsigned char __padding1[4];

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//  Expected ABI classification: integer.  (For indirect confirmation, see
//  the disassembly at https://godbolt.org/z/b7eeGcrGn).
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
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::structs::abi_classification::StructInteger create(::std::int32_t i);

  // CRUBIT_ANNOTATE: must_bind=
  static ::structs::abi_classification::StructInteger multiply(
      ::structs::abi_classification::StructInteger x,
      ::structs::abi_classification::StructInteger y);

  // CRUBIT_ANNOTATE: must_bind=
  static ::std::int32_t inspect(::structs::abi_classification::StructInteger s);

 private:
  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//  Expected ABI classification: memory.  (For indirect confirmation, see
//  the disassembly at https://godbolt.org/z/b7eeGcrGn).
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
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::structs::abi_classification::StructMemory create(::std::int32_t i);

  // CRUBIT_ANNOTATE: must_bind=
  static ::structs::abi_classification::StructMemory multiply(
      ::structs::abi_classification::StructMemory x,
      ::structs::abi_classification::StructMemory y);

  // CRUBIT_ANNOTATE: must_bind=
  static ::std::int32_t inspect(::structs::abi_classification::StructMemory s);

 private:
  union {
    ::std::uint8_t _padding;
  };
  union {
    ::std::int32_t i;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace structs::abi_classification

namespace structs::default_repr {

// CRUBIT_ANNOTATE: must_bind=
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
    ::std::memcpy(this, &value, sizeof(value));
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

// CRUBIT_ANNOTATE: must_bind=
::structs::default_repr::Point create(::std::int32_t x, ::std::int32_t y);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t get_x(::structs::default_repr::Point p);

}  // namespace structs::default_repr

namespace structs::display {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: structs_golden :: display :: DisplayStruct") alignas(4)
    [[clang::trivial_abi]] DisplayStruct final {
 public:
  // `structs_golden::display::DisplayStruct` doesn't implement the `Default`
  // trait
  DisplayStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~DisplayStruct() = default;
  DisplayStruct(DisplayStruct&&) = default;
  DisplayStruct& operator=(DisplayStruct&&) = default;

  // `structs_golden::display::DisplayStruct` doesn't implement the `Clone`
  // trait
  DisplayStruct(const DisplayStruct&) = delete;
  DisplayStruct& operator=(const DisplayStruct&) = delete;
  DisplayStruct(::crubit::UnsafeRelocateTag, DisplayStruct&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  // AbslStringify and std::ostream support via std::fmt::Display
  template <typename Sink, typename Str = rs::alloc::string::String>
  friend void AbslStringify(Sink& sink, const DisplayStruct& self) {
    crubit::Slot<Str> s;
    __crubit_thunk_ToString_uto_ustring_ustructs_ugolden_x0000003a_x0000003adisplay_x0000003a_x0000003aDisplayStruct(
        self, s.Get());
    AbslStringify(sink, ::std::move(s).AssumeInitAndTakeValue().as_str());
  }
  template <typename Str = rs::alloc::string::String>
  friend ::std::ostream& operator<<(::std::ostream& os,
                                    const DisplayStruct& self) {
    crubit::Slot<Str> s;
    __crubit_thunk_ToString_uto_ustring_ustructs_ugolden_x0000003a_x0000003adisplay_x0000003a_x0000003aDisplayStruct(
        self, s.Get());
    return os << ::std::string_view(
               ::std::move(s).AssumeInitAndTakeValue().as_str());
  }

  union {
    ::std::int32_t value;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
::structs::display::DisplayStruct create(::std::int32_t value);

}  // namespace structs::display

namespace structs::dynamically_sized_type {

// Error generating bindings for struct
// `structs_golden::dynamically_sized_type::DynamicallySizedStruct` defined at
// cc_bindings_from_rs/test/structs/structs.rs;l=309:
// Bindings for dynamically sized types are not supported.

}

namespace structs::interior_mutability {

// CRUBIT_ANNOTATE: must_bind=
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
    ::std::memcpy(this, &value, sizeof(value));
  }

 private:
  // Field type has been replaced with a blob of bytes: Generic types are not
  // supported yet (b/259749095)
  ::std::array<unsigned char, 4> field;

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace structs::interior_mutability

namespace structs::keyword_named_fields_and_methods {

// CRUBIT_ANNOTATE: must_bind=
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
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  ::std::int32_t operator_() const;

 private:
  union {
    ::std::int32_t operator__;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace structs::keyword_named_fields_and_methods

namespace structs::nested_ptr_type_mutability_qualifiers {

// CRUBIT_ANNOTATE: must_bind=
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
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    float const** mut_const_ptr;
  };
  union {
    float* const* const_mut_ptr;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace structs::nested_ptr_type_mutability_qualifiers

namespace structs::non_cpp_movable {

// CRUBIT_ANNOTATE: must_bind=
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
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  Point(Point&&) = delete;
  ::structs::non_cpp_movable::Point& operator=(Point&&) = delete;
  // `structs_golden::non_cpp_movable::Point` doesn't implement the `Clone`
  // trait
  Point(const Point&) = delete;
  Point& operator=(const Point&) = delete;
  Point(::crubit::UnsafeRelocateTag, Point&& value) {
    ::std::memcpy(this, &value, sizeof(value));
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

// CRUBIT_ANNOTATE: must_bind=
::structs::non_cpp_movable::Point create(::std::int32_t x, ::std::int32_t y);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t get_x(::structs::non_cpp_movable::Point const& p);

}  // namespace structs::non_cpp_movable

namespace structs::repr_c {

// CRUBIT_ANNOTATE: must_bind=
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
    ::std::memcpy(this, &value, sizeof(value));
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

// CRUBIT_ANNOTATE: must_bind=
::structs::repr_c::Point create(::std::int32_t x, ::std::int32_t y);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t get_x(::structs::repr_c::Point p);

}  // namespace structs::repr_c

namespace structs::struct_by_float_passing_with_no_cc_definition {

// CRUBIT_ANNOTATE: must_bind=
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
    ::std::memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    double __field0;
  };
  union {
    float __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
::structs::struct_by_float_passing_with_no_cc_definition::StructFloat
no_mangle_create(float f);

// CRUBIT_ANNOTATE: must_bind=
float no_mangle_inspect(
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat s);

// CRUBIT_ANNOTATE: must_bind=
::structs::struct_by_float_passing_with_no_cc_definition::StructFloat
no_mangle_multiply(
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat x,
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat y);

}  // namespace structs::struct_by_float_passing_with_no_cc_definition

namespace structs::struct_by_float_passing_with_no_thunk {

// CRUBIT_ANNOTATE: must_bind=
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
  ::structs::struct_by_float_passing_with_no_thunk::StructFloat& operator=(
      const StructFloat&);

  StructFloat(::crubit::UnsafeRelocateTag, StructFloat&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    double __field0;
  };
  union {
    float __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
::structs::struct_by_float_passing_with_no_thunk::StructFloat thunkless_create(
    float f);

// CRUBIT_ANNOTATE: must_bind=
float thunkless_inspect(
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat s);

// CRUBIT_ANNOTATE: must_bind=
::structs::struct_by_float_passing_with_no_thunk::StructFloat
thunkless_multiply(
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat x,
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat y);

}  // namespace structs::struct_by_float_passing_with_no_thunk

namespace structs::unsupported_types {

// CRUBIT_ANNOTATE: must_bind=
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
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::structs::unsupported_types::SomeStruct create(rs_std::char_ x);

  union {
    rs_std::char_ unsupported_field;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace structs::unsupported_types

namespace structs::zst_fields {

// Error generating bindings for struct `structs_golden::zst_fields::Zst1`
// defined at
// cc_bindings_from_rs/test/structs/structs.rs;l=88:
// Zero-sized types (ZSTs) are not supported (b/258259459)

// Error generating bindings for struct `structs_golden::zst_fields::Zst2`
// defined at
// cc_bindings_from_rs/test/structs/structs.rs;l=89:
// Zero-sized types (ZSTs) are not supported (b/258259459)

// Error generating bindings for struct `structs_golden::zst_fields::Zst3`
// defined at
// cc_bindings_from_rs/test/structs/structs.rs;l=90:
// Zero-sized types (ZSTs) are not supported (b/258259459)

// CRUBIT_ANNOTATE: must_bind=
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
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::int32_t value;
  };
  // Field `zst1` omitted: C++ does not support zero-sized types.

  // Field `zst2` omitted: C++ does not support zero-sized types.

  // Field `zst3` omitted: C++ does not support zero-sized types.
 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
::structs::zst_fields::ZstFields create(::std::int32_t value);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t get_value(::structs::zst_fields::ZstFields x);

}  // namespace structs::zst_fields

template <>
struct rs_std::impl<::structs::display::DisplayStruct,
                    ::rs::core::fmt::Display> {
  static constexpr bool kIsImplemented = true;

  // Error generating bindings for associated function
  // `<structs_golden::display::DisplayStruct as std::fmt::Display>::fmt`
  // defined at
  // cc_bindings_from_rs/test/structs/structs.rs;l=410:
  // Error formatting function return type `std::result::Result<(),
  // std::fmt::Error>`: Generic types are not supported yet (b/259749095)
};

template <>
struct rs_std::impl<::structs::interior_mutability::SomeStruct,
                    ::rs::core::fmt::Debug> {
  static constexpr bool kIsImplemented = true;

  // Error generating bindings for associated function
  // `<structs_golden::interior_mutability::SomeStruct as std::fmt::Debug>::fmt`
  // defined at
  // cc_bindings_from_rs/test/structs/structs.rs;l=358:
  // Error formatting function return type `std::result::Result<(),
  // std::fmt::Error>`: Generic types are not supported yet (b/259749095)
};

namespace structs::abi_classification {

static_assert(
    sizeof(StructFloat) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructFloat) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<StructFloat>);
static_assert(::std::is_trivially_move_constructible_v<
              ::structs::abi_classification::StructFloat>);
static_assert(::std::is_trivially_move_assignable_v<
              ::structs::abi_classification::StructFloat>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    float, ::structs::abi_classification::StructFloat* __ret_ptr);
}
inline ::structs::abi_classification::StructFloat StructFloat::create(float f) {
  crubit::Slot<::structs::abi_classification::StructFloat>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(f, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
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
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
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
    sizeof(StructInteger) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructInteger) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<StructInteger>);
static_assert(::std::is_trivially_move_constructible_v<
              ::structs::abi_classification::StructInteger>);
static_assert(::std::is_trivially_move_assignable_v<
              ::structs::abi_classification::StructInteger>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::structs::abi_classification::StructInteger* __ret_ptr);
}
inline ::structs::abi_classification::StructInteger StructInteger::create(
    ::std::int32_t i) {
  crubit::Slot<::structs::abi_classification::StructInteger>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(i, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
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
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_inspect(
    ::structs::abi_classification::StructInteger*);
}
inline ::std::int32_t StructInteger::inspect(
    ::structs::abi_classification::StructInteger s) {
  return __crubit_internal::__crubit_thunk_inspect(&s);
}
inline void StructInteger::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructInteger, __field0));
}
static_assert(
    sizeof(StructMemory) == 5,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructMemory) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<StructMemory>);
static_assert(::std::is_trivially_move_constructible_v<
              ::structs::abi_classification::StructMemory>);
static_assert(::std::is_trivially_move_assignable_v<
              ::structs::abi_classification::StructMemory>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::structs::abi_classification::StructMemory* __ret_ptr);
}
inline ::structs::abi_classification::StructMemory StructMemory::create(
    ::std::int32_t i) {
  crubit::Slot<::structs::abi_classification::StructMemory>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(i, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
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
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_inspect(
    ::structs::abi_classification::StructMemory*);
}
inline ::std::int32_t StructMemory::inspect(
    ::structs::abi_classification::StructMemory s) {
  return __crubit_internal::__crubit_thunk_inspect(&s);
}
inline void StructMemory::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructMemory, _padding));
  static_assert(1 == offsetof(StructMemory, i));
}
}  // namespace structs::abi_classification

namespace structs::default_repr {

static_assert(
    sizeof(Point) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Point) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<Point>);
static_assert(
    ::std::is_trivially_move_constructible_v<::structs::default_repr::Point>);
static_assert(
    ::std::is_trivially_move_assignable_v<::structs::default_repr::Point>);
inline void Point::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Point, x));
  static_assert(4 == offsetof(Point, y));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::std::int32_t, ::structs::default_repr::Point* __ret_ptr);
}
inline ::structs::default_repr::Point create(::std::int32_t x,
                                             ::std::int32_t y) {
  crubit::Slot<::structs::default_repr::Point> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(x, y, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_get_ux(
    ::structs::default_repr::Point*);
}
inline ::std::int32_t get_x(::structs::default_repr::Point p) {
  return __crubit_internal::__crubit_thunk_get_ux(&p);
}

}  // namespace structs::default_repr

namespace structs::display {

static_assert(
    sizeof(DisplayStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(DisplayStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<DisplayStruct>);
static_assert(::std::is_trivially_move_constructible_v<
              ::structs::display::DisplayStruct>);
static_assert(
    ::std::is_trivially_move_assignable_v<::structs::display::DisplayStruct>);
extern "C" void
__crubit_thunk_ToString_uto_ustring_ustructs_ugolden_x0000003a_x0000003adisplay_x0000003a_x0000003aDisplayStruct(
    ::structs::display::DisplayStruct const&,
    ::rs::alloc::string::String* __ret_ptr);
inline void DisplayStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(DisplayStruct, value));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::structs::display::DisplayStruct* __ret_ptr);
}
inline ::structs::display::DisplayStruct create(::std::int32_t value) {
  crubit::Slot<::structs::display::DisplayStruct> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace structs::display

namespace structs::interior_mutability {

static_assert(
    sizeof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_ustructs_ugolden_x0000003a_x0000003ainterior_umutability_x0000003a_x0000003aSomeStruct(
    ::structs::interior_mutability::SomeStruct* __ret_ptr);
}
inline ::structs::interior_mutability::SomeStruct::SomeStruct() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_ustructs_ugolden_x0000003a_x0000003ainterior_umutability_x0000003a_x0000003aSomeStruct(
          this);
}
static_assert(::std::is_trivially_destructible_v<SomeStruct>);
static_assert(::std::is_trivially_move_constructible_v<
              ::structs::interior_mutability::SomeStruct>);
static_assert(::std::is_trivially_move_assignable_v<
              ::structs::interior_mutability::SomeStruct>);
inline void SomeStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(SomeStruct, field));
}
}  // namespace structs::interior_mutability

namespace structs::keyword_named_fields_and_methods {

static_assert(
    sizeof(AField) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(AField) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<AField>);
static_assert(::std::is_trivially_move_constructible_v<
              ::structs::keyword_named_fields_and_methods::AField>);
static_assert(::std::is_trivially_move_assignable_v<
              ::structs::keyword_named_fields_and_methods::AField>);
namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_operator(
    ::structs::keyword_named_fields_and_methods::AField const&);
}
inline ::std::int32_t AField::operator_() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_operator(self);
}
inline void AField::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(AField, operator__));
}
}  // namespace structs::keyword_named_fields_and_methods

namespace structs::nested_ptr_type_mutability_qualifiers {

static_assert(
    sizeof(SomeStruct) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(SomeStruct) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_ustructs_ugolden_x0000003a_x0000003anested_uptr_utype_umutability_uqualifiers_x0000003a_x0000003aSomeStruct(
    ::structs::nested_ptr_type_mutability_qualifiers::SomeStruct* __ret_ptr);
}
inline ::structs::nested_ptr_type_mutability_qualifiers::SomeStruct::
    SomeStruct() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_ustructs_ugolden_x0000003a_x0000003anested_uptr_utype_umutability_uqualifiers_x0000003a_x0000003aSomeStruct(
          this);
}
static_assert(::std::is_trivially_destructible_v<SomeStruct>);
static_assert(::std::is_trivially_move_constructible_v<
              ::structs::nested_ptr_type_mutability_qualifiers::SomeStruct>);
static_assert(::std::is_trivially_move_assignable_v<
              ::structs::nested_ptr_type_mutability_qualifiers::SomeStruct>);
inline void SomeStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(SomeStruct, mut_const_ptr));
  static_assert(8 == offsetof(SomeStruct, const_mut_ptr));
}
}  // namespace structs::nested_ptr_type_mutability_qualifiers

namespace structs::non_cpp_movable {

static_assert(
    sizeof(Point) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Point) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_ustructs_ugolden_x0000003a_x0000003anon_ucpp_umovable_x0000003a_x0000003aPoint(
    ::structs::non_cpp_movable::Point&);
}
inline Point::~Point() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_ustructs_ugolden_x0000003a_x0000003anon_ucpp_umovable_x0000003a_x0000003aPoint(
          *this);
}
inline void Point::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Point, x));
  static_assert(4 == offsetof(Point, y));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::std::int32_t,
    ::structs::non_cpp_movable::Point* __ret_ptr);
}
inline ::structs::non_cpp_movable::Point create(::std::int32_t x,
                                                ::std::int32_t y) {
  crubit::Slot<::structs::non_cpp_movable::Point> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(x, y, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_get_ux(
    ::structs::non_cpp_movable::Point const&);
}
inline ::std::int32_t get_x(::structs::non_cpp_movable::Point const& p) {
  return __crubit_internal::__crubit_thunk_get_ux(p);
}

}  // namespace structs::non_cpp_movable

namespace structs::repr_c {

static_assert(
    sizeof(Point) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Point) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<Point>);
static_assert(
    ::std::is_trivially_move_constructible_v<::structs::repr_c::Point>);
static_assert(::std::is_trivially_move_assignable_v<::structs::repr_c::Point>);
inline void Point::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Point, x));
  static_assert(4 == offsetof(Point, y));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(::std::int32_t, ::std::int32_t,
                                      ::structs::repr_c::Point* __ret_ptr);
}
inline ::structs::repr_c::Point create(::std::int32_t x, ::std::int32_t y) {
  crubit::Slot<::structs::repr_c::Point> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(x, y, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_get_ux(::structs::repr_c::Point*);
}
inline ::std::int32_t get_x(::structs::repr_c::Point p) {
  return __crubit_internal::__crubit_thunk_get_ux(&p);
}

}  // namespace structs::repr_c

namespace structs::struct_by_float_passing_with_no_cc_definition {

static_assert(
    sizeof(StructFloat) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructFloat) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<StructFloat>);
static_assert(
    ::std::is_trivially_move_constructible_v<
        ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat>);
static_assert(
    ::std::is_trivially_move_assignable_v<
        ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat>);
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
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" float __crubit_thunk_no_umangle_uinspect(
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat*);
}
inline float no_mangle_inspect(
    ::structs::struct_by_float_passing_with_no_cc_definition::StructFloat s) {
  return __crubit_internal::__crubit_thunk_no_umangle_uinspect(&s);
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
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace structs::struct_by_float_passing_with_no_cc_definition

namespace structs::struct_by_float_passing_with_no_thunk {

static_assert(
    sizeof(StructFloat) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructFloat) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<StructFloat>);
static_assert(::std::is_trivially_move_constructible_v<
              ::structs::struct_by_float_passing_with_no_thunk::StructFloat>);
static_assert(::std::is_trivially_move_assignable_v<
              ::structs::struct_by_float_passing_with_no_thunk::StructFloat>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustructs_ugolden_x0000003a_x0000003astruct_uby_ufloat_upassing_uwith_uno_uthunk_x0000003a_x0000003aStructFloat(
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat const&,
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustructs_ugolden_x0000003a_x0000003astruct_uby_ufloat_upassing_uwith_uno_uthunk_x0000003a_x0000003aStructFloat(
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat&,
    ::structs::struct_by_float_passing_with_no_thunk::StructFloat const&);
}
inline ::structs::struct_by_float_passing_with_no_thunk::StructFloat::
    StructFloat(const StructFloat& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustructs_ugolden_x0000003a_x0000003astruct_uby_ufloat_upassing_uwith_uno_uthunk_x0000003a_x0000003aStructFloat(
          other, this);
}
inline ::structs::struct_by_float_passing_with_no_thunk::StructFloat& ::
structs::struct_by_float_passing_with_no_thunk::StructFloat::operator=(
    const StructFloat& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustructs_ugolden_x0000003a_x0000003astruct_uby_ufloat_upassing_uwith_uno_uthunk_x0000003a_x0000003aStructFloat(
            *this, other);
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
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
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
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace structs::struct_by_float_passing_with_no_thunk

namespace structs::unsupported_types {

static_assert(
    sizeof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_ustructs_ugolden_x0000003a_x0000003aunsupported_utypes_x0000003a_x0000003aSomeStruct(
    ::structs::unsupported_types::SomeStruct* __ret_ptr);
}
inline ::structs::unsupported_types::SomeStruct::SomeStruct() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_ustructs_ugolden_x0000003a_x0000003aunsupported_utypes_x0000003a_x0000003aSomeStruct(
          this);
}
static_assert(::std::is_trivially_destructible_v<SomeStruct>);
static_assert(::std::is_trivially_move_constructible_v<
              ::structs::unsupported_types::SomeStruct>);
static_assert(::std::is_trivially_move_assignable_v<
              ::structs::unsupported_types::SomeStruct>);
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
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void SomeStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(SomeStruct, unsupported_field));
}
}  // namespace structs::unsupported_types

namespace structs::zst_fields {

static_assert(
    sizeof(ZstFields) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(ZstFields) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<ZstFields>);
static_assert(
    ::std::is_trivially_move_constructible_v<::structs::zst_fields::ZstFields>);
static_assert(
    ::std::is_trivially_move_assignable_v<::structs::zst_fields::ZstFields>);
inline void ZstFields::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ZstFields, value));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::structs::zst_fields::ZstFields* __ret_ptr);
}
inline ::structs::zst_fields::ZstFields create(::std::int32_t value) {
  crubit::Slot<::structs::zst_fields::ZstFields> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_get_uvalue(
    ::structs::zst_fields::ZstFields*);
}
inline ::std::int32_t get_value(::structs::zst_fields::ZstFields x) {
  return __crubit_internal::__crubit_thunk_get_uvalue(&x);
}

}  // namespace structs::zst_fields

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_STRUCTS_GOLDEN
