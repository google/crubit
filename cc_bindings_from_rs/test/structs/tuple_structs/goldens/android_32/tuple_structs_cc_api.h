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
#include "support/rs_std/traits.h"
#include "support/rs_std/tuple.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <memory>
#include <tuple>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_core.h"

namespace tuple_structs {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: CloneNoDefault") alignas(4)
    [[clang::trivial_abi]] CloneNoDefault final {
 public:
  // Type is not a C++ aggregate: Field `value` has a type unsupported in C++

  // `tuple_structs_golden::CloneNoDefault` doesn't implement the `Default`
  // trait
  CloneNoDefault() = delete;

  // Drop::drop
  ~CloneNoDefault();

  // Clone::clone
  CloneNoDefault(const CloneNoDefault&);

  // Clone::clone_from
  ::tuple_structs::CloneNoDefault& operator=(const CloneNoDefault&);

  CloneNoDefault(::crubit::UnsafeRelocateTag, CloneNoDefault&& value);

 private:
  // Field type has been replaced with a blob of bytes: Generic types are not
  // supported yet (b/259749095)
  ::std::array<unsigned char, 4> value;

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: CopyNoDefault") alignas(4)
    [[clang::trivial_abi]] CopyNoDefault final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::tuple_structs::CopyNoDefault create(::std::int32_t value);

  ::std::int32_t value = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: DefaultAndCloneNoUnpin") alignas(4)
    [[clang::trivial_abi]] DefaultAndCloneNoUnpin final {
 public:
  // Type is not a C++ aggregate: Field `_marker` has a type unsupported in C++

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
                         DefaultAndCloneNoUnpin&& value);

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
  ::std::int32_t value = {};

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: DontMoveMe") alignas(4) [[clang::trivial_abi]]
DontMoveMe final {
 public:
  // Type is not a C++ aggregate: Field `value` has a type unsupported in C++

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
  DontMoveMe(::crubit::UnsafeRelocateTag, DontMoveMe&& value);

 private:
  // Field type has been replaced with a blob of bytes: Generic types are not
  // supported yet (b/259749095)
  ::std::array<unsigned char, 4> value;

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
                           TupleStructOnePrivateArg&& value);

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
  // CRUBIT_ANNOTATE: must_bind=
  static ::tuple_structs::TupleStructOnePublicArg create(::std::int32_t arg);

  // CRUBIT_ANNOTATE: must_bind=
  ::std::int32_t get_arg() const;

  ::std::int32_t __field0 = {};

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
  // Type is not a C++ aggregate: Field `1` is not public

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
      TupleStructOnePublicArgOnePrivateArg&& value);

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
                            TupleStructTwoPrivateArgs&& value);

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
  // CRUBIT_ANNOTATE: must_bind=
  static ::tuple_structs::TupleStructTwoPublicArgs create(
      ::std::int32_t first_arg, ::std::int32_t second_arg);

  // CRUBIT_ANNOTATE: must_bind=
  ::std::int32_t get_first_arg() const;

  // CRUBIT_ANNOTATE: must_bind=
  ::std::int32_t get_second_arg() const;

  ::std::int32_t __field0 = {};
  ::std::int32_t __field1 = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: TupleStructWithCloneNoDefault") alignas(4)
    [[clang::trivial_abi]] TupleStructWithCloneNoDefault final {
 public:
  // Type is not a C++ aggregate: Field `0` is not default-constructible in C++

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
                                TupleStructWithCloneNoDefault&& value);

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
    ":: tuple_structs_golden :: TupleStructWithCppImmovableType") alignas(4)
    [[clang::trivial_abi]] TupleStructWithCppImmovableType final {
 public:
  // Type is not a C++ aggregate: Field `1` is not default-constructible in C++

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
                                  TupleStructWithCppImmovableType&& value);

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
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct
    CRUBIT_INTERNAL_RUST_TYPE(
        ":: tuple_structs_golden :: "
        "TupleStructWithDefaultAndCloneNoUnpin") alignas(4)
        [[clang::trivial_abi]] TupleStructWithDefaultAndCloneNoUnpin final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::tuple_structs::TupleStructWithDefaultAndCloneNoUnpin create();

  // CRUBIT_ANNOTATE: must_bind=
  ::std::int32_t get_arg() const;

  ::tuple_structs::DefaultAndCloneNoUnpin __field0 = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: TupleStructWithDefaultNoCopyNoClone") alignas(4)
    [[clang::trivial_abi]] TupleStructWithDefaultNoCopyNoClone final {
 public:
  ::tuple_structs::DefaultNoCopyNoClone __field0 = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: TupleStructWithNoDefault") alignas(4)
    [[clang::trivial_abi]] TupleStructWithNoDefault final {
 public:
  ::tuple_structs::CopyNoDefault __field0 = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: TupleStructWithNonExhaustiveCtor") alignas(4)
    [[clang::trivial_abi]] TupleStructWithNonExhaustiveCtor final {
 public:
  // Type is not a C++ aggregate: Type is marked `#[non_exhaustive]`

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
                                   TupleStructWithNonExhaustiveCtor&& value);

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

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value);
  Tuple(std::tuple<::std::int32_t, ::std::int32_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::int32_t, ::std::int32_t>() && noexcept;
  union {
    ::std::int32_t __field0;
  };
  union {
    ::std::int32_t __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};
#endif

namespace tuple_structs {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: TupleStructWithTupleFieldType") alignas(4)
    [[clang::trivial_abi]] TupleStructWithTupleFieldType final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::tuple_structs::TupleStructWithTupleFieldType create(
      ::std::tuple<::std::int32_t, ::std::int32_t> __param_0);

  // CRUBIT_ANNOTATE: must_bind=
  ::std::tuple<::std::int32_t, ::std::int32_t> get_arg() const;

  rs_std::Tuple<::std::int32_t, ::std::int32_t> __field0 = {};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace tuple_structs

template <>
struct rs_std::impl<::tuple_structs::DefaultAndCloneNoUnpin,
                    ::rs::core::marker::Unpin> {
  static constexpr bool kIsImplemented = false;
};

template <>
struct rs_std::impl<::tuple_structs::TupleStructWithDefaultAndCloneNoUnpin,
                    ::rs::core::marker::Unpin> {
  static constexpr bool kIsImplemented = false;
};

namespace tuple_structs {

static_assert(
    sizeof(CloneNoDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneNoDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_utuple_ustructs_ugolden_x0000003a_x0000003aCloneNoDefault(
    ::tuple_structs::CloneNoDefault&);
}
inline CloneNoDefault::~CloneNoDefault() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_utuple_ustructs_ugolden_x0000003a_x0000003aCloneNoDefault(
          *this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_utuple_ustructs_ugolden_x0000003a_x0000003aCloneNoDefault(
    ::tuple_structs::CloneNoDefault const&,
    ::tuple_structs::CloneNoDefault* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_utuple_ustructs_ugolden_x0000003a_x0000003aCloneNoDefault(
    ::tuple_structs::CloneNoDefault&, ::tuple_structs::CloneNoDefault const&);
}
inline ::tuple_structs::CloneNoDefault::CloneNoDefault(
    const CloneNoDefault& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_utuple_ustructs_ugolden_x0000003a_x0000003aCloneNoDefault(
          other, this);
}
inline ::tuple_structs::CloneNoDefault& ::tuple_structs::CloneNoDefault::
operator=(const CloneNoDefault& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_utuple_ustructs_ugolden_x0000003a_x0000003aCloneNoDefault(
            *this, other);
  }
  return *this;
}
inline ::tuple_structs::CloneNoDefault::CloneNoDefault(
    ::crubit::UnsafeRelocateTag, CloneNoDefault&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::tuple_structs::CopyNoDefault* __ret_ptr);
}
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
extern "C" void
__crubit_thunk_Default_udefault_utuple_ustructs_ugolden_x0000003a_x0000003aDefaultAndCloneNoUnpin(
    ::tuple_structs::DefaultAndCloneNoUnpin* __ret_ptr);
}
inline ::tuple_structs::DefaultAndCloneNoUnpin::DefaultAndCloneNoUnpin() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_utuple_ustructs_ugolden_x0000003a_x0000003aDefaultAndCloneNoUnpin(
          this);
}
static_assert(::std::is_trivially_destructible_v<DefaultAndCloneNoUnpin>);
static_assert(::std::is_trivially_move_constructible_v<
              ::tuple_structs::DefaultAndCloneNoUnpin>);
static_assert(::std::is_trivially_move_assignable_v<
              ::tuple_structs::DefaultAndCloneNoUnpin>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_utuple_ustructs_ugolden_x0000003a_x0000003aDefaultAndCloneNoUnpin(
    ::tuple_structs::DefaultAndCloneNoUnpin const&,
    ::tuple_structs::DefaultAndCloneNoUnpin* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_utuple_ustructs_ugolden_x0000003a_x0000003aDefaultAndCloneNoUnpin(
    ::tuple_structs::DefaultAndCloneNoUnpin&,
    ::tuple_structs::DefaultAndCloneNoUnpin const&);
}
inline ::tuple_structs::DefaultAndCloneNoUnpin::DefaultAndCloneNoUnpin(
    const DefaultAndCloneNoUnpin& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_utuple_ustructs_ugolden_x0000003a_x0000003aDefaultAndCloneNoUnpin(
          other, this);
}
inline ::tuple_structs::DefaultAndCloneNoUnpin& ::tuple_structs::
    DefaultAndCloneNoUnpin::operator=(const DefaultAndCloneNoUnpin& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_utuple_ustructs_ugolden_x0000003a_x0000003aDefaultAndCloneNoUnpin(
            *this, other);
  }
  return *this;
}
inline ::tuple_structs::DefaultAndCloneNoUnpin::DefaultAndCloneNoUnpin(
    ::crubit::UnsafeRelocateTag, DefaultAndCloneNoUnpin&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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
static_assert(::std::is_trivially_destructible_v<DefaultNoCopyNoClone>);
static_assert(::std::is_trivially_move_constructible_v<
              ::tuple_structs::DefaultNoCopyNoClone>);
static_assert(::std::is_trivially_move_assignable_v<
              ::tuple_structs::DefaultNoCopyNoClone>);
inline void DefaultNoCopyNoClone::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(DefaultNoCopyNoClone, value));
}
static_assert(
    sizeof(DontMoveMe) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(DontMoveMe) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_utuple_ustructs_ugolden_x0000003a_x0000003aDontMoveMe(
    ::tuple_structs::DontMoveMe&);
}
inline DontMoveMe::~DontMoveMe() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_utuple_ustructs_ugolden_x0000003a_x0000003aDontMoveMe(
          *this);
}
inline ::tuple_structs::DontMoveMe::DontMoveMe(::crubit::UnsafeRelocateTag,
                                               DontMoveMe&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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
inline ::tuple_structs::TupleStructOnePrivateArg::TupleStructOnePrivateArg(
    ::crubit::UnsafeRelocateTag, TupleStructOnePrivateArg&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::tuple_structs::TupleStructOnePrivateArg* __ret_ptr);
}
inline ::tuple_structs::TupleStructOnePrivateArg
TupleStructOnePrivateArg::create(::std::int32_t arg) {
  crubit::Slot<::tuple_structs::TupleStructOnePrivateArg>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(arg, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_get_uarg(
    ::tuple_structs::TupleStructOnePrivateArg*);
}
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
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::tuple_structs::TupleStructOnePublicArg* __ret_ptr);
}
inline ::tuple_structs::TupleStructOnePublicArg TupleStructOnePublicArg::create(
    ::std::int32_t arg) {
  crubit::Slot<::tuple_structs::TupleStructOnePublicArg>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(arg, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_get_uarg(
    ::tuple_structs::TupleStructOnePublicArg*);
}
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
inline ::tuple_structs::TupleStructOnePublicArgOnePrivateArg::
    TupleStructOnePublicArgOnePrivateArg(
        ::crubit::UnsafeRelocateTag,
        TupleStructOnePublicArgOnePrivateArg&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::std::int32_t,
    ::tuple_structs::TupleStructOnePublicArgOnePrivateArg* __ret_ptr);
}
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
extern "C" ::std::int32_t __crubit_thunk_get_usecond_uarg(
    ::tuple_structs::TupleStructOnePublicArgOnePrivateArg*);
}
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
inline ::tuple_structs::TupleStructTwoPrivateArgs::TupleStructTwoPrivateArgs(
    ::crubit::UnsafeRelocateTag, TupleStructTwoPrivateArgs&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::std::int32_t,
    ::tuple_structs::TupleStructTwoPrivateArgs* __ret_ptr);
}
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
extern "C" ::std::int32_t __crubit_thunk_get_ufirst_uarg(
    ::tuple_structs::TupleStructTwoPrivateArgs*);
}
inline ::std::int32_t TupleStructTwoPrivateArgs::get_first_arg() const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_get_ufirst_uarg(&self);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_get_usecond_uarg(
    ::tuple_structs::TupleStructTwoPrivateArgs*);
}
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
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::std::int32_t,
    ::tuple_structs::TupleStructTwoPublicArgs* __ret_ptr);
}
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
extern "C" ::std::int32_t __crubit_thunk_get_ufirst_uarg(
    ::tuple_structs::TupleStructTwoPublicArgs*);
}
inline ::std::int32_t TupleStructTwoPublicArgs::get_first_arg() const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_get_ufirst_uarg(&self);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_get_usecond_uarg(
    ::tuple_structs::TupleStructTwoPublicArgs*);
}
inline ::std::int32_t TupleStructTwoPublicArgs::get_second_arg() const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_get_usecond_uarg(&self);
}
inline void TupleStructTwoPublicArgs::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStructTwoPublicArgs, __field0));
  static_assert(4 == offsetof(TupleStructTwoPublicArgs, __field1));
}
static_assert(
    sizeof(TupleStructWithCloneNoDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStructWithCloneNoDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_utuple_ustructs_ugolden_x0000003a_x0000003aTupleStructWithCloneNoDefault(
    ::tuple_structs::TupleStructWithCloneNoDefault&);
}
inline TupleStructWithCloneNoDefault::~TupleStructWithCloneNoDefault() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_utuple_ustructs_ugolden_x0000003a_x0000003aTupleStructWithCloneNoDefault(
          *this);
}
inline ::tuple_structs::TupleStructWithCloneNoDefault::
    TupleStructWithCloneNoDefault(::crubit::UnsafeRelocateTag,
                                  TupleStructWithCloneNoDefault&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::tuple_structs::TupleStructWithCloneNoDefault* __ret_ptr);
}
inline ::tuple_structs::TupleStructWithCloneNoDefault
TupleStructWithCloneNoDefault::create(::std::int32_t value) {
  crubit::Slot<::tuple_structs::TupleStructWithCloneNoDefault>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t const& $(__anon1) __crubit_thunk_get_uvalue(
    ::tuple_structs::TupleStructWithCloneNoDefault const&);
}
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
    sizeof(TupleStructWithCppImmovableType) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStructWithCppImmovableType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_utuple_ustructs_ugolden_x0000003a_x0000003aTupleStructWithCppImmovableType(
    ::tuple_structs::TupleStructWithCppImmovableType&);
}
inline TupleStructWithCppImmovableType::~TupleStructWithCppImmovableType() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_utuple_ustructs_ugolden_x0000003a_x0000003aTupleStructWithCppImmovableType(
          *this);
}
inline ::tuple_structs::TupleStructWithCppImmovableType::
    TupleStructWithCppImmovableType(::crubit::UnsafeRelocateTag,
                                    TupleStructWithCppImmovableType&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::std::int32_t,
    ::tuple_structs::TupleStructWithCppImmovableType* __ret_ptr);
}
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
extern "C" ::std::int32_t __crubit_thunk_get_ufirst_uarg(
    ::tuple_structs::TupleStructWithCppImmovableType const&);
}
inline ::std::int32_t TupleStructWithCppImmovableType::get_first_arg() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_ufirst_uarg(self);
}

