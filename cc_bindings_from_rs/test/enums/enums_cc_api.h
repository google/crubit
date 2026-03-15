// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// enums_golden
// Features: assume_lifetimes, assume_this_lifetimes, callables,
// check_default_initialized, experimental, fmt, supported, types, unsafe_view,
// wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_ENUMS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_ENUMS_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#include "support/annotations_internal.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

namespace enums::repr_c {

// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=11
struct CRUBIT_INTERNAL_RUST_TYPE(":: enums_golden :: repr_c :: MyEnum") alignas(
    8) [[clang::trivial_abi]] MyEnum final {
 public:
  // Default::default
  MyEnum();

  // Error generating bindings for `enums_golden::repr_c::MyEnum::E` defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=12:
  // Definition `std::string::String` comes from the `alloc` crate, but no
  // `--crate-header` was specified for this crate

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=13
  static ::enums::repr_c::MyEnum MakeA(std::int32_t __param_0,
                                       std::int64_t __param_1);

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=14
  static MyEnum MakeF();

  // Error generating bindings for `enums_golden::repr_c::MyEnum::Z` defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=15:
  // Tuple types cannot be used inside of compound data types, because
  // std::tuple is not layout-compatible with a Rust tuple.

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=16
  static MyEnum MakeG();

  // Error generating bindings for `enums_golden::repr_c::MyEnum::B` defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=17:
  // Constructing non-tuple, struct-like enum variants is not supported:
  // b/487357254

  // Error generating bindings for `enums_golden::repr_c::MyEnum::C` defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=18:
  // Constructing non-tuple, struct-like enum variants is not supported:
  // b/487357254

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=19
  static MyEnum MakeD();

  // Drop::drop
  ~MyEnum();

  MyEnum(MyEnum&&);
  ::enums::repr_c::MyEnum& operator=(MyEnum&&);

  // `enums_golden::repr_c::MyEnum` doesn't implement the `Clone` trait
  MyEnum(const MyEnum&) = delete;
  MyEnum& operator=(const MyEnum&) = delete;
  MyEnum(::crubit::UnsafeRelocateTag, MyEnum&& value) {
    std::memcpy(this, &value, sizeof(value));
  }
  struct alignas(0) __crubit_E_struct {
   private:
    // Field type has been replaced with a blob of bytes: Definition
    // `std::string::String` comes from the `alloc` crate, but no
    // `--crate-header` was specified for this crate
    std::array<unsigned char, 24> __field0;

   public:
    std::int32_t __field1;
  };
  struct alignas(0) __crubit_A_struct {
   public:
    std::int32_t __field0;
    std::int64_t __field1;
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
    std::int32_t a;
    std::int32_t b;
    std::int32_t c;
  };
  // Variant D has no size, so no struct is generated.

  enum class Tag : std::int64_t {
    E = 0,
    A = 1,
    F = 2,
    Z = 3,
    G = 4,
    B = 10000,
    C = 10001,
    D = 10002,
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

}  // namespace enums::repr_c

namespace enums::repr_c_clone_active_variant {

// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=79
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c_clone_active_variant :: "
    "CloneActiveVariant") alignas(4) [[clang::trivial_abi]]
CloneActiveVariant final {
 public:
  // Default::default
  CloneActiveVariant();

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=80
  static ::enums::repr_c_clone_active_variant::CloneActiveVariant MakeA(
      std::int32_t __param_0);

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=81
  static ::enums::repr_c_clone_active_variant::CloneActiveVariant MakeB(
      std::int32_t __param_0);

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=82
  static ::enums::repr_c_clone_active_variant::CloneActiveVariant MakeC(
      std::int32_t __param_0);

  // No custom `Drop` impl and no custom "drop glue" required
  ~CloneActiveVariant() = default;
  CloneActiveVariant(CloneActiveVariant&&) = default;
  ::enums::repr_c_clone_active_variant::CloneActiveVariant& operator=(
      CloneActiveVariant&&) = default;

  // Clone::clone
  CloneActiveVariant(const CloneActiveVariant&);

  // Clone::clone_from
  ::enums::repr_c_clone_active_variant::CloneActiveVariant& operator=(
      const CloneActiveVariant&);

  CloneActiveVariant(::crubit::UnsafeRelocateTag, CloneActiveVariant&& value) {
    std::memcpy(this, &value, sizeof(value));
  }
  struct alignas(0) __crubit_A_struct {
   public:
    std::int32_t __field0;
  };
  struct alignas(0) __crubit_B_struct {
   public:
    std::int32_t __field0;
  };
  struct alignas(0) __crubit_C_struct {
   public:
    std::int32_t __field0;
  };

  enum class Tag : std::int8_t {
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

// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=101
bool is_a(::enums::repr_c_clone_active_variant::CloneActiveVariant const& e);

// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=105
bool is_b(::enums::repr_c_clone_active_variant::CloneActiveVariant const& e);

// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=109
bool is_c(::enums::repr_c_clone_active_variant::CloneActiveVariant const& e);

}  // namespace enums::repr_c_clone_active_variant

namespace enums::repr_c_clone_counter {

// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=55
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c_clone_counter :: CloneCount") alignas(8)
    [[clang::trivial_abi]] CloneCount final {
 public:
  // Default::default
  CloneCount();

  // Error generating bindings for
  // `enums_golden::repr_c_clone_counter::CloneCount::A` defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=56:
  // Constructing non-tuple, struct-like enum variants is not supported:
  // b/487357254

  // No custom `Drop` impl and no custom "drop glue" required
  ~CloneCount() = default;
  CloneCount(CloneCount&&) = default;
  ::enums::repr_c_clone_counter::CloneCount& operator=(CloneCount&&) = default;

  // Clone::clone
  CloneCount(const CloneCount&);

  // Clone::clone_from
  ::enums::repr_c_clone_counter::CloneCount& operator=(const CloneCount&);

  CloneCount(::crubit::UnsafeRelocateTag, CloneCount&& value) {
    std::memcpy(this, &value, sizeof(value));
  }
  struct alignas(0) __crubit_A_struct {
   public:
    std::int32_t* p;
  };

  enum class Tag : std::int8_t {
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

// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=31
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c_drop :: DropMe") alignas(8)
    [[clang::trivial_abi]] DropMe final {
 public:
  // Default::default
  DropMe();

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=32
  static ::enums::repr_c_drop::DropMe MakeA(std::int32_t __param_0);

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=33
  static ::enums::repr_c_drop::DropMe MakeB(std::int64_t __param_0);

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=34
  static DropMe MakeQ();

  // Error generating bindings for `enums_golden::repr_c_drop::DropMe::C`
  // defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=35:
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
    std::memcpy(this, &value, sizeof(value));
  }
  struct alignas(0) __crubit_A_struct {
   public:
    std::int32_t __field0;
  };
  struct alignas(0) __crubit_B_struct {
   public:
    std::int64_t __field0;
  };
  // Variant Q has no size, so no struct is generated.

  struct alignas(0) __crubit_C_struct {
   public:
    std::int32_t* p;
  };

  enum class Tag : std::uint32_t {
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
//
// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=188
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_int :: IntReprEnumWithNoPayload") alignas(4)
    [[clang::trivial_abi]] IntReprEnumWithNoPayload final {
 public:
  // `enums_golden::repr_int::IntReprEnumWithNoPayload` doesn't implement the
  // `Default` trait
  IntReprEnumWithNoPayload() = delete;

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=189
  static constexpr IntReprEnumWithNoPayload MakeNoPayload1();

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=190
  static constexpr IntReprEnumWithNoPayload MakeNoPayload2();

  // No custom `Drop` impl and no custom "drop glue" required
  ~IntReprEnumWithNoPayload() = default;
  IntReprEnumWithNoPayload(IntReprEnumWithNoPayload&&) = default;
  ::enums::repr_int::IntReprEnumWithNoPayload& operator=(
      IntReprEnumWithNoPayload&&) = default;

  // `enums_golden::repr_int::IntReprEnumWithNoPayload` doesn't implement the
  // `Clone` trait
  IntReprEnumWithNoPayload(const IntReprEnumWithNoPayload&) = delete;
  IntReprEnumWithNoPayload& operator=(const IntReprEnumWithNoPayload&) = delete;
  IntReprEnumWithNoPayload(::crubit::UnsafeRelocateTag,
                           IntReprEnumWithNoPayload&& value) {
    std::memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=194
  bool is_no_payload1() const;

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=197
  bool is_no_payload2() const;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  std::array<unsigned char, 4> __opaque_blob_of_bytes;

 private:
  struct PrivateBytesTag {};
  constexpr IntReprEnumWithNoPayload(PrivateBytesTag,
                                     std::array<unsigned char, 4> bytes)
      : __opaque_blob_of_bytes(bytes) {}

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace enums::repr_int

namespace enums::repr_rust {

//  Doc comment of RustReprEnum.
//
// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=116
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_rust :: RustReprEnum") alignas(4)
    [[clang::trivial_abi]] RustReprEnum final {
 public:
  // `enums_golden::repr_rust::RustReprEnum` doesn't implement the `Default`
  // trait
  RustReprEnum() = delete;

  //  Doc comment of Variant1.
  //
  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=118
  static constexpr RustReprEnum MakeVariant1();

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=119
  static constexpr RustReprEnum MakeVariant2();

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=120
  static constexpr RustReprEnum MakeVariant3();

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=121
  static ::enums::repr_rust::RustReprEnum MakeTuplePayloadVariant(
      std::int32_t __param_0, std::int32_t __param_1);

  // Error generating bindings for
  // `enums_golden::repr_rust::RustReprEnum::StructPayloadVariant` defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=122:
  // Constructing non-tuple, struct-like enum variants is not supported:
  // b/487357254

  // No custom `Drop` impl and no custom "drop glue" required
  ~RustReprEnum() = default;
  RustReprEnum(RustReprEnum&&) = default;
  ::enums::repr_rust::RustReprEnum& operator=(RustReprEnum&&) = default;

  // `enums_golden::repr_rust::RustReprEnum` doesn't implement the `Clone` trait
  RustReprEnum(const RustReprEnum&) = delete;
  RustReprEnum& operator=(const RustReprEnum&) = delete;
  RustReprEnum(::crubit::UnsafeRelocateTag, RustReprEnum&& value) {
    std::memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=129
  std::int32_t get_variant_number() const;

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=139
  bool is_tuple_payload_variant() const;

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=143
  std::int32_t get_first_item_from_tuple_payload() const;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  std::array<unsigned char, 12> __opaque_blob_of_bytes;

 private:
  struct PrivateBytesTag {};
  constexpr RustReprEnum(PrivateBytesTag, std::array<unsigned char, 12> bytes)
      : __opaque_blob_of_bytes(bytes) {}

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=159
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_rust :: "
    "RustReprWithNamingConflictBetweenCtorsAndMethods") alignas(4)
    [[clang::trivial_abi]]
    RustReprWithNamingConflictBetweenCtorsAndMethods final {
 public:
  // `enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods`
  // doesn't implement the `Default` trait
  RustReprWithNamingConflictBetweenCtorsAndMethods() = delete;

  // Error generating bindings for
  // `enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods::NoPayloadVariant`
  // defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=160:
  // Conflicting member function name: MakeNoPayloadVariant

  // Error generating bindings for
  // `enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods::TuplePayloadVariant`
  // defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=161:
  // Conflicting member function name: MakeTuplePayloadVariant

  // Error generating bindings for
  // `enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods::StructPayloadVariant`
  // defined at
  // cc_bindings_from_rs/test/enums/enums.rs;l=162:
  // Constructing non-tuple, struct-like enum variants is not supported:
  // b/487357254

  // No custom `Drop` impl and no custom "drop glue" required
  ~RustReprWithNamingConflictBetweenCtorsAndMethods() = default;
  RustReprWithNamingConflictBetweenCtorsAndMethods(
      RustReprWithNamingConflictBetweenCtorsAndMethods&&) = default;
  ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods&
  operator=(RustReprWithNamingConflictBetweenCtorsAndMethods&&) = default;

  // `enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods`
  // doesn't implement the `Clone` trait
  RustReprWithNamingConflictBetweenCtorsAndMethods(
      const RustReprWithNamingConflictBetweenCtorsAndMethods&) = delete;
  RustReprWithNamingConflictBetweenCtorsAndMethods& operator=(
      const RustReprWithNamingConflictBetweenCtorsAndMethods&) = delete;
  RustReprWithNamingConflictBetweenCtorsAndMethods(
      ::crubit::UnsafeRelocateTag,
      RustReprWithNamingConflictBetweenCtorsAndMethods&& value) {
    std::memcpy(this, &value, sizeof(value));
  }

  //  Presence of this function tests the scenario where `MakeNoPayloadVariant`
  //  is a name of:
  //  1. A static method (here/below).
  //  2. An auto-generated factory/constructor static method
  //
  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=170
  static ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods
  MakeNoPayloadVariant();

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=173
  static ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods
  MakeTuplePayloadVariant(std::int32_t i);

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=176
  static ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods
  MakeStructPayloadVariant(std::int32_t x);

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  std::array<unsigned char, 8> __opaque_blob_of_bytes;

 private:
  struct PrivateBytesTag {};
  constexpr RustReprWithNamingConflictBetweenCtorsAndMethods(
      PrivateBytesTag, std::array<unsigned char, 8> bytes)
      : __opaque_blob_of_bytes(bytes) {}

 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for
// `enums_golden::repr_rust::RustReprWithSingleNoPayloadVariant` defined at
// cc_bindings_from_rs/test/enums/enums.rs;l=151:
// Zero-sized types (ZSTs) are not supported (b/258259459)

// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=155
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_rust :: "
    "RustReprWithSingleTuplePayloadVariant") alignas(4) [[clang::trivial_abi]]
RustReprWithSingleTuplePayloadVariant final {
 public:
  // `enums_golden::repr_rust::RustReprWithSingleTuplePayloadVariant` doesn't
  // implement the `Default` trait
  RustReprWithSingleTuplePayloadVariant() = delete;

  // Generated from:
  // cc_bindings_from_rs/test/enums/enums.rs;l=156
  static ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant
  MakeSingleVariant(std::int32_t __param_0);

  // No custom `Drop` impl and no custom "drop glue" required
  ~RustReprWithSingleTuplePayloadVariant() = default;
  RustReprWithSingleTuplePayloadVariant(
      RustReprWithSingleTuplePayloadVariant&&) = default;
  ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant& operator=(
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
    std::memcpy(this, &value, sizeof(value));
  }

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  std::array<unsigned char, 4> __opaque_blob_of_bytes;

 private:
  struct PrivateBytesTag {};
  constexpr RustReprWithSingleTuplePayloadVariant(
      PrivateBytesTag, std::array<unsigned char, 4> bytes)
      : __opaque_blob_of_bytes(bytes) {}

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace enums::repr_rust

namespace enums::repr_c {

static_assert(
    sizeof(MyEnum) == 40,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyEnum) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::enums::repr_c::MyEnum* __ret_ptr);
}
inline ::enums::repr_c::MyEnum::MyEnum() {
  __crubit_internal::__crubit_thunk_default(this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_A(std::int32_t, std::int64_t,
                                 ::enums::repr_c::MyEnum* __ret_ptr);
}
inline ::enums::repr_c::MyEnum MyEnum::MakeA(std::int32_t __param_0,
                                             std::int64_t __param_1) {
  crubit::Slot<::enums::repr_c::MyEnum> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_A(__param_0, __param_1,
                                      __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
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
  *this = std::move(other);
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
  static_assert(24 == offsetof(MyEnum::__crubit_E_struct, __field1));
  static_assert(0 == offsetof(MyEnum::__crubit_A_struct, __field0));
  static_assert(8 == offsetof(MyEnum::__crubit_A_struct, __field1));
  static_assert(0 == offsetof(MyEnum::__crubit_B_struct, h));
  static_assert(1 == offsetof(MyEnum::__crubit_B_struct, i));
  static_assert(0 == offsetof(MyEnum::__crubit_C_struct, a));
  static_assert(4 == offsetof(MyEnum::__crubit_C_struct, b));
  static_assert(8 == offsetof(MyEnum::__crubit_C_struct, c));
}
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
    ::enums::repr_c_clone_active_variant::CloneActiveVariant* __ret_ptr);
}
inline ::enums::repr_c_clone_active_variant::CloneActiveVariant::
    CloneActiveVariant() {
  __crubit_internal::__crubit_thunk_default(this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_A(
    std::int32_t,
    ::enums::repr_c_clone_active_variant::CloneActiveVariant* __ret_ptr);
}
inline ::enums::repr_c_clone_active_variant::CloneActiveVariant
CloneActiveVariant::MakeA(std::int32_t __param_0) {
  crubit::Slot<::enums::repr_c_clone_active_variant::CloneActiveVariant>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_A(__param_0, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_B(
    std::int32_t,
    ::enums::repr_c_clone_active_variant::CloneActiveVariant* __ret_ptr);
}
inline ::enums::repr_c_clone_active_variant::CloneActiveVariant
CloneActiveVariant::MakeB(std::int32_t __param_0) {
  crubit::Slot<::enums::repr_c_clone_active_variant::CloneActiveVariant>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_B(__param_0, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_C(
    std::int32_t,
    ::enums::repr_c_clone_active_variant::CloneActiveVariant* __ret_ptr);
}
inline ::enums::repr_c_clone_active_variant::CloneActiveVariant
CloneActiveVariant::MakeC(std::int32_t __param_0) {
  crubit::Slot<::enums::repr_c_clone_active_variant::CloneActiveVariant>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_C(__param_0, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
static_assert(std::is_trivially_destructible_v<CloneActiveVariant>);
static_assert(std::is_trivially_move_constructible_v<
              ::enums::repr_c_clone_active_variant::CloneActiveVariant>);
static_assert(std::is_trivially_move_assignable_v<
              ::enums::repr_c_clone_active_variant::CloneActiveVariant>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const&,
    ::enums::repr_c_clone_active_variant::CloneActiveVariant* __ret_ptr);
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
    ::enums::repr_c_clone_counter::CloneCount* __ret_ptr);
}
inline ::enums::repr_c_clone_counter::CloneCount::CloneCount() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<CloneCount>);
static_assert(std::is_trivially_move_constructible_v<
              ::enums::repr_c_clone_counter::CloneCount>);
static_assert(std::is_trivially_move_assignable_v<
              ::enums::repr_c_clone_counter::CloneCount>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    ::enums::repr_c_clone_counter::CloneCount const&,
    ::enums::repr_c_clone_counter::CloneCount* __ret_ptr);
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
extern "C" void __crubit_thunk_default(::enums::repr_c_drop::DropMe* __ret_ptr);
}
inline ::enums::repr_c_drop::DropMe::DropMe() {
  __crubit_internal::__crubit_thunk_default(this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_A(std::int32_t,
                                 ::enums::repr_c_drop::DropMe* __ret_ptr);
}
inline ::enums::repr_c_drop::DropMe DropMe::MakeA(std::int32_t __param_0) {
  crubit::Slot<::enums::repr_c_drop::DropMe> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_A(__param_0, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_B(std::int64_t,
                                 ::enums::repr_c_drop::DropMe* __ret_ptr);
}
inline ::enums::repr_c_drop::DropMe DropMe::MakeB(std::int64_t __param_0) {
  crubit::Slot<::enums::repr_c_drop::DropMe> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_B(__param_0, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
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
  *this = std::move(other);
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
static_assert(std::is_trivially_destructible_v<IntReprEnumWithNoPayload>);
static_assert(std::is_trivially_move_constructible_v<
              ::enums::repr_int::IntReprEnumWithNoPayload>);
static_assert(std::is_trivially_move_assignable_v<
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
    std::int32_t, std::int32_t, ::enums::repr_rust::RustReprEnum* __ret_ptr);
}
inline ::enums::repr_rust::RustReprEnum RustReprEnum::MakeTuplePayloadVariant(
    std::int32_t __param_0, std::int32_t __param_1) {
  crubit::Slot<::enums::repr_rust::RustReprEnum> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_TuplePayloadVariant(__param_0, __param_1,
                                                        __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
static_assert(std::is_trivially_destructible_v<RustReprEnum>);
static_assert(
    std::is_trivially_move_constructible_v<::enums::repr_rust::RustReprEnum>);
static_assert(
    std::is_trivially_move_assignable_v<::enums::repr_rust::RustReprEnum>);
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_get_uvariant_unumber(
    ::enums::repr_rust::RustReprEnum const&);
}
inline std::int32_t RustReprEnum::get_variant_number() const {
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
extern "C" std::int32_t __crubit_thunk_get_ufirst_uitem_ufrom_utuple_upayload(
    ::enums::repr_rust::RustReprEnum const&);
}
inline std::int32_t RustReprEnum::get_first_item_from_tuple_payload() const {
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
static_assert(std::is_trivially_destructible_v<
              RustReprWithNamingConflictBetweenCtorsAndMethods>);
static_assert(
    std::is_trivially_move_constructible_v<
        ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods>);
static_assert(
    std::is_trivially_move_assignable_v<
        ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_MakeNoPayloadVariant(
    ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods*
        __ret_ptr);
}
inline ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods
RustReprWithNamingConflictBetweenCtorsAndMethods::MakeNoPayloadVariant() {
  crubit::Slot<
      ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_MakeNoPayloadVariant(
      __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_MakeTuplePayloadVariant(
    std::int32_t,
    ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods*
        __ret_ptr);
}
inline ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods
RustReprWithNamingConflictBetweenCtorsAndMethods::MakeTuplePayloadVariant(
    std::int32_t i) {
  crubit::Slot<
      ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_MakeTuplePayloadVariant(
      i, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_MakeStructPayloadVariant(
    std::int32_t,
    ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods*
        __ret_ptr);
}
inline ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods
RustReprWithNamingConflictBetweenCtorsAndMethods::MakeStructPayloadVariant(
    std::int32_t x) {
  crubit::Slot<
      ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_MakeStructPayloadVariant(
      x, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
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
    std::int32_t,
    ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant* __ret_ptr);
}
inline ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant
RustReprWithSingleTuplePayloadVariant::MakeSingleVariant(
    std::int32_t __param_0) {
  crubit::Slot<::enums::repr_rust::RustReprWithSingleTuplePayloadVariant>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_SingleVariant(__param_0,
                                                  __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
static_assert(
    std::is_trivially_destructible_v<RustReprWithSingleTuplePayloadVariant>);
static_assert(std::is_trivially_move_constructible_v<
              ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant>);
static_assert(std::is_trivially_move_assignable_v<
              ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant>);
inline void
RustReprWithSingleTuplePayloadVariant::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(RustReprWithSingleTuplePayloadVariant,
                              __opaque_blob_of_bytes));
}
}  // namespace enums::repr_rust

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_ENUMS_GOLDEN
