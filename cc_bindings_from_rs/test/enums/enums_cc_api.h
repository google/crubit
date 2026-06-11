// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// enums_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_ENUMS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_ENUMS_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"
#include "support/rs_std/traits.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_alloc.h"
#include "support/rs_std/rs_core.h"

namespace enums::qr_error {
struct StructuredQrError;
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: qr_error :: QrError") alignas(8) [[clang::trivial_abi]]
QrError final {
 public:
  // `enums_golden::qr_error::QrError` doesn't implement the `Default` trait
  QrError() = delete;

  static constexpr QrError MakeDataTooLong();

  static constexpr QrError MakeInvalidVersion();

  static constexpr QrError MakeUnsupportedCharacterSet();

  static constexpr QrError MakeInvalidEciDesignator();

  static constexpr QrError MakeInvalidCharacter();

  static ::enums::qr_error::QrError MakeStructured(
      ::enums::qr_error::StructuredQrError __param_0);

  // No custom `Drop` impl and no custom "drop glue" required
  ~QrError() = default;
  QrError(QrError&&) = default;
  QrError& operator=(QrError&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  QrError(const QrError&) = default;
  QrError& operator=(const QrError&) = default;
  QrError(::crubit::UnsafeRelocateTag, QrError&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  bool is_data_too_long() const;

  bool operator==(::enums::qr_error::QrError const& other) const;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  ::std::array<unsigned char, 24> __opaque_blob_of_bytes;

 private:
  struct PrivateBytesTag {};
  constexpr QrError(PrivateBytesTag, ::std::array<unsigned char, 24> bytes)
      : __opaque_blob_of_bytes(bytes) {}

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: qr_error :: StructuredQrError") alignas(8)
    [[clang::trivial_abi]] StructuredQrError final {
 public:
  // `enums_golden::qr_error::StructuredQrError` doesn't implement the `Default`
  // trait
  StructuredQrError() = delete;

  static constexpr StructuredQrError MakeAtLeast2Pieces();

  static ::enums::qr_error::StructuredQrError MakeTotalMismatch(
      ::std::uintptr_t __param_0);

  static constexpr StructuredQrError MakeMissingParts();

  static constexpr StructuredQrError MakeParity();

  static constexpr StructuredQrError MakeTooShort();

  static constexpr StructuredQrError MakeStructuredWrongMode();

  static constexpr StructuredQrError MakeStructuredWrongEnc();

  static ::enums::qr_error::StructuredQrError MakeSeqGreaterThanTotal(
      ::std::uint8_t __param_0, ::std::uint8_t __param_1);

  static ::enums::qr_error::StructuredQrError MakeLengthMismatch(
      ::std::uintptr_t __param_0, ::std::uintptr_t __param_1);

  static ::enums::qr_error::StructuredQrError MakeUnsupportedVersion(
      ::std::int16_t __param_0);

  static ::enums::qr_error::StructuredQrError MakeSplitMax16(
      ::std::uintptr_t __param_0);

  // No custom `Drop` impl and no custom "drop glue" required
  ~StructuredQrError() = default;
  StructuredQrError(StructuredQrError&&) = default;
  StructuredQrError& operator=(StructuredQrError&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  StructuredQrError(const StructuredQrError&) = default;
  StructuredQrError& operator=(const StructuredQrError&) = default;
  StructuredQrError(::crubit::UnsafeRelocateTag, StructuredQrError&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  bool operator==(::enums::qr_error::StructuredQrError const& other) const;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  ::std::array<unsigned char, 24> __opaque_blob_of_bytes;

 private:
  struct PrivateBytesTag {};
  constexpr StructuredQrError(PrivateBytesTag,
                              ::std::array<unsigned char, 24> bytes)
      : __opaque_blob_of_bytes(bytes) {}

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace enums::qr_error

namespace enums::repr_128 {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_128 :: ReprI128") alignas(16)
    [[clang::trivial_abi]] ReprI128 final {
 public:
  // `enums_golden::repr_128::ReprI128` doesn't implement the `Default` trait
  ReprI128() = delete;

  static constexpr ReprI128 MakeZero();

  static constexpr ReprI128 MakeMinI128();

  static constexpr ReprI128 MakeMaxI128();

  // No custom `Drop` impl and no custom "drop glue" required
  ~ReprI128() = default;
  ReprI128(ReprI128&&) = default;
  ReprI128& operator=(ReprI128&&) = default;

  // `enums_golden::repr_128::ReprI128` doesn't implement the `Clone` trait
  ReprI128(const ReprI128&) = delete;
  ReprI128& operator=(const ReprI128&) = delete;
  ReprI128(::crubit::UnsafeRelocateTag, ReprI128&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  bool is_min_i128() const;

  bool is_max_i128() const;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  ::std::array<unsigned char, 16> __opaque_blob_of_bytes;

 private:
  struct PrivateBytesTag {};
  constexpr ReprI128(PrivateBytesTag, ::std::array<unsigned char, 16> bytes)
      : __opaque_blob_of_bytes(bytes) {}

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_128 :: ReprU128") alignas(16)
    [[clang::trivial_abi]] ReprU128 final {
 public:
  // `enums_golden::repr_128::ReprU128` doesn't implement the `Default` trait
  ReprU128() = delete;

  static constexpr ReprU128 MakeZero();

  static constexpr ReprU128 MakeMaxU128();

  // No custom `Drop` impl and no custom "drop glue" required
  ~ReprU128() = default;
  ReprU128(ReprU128&&) = default;
  ReprU128& operator=(ReprU128&&) = default;

  // `enums_golden::repr_128::ReprU128` doesn't implement the `Clone` trait
  ReprU128(const ReprU128&) = delete;
  ReprU128& operator=(const ReprU128&) = delete;
  ReprU128(::crubit::UnsafeRelocateTag, ReprU128&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  bool is_max_u128() const;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  ::std::array<unsigned char, 16> __opaque_blob_of_bytes;

 private:
  struct PrivateBytesTag {};
  constexpr ReprU128(PrivateBytesTag, ::std::array<unsigned char, 16> bytes)
      : __opaque_blob_of_bytes(bytes) {}

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace enums::repr_128

namespace enums::repr_c {

struct CRUBIT_INTERNAL_RUST_TYPE(":: enums_golden :: repr_c :: MyEnum") alignas(
    8) [[clang::trivial_abi]] MyEnum final {
 public:
  // Default::default
  MyEnum();

  static ::enums::repr_c::MyEnum MakeE(::rs::alloc::string::String __param_0,
                                       ::std::int32_t __param_1);

  static ::enums::repr_c::MyEnum MakeA(::std::int32_t __param_0,
                                       ::std::int64_t __param_1);

  static MyEnum MakeF();

  // Error generating bindings for variant `enums_golden::repr_c::MyEnum::Z`
  // defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=15:
  // Tuple type `()` is not supported in this context

  static MyEnum MakeG();

  // Error generating bindings for variant `enums_golden::repr_c::MyEnum::B`
  // defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=17:
  // Constructing non-tuple, struct-like enum variants is not supported:
  // b/487357254

  // Error generating bindings for variant `enums_golden::repr_c::MyEnum::C`
  // defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=18:
  // Constructing non-tuple, struct-like enum variants is not supported:
  // b/487357254

  static MyEnum MakeD();

  // Drop::drop
  ~MyEnum();

  MyEnum(MyEnum&&);
  ::enums::repr_c::MyEnum& operator=(MyEnum&&);

  // `enums_golden::repr_c::MyEnum` doesn't implement the `Clone` trait
  MyEnum(const MyEnum&) = delete;
  MyEnum& operator=(const MyEnum&) = delete;
  MyEnum(::crubit::UnsafeRelocateTag, MyEnum&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  struct alignas(0) __crubit_E_struct {
   public:
    ::rs::alloc::string::String __field0;
    ::std::int32_t __field1;
  };
  struct alignas(0) __crubit_A_struct {
   public:
    ::std::int32_t __field0;
    ::std::int64_t __field1;
  };
  // Variant F has no size, so no struct is generated.

  // Variant Z has no size, so no struct is generated.

  // Variant G has no size, so no struct is generated.

  struct alignas(0) __crubit_B_struct {
   public:
    bool h;
    bool i;
  };
  struct alignas(0) __crubit_C_struct {
   public:
    ::std::int32_t a;
    ::std::int32_t b;
    ::std::int32_t c;
  };
  // Variant D has no size, so no struct is generated.

  enum class Tag : ::std::int64_t {
    E = INT64_C(0),
    A = INT64_C(1),
    F = INT64_C(2),
    Z = INT64_C(3),
    G = INT64_C(4),
    B = INT64_C(10000),
    C = INT64_C(10001),
    D = INT64_C(10002),
  };

 public:
  Tag tag;

 public:
  union {
    __crubit_E_struct E;
    __crubit_A_struct A;
    __crubit_B_struct B;
    __crubit_C_struct C;
  };

 private:
  struct PrivateTagCtorTag {};
  constexpr MyEnum(PrivateTagCtorTag, Tag tag) : tag(tag) {}

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c :: ReprCWithExtremeDiscriminants") alignas(4)
    [[clang::trivial_abi]] ReprCWithExtremeDiscriminants final {
 public:
  // `enums_golden::repr_c::ReprCWithExtremeDiscriminants` doesn't implement the
  // `Default` trait
  ReprCWithExtremeDiscriminants() = delete;

  //  `MinusOne` is a regression test against bindings that used to result
  //  in a C++ compilation error:
  //
  //  ```
  //  .../test/enums/enums.h:480:16: error: integer literal is too large
  //  to be represented in a signed integer type, interpreting as unsigned
  //  [-Werror,-Wimplicitly-unsigned-literal]
  //   480 |     MinusOne = 18446744073709551615,
  //       |                ^
  //  .../test/enums/enums.h:480:16: error: enumerator value evaluates to
  //  18446744073709551615, which cannot be narrowed to type
  //  '::std::int32_t' (aka 'int') [-Wc++11-narrowing]
  //  ```
  static constexpr ReprCWithExtremeDiscriminants MakeMinusOne();

  static constexpr ReprCWithExtremeDiscriminants MakeMinusTwo();

  //  Based on https://github.com/rust-lang/rust/issues/124403:
  //  * Historically, Rust allowed `#[repr(C)]` enums to have
  //    discriminants of arbitrary size.  However, in C, the default enum
  //    size is typically `int` (32-bit signed). This mismatch creates
  //    non-portable layout differences between Rust and C/C++.
  //  * Rust has introduced the `repr_c_enums_larger_than_int` lint (which
  //    is part of the `future_incompatible` lint group). It warns when a
  //    `#[repr(C)]` enum's discriminant does not fit into a C `int` or
  //    `unsigned int` (essentially limiting portably supported values to
  //    the signed 32-bit range: `[i32::MIN, i32::MAX]`). This warning is
  //    planned to become a hard compiler error in a future Rust release.
  static constexpr ReprCWithExtremeDiscriminants MakeMinI32();

  static constexpr ReprCWithExtremeDiscriminants MakeMaxI32();

  // No custom `Drop` impl and no custom "drop glue" required
  ~ReprCWithExtremeDiscriminants() = default;
  ReprCWithExtremeDiscriminants(ReprCWithExtremeDiscriminants&&) = default;
  ReprCWithExtremeDiscriminants& operator=(ReprCWithExtremeDiscriminants&&) =
      default;

  // `enums_golden::repr_c::ReprCWithExtremeDiscriminants` doesn't implement the
  // `Clone` trait
  ReprCWithExtremeDiscriminants(const ReprCWithExtremeDiscriminants&) = delete;
  ReprCWithExtremeDiscriminants& operator=(
      const ReprCWithExtremeDiscriminants&) = delete;
  ReprCWithExtremeDiscriminants(::crubit::UnsafeRelocateTag,
                                ReprCWithExtremeDiscriminants&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  bool is_minus_one() const;

  bool is_minus_two() const;

  bool is_min_i32() const;

  bool is_max_i32() const;

  // Variant MinusOne has no size, so no struct is generated.

  // Variant MinusTwo has no size, so no struct is generated.

  // Variant MinI32 has no size, so no struct is generated.

  // Variant MaxI32 has no size, so no struct is generated.

  enum class Tag : ::std::int32_t {
    MinusOne = INT32_C(-1),
    MinusTwo = INT32_C(-2),
    MinI32 = INT32_C(-2147483648),
    MaxI32 = INT32_C(2147483647),
  };

 public:
  Tag tag;

 private:
  struct PrivateTagCtorTag {};
  constexpr ReprCWithExtremeDiscriminants(PrivateTagCtorTag, Tag tag)
      : tag(tag) {}

 private:
  static void __crubit_field_offset_assertions();
};

//  This enum is **not** a "ZST" (Zero-Sized Type), because of the C
//  representation (even though it has only a single variant with no payload).
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c :: ReprCWithSingleNoPayloadVariant") alignas(4)
    [[clang::trivial_abi]] ReprCWithSingleNoPayloadVariant final {
 public:
  // `enums_golden::repr_c::ReprCWithSingleNoPayloadVariant` doesn't implement
  // the `Default` trait
  ReprCWithSingleNoPayloadVariant() = delete;

  static constexpr ReprCWithSingleNoPayloadVariant MakeSingleVariant();

  // No custom `Drop` impl and no custom "drop glue" required
  ~ReprCWithSingleNoPayloadVariant() = default;
  ReprCWithSingleNoPayloadVariant(ReprCWithSingleNoPayloadVariant&&) = default;
  ReprCWithSingleNoPayloadVariant& operator=(
      ReprCWithSingleNoPayloadVariant&&) = default;

  // `enums_golden::repr_c::ReprCWithSingleNoPayloadVariant` doesn't implement
  // the `Clone` trait
  ReprCWithSingleNoPayloadVariant(const ReprCWithSingleNoPayloadVariant&) =
      delete;
  ReprCWithSingleNoPayloadVariant& operator=(
      const ReprCWithSingleNoPayloadVariant&) = delete;
  ReprCWithSingleNoPayloadVariant(::crubit::UnsafeRelocateTag,
                                  ReprCWithSingleNoPayloadVariant&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  bool is_single_variant() const;

  // Variant SingleVariant has no size, so no struct is generated.

  enum class Tag : ::std::uint32_t {
    SingleVariant = 0,
  };

 public:
  Tag tag;

 private:
  struct PrivateTagCtorTag {};
  constexpr ReprCWithSingleNoPayloadVariant(PrivateTagCtorTag, Tag tag)
      : tag(tag) {}

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace enums::repr_c

namespace enums::repr_c_clone_active_variant {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c_clone_active_variant :: "
    "CloneActiveVariant") alignas(4) [[clang::trivial_abi]]
CloneActiveVariant final {
 public:
  // Default::default
  CloneActiveVariant();

  static ::enums::repr_c_clone_active_variant::CloneActiveVariant MakeA(
      ::std::int32_t __param_0);

  static ::enums::repr_c_clone_active_variant::CloneActiveVariant MakeB(
      ::std::int32_t __param_0);

  static ::enums::repr_c_clone_active_variant::CloneActiveVariant MakeC(
      ::std::int32_t __param_0);

  // No custom `Drop` impl and no custom "drop glue" required
  ~CloneActiveVariant() = default;
  CloneActiveVariant(CloneActiveVariant&&) = default;
  CloneActiveVariant& operator=(CloneActiveVariant&&) = default;

  // Clone::clone
  CloneActiveVariant(const CloneActiveVariant&);

  // Clone::clone_from
  ::enums::repr_c_clone_active_variant::CloneActiveVariant& operator=(
      const CloneActiveVariant&);

  CloneActiveVariant(::crubit::UnsafeRelocateTag, CloneActiveVariant&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  struct alignas(0) __crubit_A_struct {
   public:
    ::std::int32_t __field0;
  };
  struct alignas(0) __crubit_B_struct {
   public:
    ::std::int32_t __field0;
  };
  struct alignas(0) __crubit_C_struct {
   public:
    ::std::int32_t __field0;
  };

  enum class Tag : ::std::int8_t {
    A = 0,
    B = 1,
    C = 2,
  };

 public:
  Tag tag;

 public:
  union {
    __crubit_A_struct A;
    __crubit_B_struct B;
    __crubit_C_struct C;
  };

 private:
  struct PrivateTagCtorTag {};
  constexpr CloneActiveVariant(PrivateTagCtorTag, Tag tag) : tag(tag) {}

 private:
  static void __crubit_field_offset_assertions();
};

bool is_a(::enums::repr_c_clone_active_variant::CloneActiveVariant const& e);

bool is_b(::enums::repr_c_clone_active_variant::CloneActiveVariant const& e);

bool is_c(::enums::repr_c_clone_active_variant::CloneActiveVariant const& e);

}  // namespace enums::repr_c_clone_active_variant

namespace enums::repr_c_clone_counter {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c_clone_counter :: CloneCount") alignas(8)
    [[clang::trivial_abi]] CloneCount final {
 public:
  // Default::default
  CloneCount();

  // Error generating bindings for variant
  // `enums_golden::repr_c_clone_counter::CloneCount::A` defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=116:
  // Constructing non-tuple, struct-like enum variants is not supported:
  // b/487357254

  // No custom `Drop` impl and no custom "drop glue" required
  ~CloneCount() = default;
  CloneCount(CloneCount&&) = default;
  CloneCount& operator=(CloneCount&&) = default;

  // Clone::clone
  CloneCount(const CloneCount&);

  // Clone::clone_from
  ::enums::repr_c_clone_counter::CloneCount& operator=(const CloneCount&);

  CloneCount(::crubit::UnsafeRelocateTag, CloneCount&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  struct alignas(0) __crubit_A_struct {
   public:
    ::std::int32_t* crubit_nullability_unknown p;
  };

  enum class Tag : ::std::int8_t {
    A = 0,
  };

 public:
  Tag tag;

 public:
  union {
    __crubit_A_struct A;
  };

 private:
  struct PrivateTagCtorTag {};
  constexpr CloneCount(PrivateTagCtorTag, Tag tag) : tag(tag) {}

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace enums::repr_c_clone_counter

namespace enums::repr_c_drop {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c_drop :: DropMe") alignas(8)
    [[clang::trivial_abi]] DropMe final {
 public:
  // Default::default
  DropMe();

  static ::enums::repr_c_drop::DropMe MakeA(::std::int32_t __param_0);

  static ::enums::repr_c_drop::DropMe MakeB(::std::int64_t __param_0);

  static DropMe MakeQ();

  // Error generating bindings for variant
  // `enums_golden::repr_c_drop::DropMe::C` defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=95:
  // Constructing non-tuple, struct-like enum variants is not supported:
  // b/487357254

  // Drop::drop
  ~DropMe();

  DropMe(DropMe&&);
  ::enums::repr_c_drop::DropMe& operator=(DropMe&&);

  // `enums_golden::repr_c_drop::DropMe` doesn't implement the `Clone` trait
  DropMe(const DropMe&) = delete;
  DropMe& operator=(const DropMe&) = delete;
  DropMe(::crubit::UnsafeRelocateTag, DropMe&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  struct alignas(0) __crubit_A_struct {
   public:
    ::std::int32_t __field0;
  };
  struct alignas(0) __crubit_B_struct {
   public:
    ::std::int64_t __field0;
  };
  // Variant Q has no size, so no struct is generated.

  struct alignas(0) __crubit_C_struct {
   public:
    ::std::int32_t* crubit_nullability_unknown p;
  };

  enum class Tag : ::std::uint32_t {
    A = 0,
    B = 1,
    Q = 2,
    C = 3,
  };

 public:
  Tag tag;

 public:
  union {
    __crubit_A_struct A;
    __crubit_B_struct B;
    __crubit_C_struct C;
  };

 private:
  struct PrivateTagCtorTag {};
  constexpr DropMe(PrivateTagCtorTag, Tag tag) : tag(tag) {}

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace enums::repr_c_drop

namespace enums::repr_int {

//  Two `NoPayloadX` variants to test that the tag is correctly set
//  (`NoPayload1` should have a tag of 0 and therefore `NoPayload2` is a
//  slightly better test for things like encoding the tag value with the
//  proper endianness, especially given that the tag is 4 bytes wide).
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_int :: IntReprEnumWithNoPayload") alignas(4)
    [[clang::trivial_abi]] IntReprEnumWithNoPayload final {
 public:
  // `enums_golden::repr_int::IntReprEnumWithNoPayload` doesn't implement the
  // `Default` trait
  IntReprEnumWithNoPayload() = delete;

  static constexpr IntReprEnumWithNoPayload MakeNoPayload1();

  static constexpr IntReprEnumWithNoPayload MakeNoPayload2();

  // No custom `Drop` impl and no custom "drop glue" required
  ~IntReprEnumWithNoPayload() = default;
  IntReprEnumWithNoPayload(IntReprEnumWithNoPayload&&) = default;
  IntReprEnumWithNoPayload& operator=(IntReprEnumWithNoPayload&&) = default;

  // `enums_golden::repr_int::IntReprEnumWithNoPayload` doesn't implement the
  // `Clone` trait
  IntReprEnumWithNoPayload(const IntReprEnumWithNoPayload&) = delete;
  IntReprEnumWithNoPayload& operator=(const IntReprEnumWithNoPayload&) = delete;
  IntReprEnumWithNoPayload(::crubit::UnsafeRelocateTag,
                           IntReprEnumWithNoPayload&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  bool is_no_payload1() const;

  bool is_no_payload2() const;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  ::std::array<unsigned char, 4> __opaque_blob_of_bytes;

 private:
  struct PrivateBytesTag {};
  constexpr IntReprEnumWithNoPayload(PrivateBytesTag,
                                     ::std::array<unsigned char, 4> bytes)
      : __opaque_blob_of_bytes(bytes) {}

 private:
  static void __crubit_field_offset_assertions();
};

//  This enum is **not** a "ZST" (Zero-Sized Type), because of `#[repr(u32)]`
//  (even though it has only a single variant with no payload).
struct
    CRUBIT_INTERNAL_RUST_TYPE(
        ":: enums_golden :: repr_int :: "
        "IntReprWithSingleNoPayloadVariant") alignas(4) [[clang::trivial_abi]]
    IntReprWithSingleNoPayloadVariant final {
 public:
  // `enums_golden::repr_int::IntReprWithSingleNoPayloadVariant` doesn't
  // implement the `Default` trait
  IntReprWithSingleNoPayloadVariant() = delete;

  static constexpr IntReprWithSingleNoPayloadVariant MakeSingleVariant();

  // No custom `Drop` impl and no custom "drop glue" required
  ~IntReprWithSingleNoPayloadVariant() = default;
  IntReprWithSingleNoPayloadVariant(IntReprWithSingleNoPayloadVariant&&) =
      default;
  IntReprWithSingleNoPayloadVariant& operator=(
      IntReprWithSingleNoPayloadVariant&&) = default;

  // `enums_golden::repr_int::IntReprWithSingleNoPayloadVariant` doesn't
  // implement the `Clone` trait
  IntReprWithSingleNoPayloadVariant(const IntReprWithSingleNoPayloadVariant&) =
      delete;
  IntReprWithSingleNoPayloadVariant& operator=(
      const IntReprWithSingleNoPayloadVariant&) = delete;
  IntReprWithSingleNoPayloadVariant(::crubit::UnsafeRelocateTag,
                                    IntReprWithSingleNoPayloadVariant&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  bool is_single_variant() const;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  ::std::array<unsigned char, 4> __opaque_blob_of_bytes;

 private:
  struct PrivateBytesTag {};
  constexpr IntReprWithSingleNoPayloadVariant(
      PrivateBytesTag, ::std::array<unsigned char, 4> bytes)
      : __opaque_blob_of_bytes(bytes) {}

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_int :: NegReprIntEnum") alignas(1)
    [[clang::trivial_abi]] NegReprIntEnum final {
 public:
  // `enums_golden::repr_int::NegReprIntEnum` doesn't implement the `Default`
  // trait
  NegReprIntEnum() = delete;

  static constexpr NegReprIntEnum MakeMinusOne();

  static constexpr NegReprIntEnum MakeMinusTwo();

  // No custom `Drop` impl and no custom "drop glue" required
  ~NegReprIntEnum() = default;
  NegReprIntEnum(NegReprIntEnum&&) = default;
  NegReprIntEnum& operator=(NegReprIntEnum&&) = default;

  // `enums_golden::repr_int::NegReprIntEnum` doesn't implement the `Clone`
  // trait
  NegReprIntEnum(const NegReprIntEnum&) = delete;
  NegReprIntEnum& operator=(const NegReprIntEnum&) = delete;
  NegReprIntEnum(::crubit::UnsafeRelocateTag, NegReprIntEnum&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  bool is_minus_one() const;

  bool is_minus_two() const;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  ::std::array<unsigned char, 1> __opaque_blob_of_bytes;

 private:
  struct PrivateBytesTag {};
  constexpr NegReprIntEnum(PrivateBytesTag,
                           ::std::array<unsigned char, 1> bytes)
      : __opaque_blob_of_bytes(bytes) {}

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace enums::repr_int

namespace enums::repr_rust {

//  Doc comment of RustReprEnum.
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_rust :: RustReprEnum") alignas(4)
    [[clang::trivial_abi]] RustReprEnum final {
 public:
  // `enums_golden::repr_rust::RustReprEnum` doesn't implement the `Default`
  // trait
  RustReprEnum() = delete;

  //  Doc comment of Variant1.
  static constexpr RustReprEnum MakeVariant1();

  static constexpr RustReprEnum MakeVariant2();

  static constexpr RustReprEnum MakeVariant3();

  static ::enums::repr_rust::RustReprEnum MakeTuplePayloadVariant(
      ::std::int32_t __param_0, ::std::int32_t __param_1);

  // Error generating bindings for variant
  // `enums_golden::repr_rust::RustReprEnum::StructPayloadVariant` defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=182:
  // Constructing non-tuple, struct-like enum variants is not supported:
  // b/487357254

  // No custom `Drop` impl and no custom "drop glue" required
  ~RustReprEnum() = default;
  RustReprEnum(RustReprEnum&&) = default;
  RustReprEnum& operator=(RustReprEnum&&) = default;

  // `enums_golden::repr_rust::RustReprEnum` doesn't implement the `Clone` trait
  RustReprEnum(const RustReprEnum&) = delete;
  RustReprEnum& operator=(const RustReprEnum&) = delete;
  RustReprEnum(::crubit::UnsafeRelocateTag, RustReprEnum&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  ::std::int32_t get_variant_number() const;

  bool is_tuple_payload_variant() const;

  ::std::int32_t get_first_item_from_tuple_payload() const;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  ::std::array<unsigned char, 12> __opaque_blob_of_bytes;

 private:
  struct PrivateBytesTag {};
  constexpr RustReprEnum(PrivateBytesTag, ::std::array<unsigned char, 12> bytes)
      : __opaque_blob_of_bytes(bytes) {}

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_rust :: "
    "RustReprWithNamingConflictBetweenCtorsAndMethods") alignas(4)
    [[clang::trivial_abi]]
    RustReprWithNamingConflictBetweenCtorsAndMethods final {
 public:
  // `enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods`
  // doesn't implement the `Default` trait
  RustReprWithNamingConflictBetweenCtorsAndMethods() = delete;

  // Error generating bindings for variant
  // `enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods::NoPayloadVariant`
  // defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=232:
  // Conflicting member function name: MakeNoPayloadVariant

  // Error generating bindings for variant
  // `enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods::TuplePayloadVariant`
  // defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=233:
  // Conflicting member function name: MakeTuplePayloadVariant

  // Error generating bindings for variant
  // `enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods::StructPayloadVariant`
  // defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=234:
  // Constructing non-tuple, struct-like enum variants is not supported:
  // b/487357254

  // No custom `Drop` impl and no custom "drop glue" required
  ~RustReprWithNamingConflictBetweenCtorsAndMethods() = default;
  RustReprWithNamingConflictBetweenCtorsAndMethods(
      RustReprWithNamingConflictBetweenCtorsAndMethods&&) = default;
  RustReprWithNamingConflictBetweenCtorsAndMethods& operator=(
      RustReprWithNamingConflictBetweenCtorsAndMethods&&) = default;

  // `enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods`
  // doesn't implement the `Clone` trait
  RustReprWithNamingConflictBetweenCtorsAndMethods(
      const RustReprWithNamingConflictBetweenCtorsAndMethods&) = delete;
  RustReprWithNamingConflictBetweenCtorsAndMethods& operator=(
      const RustReprWithNamingConflictBetweenCtorsAndMethods&) = delete;
  RustReprWithNamingConflictBetweenCtorsAndMethods(
      ::crubit::UnsafeRelocateTag,
      RustReprWithNamingConflictBetweenCtorsAndMethods&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  //  Presence of this function tests the scenario where `MakeNoPayloadVariant`
  //  is a name of:
  //  1. A static method (here/below).
  //  2. An auto-generated factory/constructor static method
  static ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods
  MakeNoPayloadVariant();

  static ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods
  MakeTuplePayloadVariant(::std::int32_t i);

  static ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods
  MakeStructPayloadVariant(::std::int32_t x);

  ::std::int32_t get_variant_number() const;

  ::std::int32_t get_value() const;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  ::std::array<unsigned char, 8> __opaque_blob_of_bytes;

 private:
  struct PrivateBytesTag {};
  constexpr RustReprWithNamingConflictBetweenCtorsAndMethods(
      PrivateBytesTag, ::std::array<unsigned char, 8> bytes)
      : __opaque_blob_of_bytes(bytes) {}

 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for enum
// `enums_golden::repr_rust::RustReprWithSingleNoPayloadVariant` defined at
// cc_bindings_from_rs/test/enums/enums.rs;l=213:
// Zero-sized types (ZSTs) are not supported (b/258259459)

//  This enum is not a "ZST" (Zero-Sized Type), because of the payload.
//  There is no tag / discriminant field, because there is only one variant.
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_rust :: "
    "RustReprWithSingleTuplePayloadVariant") alignas(4) [[clang::trivial_abi]]
RustReprWithSingleTuplePayloadVariant final {
 public:
  // `enums_golden::repr_rust::RustReprWithSingleTuplePayloadVariant` doesn't
  // implement the `Default` trait
  RustReprWithSingleTuplePayloadVariant() = delete;

  static ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant
  MakeSingleVariant(::std::int32_t __param_0);

  // No custom `Drop` impl and no custom "drop glue" required
  ~RustReprWithSingleTuplePayloadVariant() = default;
  RustReprWithSingleTuplePayloadVariant(
      RustReprWithSingleTuplePayloadVariant&&) = default;
  RustReprWithSingleTuplePayloadVariant& operator=(
      RustReprWithSingleTuplePayloadVariant&&) = default;

  // `enums_golden::repr_rust::RustReprWithSingleTuplePayloadVariant` doesn't
  // implement the `Clone` trait
  RustReprWithSingleTuplePayloadVariant(
      const RustReprWithSingleTuplePayloadVariant&) = delete;
  RustReprWithSingleTuplePayloadVariant& operator=(
      const RustReprWithSingleTuplePayloadVariant&) = delete;
  RustReprWithSingleTuplePayloadVariant(
      ::crubit::UnsafeRelocateTag,
      RustReprWithSingleTuplePayloadVariant&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  ::std::int32_t get_single_item_from_tuple_payload() const;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  ::std::array<unsigned char, 4> __opaque_blob_of_bytes;

 private:
  struct PrivateBytesTag {};
  constexpr RustReprWithSingleTuplePayloadVariant(
      PrivateBytesTag, ::std::array<unsigned char, 4> bytes)
      : __opaque_blob_of_bytes(bytes) {}

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace enums::repr_rust

template <>
struct rs_std::impl<::enums::qr_error::QrError, ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct rs_std::impl<::enums::qr_error::QrError, ::rs::core::fmt::Debug> {
  static constexpr bool kIsImplemented = true;

  // Error generating bindings for associated function
  // `<enums_golden::qr_error::QrError as std::fmt::Debug>::fmt` defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=349:
  // Error formatting function return type `std::result::Result<(),
  // std::fmt::Error>`: Generic types are not supported yet (b/259749095)
};

template <>
struct rs_std::impl<::enums::qr_error::StructuredQrError, ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct rs_std::impl<::enums::qr_error::StructuredQrError,
                    ::rs::core::fmt::Debug> {
  static constexpr bool kIsImplemented = true;

  // Error generating bindings for associated function
  // `<enums_golden::qr_error::StructuredQrError as std::fmt::Debug>::fmt`
  // defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=365:
  // Error formatting function return type `std::result::Result<(),
  // std::fmt::Error>`: Generic types are not supported yet (b/259749095)
};

namespace enums::qr_error {

static_assert(
    sizeof(QrError) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(QrError) == 8,
    "Verify that ADT layout didn't change since this header got generated");

// `static` constructor
inline constexpr QrError QrError::MakeDataTooLong() {
  return QrError(PrivateBytesTag{}, {11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0,  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
}

// `static` constructor
inline constexpr QrError QrError::MakeInvalidVersion() {
  return QrError(PrivateBytesTag{}, {12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0,  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
}

// `static` constructor
inline constexpr QrError QrError::MakeUnsupportedCharacterSet() {
  return QrError(PrivateBytesTag{}, {13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0,  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
}

// `static` constructor
inline constexpr QrError QrError::MakeInvalidEciDesignator() {
  return QrError(PrivateBytesTag{}, {14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0,  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
}

// `static` constructor
inline constexpr QrError QrError::MakeInvalidCharacter() {
  return QrError(PrivateBytesTag{}, {15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0,  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_Structured(
    ::enums::qr_error::StructuredQrError*,
    ::enums::qr_error::QrError* crubit_nonnull __ret_ptr);
}
inline ::enums::qr_error::QrError QrError::MakeStructured(
    ::enums::qr_error::StructuredQrError __param_0) {
  crubit::Slot<::enums::qr_error::QrError> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_Structured(&__param_0,
                                               __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
static_assert(::std::is_trivially_destructible_v<QrError>);
static_assert(
    ::std::is_trivially_move_constructible_v<::enums::qr_error::QrError>);
static_assert(
    ::std::is_trivially_move_assignable_v<::enums::qr_error::QrError>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::enums::qr_error::QrError>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::enums::qr_error::QrError>);
namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_udata_utoo_ulong(
    ::enums::qr_error::QrError const&);
}
inline bool QrError::is_data_too_long() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_udata_utoo_ulong(self);
}

namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_uenums_ugolden_x0000003a_x0000003aqr_uerror_x0000003a_x0000003aQrError_uenums_ugolden_x0000003a_x0000003aqr_uerror_x0000003a_x0000003aQrError(
    ::enums::qr_error::QrError const&, ::enums::qr_error::QrError const&);
}
inline bool QrError::operator==(::enums::qr_error::QrError const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_uenums_ugolden_x0000003a_x0000003aqr_uerror_x0000003a_x0000003aQrError_uenums_ugolden_x0000003a_x0000003aqr_uerror_x0000003a_x0000003aQrError(
          self, other);
}
inline void QrError::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(QrError, __opaque_blob_of_bytes));
}
static_assert(
    sizeof(StructuredQrError) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructuredQrError) == 8,
    "Verify that ADT layout didn't change since this header got generated");

// `static` constructor
inline constexpr StructuredQrError StructuredQrError::MakeAtLeast2Pieces() {
  return StructuredQrError(
      PrivateBytesTag{},
      {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_TotalMismatch(
    ::std::uintptr_t,
    ::enums::qr_error::StructuredQrError* crubit_nonnull __ret_ptr);
}
inline ::enums::qr_error::StructuredQrError
StructuredQrError::MakeTotalMismatch(::std::uintptr_t __param_0) {
  crubit::Slot<::enums::qr_error::StructuredQrError>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_TotalMismatch(__param_0,
                                                  __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

// `static` constructor
inline constexpr StructuredQrError StructuredQrError::MakeMissingParts() {
  return StructuredQrError(
      PrivateBytesTag{},
      {2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
}

// `static` constructor
inline constexpr StructuredQrError StructuredQrError::MakeParity() {
  return StructuredQrError(
      PrivateBytesTag{},
      {3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
}

// `static` constructor
inline constexpr StructuredQrError StructuredQrError::MakeTooShort() {
  return StructuredQrError(
      PrivateBytesTag{},
      {4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
}

// `static` constructor
inline constexpr StructuredQrError
StructuredQrError::MakeStructuredWrongMode() {
  return StructuredQrError(
      PrivateBytesTag{},
      {5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
}

// `static` constructor
inline constexpr StructuredQrError StructuredQrError::MakeStructuredWrongEnc() {
  return StructuredQrError(
      PrivateBytesTag{},
      {6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_SeqGreaterThanTotal(
    ::std::uint8_t, ::std::uint8_t,
    ::enums::qr_error::StructuredQrError* crubit_nonnull __ret_ptr);
}
inline ::enums::qr_error::StructuredQrError
StructuredQrError::MakeSeqGreaterThanTotal(::std::uint8_t __param_0,
                                           ::std::uint8_t __param_1) {
  crubit::Slot<::enums::qr_error::StructuredQrError>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_SeqGreaterThanTotal(__param_0, __param_1,
                                                        __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_LengthMismatch(
    ::std::uintptr_t, ::std::uintptr_t,
    ::enums::qr_error::StructuredQrError* crubit_nonnull __ret_ptr);
}
inline ::enums::qr_error::StructuredQrError
StructuredQrError::MakeLengthMismatch(::std::uintptr_t __param_0,
                                      ::std::uintptr_t __param_1) {
  crubit::Slot<::enums::qr_error::StructuredQrError>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_LengthMismatch(__param_0, __param_1,
                                                   __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_UnsupportedVersion(
    ::std::int16_t,
    ::enums::qr_error::StructuredQrError* crubit_nonnull __ret_ptr);
}
inline ::enums::qr_error::StructuredQrError
StructuredQrError::MakeUnsupportedVersion(::std::int16_t __param_0) {
  crubit::Slot<::enums::qr_error::StructuredQrError>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_UnsupportedVersion(__param_0,
                                                       __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_SplitMax16(
    ::std::uintptr_t,
    ::enums::qr_error::StructuredQrError* crubit_nonnull __ret_ptr);
}
inline ::enums::qr_error::StructuredQrError StructuredQrError::MakeSplitMax16(
    ::std::uintptr_t __param_0) {
  crubit::Slot<::enums::qr_error::StructuredQrError>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_SplitMax16(__param_0,
                                               __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
static_assert(::std::is_trivially_destructible_v<StructuredQrError>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::qr_error::StructuredQrError>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::qr_error::StructuredQrError>);
static_assert(::std::is_trivially_copy_constructible_v<
              ::enums::qr_error::StructuredQrError>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::enums::qr_error::StructuredQrError>);
namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_uenums_ugolden_x0000003a_x0000003aqr_uerror_x0000003a_x0000003aStructuredQrError_uenums_ugolden_x0000003a_x0000003aqr_uerror_x0000003a_x0000003aStructuredQrError(
    ::enums::qr_error::StructuredQrError const&,
    ::enums::qr_error::StructuredQrError const&);
}
inline bool StructuredQrError::operator==(
    ::enums::qr_error::StructuredQrError const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_uenums_ugolden_x0000003a_x0000003aqr_uerror_x0000003a_x0000003aStructuredQrError_uenums_ugolden_x0000003a_x0000003aqr_uerror_x0000003a_x0000003aStructuredQrError(
          self, other);
}
inline void StructuredQrError::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructuredQrError, __opaque_blob_of_bytes));
}
}  // namespace enums::qr_error

namespace enums::repr_128 {

static_assert(
    sizeof(ReprI128) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(ReprI128) == 16,
    "Verify that ADT layout didn't change since this header got generated");

// `static` constructor
inline constexpr ReprI128 ReprI128::MakeZero() {
  return ReprI128(PrivateBytesTag{},
                  {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
}

// `static` constructor
inline constexpr ReprI128 ReprI128::MakeMinI128() {
  return ReprI128(PrivateBytesTag{},
                  {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128});
}

// `static` constructor
inline constexpr ReprI128 ReprI128::MakeMaxI128() {
  return ReprI128(PrivateBytesTag{}, {255, 255, 255, 255, 255, 255, 255, 255,
                                      255, 255, 255, 255, 255, 255, 255, 127});
}
static_assert(::std::is_trivially_destructible_v<ReprI128>);
static_assert(
    ::std::is_trivially_move_constructible_v<::enums::repr_128::ReprI128>);
static_assert(
    ::std::is_trivially_move_assignable_v<::enums::repr_128::ReprI128>);
namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_umin_ui128(
    ::enums::repr_128::ReprI128 const&);
}
inline bool ReprI128::is_min_i128() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_umin_ui128(self);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_umax_ui128(
    ::enums::repr_128::ReprI128 const&);
}
inline bool ReprI128::is_max_i128() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_umax_ui128(self);
}
inline void ReprI128::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ReprI128, __opaque_blob_of_bytes));
}
static_assert(
    sizeof(ReprU128) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(ReprU128) == 16,
    "Verify that ADT layout didn't change since this header got generated");

// `static` constructor
inline constexpr ReprU128 ReprU128::MakeZero() {
  return ReprU128(PrivateBytesTag{},
                  {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
}

// `static` constructor
inline constexpr ReprU128 ReprU128::MakeMaxU128() {
  return ReprU128(PrivateBytesTag{}, {255, 255, 255, 255, 255, 255, 255, 255,
                                      255, 255, 255, 255, 255, 255, 255, 255});
}
static_assert(::std::is_trivially_destructible_v<ReprU128>);
static_assert(
    ::std::is_trivially_move_constructible_v<::enums::repr_128::ReprU128>);
static_assert(
    ::std::is_trivially_move_assignable_v<::enums::repr_128::ReprU128>);
namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_umax_uu128(
    ::enums::repr_128::ReprU128 const&);
}
inline bool ReprU128::is_max_u128() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_umax_uu128(self);
}
inline void ReprU128::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ReprU128, __opaque_blob_of_bytes));
}
}  // namespace enums::repr_128

namespace enums::repr_c {

static_assert(
    sizeof(MyEnum) == 40,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyEnum) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    ::enums::repr_c::MyEnum* crubit_nonnull __ret_ptr);
}
inline ::enums::repr_c::MyEnum::MyEnum() {
  __crubit_internal::__crubit_thunk_default(this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_E(::rs::alloc::string::String*, ::std::int32_t,
                                 ::enums::repr_c::MyEnum* crubit_nonnull
                                     __ret_ptr);
}
inline ::enums::repr_c::MyEnum MyEnum::MakeE(
    ::rs::alloc::string::String __param_0, ::std::int32_t __param_1) {
  crubit::Slot __param_0_slot((::std::move(__param_0)));
  crubit::Slot<::enums::repr_c::MyEnum> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_E(__param_0_slot.Get(), __param_1,
                                      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_A(::std::int32_t, ::std::int64_t,
                                 ::enums::repr_c::MyEnum* crubit_nonnull
                                     __ret_ptr);
}
inline ::enums::repr_c::MyEnum MyEnum::MakeA(::std::int32_t __param_0,
                                             ::std::int64_t __param_1) {
  crubit::Slot<::enums::repr_c::MyEnum> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_A(__param_0, __param_1,
                                      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

// `static` constructor
inline MyEnum MyEnum::MakeF() {
  return MyEnum(PrivateTagCtorTag{}, Tag{INT64_C(2)});
}

// `static` constructor
inline MyEnum MyEnum::MakeG() {
  return MyEnum(PrivateTagCtorTag{}, Tag{INT64_C(4)});
}

// `static` constructor
inline MyEnum MyEnum::MakeD() {
  return MyEnum(PrivateTagCtorTag{}, Tag{INT64_C(10002)});
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::enums::repr_c::MyEnum&);
}
inline MyEnum::~MyEnum() { __crubit_internal::__crubit_thunk_drop(*this); }
inline ::enums::repr_c::MyEnum::MyEnum(MyEnum&& other) : MyEnum() {
  *this = ::std::move(other);
}
inline ::enums::repr_c::MyEnum& ::enums::repr_c::MyEnum::operator=(
    MyEnum&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline void MyEnum::__crubit_field_offset_assertions() {
  static_assert(8 == offsetof(MyEnum, E));
  static_assert(8 == offsetof(MyEnum, A));
  static_assert(8 == offsetof(MyEnum, B));
  static_assert(8 == offsetof(MyEnum, C));
  static_assert(0 == offsetof(MyEnum::__crubit_E_struct, __field0));
  static_assert(24 == offsetof(MyEnum::__crubit_E_struct, __field1));
  static_assert(0 == offsetof(MyEnum::__crubit_A_struct, __field0));
  static_assert(8 == offsetof(MyEnum::__crubit_A_struct, __field1));
  static_assert(0 == offsetof(MyEnum::__crubit_B_struct, h));
  static_assert(1 == offsetof(MyEnum::__crubit_B_struct, i));
  static_assert(0 == offsetof(MyEnum::__crubit_C_struct, a));
  static_assert(4 == offsetof(MyEnum::__crubit_C_struct, b));
  static_assert(8 == offsetof(MyEnum::__crubit_C_struct, c));
}
static_assert(
    sizeof(ReprCWithExtremeDiscriminants) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(ReprCWithExtremeDiscriminants) == 4,
    "Verify that ADT layout didn't change since this header got generated");

// `static` constructor
inline constexpr ReprCWithExtremeDiscriminants
ReprCWithExtremeDiscriminants::MakeMinusOne() {
  return ReprCWithExtremeDiscriminants(PrivateTagCtorTag{}, Tag{INT64_C(-1)});
}

// `static` constructor
inline constexpr ReprCWithExtremeDiscriminants
ReprCWithExtremeDiscriminants::MakeMinusTwo() {
  return ReprCWithExtremeDiscriminants(PrivateTagCtorTag{}, Tag{INT64_C(-2)});
}

// `static` constructor
inline constexpr ReprCWithExtremeDiscriminants
ReprCWithExtremeDiscriminants::MakeMinI32() {
  return ReprCWithExtremeDiscriminants(PrivateTagCtorTag{},
                                       Tag{INT64_C(-2147483648)});
}

// `static` constructor
inline constexpr ReprCWithExtremeDiscriminants
ReprCWithExtremeDiscriminants::MakeMaxI32() {
  return ReprCWithExtremeDiscriminants(PrivateTagCtorTag{},
                                       Tag{INT64_C(2147483647)});
}
static_assert(
    ::std::is_trivially_destructible_v<ReprCWithExtremeDiscriminants>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_c::ReprCWithExtremeDiscriminants>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::repr_c::ReprCWithExtremeDiscriminants>);
namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_uminus_uone(
    ::enums::repr_c::ReprCWithExtremeDiscriminants const&);
}
inline bool ReprCWithExtremeDiscriminants::is_minus_one() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_uminus_uone(self);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_uminus_utwo(
    ::enums::repr_c::ReprCWithExtremeDiscriminants const&);
}
inline bool ReprCWithExtremeDiscriminants::is_minus_two() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_uminus_utwo(self);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_umin_ui32(
    ::enums::repr_c::ReprCWithExtremeDiscriminants const&);
}
inline bool ReprCWithExtremeDiscriminants::is_min_i32() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_umin_ui32(self);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_umax_ui32(
    ::enums::repr_c::ReprCWithExtremeDiscriminants const&);
}
inline bool ReprCWithExtremeDiscriminants::is_max_i32() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_umax_ui32(self);
}
inline void ReprCWithExtremeDiscriminants::__crubit_field_offset_assertions() {}
static_assert(
    sizeof(ReprCWithSingleNoPayloadVariant) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(ReprCWithSingleNoPayloadVariant) == 4,
    "Verify that ADT layout didn't change since this header got generated");

// `static` constructor
inline constexpr ReprCWithSingleNoPayloadVariant
ReprCWithSingleNoPayloadVariant::MakeSingleVariant() {
  return ReprCWithSingleNoPayloadVariant(PrivateTagCtorTag{}, Tag{INT64_C(0)});
}
static_assert(
    ::std::is_trivially_destructible_v<ReprCWithSingleNoPayloadVariant>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_c::ReprCWithSingleNoPayloadVariant>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::repr_c::ReprCWithSingleNoPayloadVariant>);
namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_usingle_uvariant(
    ::enums::repr_c::ReprCWithSingleNoPayloadVariant const&);
}
inline bool ReprCWithSingleNoPayloadVariant::is_single_variant() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_usingle_uvariant(self);
}
inline void
ReprCWithSingleNoPayloadVariant::__crubit_field_offset_assertions() {}
}  // namespace enums::repr_c

namespace enums::repr_c_clone_active_variant {

static_assert(
    sizeof(CloneActiveVariant) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneActiveVariant) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant* crubit_nonnull
        __ret_ptr);
}
inline ::enums::repr_c_clone_active_variant::CloneActiveVariant::
    CloneActiveVariant() {
  __crubit_internal::__crubit_thunk_default(this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_A(
    ::std::int32_t,
    ::enums::repr_c_clone_active_variant::CloneActiveVariant* crubit_nonnull
        __ret_ptr);
}
inline ::enums::repr_c_clone_active_variant::CloneActiveVariant
CloneActiveVariant::MakeA(::std::int32_t __param_0) {
  crubit::Slot<::enums::repr_c_clone_active_variant::CloneActiveVariant>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_A(__param_0, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_B(
    ::std::int32_t,
    ::enums::repr_c_clone_active_variant::CloneActiveVariant* crubit_nonnull
        __ret_ptr);
}
inline ::enums::repr_c_clone_active_variant::CloneActiveVariant
CloneActiveVariant::MakeB(::std::int32_t __param_0) {
  crubit::Slot<::enums::repr_c_clone_active_variant::CloneActiveVariant>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_B(__param_0, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_C(
    ::std::int32_t,
    ::enums::repr_c_clone_active_variant::CloneActiveVariant* crubit_nonnull
        __ret_ptr);
}
inline ::enums::repr_c_clone_active_variant::CloneActiveVariant
CloneActiveVariant::MakeC(::std::int32_t __param_0) {
  crubit::Slot<::enums::repr_c_clone_active_variant::CloneActiveVariant>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_C(__param_0, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
static_assert(::std::is_trivially_destructible_v<CloneActiveVariant>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_c_clone_active_variant::CloneActiveVariant>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::repr_c_clone_active_variant::CloneActiveVariant>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const&,
    ::enums::repr_c_clone_active_variant::CloneActiveVariant* crubit_nonnull
        __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant&,
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const&);
}
inline ::enums::repr_c_clone_active_variant::CloneActiveVariant::
    CloneActiveVariant(const CloneActiveVariant& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline ::enums::repr_c_clone_active_variant::CloneActiveVariant& ::enums::
    repr_c_clone_active_variant::CloneActiveVariant::operator=(
        const CloneActiveVariant& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
inline void CloneActiveVariant::__crubit_field_offset_assertions() {
  static_assert(4 == offsetof(CloneActiveVariant, A));
  static_assert(4 == offsetof(CloneActiveVariant, B));
  static_assert(4 == offsetof(CloneActiveVariant, C));
  static_assert(0 == offsetof(CloneActiveVariant::__crubit_A_struct, __field0));
  static_assert(0 == offsetof(CloneActiveVariant::__crubit_B_struct, __field0));
  static_assert(0 == offsetof(CloneActiveVariant::__crubit_C_struct, __field0));
}
namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_ua(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const&);
}
inline bool is_a(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const& e) {
  return __crubit_internal::__crubit_thunk_is_ua(e);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_ub(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const&);
}
inline bool is_b(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const& e) {
  return __crubit_internal::__crubit_thunk_is_ub(e);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_uc(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const&);
}
inline bool is_c(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const& e) {
  return __crubit_internal::__crubit_thunk_is_uc(e);
}

}  // namespace enums::repr_c_clone_active_variant

namespace enums::repr_c_clone_counter {

static_assert(
    sizeof(CloneCount) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneCount) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    ::enums::repr_c_clone_counter::CloneCount* crubit_nonnull __ret_ptr);
}
inline ::enums::repr_c_clone_counter::CloneCount::CloneCount() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(::std::is_trivially_destructible_v<CloneCount>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_c_clone_counter::CloneCount>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::repr_c_clone_counter::CloneCount>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    ::enums::repr_c_clone_counter::CloneCount const&,
    ::enums::repr_c_clone_counter::CloneCount* crubit_nonnull __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(
    ::enums::repr_c_clone_counter::CloneCount&,
    ::enums::repr_c_clone_counter::CloneCount const&);
}
inline ::enums::repr_c_clone_counter::CloneCount::CloneCount(
    const CloneCount& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline ::enums::repr_c_clone_counter::CloneCount& ::enums::
    repr_c_clone_counter::CloneCount::operator=(const CloneCount& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
inline void CloneCount::__crubit_field_offset_assertions() {
  static_assert(8 == offsetof(CloneCount, A));
  static_assert(0 == offsetof(CloneCount::__crubit_A_struct, p));
}
}  // namespace enums::repr_c_clone_counter

namespace enums::repr_c_drop {

static_assert(
    sizeof(DropMe) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(DropMe) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    ::enums::repr_c_drop::DropMe* crubit_nonnull __ret_ptr);
}
inline ::enums::repr_c_drop::DropMe::DropMe() {
  __crubit_internal::__crubit_thunk_default(this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_A(
    ::std::int32_t, ::enums::repr_c_drop::DropMe* crubit_nonnull __ret_ptr);
}
inline ::enums::repr_c_drop::DropMe DropMe::MakeA(::std::int32_t __param_0) {
  crubit::Slot<::enums::repr_c_drop::DropMe> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_A(__param_0, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_B(
    ::std::int64_t, ::enums::repr_c_drop::DropMe* crubit_nonnull __ret_ptr);
}
inline ::enums::repr_c_drop::DropMe DropMe::MakeB(::std::int64_t __param_0) {
  crubit::Slot<::enums::repr_c_drop::DropMe> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_B(__param_0, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

// `static` constructor
inline DropMe DropMe::MakeQ() {
  return DropMe(PrivateTagCtorTag{}, Tag{INT64_C(2)});
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::enums::repr_c_drop::DropMe&);
}
inline DropMe::~DropMe() { __crubit_internal::__crubit_thunk_drop(*this); }
inline ::enums::repr_c_drop::DropMe::DropMe(DropMe&& other) : DropMe() {
  *this = ::std::move(other);
}
inline ::enums::repr_c_drop::DropMe& ::enums::repr_c_drop::DropMe::operator=(
    DropMe&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline void DropMe::__crubit_field_offset_assertions() {
  static_assert(8 == offsetof(DropMe, A));
  static_assert(8 == offsetof(DropMe, B));
  static_assert(8 == offsetof(DropMe, C));
  static_assert(0 == offsetof(DropMe::__crubit_A_struct, __field0));
  static_assert(0 == offsetof(DropMe::__crubit_B_struct, __field0));
  static_assert(0 == offsetof(DropMe::__crubit_C_struct, p));
}
}  // namespace enums::repr_c_drop

namespace enums::repr_int {

static_assert(
    sizeof(IntReprEnumWithNoPayload) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(IntReprEnumWithNoPayload) == 4,
    "Verify that ADT layout didn't change since this header got generated");

// `static` constructor
inline constexpr IntReprEnumWithNoPayload
IntReprEnumWithNoPayload::MakeNoPayload1() {
  return IntReprEnumWithNoPayload(PrivateBytesTag{}, {0, 0, 0, 0});
}

// `static` constructor
inline constexpr IntReprEnumWithNoPayload
IntReprEnumWithNoPayload::MakeNoPayload2() {
  return IntReprEnumWithNoPayload(PrivateBytesTag{}, {210, 4, 0, 0});
}
static_assert(::std::is_trivially_destructible_v<IntReprEnumWithNoPayload>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_int::IntReprEnumWithNoPayload>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::repr_int::IntReprEnumWithNoPayload>);
namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_uno_upayload1(
    ::enums::repr_int::IntReprEnumWithNoPayload const&);
}
inline bool IntReprEnumWithNoPayload::is_no_payload1() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_uno_upayload1(self);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_uno_upayload2(
    ::enums::repr_int::IntReprEnumWithNoPayload const&);
}
inline bool IntReprEnumWithNoPayload::is_no_payload2() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_uno_upayload2(self);
}
inline void IntReprEnumWithNoPayload::__crubit_field_offset_assertions() {
  static_assert(0 ==
                offsetof(IntReprEnumWithNoPayload, __opaque_blob_of_bytes));
}
static_assert(
    sizeof(IntReprWithSingleNoPayloadVariant) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(IntReprWithSingleNoPayloadVariant) == 4,
    "Verify that ADT layout didn't change since this header got generated");

// `static` constructor
inline constexpr IntReprWithSingleNoPayloadVariant
IntReprWithSingleNoPayloadVariant::MakeSingleVariant() {
  return IntReprWithSingleNoPayloadVariant(PrivateBytesTag{}, {0, 0, 0, 0});
}
static_assert(
    ::std::is_trivially_destructible_v<IntReprWithSingleNoPayloadVariant>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_int::IntReprWithSingleNoPayloadVariant>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::repr_int::IntReprWithSingleNoPayloadVariant>);
namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_usingle_uvariant(
    ::enums::repr_int::IntReprWithSingleNoPayloadVariant const&);
}
inline bool IntReprWithSingleNoPayloadVariant::is_single_variant() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_usingle_uvariant(self);
}
inline void
IntReprWithSingleNoPayloadVariant::__crubit_field_offset_assertions() {
  static_assert(
      0 == offsetof(IntReprWithSingleNoPayloadVariant, __opaque_blob_of_bytes));
}
static_assert(
    sizeof(NegReprIntEnum) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NegReprIntEnum) == 1,
    "Verify that ADT layout didn't change since this header got generated");

// `static` constructor
inline constexpr NegReprIntEnum NegReprIntEnum::MakeMinusOne() {
  return NegReprIntEnum(PrivateBytesTag{}, {255});
}

// `static` constructor
inline constexpr NegReprIntEnum NegReprIntEnum::MakeMinusTwo() {
  return NegReprIntEnum(PrivateBytesTag{}, {254});
}
static_assert(::std::is_trivially_destructible_v<NegReprIntEnum>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_int::NegReprIntEnum>);
static_assert(
    ::std::is_trivially_move_assignable_v<::enums::repr_int::NegReprIntEnum>);
namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_uminus_uone(
    ::enums::repr_int::NegReprIntEnum const&);
}
inline bool NegReprIntEnum::is_minus_one() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_uminus_uone(self);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_uminus_utwo(
    ::enums::repr_int::NegReprIntEnum const&);
}
inline bool NegReprIntEnum::is_minus_two() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_uminus_utwo(self);
}
inline void NegReprIntEnum::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NegReprIntEnum, __opaque_blob_of_bytes));
}
}  // namespace enums::repr_int

namespace enums::repr_rust {

static_assert(
    sizeof(RustReprEnum) == 12,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(RustReprEnum) == 4,
    "Verify that ADT layout didn't change since this header got generated");

// `static` constructor
inline constexpr RustReprEnum RustReprEnum::MakeVariant1() {
  return RustReprEnum(PrivateBytesTag{}, {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
}

// `static` constructor
inline constexpr RustReprEnum RustReprEnum::MakeVariant2() {
  return RustReprEnum(PrivateBytesTag{}, {1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
}

// `static` constructor
inline constexpr RustReprEnum RustReprEnum::MakeVariant3() {
  return RustReprEnum(PrivateBytesTag{}, {2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_TuplePayloadVariant(
    ::std::int32_t, ::std::int32_t,
    ::enums::repr_rust::RustReprEnum* crubit_nonnull __ret_ptr);
}
inline ::enums::repr_rust::RustReprEnum RustReprEnum::MakeTuplePayloadVariant(
    ::std::int32_t __param_0, ::std::int32_t __param_1) {
  crubit::Slot<::enums::repr_rust::RustReprEnum> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_TuplePayloadVariant(__param_0, __param_1,
                                                        __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
static_assert(::std::is_trivially_destructible_v<RustReprEnum>);
static_assert(
    ::std::is_trivially_move_constructible_v<::enums::repr_rust::RustReprEnum>);
static_assert(
    ::std::is_trivially_move_assignable_v<::enums::repr_rust::RustReprEnum>);
namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_get_uvariant_unumber(
    ::enums::repr_rust::RustReprEnum const&);
}
inline ::std::int32_t RustReprEnum::get_variant_number() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_uvariant_unumber(self);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_utuple_upayload_uvariant(
    ::enums::repr_rust::RustReprEnum const&);
}
inline bool RustReprEnum::is_tuple_payload_variant() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_utuple_upayload_uvariant(self);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_get_ufirst_uitem_ufrom_utuple_upayload(
    ::enums::repr_rust::RustReprEnum const&);
}
inline ::std::int32_t RustReprEnum::get_first_item_from_tuple_payload() const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_get_ufirst_uitem_ufrom_utuple_upayload(self);
}
inline void RustReprEnum::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(RustReprEnum, __opaque_blob_of_bytes));
}
static_assert(
    sizeof(RustReprWithNamingConflictBetweenCtorsAndMethods) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(RustReprWithNamingConflictBetweenCtorsAndMethods) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              RustReprWithNamingConflictBetweenCtorsAndMethods>);
static_assert(
    ::std::is_trivially_move_constructible_v<
        ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods>);
static_assert(
    ::std::is_trivially_move_assignable_v<
        ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_MakeNoPayloadVariant(
    ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods*
        crubit_nonnull __ret_ptr);
}
inline ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods
RustReprWithNamingConflictBetweenCtorsAndMethods::MakeNoPayloadVariant() {
  crubit::Slot<
      ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_MakeNoPayloadVariant(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_MakeTuplePayloadVariant(
    ::std::int32_t,
    ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods*
        crubit_nonnull __ret_ptr);
}
inline ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods
RustReprWithNamingConflictBetweenCtorsAndMethods::MakeTuplePayloadVariant(
    ::std::int32_t i) {
  crubit::Slot<
      ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_MakeTuplePayloadVariant(
      i, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_MakeStructPayloadVariant(
    ::std::int32_t,
    ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods*
        crubit_nonnull __ret_ptr);
}
inline ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods
RustReprWithNamingConflictBetweenCtorsAndMethods::MakeStructPayloadVariant(
    ::std::int32_t x) {
  crubit::Slot<
      ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_MakeStructPayloadVariant(
      x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_get_uvariant_unumber(
    ::enums::repr_rust::
        RustReprWithNamingConflictBetweenCtorsAndMethods const&);
}
inline ::std::int32_t
RustReprWithNamingConflictBetweenCtorsAndMethods::get_variant_number() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_uvariant_unumber(self);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_get_uvalue(
    ::enums::repr_rust::
        RustReprWithNamingConflictBetweenCtorsAndMethods const&);
}
inline ::std::int32_t
RustReprWithNamingConflictBetweenCtorsAndMethods::get_value() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_uvalue(self);
}
inline void RustReprWithNamingConflictBetweenCtorsAndMethods::
    __crubit_field_offset_assertions() {
  static_assert(0 == offsetof(RustReprWithNamingConflictBetweenCtorsAndMethods,
                              __opaque_blob_of_bytes));
}
static_assert(
    sizeof(RustReprWithSingleTuplePayloadVariant) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(RustReprWithSingleTuplePayloadVariant) == 4,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_SingleVariant(
    ::std::int32_t,
    ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant* crubit_nonnull
        __ret_ptr);
}
inline ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant
RustReprWithSingleTuplePayloadVariant::MakeSingleVariant(
    ::std::int32_t __param_0) {
  crubit::Slot<::enums::repr_rust::RustReprWithSingleTuplePayloadVariant>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_SingleVariant(__param_0,
                                                  __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
static_assert(
    ::std::is_trivially_destructible_v<RustReprWithSingleTuplePayloadVariant>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant>);
namespace __crubit_internal {
extern "C" ::std::int32_t
__crubit_thunk_get_usingle_uitem_ufrom_utuple_upayload(
    ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant const&);
}
inline ::std::int32_t
RustReprWithSingleTuplePayloadVariant::get_single_item_from_tuple_payload()
    const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_get_usingle_uitem_ufrom_utuple_upayload(self);
}
inline void
RustReprWithSingleTuplePayloadVariant::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(RustReprWithSingleTuplePayloadVariant,
                              __opaque_blob_of_bytes));
}
}  // namespace enums::repr_rust

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_ENUMS_GOLDEN