namespace __crubit_internal {
extern "C" ::std::int32_t const& $(__anon1) __crubit_thunk_get_usecond_uarg(
    ::tuple_structs::TupleStructWithCppImmovableType const&);
}
inline ::std::int32_t const& $(__anon1)
    TupleStructWithCppImmovableType::get_second_arg() const& $(__anon1)
        CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_usecond_uarg(self);
}
inline void
TupleStructWithCppImmovableType::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStructWithCppImmovableType, __field1));
  static_assert(4 == offsetof(TupleStructWithCppImmovableType, __field0));
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
extern "C" void __crubit_thunk_create(
    ::tuple_structs::TupleStructWithDefaultAndCloneNoUnpin* __ret_ptr);
}
inline ::tuple_structs::TupleStructWithDefaultAndCloneNoUnpin
TupleStructWithDefaultAndCloneNoUnpin::create() {
  crubit::Slot<::tuple_structs::TupleStructWithDefaultAndCloneNoUnpin>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_get_uarg(
    ::tuple_structs::TupleStructWithDefaultAndCloneNoUnpin const&);
}
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
extern "C" void
__crubit_thunk_Default_udefault_utuple_ustructs_ugolden_x0000003a_x0000003aTupleStructWithNonExhaustiveCtor(
    ::tuple_structs::TupleStructWithNonExhaustiveCtor* __ret_ptr);
}
inline ::tuple_structs::TupleStructWithNonExhaustiveCtor::
    TupleStructWithNonExhaustiveCtor() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_utuple_ustructs_ugolden_x0000003a_x0000003aTupleStructWithNonExhaustiveCtor(
          this);
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
inline ::tuple_structs::TupleStructWithNonExhaustiveCtor::
    TupleStructWithNonExhaustiveCtor(::crubit::UnsafeRelocateTag,
                                     TupleStructWithNonExhaustiveCtor&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    ::std::int32_t, ::std::int32_t,
    ::tuple_structs::TupleStructWithNonExhaustiveCtor* __ret_ptr);
}
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
extern "C" void __crubit_thunk_create(
    void**, ::tuple_structs::TupleStructWithTupleFieldType* __ret_ptr);
}
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
extern "C" void __crubit_thunk_get_uarg(
    ::tuple_structs::TupleStructWithTupleFieldType*, void** __ret_ptr);
}
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
extern "C" void
__crubit_thunk_Default_udefault_u_x00000028i32_x0000002c_x00000020i32_x00000029(
    rs_std::Tuple<::std::int32_t, ::std::int32_t>* __ret_ptr);
}
inline ::rs_std::Tuple<::std::int32_t, ::std::int32_t>::Tuple() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_u_x00000028i32_x0000002c_x00000020i32_x00000029(
          this);
}
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs_std::Tuple<::std::int32_t, ::std::int32_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs_std::Tuple<::std::int32_t, ::std::int32_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::std::int32_t, ::std::int32_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::std::int32_t, ::std::int32_t>>);
inline ::rs_std::Tuple<::std::int32_t, ::std::int32_t>::Tuple(
    ::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<::std::int32_t, ::std::int32_t>::Tuple(
    std::tuple<::std::int32_t, ::std::int32_t>&& tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
  std::construct_at(&this->__field1, std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::int32_t, ::std::int32_t>::operator std::tuple<
    ::std::int32_t, ::std::int32_t>() && noexcept {
  return std::tuple<::std::int32_t, ::std::int32_t>(std::move(this->__field0),
                                                    std::move(this->__field1));
}

inline void ::rs_std::Tuple<
    ::std::int32_t, ::std::int32_t>::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(4 == offsetof(Tuple, __field1));
}
#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_TUPLE_STRUCTS_TUPLE_STRUCTS_GOLDEN
