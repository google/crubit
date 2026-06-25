// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// tuple_structs_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_TUPLE_STRUCTS_TUPLE_STRUCTS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_TUPLE_STRUCTS_TUPLE_STRUCTS_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/tuple.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <tuple>
#include <type_traits>
#include <utility>

namespace tuple_structs {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: CloneNoDefault") alignas(8)
    [[clang::trivial_abi]] CloneNoDefault final {
 public:
  // `tuple_structs_golden::CloneNoDefault` doesn't implement the `Default`
  // trait
  CloneNoDefault() = delete;

  // Drop::drop
  ~CloneNoDefault();

  // Clone::clone
  CloneNoDefault(const CloneNoDefault&);

  // Clone::clone_from
  ::tuple_structs::CloneNoDefault& operator=(const CloneNoDefault&);

  CloneNoDefault(::crubit::UnsafeRelocateTag, CloneNoDefault&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

 private:
  // Field type has been replaced with a blob of bytes: Generic types are not
  // supported yet (b/259749095)
  ::std::array<unsigned char, 8> value;

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: CopyNoDefault") alignas(4)
    [[clang::trivial_abi]] CopyNoDefault final {
 public:
  // `tuple_structs_golden::CopyNoDefault` doesn't implement the `Default` trait
  CopyNoDefault() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~CopyNoDefault() = default;
  CopyNoDefault(CopyNoDefault&&) = default;
  CopyNoDefault& operator=(CopyNoDefault&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  CopyNoDefault(const CopyNoDefault&) = default;
  CopyNoDefault& operator=(const CopyNoDefault&) = default;
  CopyNoDefault(::crubit::UnsafeRelocateTag, CopyNoDefault&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::tuple_structs::CopyNoDefault create(::std::int32_t value);

  union {
    ::std::int32_t value;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: DefaultAndCloneNoUnpin") alignas(4)
    [[clang::trivial_abi]] DefaultAndCloneNoUnpin final {
 public:
  // Default::default
  DefaultAndCloneNoUnpin();

  // No custom `Drop` impl and no custom "drop glue" required
  ~DefaultAndCloneNoUnpin() = default;
  DefaultAndCloneNoUnpin(DefaultAndCloneNoUnpin&&) = default;
  DefaultAndCloneNoUnpin& operator=(DefaultAndCloneNoUnpin&&) = default;

  // Clone::clone
  DefaultAndCloneNoUnpin(const DefaultAndCloneNoUnpin&);

  // Clone::clone_from
  ::tuple_structs::DefaultAndCloneNoUnpin& operator=(
      const DefaultAndCloneNoUnpin&);

  DefaultAndCloneNoUnpin(::crubit::UnsafeRelocateTag,
                         DefaultAndCloneNoUnpin&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::int32_t value;
  };
  // Field `_marker` omitted: C++ does not support zero-sized types.
 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: DefaultNoCopyNoClone") alignas(4)
    [[clang::trivial_abi]] DefaultNoCopyNoClone final {
 public:
  // Default::default
  DefaultNoCopyNoClone();

  // No custom `Drop` impl and no custom "drop glue" required
  ~DefaultNoCopyNoClone() = default;
  DefaultNoCopyNoClone(DefaultNoCopyNoClone&&) = default;
  DefaultNoCopyNoClone& operator=(DefaultNoCopyNoClone&&) = default;

  // `tuple_structs_golden::DefaultNoCopyNoClone` doesn't implement the `Clone`
  // trait
  DefaultNoCopyNoClone(const DefaultNoCopyNoClone&) = delete;
  DefaultNoCopyNoClone& operator=(const DefaultNoCopyNoClone&) = delete;
  DefaultNoCopyNoClone(::crubit::UnsafeRelocateTag,
                       DefaultNoCopyNoClone&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::int32_t value;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: DontMoveMe") alignas(8) [[clang::trivial_abi]]
DontMoveMe final {
 public:
  // `tuple_structs_golden::DontMoveMe` doesn't implement the `Default` trait
  DontMoveMe() = delete;

  // Drop::drop
  ~DontMoveMe();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  DontMoveMe(DontMoveMe&&) = delete;
  ::tuple_structs::DontMoveMe& operator=(DontMoveMe&&) = delete;
  // `tuple_structs_golden::DontMoveMe` doesn't implement the `Clone` trait
  DontMoveMe(const DontMoveMe&) = delete;
  DontMoveMe& operator=(const DontMoveMe&) = delete;
  DontMoveMe(::crubit::UnsafeRelocateTag, DontMoveMe&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

 private:
  // Field type has been replaced with a blob of bytes: Generic types are not
  // supported yet (b/259749095)
  ::std::array<unsigned char, 8> value;

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: TupleStructOnePrivateArg") alignas(4)
    [[clang::trivial_abi]] TupleStructOnePrivateArg final {
 public:
  // `tuple_structs_golden::TupleStructOnePrivateArg` doesn't implement the
  // `Default` trait
  TupleStructOnePrivateArg() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~TupleStructOnePrivateArg() = default;
  TupleStructOnePrivateArg(TupleStructOnePrivateArg&&) = default;
  TupleStructOnePrivateArg& operator=(TupleStructOnePrivateArg&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  TupleStructOnePrivateArg(const TupleStructOnePrivateArg&) = default;
  TupleStructOnePrivateArg& operator=(const TupleStructOnePrivateArg&) =
      default;
  TupleStructOnePrivateArg(::crubit::UnsafeRelocateTag,
                           TupleStructOnePrivateArg&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::tuple_structs::TupleStructOnePrivateArg create(::std::int32_t arg);

  // CRUBIT_ANNOTATE: must_bind=
  ::std::int32_t get_arg() const;

 private:
  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: TupleStructOnePublicArg") alignas(4)
    [[clang::trivial_abi]] TupleStructOnePublicArg final {
 public:
  // `tuple_structs_golden::TupleStructOnePublicArg` doesn't implement the
  // `Default` trait
  TupleStructOnePublicArg() = delete;

  // Synthesized tuple constructor
  explicit TupleStructOnePublicArg(::std::int32_t __field0)
      : __field0(::std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~TupleStructOnePublicArg() = default;
  TupleStructOnePublicArg(TupleStructOnePublicArg&&) = default;
  TupleStructOnePublicArg& operator=(TupleStructOnePublicArg&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  TupleStructOnePublicArg(const TupleStructOnePublicArg&) = default;
  TupleStructOnePublicArg& operator=(const TupleStructOnePublicArg&) = default;
  TupleStructOnePublicArg(::crubit::UnsafeRelocateTag,
                          TupleStructOnePublicArg&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::tuple_structs::TupleStructOnePublicArg create(::std::int32_t arg);

  // CRUBIT_ANNOTATE: must_bind=
  ::std::int32_t get_arg() const;

  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct
    CRUBIT_INTERNAL_RUST_TYPE(
        ":: tuple_structs_golden :: "
        "TupleStructOnePublicArgOnePrivateArg") alignas(4)
        [[clang::trivial_abi]] TupleStructOnePublicArgOnePrivateArg final {
 public:
  // `tuple_structs_golden::TupleStructOnePublicArgOnePrivateArg` doesn't
  // implement the `Default` trait
  TupleStructOnePublicArgOnePrivateArg() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~TupleStructOnePublicArgOnePrivateArg() = default;
  TupleStructOnePublicArgOnePrivateArg(TupleStructOnePublicArgOnePrivateArg&&) =
      default;
  TupleStructOnePublicArgOnePrivateArg& operator=(
      TupleStructOnePublicArgOnePrivateArg&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  TupleStructOnePublicArgOnePrivateArg(
      const TupleStructOnePublicArgOnePrivateArg&) = default;
  TupleStructOnePublicArgOnePrivateArg& operator=(
      const TupleStructOnePublicArgOnePrivateArg&) = default;
  TupleStructOnePublicArgOnePrivateArg(
      ::crubit::UnsafeRelocateTag,
      TupleStructOnePublicArgOnePrivateArg&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::tuple_structs::TupleStructOnePublicArgOnePrivateArg create(
      ::std::int32_t first_arg, ::std::int32_t second_arg);

  // CRUBIT_ANNOTATE: must_bind=
  ::std::int32_t get_second_arg() const;

  union {
    ::std::int32_t __field0;
  };

 private:
  union {
    ::std::int32_t __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: TupleStructTwoPrivateArgs") alignas(4)
    [[clang::trivial_abi]] TupleStructTwoPrivateArgs final {
 public:
  // `tuple_structs_golden::TupleStructTwoPrivateArgs` doesn't implement the
  // `Default` trait
  TupleStructTwoPrivateArgs() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~TupleStructTwoPrivateArgs() = default;
  TupleStructTwoPrivateArgs(TupleStructTwoPrivateArgs&&) = default;
  TupleStructTwoPrivateArgs& operator=(TupleStructTwoPrivateArgs&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  TupleStructTwoPrivateArgs(const TupleStructTwoPrivateArgs&) = default;
  TupleStructTwoPrivateArgs& operator=(const TupleStructTwoPrivateArgs&) =
      default;
  TupleStructTwoPrivateArgs(::crubit::UnsafeRelocateTag,
                            TupleStructTwoPrivateArgs&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::tuple_structs::TupleStructTwoPrivateArgs create(
      ::std::int32_t first_arg, ::std::int32_t second_arg);

  // CRUBIT_ANNOTATE: must_bind=
  ::std::int32_t get_first_arg() const;

  // CRUBIT_ANNOTATE: must_bind=
  ::std::int32_t get_second_arg() const;

 private:
  union {
    ::std::int32_t __field0;
  };
  union {
    ::std::int32_t __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: TupleStructTwoPublicArgs") alignas(4)
    [[clang::trivial_abi]] TupleStructTwoPublicArgs final {
 public:
  // `tuple_structs_golden::TupleStructTwoPublicArgs` doesn't implement the
  // `Default` trait
  TupleStructTwoPublicArgs() = delete;

  // Synthesized tuple constructor
  TupleStructTwoPublicArgs(::std::int32_t __field0, ::std::int32_t __field1)
      : __field0(::std::move(__field0)), __field1(::std::move(__field1)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~TupleStructTwoPublicArgs() = default;
  TupleStructTwoPublicArgs(TupleStructTwoPublicArgs&&) = default;
  TupleStructTwoPublicArgs& operator=(TupleStructTwoPublicArgs&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  TupleStructTwoPublicArgs(const TupleStructTwoPublicArgs&) = default;
  TupleStructTwoPublicArgs& operator=(const TupleStructTwoPublicArgs&) =
      default;
  TupleStructTwoPublicArgs(::crubit::UnsafeRelocateTag,
                           TupleStructTwoPublicArgs&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::tuple_structs::TupleStructTwoPublicArgs create(
      ::std::int32_t first_arg, ::std::int32_t second_arg);

  // CRUBIT_ANNOTATE: must_bind=
  ::std::int32_t get_first_arg() const;

  // CRUBIT_ANNOTATE: must_bind=
  ::std::int32_t get_second_arg() const;

  union {
    ::std::int32_t __field0;
  };
  union {
    ::std::int32_t __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: TupleStructWithCloneNoDefault") alignas(8)
    [[clang::trivial_abi]] TupleStructWithCloneNoDefault final {
 public:
  // `tuple_structs_golden::TupleStructWithCloneNoDefault` doesn't implement the
  // `Default` trait
  TupleStructWithCloneNoDefault() = delete;

  // Drop::drop
  ~TupleStructWithCloneNoDefault();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  TupleStructWithCloneNoDefault(TupleStructWithCloneNoDefault&&) = delete;
  ::tuple_structs::TupleStructWithCloneNoDefault& operator=(
      TupleStructWithCloneNoDefault&&) = delete;
  // `tuple_structs_golden::TupleStructWithCloneNoDefault` doesn't implement the
  // `Clone` trait
  TupleStructWithCloneNoDefault(const TupleStructWithCloneNoDefault&) = delete;
  TupleStructWithCloneNoDefault& operator=(
      const TupleStructWithCloneNoDefault&) = delete;
  TupleStructWithCloneNoDefault(::crubit::UnsafeRelocateTag,
                                TupleStructWithCloneNoDefault&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::tuple_structs::TupleStructWithCloneNoDefault create(
      ::std::int32_t value);

  // CRUBIT_ANNOTATE: must_bind=
  ::std::int32_t const& $(__anon1)
      get_value() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  union {
    ::tuple_structs::CloneNoDefault __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: TupleStructWithCppImmovableType") alignas(8)
    [[clang::trivial_abi]] TupleStructWithCppImmovableType final {
 public:
  // `tuple_structs_golden::TupleStructWithCppImmovableType` doesn't implement
  // the `Default` trait
  TupleStructWithCppImmovableType() = delete;

  // Drop::drop
  ~TupleStructWithCppImmovableType();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  TupleStructWithCppImmovableType(TupleStructWithCppImmovableType&&) = delete;
  ::tuple_structs::TupleStructWithCppImmovableType& operator=(
      TupleStructWithCppImmovableType&&) = delete;
  // `tuple_structs_golden::TupleStructWithCppImmovableType` doesn't implement
  // the `Clone` trait
  TupleStructWithCppImmovableType(const TupleStructWithCppImmovableType&) =
      delete;
  TupleStructWithCppImmovableType& operator=(
      const TupleStructWithCppImmovableType&) = delete;
  TupleStructWithCppImmovableType(::crubit::UnsafeRelocateTag,
                                  TupleStructWithCppImmovableType&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::tuple_structs::TupleStructWithCppImmovableType create(
      ::std::int32_t first_arg, ::std::int32_t second_arg);

  // CRUBIT_ANNOTATE: must_bind=
  ::std::int32_t get_first_arg() const;

  // CRUBIT_ANNOTATE: must_bind=
  ::std::int32_t const& $(__anon1)
      get_second_arg() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  union {
    ::tuple_structs::DontMoveMe __field1;
  };
  union {
    ::std::int32_t __field0;
  };

 private:
  unsigned char __padding0[4];

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct
    CRUBIT_INTERNAL_RUST_TYPE(
        ":: tuple_structs_golden :: "
        "TupleStructWithDefaultAndCloneNoUnpin") alignas(4)
        [[clang::trivial_abi]] TupleStructWithDefaultAndCloneNoUnpin final {
 public:
  // `tuple_structs_golden::TupleStructWithDefaultAndCloneNoUnpin` doesn't
  // implement the `Default` trait
  TupleStructWithDefaultAndCloneNoUnpin() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~TupleStructWithDefaultAndCloneNoUnpin() = default;
  TupleStructWithDefaultAndCloneNoUnpin(
      TupleStructWithDefaultAndCloneNoUnpin&&) = default;
  TupleStructWithDefaultAndCloneNoUnpin& operator=(
      TupleStructWithDefaultAndCloneNoUnpin&&) = default;

  // `tuple_structs_golden::TupleStructWithDefaultAndCloneNoUnpin` doesn't
  // implement the `Clone` trait
  TupleStructWithDefaultAndCloneNoUnpin(
      const TupleStructWithDefaultAndCloneNoUnpin&) = delete;
  TupleStructWithDefaultAndCloneNoUnpin& operator=(
      const TupleStructWithDefaultAndCloneNoUnpin&) = delete;
  TupleStructWithDefaultAndCloneNoUnpin(
      ::crubit::UnsafeRelocateTag,
      TupleStructWithDefaultAndCloneNoUnpin&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::tuple_structs::TupleStructWithDefaultAndCloneNoUnpin create();

  // CRUBIT_ANNOTATE: must_bind=
  ::std::int32_t get_arg() const;

  union {
    ::tuple_structs::DefaultAndCloneNoUnpin __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: TupleStructWithDefaultNoCopyNoClone") alignas(4)
    [[clang::trivial_abi]] TupleStructWithDefaultNoCopyNoClone final {
 public:
  // `tuple_structs_golden::TupleStructWithDefaultNoCopyNoClone` doesn't
  // implement the `Default` trait
  TupleStructWithDefaultNoCopyNoClone() = delete;

  // Synthesized tuple constructor
  explicit TupleStructWithDefaultNoCopyNoClone(
      ::tuple_structs::DefaultNoCopyNoClone __field0)
      : __field0(::std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~TupleStructWithDefaultNoCopyNoClone() = default;
  TupleStructWithDefaultNoCopyNoClone(TupleStructWithDefaultNoCopyNoClone&&) =
      default;
  TupleStructWithDefaultNoCopyNoClone& operator=(
      TupleStructWithDefaultNoCopyNoClone&&) = default;

  // `tuple_structs_golden::TupleStructWithDefaultNoCopyNoClone` doesn't
  // implement the `Clone` trait
  TupleStructWithDefaultNoCopyNoClone(
      const TupleStructWithDefaultNoCopyNoClone&) = delete;
  TupleStructWithDefaultNoCopyNoClone& operator=(
      const TupleStructWithDefaultNoCopyNoClone&) = delete;
  TupleStructWithDefaultNoCopyNoClone(
      ::crubit::UnsafeRelocateTag,
      TupleStructWithDefaultNoCopyNoClone&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::tuple_structs::DefaultNoCopyNoClone __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: TupleStructWithNoDefault") alignas(4)
    [[clang::trivial_abi]] TupleStructWithNoDefault final {
 public:
  // `tuple_structs_golden::TupleStructWithNoDefault` doesn't implement the
  // `Default` trait
  TupleStructWithNoDefault() = delete;

  // Synthesized tuple constructor
  explicit TupleStructWithNoDefault(::tuple_structs::CopyNoDefault __field0)
      : __field0(::std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~TupleStructWithNoDefault() = default;
  TupleStructWithNoDefault(TupleStructWithNoDefault&&) = default;
  TupleStructWithNoDefault& operator=(TupleStructWithNoDefault&&) = default;

  // `tuple_structs_golden::TupleStructWithNoDefault` doesn't implement the
  // `Clone` trait
  TupleStructWithNoDefault(const TupleStructWithNoDefault&) = delete;
  TupleStructWithNoDefault& operator=(const TupleStructWithNoDefault&) = delete;
  TupleStructWithNoDefault(::crubit::UnsafeRelocateTag,
                           TupleStructWithNoDefault&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::tuple_structs::CopyNoDefault __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: TupleStructWithNonExhaustiveCtor") alignas(4)
    [[clang::trivial_abi]] TupleStructWithNonExhaustiveCtor final {
 public:
  // Default::default
  TupleStructWithNonExhaustiveCtor();

  // No custom `Drop` impl and no custom "drop glue" required
  ~TupleStructWithNonExhaustiveCtor() = default;
  TupleStructWithNonExhaustiveCtor(TupleStructWithNonExhaustiveCtor&&) =
      default;
  TupleStructWithNonExhaustiveCtor& operator=(
      TupleStructWithNonExhaustiveCtor&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  TupleStructWithNonExhaustiveCtor(const TupleStructWithNonExhaustiveCtor&) =
      default;
  TupleStructWithNonExhaustiveCtor& operator=(
      const TupleStructWithNonExhaustiveCtor&) = default;
  TupleStructWithNonExhaustiveCtor(::crubit::UnsafeRelocateTag,
                                   TupleStructWithNonExhaustiveCtor&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::tuple_structs::TupleStructWithNonExhaustiveCtor create(
      ::std::int32_t first_arg, ::std::int32_t second_arg);

  union {
    ::std::int32_t __field0;
  };
  union {
    ::std::int32_t __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace tuple_structs

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "(i32 , i32 ,)") rs_std::Tuple<::std::int32_t, ::std::int32_t> {
 public:
  // Default::default
  Tuple();

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Tuple(const Tuple&) = default;
  Tuple& operator=(const Tuple&) = default;
  Tuple(Tuple&&) = default;
  Tuple& operator=(Tuple&&) = default;

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Tuple(std::tuple<::std::int32_t, ::std::int32_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::int32_t, ::std::int32_t>() && noexcept;

 private:
  unsigned char storage_[8];
};
#endif

namespace tuple_structs {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: TupleStructWithTupleFieldType") alignas(4)
    [[clang::trivial_abi]] TupleStructWithTupleFieldType final {
 public:
  // Default::default
  TupleStructWithTupleFieldType();

  // Synthesized tuple constructor
  explicit TupleStructWithTupleFieldType(
      rs_std::Tuple<::std::int32_t, ::std::int32_t> __field0)
      : __field0(::std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~TupleStructWithTupleFieldType() = default;
  TupleStructWithTupleFieldType(TupleStructWithTupleFieldType&&) = default;
  TupleStructWithTupleFieldType& operator=(TupleStructWithTupleFieldType&&) =
      default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  TupleStructWithTupleFieldType(const TupleStructWithTupleFieldType&) = default;
  TupleStructWithTupleFieldType& operator=(
      const TupleStructWithTupleFieldType&) = default;
  TupleStructWithTupleFieldType(::crubit::UnsafeRelocateTag,
                                TupleStructWithTupleFieldType&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::tuple_structs::TupleStructWithTupleFieldType create(
      ::std::tuple<::std::int32_t, ::std::int32_t> __param_0);

  // CRUBIT_ANNOTATE: must_bind=
  ::std::tuple<::std::int32_t, ::std::int32_t> get_arg() const;

  union {
    rs_std::Tuple<::std::int32_t, ::std::int32_t> __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(CloneNoDefault) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneNoDefault) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_drop(::tuple_structs::CloneNoDefault&);
/// \endcond
}  // namespace __crubit_internal
inline CloneNoDefault::~CloneNoDefault() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_clone(
    ::tuple_structs::CloneNoDefault const&,
    ::tuple_structs::CloneNoDefault* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_clone_ufrom(
    ::tuple_structs::CloneNoDefault&, ::tuple_structs::CloneNoDefault const&);
/// \endcond
}  // namespace __crubit_internal
inline ::tuple_structs::CloneNoDefault::CloneNoDefault(
    const CloneNoDefault& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline ::tuple_structs::CloneNoDefault& ::tuple_structs::CloneNoDefault::
operator=(const CloneNoDefault& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
inline void CloneNoDefault::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CloneNoDefault, value));
}
static_assert(
    sizeof(CopyNoDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CopyNoDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CopyNoDefault>);
static_assert(
    ::std::is_trivially_move_constructible_v<::tuple_structs::CopyNoDefault>);
static_assert(
    ::std::is_trivially_move_assignable_v<::tuple_structs::CopyNoDefault>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::tuple_structs::CopyNoDefault>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::tuple_structs::CopyNoDefault>);
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::tuple_structs::CopyNoDefault* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::tuple_structs::CopyNoDefault CopyNoDefault::create(
    ::std::int32_t value) {
  crubit::Slot<::tuple_structs::CopyNoDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void CopyNoDefault::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CopyNoDefault, value));
}
static_assert(
    sizeof(DefaultAndCloneNoUnpin) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(DefaultAndCloneNoUnpin) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_default(
    ::tuple_structs::DefaultAndCloneNoUnpin* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::tuple_structs::DefaultAndCloneNoUnpin::DefaultAndCloneNoUnpin() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(::std::is_trivially_destructible_v<DefaultAndCloneNoUnpin>);
static_assert(::std::is_trivially_move_constructible_v<
              ::tuple_structs::DefaultAndCloneNoUnpin>);
static_assert(::std::is_trivially_move_assignable_v<
              ::tuple_structs::DefaultAndCloneNoUnpin>);
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_clone(
    ::tuple_structs::DefaultAndCloneNoUnpin const&,
    ::tuple_structs::DefaultAndCloneNoUnpin* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_clone_ufrom(
    ::tuple_structs::DefaultAndCloneNoUnpin&,
    ::tuple_structs::DefaultAndCloneNoUnpin const&);
/// \endcond
}  // namespace __crubit_internal
inline ::tuple_structs::DefaultAndCloneNoUnpin::DefaultAndCloneNoUnpin(
    const DefaultAndCloneNoUnpin& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline ::tuple_structs::DefaultAndCloneNoUnpin& ::tuple_structs::
    DefaultAndCloneNoUnpin::operator=(const DefaultAndCloneNoUnpin& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
inline void DefaultAndCloneNoUnpin::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(DefaultAndCloneNoUnpin, value));
}
static_assert(
    sizeof(DefaultNoCopyNoClone) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(DefaultNoCopyNoClone) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_default(
    ::tuple_structs::DefaultNoCopyNoClone* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::tuple_structs::DefaultNoCopyNoClone::DefaultNoCopyNoClone() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(::std::is_trivially_destructible_v<DefaultNoCopyNoClone>);
static_assert(::std::is_trivially_move_constructible_v<
              ::tuple_structs::DefaultNoCopyNoClone>);
static_assert(::std::is_trivially_move_assignable_v<
              ::tuple_structs::DefaultNoCopyNoClone>);
inline void DefaultNoCopyNoClone::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(DefaultNoCopyNoClone, value));
}
static_assert(
    sizeof(DontMoveMe) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(DontMoveMe) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_drop(::tuple_structs::DontMoveMe&);
/// \endcond
}  // namespace __crubit_internal
inline DontMoveMe::~DontMoveMe() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
inline void DontMoveMe::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(DontMoveMe, value));
}
static_assert(
    sizeof(TupleStructOnePrivateArg) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStructOnePrivateArg) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<TupleStructOnePrivateArg>);
static_assert(::std::is_trivially_move_constructible_v<
              ::tuple_structs::TupleStructOnePrivateArg>);
static_assert(::std::is_trivially_move_assignable_v<
              ::tuple_structs::TupleStructOnePrivateArg>);
static_assert(::std::is_trivially_copy_constructible_v<
              ::tuple_structs::TupleStructOnePrivateArg>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::tuple_structs::TupleStructOnePrivateArg>);
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::tuple_structs::TupleStructOnePrivateArg* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::tuple_structs::TupleStructOnePrivateArg
TupleStructOnePrivateArg::create(::std::int32_t arg) {
  crubit::Slot<::tuple_structs::TupleStructOnePrivateArg>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(arg, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" ::std::int32_t __crubit_thunk_get_uarg(
    ::tuple_structs::TupleStructOnePrivateArg*);
/// \endcond
}  // namespace __crubit_internal
inline ::std::int32_t TupleStructOnePrivateArg::get_arg() const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_get_uarg(&self);
}
inline void TupleStructOnePrivateArg::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStructOnePrivateArg, __field0));
}
static_assert(
    sizeof(TupleStructOnePublicArg) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStructOnePublicArg) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<TupleStructOnePublicArg>);
static_assert(::std::is_trivially_move_constructible_v<
              ::tuple_structs::TupleStructOnePublicArg>);
static_assert(::std::is_trivially_move_assignable_v<
              ::tuple_structs::TupleStructOnePublicArg>);
static_assert(::std::is_trivially_copy_constructible_v<
              ::tuple_structs::TupleStructOnePublicArg>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::tuple_structs::TupleStructOnePublicArg>);
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::tuple_structs::TupleStructOnePublicArg* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::tuple_structs::TupleStructOnePublicArg TupleStructOnePublicArg::create(
    ::std::int32_t arg) {
  crubit::Slot<::tuple_structs::TupleStructOnePublicArg>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(arg, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" ::std::int32_t __crubit_thunk_get_uarg(
    ::tuple_structs::TupleStructOnePublicArg*);
/// \endcond
}  // namespace __crubit_internal
inline ::std::int32_t TupleStructOnePublicArg::get_arg() const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_get_uarg(&self);
}
inline void TupleStructOnePublicArg::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStructOnePublicArg, __field0));
}
static_assert(
    sizeof(TupleStructOnePublicArgOnePrivateArg) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStructOnePublicArgOnePrivateArg) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    ::std::is_trivially_destructible_v<TupleStructOnePublicArgOnePrivateArg>);
static_assert(::std::is_trivially_move_constructible_v<
              ::tuple_structs::TupleStructOnePublicArgOnePrivateArg>);
static_assert(::std::is_trivially_move_assignable_v<
              ::tuple_structs::TupleStructOnePublicArgOnePrivateArg>);
static_assert(::std::is_trivially_copy_constructible_v<
              ::tuple_structs::TupleStructOnePublicArgOnePrivateArg>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::tuple_structs::TupleStructOnePublicArgOnePrivateArg>);
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::std::int32_t,
    ::tuple_structs::TupleStructOnePublicArgOnePrivateArg* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::tuple_structs::TupleStructOnePublicArgOnePrivateArg
TupleStructOnePublicArgOnePrivateArg::create(::std::int32_t first_arg,
                                             ::std::int32_t second_arg) {
  crubit::Slot<::tuple_structs::TupleStructOnePublicArgOnePrivateArg>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(first_arg, second_arg,
                                           __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" ::std::int32_t __crubit_thunk_get_usecond_uarg(
    ::tuple_structs::TupleStructOnePublicArgOnePrivateArg*);
/// \endcond
}  // namespace __crubit_internal
inline ::std::int32_t TupleStructOnePublicArgOnePrivateArg::get_second_arg()
    const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_get_usecond_uarg(&self);
}
inline void
TupleStructOnePublicArgOnePrivateArg::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStructOnePublicArgOnePrivateArg, __field0));
  static_assert(4 == offsetof(TupleStructOnePublicArgOnePrivateArg, __field1));
}
static_assert(
    sizeof(TupleStructTwoPrivateArgs) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStructTwoPrivateArgs) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<TupleStructTwoPrivateArgs>);
static_assert(::std::is_trivially_move_constructible_v<
              ::tuple_structs::TupleStructTwoPrivateArgs>);
static_assert(::std::is_trivially_move_assignable_v<
              ::tuple_structs::TupleStructTwoPrivateArgs>);
static_assert(::std::is_trivially_copy_constructible_v<
              ::tuple_structs::TupleStructTwoPrivateArgs>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::tuple_structs::TupleStructTwoPrivateArgs>);
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::std::int32_t,
    ::tuple_structs::TupleStructTwoPrivateArgs* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::tuple_structs::TupleStructTwoPrivateArgs
TupleStructTwoPrivateArgs::create(::std::int32_t first_arg,
                                  ::std::int32_t second_arg) {
  crubit::Slot<::tuple_structs::TupleStructTwoPrivateArgs>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(first_arg, second_arg,
                                           __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" ::std::int32_t __crubit_thunk_get_ufirst_uarg(
    ::tuple_structs::TupleStructTwoPrivateArgs*);
/// \endcond
}  // namespace __crubit_internal
inline ::std::int32_t TupleStructTwoPrivateArgs::get_first_arg() const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_get_ufirst_uarg(&self);
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" ::std::int32_t __crubit_thunk_get_usecond_uarg(
    ::tuple_structs::TupleStructTwoPrivateArgs*);
/// \endcond
}  // namespace __crubit_internal
inline ::std::int32_t TupleStructTwoPrivateArgs::get_second_arg() const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_get_usecond_uarg(&self);
}
inline void TupleStructTwoPrivateArgs::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStructTwoPrivateArgs, __field0));
  static_assert(4 == offsetof(TupleStructTwoPrivateArgs, __field1));
}
static_assert(
    sizeof(TupleStructTwoPublicArgs) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStructTwoPublicArgs) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<TupleStructTwoPublicArgs>);
static_assert(::std::is_trivially_move_constructible_v<
              ::tuple_structs::TupleStructTwoPublicArgs>);
static_assert(::std::is_trivially_move_assignable_v<
              ::tuple_structs::TupleStructTwoPublicArgs>);
static_assert(::std::is_trivially_copy_constructible_v<
              ::tuple_structs::TupleStructTwoPublicArgs>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::tuple_structs::TupleStructTwoPublicArgs>);
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::std::int32_t,
    ::tuple_structs::TupleStructTwoPublicArgs* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::tuple_structs::TupleStructTwoPublicArgs
TupleStructTwoPublicArgs::create(::std::int32_t first_arg,
                                 ::std::int32_t second_arg) {
  crubit::Slot<::tuple_structs::TupleStructTwoPublicArgs>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(first_arg, second_arg,
                                           __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" ::std::int32_t __crubit_thunk_get_ufirst_uarg(
    ::tuple_structs::TupleStructTwoPublicArgs*);
/// \endcond
}  // namespace __crubit_internal
inline ::std::int32_t TupleStructTwoPublicArgs::get_first_arg() const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_get_ufirst_uarg(&self);
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" ::std::int32_t __crubit_thunk_get_usecond_uarg(
    ::tuple_structs::TupleStructTwoPublicArgs*);
/// \endcond
}  // namespace __crubit_internal
inline ::std::int32_t TupleStructTwoPublicArgs::get_second_arg() const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_get_usecond_uarg(&self);
}
inline void TupleStructTwoPublicArgs::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStructTwoPublicArgs, __field0));
  static_assert(4 == offsetof(TupleStructTwoPublicArgs, __field1));
}
static_assert(
    sizeof(TupleStructWithCloneNoDefault) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStructWithCloneNoDefault) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_drop(
    ::tuple_structs::TupleStructWithCloneNoDefault&);
/// \endcond
}  // namespace __crubit_internal
inline TupleStructWithCloneNoDefault::~TupleStructWithCloneNoDefault() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::tuple_structs::TupleStructWithCloneNoDefault* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::tuple_structs::TupleStructWithCloneNoDefault
TupleStructWithCloneNoDefault::create(::std::int32_t value) {
  crubit::Slot<::tuple_structs::TupleStructWithCloneNoDefault>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" ::std::int32_t const& $(__anon1) __crubit_thunk_get_uvalue(
    ::tuple_structs::TupleStructWithCloneNoDefault const&);
/// \endcond
}  // namespace __crubit_internal
inline ::std::int32_t const& $(__anon1)
    TupleStructWithCloneNoDefault::get_value() const& $(__anon1)
        CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_uvalue(self);
}
inline void TupleStructWithCloneNoDefault::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStructWithCloneNoDefault, __field0));
}
static_assert(
    sizeof(TupleStructWithCppImmovableType) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStructWithCppImmovableType) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_drop(
    ::tuple_structs::TupleStructWithCppImmovableType&);
/// \endcond
}  // namespace __crubit_internal
inline TupleStructWithCppImmovableType::~TupleStructWithCppImmovableType() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::std::int32_t,
    ::tuple_structs::TupleStructWithCppImmovableType* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::tuple_structs::TupleStructWithCppImmovableType
TupleStructWithCppImmovableType::create(::std::int32_t first_arg,
                                        ::std::int32_t second_arg) {
  crubit::Slot<::tuple_structs::TupleStructWithCppImmovableType>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(first_arg, second_arg,
                                           __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" ::std::int32_t __crubit_thunk_get_ufirst_uarg(
    ::tuple_structs::TupleStructWithCppImmovableType const&);
/// \endcond
}  // namespace __crubit_internal
inline ::std::int32_t TupleStructWithCppImmovableType::get_first_arg() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_ufirst_uarg(self);
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" ::std::int32_t const& $(__anon1) __crubit_thunk_get_usecond_uarg(
    ::tuple_structs::TupleStructWithCppImmovableType const&);
/// \endcond
}  // namespace __crubit_internal
inline ::std::int32_t const& $(__anon1)
    TupleStructWithCppImmovableType::get_second_arg() const& $(__anon1)
        CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_usecond_uarg(self);
}
inline void
TupleStructWithCppImmovableType::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStructWithCppImmovableType, __field1));
  static_assert(8 == offsetof(TupleStructWithCppImmovableType, __field0));
}
static_assert(
    sizeof(TupleStructWithDefaultAndCloneNoUnpin) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStructWithDefaultAndCloneNoUnpin) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    ::std::is_trivially_destructible_v<TupleStructWithDefaultAndCloneNoUnpin>);
static_assert(::std::is_trivially_move_constructible_v<
              ::tuple_structs::TupleStructWithDefaultAndCloneNoUnpin>);
static_assert(::std::is_trivially_move_assignable_v<
              ::tuple_structs::TupleStructWithDefaultAndCloneNoUnpin>);
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_create(
    ::tuple_structs::TupleStructWithDefaultAndCloneNoUnpin* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::tuple_structs::TupleStructWithDefaultAndCloneNoUnpin
TupleStructWithDefaultAndCloneNoUnpin::create() {
  crubit::Slot<::tuple_structs::TupleStructWithDefaultAndCloneNoUnpin>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" ::std::int32_t __crubit_thunk_get_uarg(
    ::tuple_structs::TupleStructWithDefaultAndCloneNoUnpin const&);
/// \endcond
}  // namespace __crubit_internal
inline ::std::int32_t TupleStructWithDefaultAndCloneNoUnpin::get_arg() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_uarg(self);
}
inline void
TupleStructWithDefaultAndCloneNoUnpin::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStructWithDefaultAndCloneNoUnpin, __field0));
}
static_assert(
    sizeof(TupleStructWithDefaultNoCopyNoClone) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStructWithDefaultNoCopyNoClone) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    ::std::is_trivially_destructible_v<TupleStructWithDefaultNoCopyNoClone>);
static_assert(::std::is_trivially_move_constructible_v<
              ::tuple_structs::TupleStructWithDefaultNoCopyNoClone>);
static_assert(::std::is_trivially_move_assignable_v<
              ::tuple_structs::TupleStructWithDefaultNoCopyNoClone>);
inline void
TupleStructWithDefaultNoCopyNoClone::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStructWithDefaultNoCopyNoClone, __field0));
}
static_assert(
    sizeof(TupleStructWithNoDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStructWithNoDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<TupleStructWithNoDefault>);
static_assert(::std::is_trivially_move_constructible_v<
              ::tuple_structs::TupleStructWithNoDefault>);
static_assert(::std::is_trivially_move_assignable_v<
              ::tuple_structs::TupleStructWithNoDefault>);
inline void TupleStructWithNoDefault::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStructWithNoDefault, __field0));
}
static_assert(
    sizeof(TupleStructWithNonExhaustiveCtor) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStructWithNonExhaustiveCtor) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_default(
    ::tuple_structs::TupleStructWithNonExhaustiveCtor* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::tuple_structs::TupleStructWithNonExhaustiveCtor::
    TupleStructWithNonExhaustiveCtor() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(
    ::std::is_trivially_destructible_v<TupleStructWithNonExhaustiveCtor>);
static_assert(::std::is_trivially_move_constructible_v<
              ::tuple_structs::TupleStructWithNonExhaustiveCtor>);
static_assert(::std::is_trivially_move_assignable_v<
              ::tuple_structs::TupleStructWithNonExhaustiveCtor>);
static_assert(::std::is_trivially_copy_constructible_v<
              ::tuple_structs::TupleStructWithNonExhaustiveCtor>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::tuple_structs::TupleStructWithNonExhaustiveCtor>);
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::std::int32_t,
    ::tuple_structs::TupleStructWithNonExhaustiveCtor* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::tuple_structs::TupleStructWithNonExhaustiveCtor
TupleStructWithNonExhaustiveCtor::create(::std::int32_t first_arg,
                                         ::std::int32_t second_arg) {
  crubit::Slot<::tuple_structs::TupleStructWithNonExhaustiveCtor>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(first_arg, second_arg,
                                           __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void
TupleStructWithNonExhaustiveCtor::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStructWithNonExhaustiveCtor, __field0));
  static_assert(4 == offsetof(TupleStructWithNonExhaustiveCtor, __field1));
}
static_assert(
    sizeof(TupleStructWithTupleFieldType) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStructWithTupleFieldType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_default(
    ::tuple_structs::TupleStructWithTupleFieldType* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::tuple_structs::TupleStructWithTupleFieldType::
    TupleStructWithTupleFieldType() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(
    ::std::is_trivially_destructible_v<TupleStructWithTupleFieldType>);
static_assert(::std::is_trivially_move_constructible_v<
              ::tuple_structs::TupleStructWithTupleFieldType>);
static_assert(::std::is_trivially_move_assignable_v<
              ::tuple_structs::TupleStructWithTupleFieldType>);
static_assert(::std::is_trivially_copy_constructible_v<
              ::tuple_structs::TupleStructWithTupleFieldType>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::tuple_structs::TupleStructWithTupleFieldType>);
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_create(
    void**, ::tuple_structs::TupleStructWithTupleFieldType* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::tuple_structs::TupleStructWithTupleFieldType
TupleStructWithTupleFieldType::create(
    ::std::tuple<::std::int32_t, ::std::int32_t> __param_0) {
  auto&& __param_0_0 = ::std::get<0>(__param_0);
  auto&& __param_0_cabi_0 = __param_0_0;
  auto&& __param_0_1 = ::std::get<1>(__param_0);
  auto&& __param_0_cabi_1 = __param_0_1;
  void* __param_0_cabi[] = {&__param_0_cabi_0, &__param_0_cabi_1};
  crubit::Slot<::tuple_structs::TupleStructWithTupleFieldType>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(__param_0_cabi,
                                           __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_get_uarg(
    ::tuple_structs::TupleStructWithTupleFieldType*, void** __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::std::tuple<::std::int32_t, ::std::int32_t>
TupleStructWithTupleFieldType::get_arg() const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  ::std::int32_t __return_value_0_ret_val_holder;
  ::std::int32_t* __return_value_0_storage = &__return_value_0_ret_val_holder;
  ::std::int32_t __return_value_1_ret_val_holder;
  ::std::int32_t* __return_value_1_storage = &__return_value_1_ret_val_holder;
  void* __return_value_storage[] = {__return_value_0_storage,
                                    __return_value_1_storage};
  __crubit_internal::__crubit_thunk_get_uarg(&self, __return_value_storage);
  return ::std::make_tuple(*__return_value_0_storage,
                           *__return_value_1_storage);
}
inline void TupleStructWithTupleFieldType::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStructWithTupleFieldType, __field0));
}
}  // namespace tuple_structs

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_default(
    rs_std::Tuple<::std::int32_t, ::std::int32_t>* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::rs_std::Tuple<::std::int32_t, ::std::int32_t>::Tuple() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs_std::Tuple<::std::int32_t, ::std::int32_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs_std::Tuple<::std::int32_t, ::std::int32_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::std::int32_t, ::std::int32_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::std::int32_t, ::std::int32_t>>);
inline rs_std::Tuple<::std::int32_t, ::std::int32_t>::Tuple(
    std::tuple<::std::int32_t, ::std::int32_t>&& tuple) noexcept {
  std::construct_at(reinterpret_cast<::std::int32_t*>(storage_ + 0),
                    std::move(std::get<0>(tuple)));
  std::construct_at(reinterpret_cast<::std::int32_t*>(storage_ + 4),
                    std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::int32_t, ::std::int32_t>::operator std::tuple<
    ::std::int32_t, ::std::int32_t>() && noexcept {
  return std::tuple<::std::int32_t, ::std::int32_t>(
      std::move(*reinterpret_cast<::std::int32_t*>(storage_ + 0)),
      std::move(*reinterpret_cast<::std::int32_t*>(storage_ + 4)));
}

#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_TUPLE_STRUCTS_TUPLE_STRUCTS_GOLDEN
