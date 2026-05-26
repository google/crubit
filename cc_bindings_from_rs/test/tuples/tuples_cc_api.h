// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// tuples_golden
// Features: assume_lifetimes, assume_this_lifetimes, callables,
// check_default_initialized, experimental, leading_colons_for_cpp_type,
// supported, template_instantiation, types, unsafe_view, wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TUPLES_TUPLES_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TUPLES_TUPLES_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/bridge.h"
#include "support/internal/check.h"
#include "support/internal/memswap.h"
#include "support/internal/move_assign.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/option.h"
#include "support/rs_std/result.h"
#include "support/rs_std/str_ref.h"
#include "support/rs_std/tuple.h"

#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <optional>
#include <tuple>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_alloc.h"

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=24
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuples_golden :: AdtHoldingFiveAndSix") alignas(4)
    [[clang::trivial_abi]] AdtHoldingFiveAndSix final {
 public:
  // `tuples_golden::AdtHoldingFiveAndSix` doesn't implement the `Default` trait
  AdtHoldingFiveAndSix() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~AdtHoldingFiveAndSix() = default;
  AdtHoldingFiveAndSix(AdtHoldingFiveAndSix&&) = default;
  AdtHoldingFiveAndSix& operator=(AdtHoldingFiveAndSix&&) = default;

  // `tuples_golden::AdtHoldingFiveAndSix` doesn't implement the `Clone` trait
  AdtHoldingFiveAndSix(const AdtHoldingFiveAndSix&) = delete;
  AdtHoldingFiveAndSix& operator=(const AdtHoldingFiveAndSix&) = delete;
  AdtHoldingFiveAndSix(::crubit::UnsafeRelocateTag,
                       AdtHoldingFiveAndSix&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=25
    ::std::int32_t five;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=26
    ::std::int32_t six;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=195
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: CloneNoDefault") alignas(
    1) [[clang::trivial_abi]] CloneNoDefault final {
 public:
  // `tuples_golden::CloneNoDefault` doesn't implement the `Default` trait
  CloneNoDefault() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~CloneNoDefault() = default;
  CloneNoDefault(CloneNoDefault&&) = default;
  CloneNoDefault& operator=(CloneNoDefault&&) = default;

  // Clone::clone
  CloneNoDefault(const CloneNoDefault&);

  // Clone::clone_from
  ::tuples::CloneNoDefault& operator=(const CloneNoDefault&);

  CloneNoDefault(::crubit::UnsafeRelocateTag, CloneNoDefault&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/tuples/tuples.rs;l=200
  static ::tuples::CloneNoDefault new_(::std::uint8_t val);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=196
    ::std::uint8_t val;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=168
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: CopyNoDefault") alignas(1)
    [[clang::trivial_abi]] CopyNoDefault final {
 public:
  // `tuples_golden::CopyNoDefault` doesn't implement the `Default` trait
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
  //
  // Generated from:
  // cc_bindings_from_rs/test/tuples/tuples.rs;l=173
  static ::tuples::CopyNoDefault new_(::std::uint8_t val);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=169
    ::std::uint8_t val;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=221
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: HasDefault") alignas(8)
    [[clang::trivial_abi]] HasDefault final {
 public:
  // Default::default
  HasDefault();

  // Drop::drop
  ~HasDefault();

  HasDefault(HasDefault&&);
  ::tuples::HasDefault& operator=(HasDefault&&);

  // `tuples_golden::HasDefault` doesn't implement the `Clone` trait
  HasDefault(const HasDefault&) = delete;
  HasDefault& operator=(const HasDefault&) = delete;
  HasDefault(::crubit::UnsafeRelocateTag, HasDefault&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/tuples/tuples.rs;l=226
  static ::tuples::HasDefault new_(rs_std::StrRef val);

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/tuples/tuples.rs;l=231
  rs_std::StrRef val() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=222
    ::rs::alloc::string::String val_;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=251
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: HasNoDefault") alignas(8)
    [[clang::trivial_abi]] HasNoDefault final {
 public:
  // `tuples_golden::HasNoDefault` doesn't implement the `Default` trait
  HasNoDefault() = delete;

  // Drop::drop
  ~HasNoDefault();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  HasNoDefault(HasNoDefault&&) = delete;
  ::tuples::HasNoDefault& operator=(HasNoDefault&&) = delete;
  // `tuples_golden::HasNoDefault` doesn't implement the `Clone` trait
  HasNoDefault(const HasNoDefault&) = delete;
  HasNoDefault& operator=(const HasNoDefault&) = delete;
  HasNoDefault(::crubit::UnsafeRelocateTag, HasNoDefault&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/tuples/tuples.rs;l=256
  rs_std::StrRef val() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=252
    ::rs::alloc::string::String val_;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//  The same as NontrivialDrop, but without a C++ move operation. This can be
//  returned by value, even inside a tuple!
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=70
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: NonCppMovable") alignas(1)
    [[clang::trivial_abi]] NonCppMovable final {
 public:
  // `tuples_golden::NonCppMovable` doesn't implement the `Default` trait
  NonCppMovable() = delete;

  // Drop::drop
  ~NonCppMovable();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  NonCppMovable(NonCppMovable&&) = delete;
  ::tuples::NonCppMovable& operator=(NonCppMovable&&) = delete;
  // `tuples_golden::NonCppMovable` doesn't implement the `Clone` trait
  NonCppMovable(const NonCppMovable&) = delete;
  NonCppMovable& operator=(const NonCppMovable&) = delete;
  NonCppMovable(::crubit::UnsafeRelocateTag, NonCppMovable&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=71
    ::std::uint8_t value;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=43
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: NontrivialDrop") alignas(
    1) [[clang::trivial_abi]] NontrivialDrop final {
 public:
  // Default::default
  NontrivialDrop();

  // Drop::drop
  ~NontrivialDrop();

  NontrivialDrop(NontrivialDrop&&);
  ::tuples::NontrivialDrop& operator=(NontrivialDrop&&);

  // `tuples_golden::NontrivialDrop` doesn't implement the `Clone` trait
  NontrivialDrop(const NontrivialDrop&) = delete;
  NontrivialDrop& operator=(const NontrivialDrop&) = delete;
  NontrivialDrop(::crubit::UnsafeRelocateTag, NontrivialDrop&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=43
    ::std::uint8_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for constant `tuples_golden::TUPLE_CONSTANT`
// defined at
// cc_bindings_from_rs/test/tuples/tuples.rs;l=125:
// Unsupported constant type: (i32,)

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=63
void assert_nontrivial_drop_count(::std::uint8_t drop_count);

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=33
void param_adt_in_tuple(::std::tuple<::tuples::AdtHoldingFiveAndSix> adt);

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=19
void param_c_abi_compatible_five_in_tuple(::std::tuple<::std::int32_t> five);

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=104
void param_ffi_alias_in_tuple(::std::tuple<::std::int8_t> five);

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=86
void param_nested_tuples(
    ::std::tuple<::std::tuple<::std::int32_t, ::std::int32_t>, ::std::int32_t>
        v);

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=59
void param_nontrivial_drop_in_tuple(
    ::std::tuple<::tuples::NontrivialDrop> nontrivial_drop);

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=295
void param_option_in_tuple(::std::tuple<::std::optional<::std::int32_t>> opt);

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=95
void param_triply_nested_tuple(
    ::std::tuple<::std::tuple<::std::tuple<::std::int32_t>>> v);

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=29
::std::tuple<::tuples::AdtHoldingFiveAndSix> return_adt_in_tuple();

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=15
::std::tuple<::std::int32_t> return_c_abi_compatible_five_in_tuple();

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=108
::std::tuple<::std::int8_t> return_ffi_alias_in_tuple();

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=90
::std::tuple<::std::tuple<::std::int32_t, ::std::int32_t>, ::std::int32_t>
return_nested_tuples();

// Error generating bindings for function
// `tuples_golden::return_new_non_cpp_movable_in_tuple` defined at
// cc_bindings_from_rs/test/tuples/tuples.rs;l=78:
// Can't return type `tuples_golden::NonCppMovable` by value inside a compound
// data type without a move constructor

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=55
::std::tuple<::tuples::NontrivialDrop> return_new_nontrivial_drop_in_tuple();

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=290
::std::tuple<::std::optional<::std::int32_t>> return_option_in_tuple();

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=99
::std::tuple<::std::tuple<::std::tuple<::std::int32_t>>>
return_triply_nested_tuple();

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=13
void return_unit_is_not_tuple();

}  // namespace tuples

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
template <>
struct alignas(4)
    CRUBIT_INTERNAL_RUST_TYPE("(i32 ,)") rs_std::Tuple<::std::int32_t> {
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
  Tuple(std::tuple<::std::int32_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::int32_t>() && noexcept;

 private:
  unsigned char storage_[4];
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=113
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: TupleStruct") alignas(4)
    [[clang::trivial_abi]] TupleStruct final {
 public:
  // `tuples_golden::TupleStruct` doesn't implement the `Default` trait
  TupleStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~TupleStruct() = default;
  TupleStruct(TupleStruct&&) = default;
  TupleStruct& operator=(TupleStruct&&) = default;

  // `tuples_golden::TupleStruct` doesn't implement the `Clone` trait
  TupleStruct(const TupleStruct&) = delete;
  TupleStruct& operator=(const TupleStruct&) = delete;
  TupleStruct(::crubit::UnsafeRelocateTag, TupleStruct&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // Error generating bindings for associated function
  // `tuples_golden::TupleStruct::tuple_not_by_value` defined at
  // cc_bindings_from_rs/test/tuples/tuples.rs;l=120:
  // Error formatting function return type `*const ()`: Failed to format the
  // pointee of the pointer type `*const ()`: Tuple type `()` is not supported
  // in this context

  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=114
    rs_std::Tuple<::std::int32_t> tuple_field;
  };
  // Skipped bindings for field `empty_tuple_field`: ZST fields are not
  // supported (b/258259459)
 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace tuples

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020intptr_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020intptr_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "(i8 , isize ,)") rs_std::Tuple<::std::int8_t, ::std::intptr_t> {
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
  Tuple(std::tuple<::std::int8_t, ::std::intptr_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::int8_t, ::std::intptr_t>() && noexcept;

 private:
  unsigned char storage_[16];
};
#endif
#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020intptr_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020intptr_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "(isize , i8 ,)") rs_std::Tuple<::std::intptr_t, ::std::int8_t> {
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
  Tuple(std::tuple<::std::intptr_t, ::std::int8_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::intptr_t, ::std::int8_t>() && noexcept;

 private:
  unsigned char storage_[16];
};
#endif
#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(1)
    CRUBIT_INTERNAL_RUST_TYPE("(:: tuples_golden :: CloneNoDefault , u8 ,)")
        rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t> {
 public:
  // `(tuples_golden::CloneNoDefault, u8)` doesn't implement the `Default` trait
  Tuple() = delete;

  // Clone::clone
  Tuple(const Tuple&);

  // Clone::clone_from
  ::rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t>& operator=(
      const Tuple&);

  Tuple(Tuple&&) = default;
  Tuple& operator=(Tuple&&) = default;

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Tuple(std::tuple<::tuples::CloneNoDefault, ::std::uint8_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::tuples::CloneNoDefault, ::std::uint8_t>() && noexcept;

 private:
  unsigned char storage_[2];
};
#endif
#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(1)
    CRUBIT_INTERNAL_RUST_TYPE("(:: tuples_golden :: CopyNoDefault , u8 ,)")
        rs_std::Tuple<::tuples::CopyNoDefault, ::std::uint8_t> {
 public:
  // `(tuples_golden::CopyNoDefault, u8)` doesn't implement the `Default` trait
  Tuple() = delete;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Tuple(const Tuple&) = default;
  Tuple& operator=(const Tuple&) = default;
  Tuple(Tuple&&) = default;
  Tuple& operator=(Tuple&&) = default;

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Tuple(std::tuple<::tuples::CopyNoDefault, ::std::uint8_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::tuples::CopyNoDefault, ::std::uint8_t>() && noexcept;

 private:
  unsigned char storage_[2];
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=275
::std::uint8_t take_tuple_copy_no_default_1(
    rs_std::Tuple<::tuples::CopyNoDefault, ::std::uint8_t> const& r);

}  // namespace tuples

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(8)
    CRUBIT_INTERNAL_RUST_TYPE("(:: tuples_golden :: HasDefault , u8 ,)")
        rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t> {
 public:
  // Default::default
  Tuple();

  // `(tuples_golden::HasDefault, u8)` doesn't implement the `Clone` trait
  Tuple(const Tuple&) = delete;
  Tuple& operator=(const Tuple&) = delete;
  Tuple(Tuple&&);
  ::rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t>& operator=(Tuple&&);
  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Tuple(std::tuple<::tuples::HasDefault, ::std::uint8_t>&& tuple) noexcept;
  ~Tuple();
  operator std::tuple<::tuples::HasDefault, ::std::uint8_t>() && noexcept;

 private:
  unsigned char storage_[32];
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=285
rs_std::StrRef take_tuple_has_default(
    rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t> const* $(__anon1)
        crubit_nonnull r CRUBIT_LIFETIME_BOUND);

}  // namespace tuples

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(8)
    CRUBIT_INTERNAL_RUST_TYPE("(:: tuples_golden :: HasNoDefault , u8 ,)")
        rs_std::Tuple<::tuples::HasNoDefault, ::std::uint8_t> {
 public:
  // `(tuples_golden::HasNoDefault, u8)` doesn't implement the `Default` trait
  Tuple() = delete;

  // `(tuples_golden::HasNoDefault, u8)` doesn't implement the `Clone` trait
  Tuple(const Tuple&) = delete;
  Tuple& operator=(const Tuple&) = delete;
  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  Tuple(Tuple&&) = delete;
  ::rs_std::Tuple<::tuples::HasNoDefault, ::std::uint8_t>& operator=(Tuple&&) =
      delete;
  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Tuple(std::tuple<::tuples::HasNoDefault, ::std::uint8_t>&& tuple) = delete;
  ~Tuple();
  operator std::tuple<::tuples::HasNoDefault, ::std::uint8_t>() && = delete;

 private:
  unsigned char storage_[32];
};
#endif
#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "(u32 , u32 ,)") rs_std::Tuple<::std::uint32_t, ::std::uint32_t> {
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
  Tuple(std::tuple<::std::uint32_t, ::std::uint32_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::uint32_t, ::std::uint32_t>() && noexcept;

 private:
  unsigned char storage_[8];
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=128
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: GetsTuple") alignas(4)
    [[clang::trivial_abi]] GetsTuple final {
 public:
  // `tuples_golden::GetsTuple` doesn't implement the `Default` trait
  GetsTuple() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~GetsTuple() = default;
  GetsTuple(GetsTuple&&) = default;
  GetsTuple& operator=(GetsTuple&&) = default;

  // `tuples_golden::GetsTuple` doesn't implement the `Clone` trait
  GetsTuple(const GetsTuple&) = delete;
  GetsTuple& operator=(const GetsTuple&) = delete;
  GetsTuple(::crubit::UnsafeRelocateTag, GetsTuple&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/tuples/tuples.rs;l=133
  static ::tuples::GetsTuple new_(::std::uint32_t val);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=129
    rs_std::Tuple<::std::uint32_t, ::std::uint32_t> value;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace tuples

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
template <>
struct alignas(4)
    CRUBIT_INTERNAL_RUST_TYPE("((u32 , u32 ,) , u32 ,)") rs_std::Tuple<
        rs_std::Tuple<::std::uint32_t, ::std::uint32_t>, ::std::uint32_t> {
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
  Tuple(std::tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                   ::std::uint32_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                      ::std::uint32_t>() && noexcept;

 private:
  unsigned char storage_[12];
};
#endif
#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE("(((u32 , u32 ,) , u32 ,) , u32 ,)")
    rs_std::Tuple<rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                                ::std::uint32_t>,
                  ::std::uint32_t> {
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
  Tuple(
      std::tuple<rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                               ::std::uint32_t>,
                 ::std::uint32_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<
      rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                    ::std::uint32_t>,
      ::std::uint32_t>() && noexcept;

 private:
  unsigned char storage_[16];
};
#endif
#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(4)
    CRUBIT_INTERNAL_RUST_TYPE("(u32 , (u32 , u32 ,) ,)") rs_std::Tuple<
        ::std::uint32_t, rs_std::Tuple<::std::uint32_t, ::std::uint32_t>> {
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
  Tuple(std::tuple<::std::uint32_t,
                   rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>&&
            tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<
      ::std::uint32_t,
      rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>() && noexcept;

 private:
  unsigned char storage_[12];
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=139
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuples_golden :: NestedTupleIntermediate1") alignas(4)
    [[clang::trivial_abi]] NestedTupleIntermediate1 final {
 public:
  // `tuples_golden::NestedTupleIntermediate1` doesn't implement the `Default`
  // trait
  NestedTupleIntermediate1() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~NestedTupleIntermediate1() = default;
  NestedTupleIntermediate1(NestedTupleIntermediate1&&) = default;
  NestedTupleIntermediate1& operator=(NestedTupleIntermediate1&&) = default;

  // `tuples_golden::NestedTupleIntermediate1` doesn't implement the `Clone`
  // trait
  NestedTupleIntermediate1(const NestedTupleIntermediate1&) = delete;
  NestedTupleIntermediate1& operator=(const NestedTupleIntermediate1&) = delete;
  NestedTupleIntermediate1(::crubit::UnsafeRelocateTag,
                           NestedTupleIntermediate1&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=140
    rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                  ::std::uint32_t>
        v1;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=141
    rs_std::Tuple<::std::uint32_t,
                  rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>
        v2;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace tuples

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(4)
    CRUBIT_INTERNAL_RUST_TYPE("(u32 , (u32 , (u32 , u32 ,) ,) ,)") rs_std::
        Tuple<::std::uint32_t,
              rs_std::Tuple<::std::uint32_t,
                            rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>> {
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
  Tuple(std::tuple<
        ::std::uint32_t,
        rs_std::Tuple<::std::uint32_t,
                      rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>&&
            tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<
      ::std::uint32_t,
      rs_std::Tuple<
          ::std::uint32_t,
          rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>() && noexcept;

 private:
  unsigned char storage_[16];
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=145
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuples_golden :: NestedTupleIntermediate2") alignas(4)
    [[clang::trivial_abi]] NestedTupleIntermediate2 final {
 public:
  // `tuples_golden::NestedTupleIntermediate2` doesn't implement the `Default`
  // trait
  NestedTupleIntermediate2() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~NestedTupleIntermediate2() = default;
  NestedTupleIntermediate2(NestedTupleIntermediate2&&) = default;
  NestedTupleIntermediate2& operator=(NestedTupleIntermediate2&&) = default;

  // `tuples_golden::NestedTupleIntermediate2` doesn't implement the `Clone`
  // trait
  NestedTupleIntermediate2(const NestedTupleIntermediate2&) = delete;
  NestedTupleIntermediate2& operator=(const NestedTupleIntermediate2&) = delete;
  NestedTupleIntermediate2(::crubit::UnsafeRelocateTag,
                           NestedTupleIntermediate2&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=146
    rs_std::Tuple<rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                                ::std::uint32_t>,
                  ::std::uint32_t>
        v1;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=147
    rs_std::Tuple<
        ::std::uint32_t,
        rs_std::Tuple<::std::uint32_t,
                      rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>
        v2;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=151
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuples_golden :: NestedTupleStruct") alignas(4) [[clang::trivial_abi]]
NestedTupleStruct final {
 public:
  // `tuples_golden::NestedTupleStruct` doesn't implement the `Default` trait
  NestedTupleStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~NestedTupleStruct() = default;
  NestedTupleStruct(NestedTupleStruct&&) = default;
  NestedTupleStruct& operator=(NestedTupleStruct&&) = default;

  // `tuples_golden::NestedTupleStruct` doesn't implement the `Clone` trait
  NestedTupleStruct(const NestedTupleStruct&) = delete;
  NestedTupleStruct& operator=(const NestedTupleStruct&) = delete;
  NestedTupleStruct(::crubit::UnsafeRelocateTag, NestedTupleStruct&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/tuples/tuples.rs;l=158
  static ::tuples::NestedTupleStruct new_(::std::uint32_t val);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=152
    rs_std::Tuple<rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                                ::std::uint32_t>,
                  ::std::uint32_t>
        in_tuple1;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=153
    rs_std::Tuple<
        ::std::uint32_t,
        rs_std::Tuple<::std::uint32_t,
                      rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>
        in_tuple2;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace tuples

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
template <>
struct alignas(1)
    CRUBIT_INTERNAL_RUST_TYPE("(u8 , :: tuples_golden :: CloneNoDefault ,)")
        rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault> {
 public:
  // `(u8, tuples_golden::CloneNoDefault)` doesn't implement the `Default` trait
  Tuple() = delete;

  // Clone::clone
  Tuple(const Tuple&);

  // Clone::clone_from
  ::rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault>& operator=(
      const Tuple&);

  Tuple(Tuple&&) = default;
  Tuple& operator=(Tuple&&) = default;

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Tuple(std::tuple<::std::uint8_t, ::tuples::CloneNoDefault>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::uint8_t, ::tuples::CloneNoDefault>() && noexcept;

 private:
  unsigned char storage_[2];
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=205
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuples_golden :: CloneNoDefaultTuple") alignas(1) [[clang::trivial_abi]]
CloneNoDefaultTuple final {
 public:
  // `tuples_golden::CloneNoDefaultTuple` doesn't implement the `Default` trait
  CloneNoDefaultTuple() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~CloneNoDefaultTuple() = default;
  CloneNoDefaultTuple(CloneNoDefaultTuple&&) = default;
  CloneNoDefaultTuple& operator=(CloneNoDefaultTuple&&) = default;

  // `tuples_golden::CloneNoDefaultTuple` doesn't implement the `Clone` trait
  CloneNoDefaultTuple(const CloneNoDefaultTuple&) = delete;
  CloneNoDefaultTuple& operator=(const CloneNoDefaultTuple&) = delete;
  CloneNoDefaultTuple(::crubit::UnsafeRelocateTag,
                      CloneNoDefaultTuple&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/tuples/tuples.rs;l=211
  static ::tuples::CloneNoDefaultTuple new_(::std::uint8_t val);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=206
    rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t> in_tuple1;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=207
    rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault> in_tuple2;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=280
::std::uint8_t take_tuple_clone_no_default_2(
    rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault> const& r);

}  // namespace tuples

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
template <>
struct alignas(1)
    CRUBIT_INTERNAL_RUST_TYPE("(u8 , :: tuples_golden :: CopyNoDefault ,)")
        rs_std::Tuple<::std::uint8_t, ::tuples::CopyNoDefault> {
 public:
  // `(u8, tuples_golden::CopyNoDefault)` doesn't implement the `Default` trait
  Tuple() = delete;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Tuple(const Tuple&) = default;
  Tuple& operator=(const Tuple&) = default;
  Tuple(Tuple&&) = default;
  Tuple& operator=(Tuple&&) = default;

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Tuple(std::tuple<::std::uint8_t, ::tuples::CopyNoDefault>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::uint8_t, ::tuples::CopyNoDefault>() && noexcept;

 private:
  unsigned char storage_[2];
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=179
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuples_golden :: CopyNoDefaultTuple") alignas(1) [[clang::trivial_abi]]
CopyNoDefaultTuple final {
 public:
  // `tuples_golden::CopyNoDefaultTuple` doesn't implement the `Default` trait
  CopyNoDefaultTuple() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~CopyNoDefaultTuple() = default;
  CopyNoDefaultTuple(CopyNoDefaultTuple&&) = default;
  CopyNoDefaultTuple& operator=(CopyNoDefaultTuple&&) = default;

  // `tuples_golden::CopyNoDefaultTuple` doesn't implement the `Clone` trait
  CopyNoDefaultTuple(const CopyNoDefaultTuple&) = delete;
  CopyNoDefaultTuple& operator=(const CopyNoDefaultTuple&) = delete;
  CopyNoDefaultTuple(::crubit::UnsafeRelocateTag, CopyNoDefaultTuple&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/tuples/tuples.rs;l=185
  static ::tuples::CopyNoDefaultTuple new_(::std::uint8_t val);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=180
    rs_std::Tuple<::tuples::CopyNoDefault, ::std::uint8_t> in_tuple1;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=181
    rs_std::Tuple<::std::uint8_t, ::tuples::CopyNoDefault> in_tuple2;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace tuples

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
template <>
struct alignas(8)
    CRUBIT_INTERNAL_RUST_TYPE("(u8 , :: tuples_golden :: HasDefault ,)")
        rs_std::Tuple<::std::uint8_t, ::tuples::HasDefault> {
 public:
  // Default::default
  Tuple();

  // `(u8, tuples_golden::HasDefault)` doesn't implement the `Clone` trait
  Tuple(const Tuple&) = delete;
  Tuple& operator=(const Tuple&) = delete;
  Tuple(Tuple&&);
  ::rs_std::Tuple<::std::uint8_t, ::tuples::HasDefault>& operator=(Tuple&&);
  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Tuple(std::tuple<::std::uint8_t, ::tuples::HasDefault>&& tuple) noexcept;
  ~Tuple();
  operator std::tuple<::std::uint8_t, ::tuples::HasDefault>() && noexcept;

 private:
  unsigned char storage_[32];
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=236
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: HasDefaultTuple") alignas(
    8) [[clang::trivial_abi]] HasDefaultTuple final {
 public:
  // `tuples_golden::HasDefaultTuple` doesn't implement the `Default` trait
  HasDefaultTuple() = delete;

  // Drop::drop
  ~HasDefaultTuple();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  HasDefaultTuple(HasDefaultTuple&&) = delete;
  ::tuples::HasDefaultTuple& operator=(HasDefaultTuple&&) = delete;
  // `tuples_golden::HasDefaultTuple` doesn't implement the `Clone` trait
  HasDefaultTuple(const HasDefaultTuple&) = delete;
  HasDefaultTuple& operator=(const HasDefaultTuple&) = delete;
  HasDefaultTuple(::crubit::UnsafeRelocateTag, HasDefaultTuple&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/tuples/tuples.rs;l=242
  static ::tuples::HasDefaultTuple new_(rs_std::StrRef val);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=237
    rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t> in_tuple1;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=238
    rs_std::Tuple<::std::uint8_t, ::tuples::HasDefault> in_tuple2;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace tuples

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
template <>
struct alignas(8)
    CRUBIT_INTERNAL_RUST_TYPE("(u8 , :: tuples_golden :: HasNoDefault ,)")
        rs_std::Tuple<::std::uint8_t, ::tuples::HasNoDefault> {
 public:
  // `(u8, tuples_golden::HasNoDefault)` doesn't implement the `Default` trait
  Tuple() = delete;

  // `(u8, tuples_golden::HasNoDefault)` doesn't implement the `Clone` trait
  Tuple(const Tuple&) = delete;
  Tuple& operator=(const Tuple&) = delete;
  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  Tuple(Tuple&&) = delete;
  ::rs_std::Tuple<::std::uint8_t, ::tuples::HasNoDefault>& operator=(Tuple&&) =
      delete;
  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Tuple(std::tuple<::std::uint8_t, ::tuples::HasNoDefault>&& tuple) = delete;
  ~Tuple();
  operator std::tuple<::std::uint8_t, ::tuples::HasNoDefault>() && = delete;

 private:
  unsigned char storage_[32];
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=261
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuples_golden :: HasNoDefaultTuple") alignas(8) [[clang::trivial_abi]]
HasNoDefaultTuple final {
 public:
  // `tuples_golden::HasNoDefaultTuple` doesn't implement the `Default` trait
  HasNoDefaultTuple() = delete;

  // Drop::drop
  ~HasNoDefaultTuple();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  HasNoDefaultTuple(HasNoDefaultTuple&&) = delete;
  ::tuples::HasNoDefaultTuple& operator=(HasNoDefaultTuple&&) = delete;
  // `tuples_golden::HasNoDefaultTuple` doesn't implement the `Clone` trait
  HasNoDefaultTuple(const HasNoDefaultTuple&) = delete;
  HasNoDefaultTuple& operator=(const HasNoDefaultTuple&) = delete;
  HasNoDefaultTuple(::crubit::UnsafeRelocateTag, HasNoDefaultTuple&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/tuples/tuples.rs;l=267
  static ::tuples::HasNoDefaultTuple new_(rs_std::StrRef val);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=262
    rs_std::Tuple<::tuples::HasNoDefault, ::std::uint8_t> in_tuple1;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=263
    rs_std::Tuple<::std::uint8_t, ::tuples::HasNoDefault> in_tuple2;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace tuples

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uintptr_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uintptr_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "(u8 , usize ,)") rs_std::Tuple<::std::uint8_t, ::std::uintptr_t> {
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
  Tuple(std::tuple<::std::uint8_t, ::std::uintptr_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::uint8_t, ::std::uintptr_t>() && noexcept;

 private:
  unsigned char storage_[16];
};
#endif
#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uintptr_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uintptr_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "(usize , u8 ,)") rs_std::Tuple<::std::uintptr_t, ::std::uint8_t> {
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
  Tuple(std::tuple<::std::uintptr_t, ::std::uint8_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::uintptr_t, ::std::uint8_t>() && noexcept;

 private:
  unsigned char storage_[16];
};
#endif

namespace tuples {

// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=316
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuples_golden :: TupleWithSizeTypes") alignas(8) [[clang::trivial_abi]]
TupleWithSizeTypes final {
 public:
  // `tuples_golden::TupleWithSizeTypes` doesn't implement the `Default` trait
  TupleWithSizeTypes() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~TupleWithSizeTypes() = default;
  TupleWithSizeTypes(TupleWithSizeTypes&&) = default;
  TupleWithSizeTypes& operator=(TupleWithSizeTypes&&) = default;

  // `tuples_golden::TupleWithSizeTypes` doesn't implement the `Clone` trait
  TupleWithSizeTypes(const TupleWithSizeTypes&) = delete;
  TupleWithSizeTypes& operator=(const TupleWithSizeTypes&) = delete;
  TupleWithSizeTypes(::crubit::UnsafeRelocateTag, TupleWithSizeTypes&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=318
    rs_std::Tuple<::std::uintptr_t, ::std::uint8_t> uval_in_tuple1;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=319
    rs_std::Tuple<::std::uint8_t, ::std::uintptr_t> uval_in_tuple2;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=320
    rs_std::Tuple<::std::intptr_t, ::std::int8_t> ival_in_tuple1;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=321
    rs_std::Tuple<::std::int8_t, ::std::intptr_t> ival_in_tuple2;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace tuples

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < i32 >") rs_std::Option<::std::int32_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  constexpr Option();

  constexpr explicit Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;

  Option(::std::int32_t&& value) noexcept;
  Option& operator=(::std::int32_t&& value) noexcept;

  explicit Option(::std::optional<::std::int32_t>&& value) noexcept;
  Option& operator=(::std::optional<::std::int32_t>&& value) noexcept;

  template <typename... Args>
  Option(::std::in_place_t, Args&&... args) noexcept;
  ~Option() noexcept = default;
  operator ::std::optional<::std::int32_t>() && noexcept;
  bool has_value() noexcept;

 private:
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;

 private:
  unsigned char storage_[8];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(4)
    CRUBIT_INTERNAL_RUST_TYPE("(:: core :: option :: Option < i32 > ,)")
        rs_std::Tuple<rs_std::Option<::std::int32_t>> {
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
  Tuple(std::tuple<rs_std::Option<::std::int32_t>>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<rs_std::Option<::std::int32_t>>() && noexcept;

 private:
  unsigned char storage_[8];
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=300
::std::optional<::std::int32_t> return_option_in_tuple_ref(
    rs_std::Tuple<rs_std::Option<::std::int32_t>> const& opt);

}  // namespace tuples

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < i32 , :: alloc :: string :: String >")
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String> {
 public:
  // Clone::clone
  Result(const Result&);

  // Clone::clone_from
  rs_std::Result<::std::int32_t, ::rs::alloc::string::String>& operator=(
      const Result&);

  Result(::crubit::UnsafeRelocateTag, Result&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Result(::std::int32_t&& ok) noexcept;
  Result& operator=(::std::int32_t&& ok) noexcept;
  Result(rs_std::unexpected<::rs::alloc::string::String>&& err) noexcept;
  Result& operator=(
      rs_std::unexpected<::rs::alloc::string::String>&& err) noexcept;
  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs_std::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  ::std::int32_t& value() &;
  ::std::int32_t&& value() &&;
  ::rs::alloc::string::String& err() &;
  ::rs::alloc::string::String&& err() &&;
  ~Result() noexcept;

 private:
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;
  void check_has_ok();
  void check_has_err();

 private:
  unsigned char __storage[24];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "(:: core :: option :: Option < i32 > , :: core :: result :: Result < i32 "
    ", :: alloc :: string :: String > ,)")
    rs_std::Tuple<rs_std::Option<::std::int32_t>,
                  rs_std::Result<::std::int32_t, ::rs::alloc::string::String>> {
 public:
  // `(std::option::Option<i32>, std::result::Result<i32, std::string::String>)`
  // doesn't implement the `Default` trait
  Tuple() = delete;

  // Clone::clone
  Tuple(const Tuple&);

  // Clone::clone_from
  ::rs_std::Tuple<rs_std::Option<::std::int32_t>,
                  rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>&
  operator=(const Tuple&);

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Tuple(
      std::tuple<rs_std::Option<::std::int32_t>,
                 rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>&&
          tuple) noexcept;
  ~Tuple();
  operator std::tuple<
      rs_std::Option<::std::int32_t>,
      rs_std::Result<::std::int32_t,
                     ::rs::alloc::string::String>>() && noexcept;

 private:
  unsigned char storage_[32];
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=305
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuples_golden :: StructWithOptionTuple") alignas(8)
    [[clang::trivial_abi]] StructWithOptionTuple final {
 public:
  // `tuples_golden::StructWithOptionTuple` doesn't implement the `Default`
  // trait
  StructWithOptionTuple() = delete;

  // Drop::drop
  ~StructWithOptionTuple();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  StructWithOptionTuple(StructWithOptionTuple&&) = delete;
  ::tuples::StructWithOptionTuple& operator=(StructWithOptionTuple&&) = delete;
  // `tuples_golden::StructWithOptionTuple` doesn't implement the `Clone` trait
  StructWithOptionTuple(const StructWithOptionTuple&) = delete;
  StructWithOptionTuple& operator=(const StructWithOptionTuple&) = delete;
  StructWithOptionTuple(::crubit::UnsafeRelocateTag,
                        StructWithOptionTuple&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/tuples/tuples.rs;l=311
  static ::tuples::StructWithOptionTuple new_(::std::int32_t val);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=306
    rs_std::Tuple<rs_std::Option<::std::int32_t>,
                  rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>
        opt_tuple;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(AdtHoldingFiveAndSix) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(AdtHoldingFiveAndSix) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<AdtHoldingFiveAndSix>);
static_assert(
    ::std::is_trivially_move_constructible_v<::tuples::AdtHoldingFiveAndSix>);
static_assert(
    ::std::is_trivially_move_assignable_v<::tuples::AdtHoldingFiveAndSix>);
inline void AdtHoldingFiveAndSix::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(AdtHoldingFiveAndSix, five));
  static_assert(4 == offsetof(AdtHoldingFiveAndSix, six));
}
static_assert(
    sizeof(CloneNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CloneNoDefault>);
static_assert(
    ::std::is_trivially_move_constructible_v<::tuples::CloneNoDefault>);
static_assert(::std::is_trivially_move_assignable_v<::tuples::CloneNoDefault>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(::tuples::CloneNoDefault const&,
                                     ::tuples::CloneNoDefault* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(::tuples::CloneNoDefault&,
                                           ::tuples::CloneNoDefault const&);
}
inline ::tuples::CloneNoDefault::CloneNoDefault(const CloneNoDefault& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline ::tuples::CloneNoDefault& ::tuples::CloneNoDefault::operator=(
    const CloneNoDefault& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uint8_t,
                                   ::tuples::CloneNoDefault* __ret_ptr);
}
inline ::tuples::CloneNoDefault CloneNoDefault::new_(::std::uint8_t val) {
  crubit::Slot<::tuples::CloneNoDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void CloneNoDefault::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CloneNoDefault, val));
}
static_assert(
    sizeof(CloneNoDefaultTuple) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneNoDefaultTuple) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CloneNoDefaultTuple>);
static_assert(
    ::std::is_trivially_move_constructible_v<::tuples::CloneNoDefaultTuple>);
static_assert(
    ::std::is_trivially_move_assignable_v<::tuples::CloneNoDefaultTuple>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uint8_t,
                                   ::tuples::CloneNoDefaultTuple* __ret_ptr);
}
inline ::tuples::CloneNoDefaultTuple CloneNoDefaultTuple::new_(
    ::std::uint8_t val) {
  crubit::Slot<::tuples::CloneNoDefaultTuple> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void CloneNoDefaultTuple::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CloneNoDefaultTuple, in_tuple1));
  static_assert(2 == offsetof(CloneNoDefaultTuple, in_tuple2));
}
static_assert(
    sizeof(CopyNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CopyNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CopyNoDefault>);
static_assert(
    ::std::is_trivially_move_constructible_v<::tuples::CopyNoDefault>);
static_assert(::std::is_trivially_move_assignable_v<::tuples::CopyNoDefault>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::tuples::CopyNoDefault>);
static_assert(::std::is_trivially_copy_assignable_v<::tuples::CopyNoDefault>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uint8_t,
                                   ::tuples::CopyNoDefault* __ret_ptr);
}
inline ::tuples::CopyNoDefault CopyNoDefault::new_(::std::uint8_t val) {
  crubit::Slot<::tuples::CopyNoDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void CopyNoDefault::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CopyNoDefault, val));
}
static_assert(
    sizeof(CopyNoDefaultTuple) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CopyNoDefaultTuple) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CopyNoDefaultTuple>);
static_assert(
    ::std::is_trivially_move_constructible_v<::tuples::CopyNoDefaultTuple>);
static_assert(
    ::std::is_trivially_move_assignable_v<::tuples::CopyNoDefaultTuple>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uint8_t,
                                   ::tuples::CopyNoDefaultTuple* __ret_ptr);
}
inline ::tuples::CopyNoDefaultTuple CopyNoDefaultTuple::new_(
    ::std::uint8_t val) {
  crubit::Slot<::tuples::CopyNoDefaultTuple> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void CopyNoDefaultTuple::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CopyNoDefaultTuple, in_tuple1));
  static_assert(2 == offsetof(CopyNoDefaultTuple, in_tuple2));
}
static_assert(
    sizeof(GetsTuple) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(GetsTuple) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<GetsTuple>);
static_assert(::std::is_trivially_move_constructible_v<::tuples::GetsTuple>);
static_assert(::std::is_trivially_move_assignable_v<::tuples::GetsTuple>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uint32_t,
                                   ::tuples::GetsTuple* __ret_ptr);
}
inline ::tuples::GetsTuple GetsTuple::new_(::std::uint32_t val) {
  crubit::Slot<::tuples::GetsTuple> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void GetsTuple::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(GetsTuple, value));
}
static_assert(
    sizeof(HasDefault) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(HasDefault) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::tuples::HasDefault* __ret_ptr);
}
inline ::tuples::HasDefault::HasDefault() {
  __crubit_internal::__crubit_thunk_default(this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::tuples::HasDefault&);
}
inline HasDefault::~HasDefault() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
inline ::tuples::HasDefault::HasDefault(HasDefault&& other) : HasDefault() {
  *this = ::std::move(other);
}
inline ::tuples::HasDefault& ::tuples::HasDefault::operator=(
    HasDefault&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(rs_std::StrRef,
                                   ::tuples::HasDefault* __ret_ptr);
}
inline ::tuples::HasDefault HasDefault::new_(rs_std::StrRef val) {
  crubit::Slot<::tuples::HasDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" rs_std::StrRef __crubit_thunk_val(::tuples::HasDefault const&);
}
inline rs_std::StrRef HasDefault::val() const& $(__anon1)
    CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_val(self);
}
inline void HasDefault::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(HasDefault, val_));
}
static_assert(
    sizeof(HasDefaultTuple) == 64,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(HasDefaultTuple) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::tuples::HasDefaultTuple&);
}
inline HasDefaultTuple::~HasDefaultTuple() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(rs_std::StrRef,
                                   ::tuples::HasDefaultTuple* __ret_ptr);
}
inline ::tuples::HasDefaultTuple HasDefaultTuple::new_(rs_std::StrRef val) {
  crubit::Slot<::tuples::HasDefaultTuple> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void HasDefaultTuple::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(HasDefaultTuple, in_tuple1));
  static_assert(32 == offsetof(HasDefaultTuple, in_tuple2));
}
static_assert(
    sizeof(HasNoDefault) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(HasNoDefault) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::tuples::HasNoDefault&);
}
inline HasNoDefault::~HasNoDefault() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
namespace __crubit_internal {
extern "C" rs_std::StrRef __crubit_thunk_val(::tuples::HasNoDefault const&);
}
inline rs_std::StrRef HasNoDefault::val() const& $(__anon1)
    CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_val(self);
}
inline void HasNoDefault::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(HasNoDefault, val_));
}
static_assert(
    sizeof(HasNoDefaultTuple) == 64,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(HasNoDefaultTuple) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::tuples::HasNoDefaultTuple&);
}
inline HasNoDefaultTuple::~HasNoDefaultTuple() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(rs_std::StrRef,
                                   ::tuples::HasNoDefaultTuple* __ret_ptr);
}
inline ::tuples::HasNoDefaultTuple HasNoDefaultTuple::new_(rs_std::StrRef val) {
  crubit::Slot<::tuples::HasNoDefaultTuple> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void HasNoDefaultTuple::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(HasNoDefaultTuple, in_tuple1));
  static_assert(32 == offsetof(HasNoDefaultTuple, in_tuple2));
}
static_assert(
    sizeof(NestedTupleIntermediate1) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NestedTupleIntermediate1) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NestedTupleIntermediate1>);
static_assert(::std::is_trivially_move_constructible_v<
              ::tuples::NestedTupleIntermediate1>);
static_assert(
    ::std::is_trivially_move_assignable_v<::tuples::NestedTupleIntermediate1>);
inline void NestedTupleIntermediate1::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NestedTupleIntermediate1, v1));
  static_assert(12 == offsetof(NestedTupleIntermediate1, v2));
}
static_assert(
    sizeof(NestedTupleIntermediate2) == 32,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NestedTupleIntermediate2) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NestedTupleIntermediate2>);
static_assert(::std::is_trivially_move_constructible_v<
              ::tuples::NestedTupleIntermediate2>);
static_assert(
    ::std::is_trivially_move_assignable_v<::tuples::NestedTupleIntermediate2>);
inline void NestedTupleIntermediate2::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NestedTupleIntermediate2, v1));
  static_assert(16 == offsetof(NestedTupleIntermediate2, v2));
}
static_assert(
    sizeof(NestedTupleStruct) == 32,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NestedTupleStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NestedTupleStruct>);
static_assert(
    ::std::is_trivially_move_constructible_v<::tuples::NestedTupleStruct>);
static_assert(
    ::std::is_trivially_move_assignable_v<::tuples::NestedTupleStruct>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uint32_t,
                                   ::tuples::NestedTupleStruct* __ret_ptr);
}
inline ::tuples::NestedTupleStruct NestedTupleStruct::new_(
    ::std::uint32_t val) {
  crubit::Slot<::tuples::NestedTupleStruct> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void NestedTupleStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NestedTupleStruct, in_tuple1));
  static_assert(16 == offsetof(NestedTupleStruct, in_tuple2));
}
static_assert(
    sizeof(NonCppMovable) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NonCppMovable) == 1,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::tuples::NonCppMovable&);
}
inline NonCppMovable::~NonCppMovable() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
inline void NonCppMovable::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NonCppMovable, value));
}
static_assert(
    sizeof(NontrivialDrop) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NontrivialDrop) == 1,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::tuples::NontrivialDrop* __ret_ptr);
}
inline ::tuples::NontrivialDrop::NontrivialDrop() {
  __crubit_internal::__crubit_thunk_default(this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::tuples::NontrivialDrop&);
}
inline NontrivialDrop::~NontrivialDrop() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
inline ::tuples::NontrivialDrop::NontrivialDrop(NontrivialDrop&& other)
    : NontrivialDrop() {
  *this = ::std::move(other);
}
inline ::tuples::NontrivialDrop& ::tuples::NontrivialDrop::operator=(
    NontrivialDrop&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline void NontrivialDrop::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NontrivialDrop, __field0));
}
static_assert(
    sizeof(StructWithOptionTuple) == 32,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructWithOptionTuple) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::tuples::StructWithOptionTuple&);
}
inline StructWithOptionTuple::~StructWithOptionTuple() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::int32_t,
                                   ::tuples::StructWithOptionTuple* __ret_ptr);
}
inline ::tuples::StructWithOptionTuple StructWithOptionTuple::new_(
    ::std::int32_t val) {
  crubit::Slot<::tuples::StructWithOptionTuple> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void StructWithOptionTuple::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructWithOptionTuple, opt_tuple));
}
static_assert(
    sizeof(TupleStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<TupleStruct>);
static_assert(::std::is_trivially_move_constructible_v<::tuples::TupleStruct>);
static_assert(::std::is_trivially_move_assignable_v<::tuples::TupleStruct>);
inline void TupleStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStruct, tuple_field));
}
static_assert(
    sizeof(TupleWithSizeTypes) == 64,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleWithSizeTypes) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<TupleWithSizeTypes>);
static_assert(
    ::std::is_trivially_move_constructible_v<::tuples::TupleWithSizeTypes>);
static_assert(
    ::std::is_trivially_move_assignable_v<::tuples::TupleWithSizeTypes>);
inline void TupleWithSizeTypes::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleWithSizeTypes, uval_in_tuple1));
  static_assert(16 == offsetof(TupleWithSizeTypes, uval_in_tuple2));
  static_assert(32 == offsetof(TupleWithSizeTypes, ival_in_tuple1));
  static_assert(48 == offsetof(TupleWithSizeTypes, ival_in_tuple2));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_assert_unontrivial_udrop_ucount(::std::uint8_t);
}
inline void assert_nontrivial_drop_count(::std::uint8_t drop_count) {
  return __crubit_internal::__crubit_thunk_assert_unontrivial_udrop_ucount(
      drop_count);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_param_uadt_uin_utuple(void**);
}
inline void param_adt_in_tuple(
    ::std::tuple<::tuples::AdtHoldingFiveAndSix> adt) {
  auto&& adt_0 = ::std::get<0>(adt);
  auto&& adt_cabi_0 = &adt_0;
  void* adt_cabi[] = {&adt_cabi_0};
  return __crubit_internal::__crubit_thunk_param_uadt_uin_utuple(adt_cabi);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_param_uc_uabi_ucompatible_ufive_uin_utuple(
    void**);
}
inline void param_c_abi_compatible_five_in_tuple(
    ::std::tuple<::std::int32_t> five) {
  auto&& five_0 = ::std::get<0>(five);
  auto&& five_cabi_0 = five_0;
  void* five_cabi[] = {&five_cabi_0};
  return __crubit_internal::
      __crubit_thunk_param_uc_uabi_ucompatible_ufive_uin_utuple(five_cabi);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_param_uffi_ualias_uin_utuple(void**);
}
inline void param_ffi_alias_in_tuple(::std::tuple<::std::int8_t> five) {
  auto&& five_0 = ::std::get<0>(five);
  auto&& five_cabi_0 = five_0;
  void* five_cabi[] = {&five_cabi_0};
  return __crubit_internal::__crubit_thunk_param_uffi_ualias_uin_utuple(
      five_cabi);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_param_unested_utuples(void**);
}
inline void param_nested_tuples(
    ::std::tuple<::std::tuple<::std::int32_t, ::std::int32_t>, ::std::int32_t>
        v) {
  auto&& v_0 = ::std::get<0>(v);
  auto&& v_0_0 = ::std::get<0>(v_0);
  auto&& v_0_cabi_0 = v_0_0;
  auto&& v_0_1 = ::std::get<1>(v_0);
  auto&& v_0_cabi_1 = v_0_1;
  void* v_0_cabi[] = {&v_0_cabi_0, &v_0_cabi_1};
  auto* v_cabi_0 = &v_0_cabi;
  auto&& v_1 = ::std::get<1>(v);
  auto&& v_cabi_1 = v_1;
  void* v_cabi[] = {&v_cabi_0, &v_cabi_1};
  return __crubit_internal::__crubit_thunk_param_unested_utuples(v_cabi);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_param_unontrivial_udrop_uin_utuple(void**);
}
inline void param_nontrivial_drop_in_tuple(
    ::std::tuple<::tuples::NontrivialDrop> nontrivial_drop) {
  auto&& nontrivial_drop_0 = ::std::get<0>(nontrivial_drop);
  crubit::Slot nontrivial_drop_0_slot((::std::move(nontrivial_drop_0)));
  auto&& nontrivial_drop_cabi_0 = nontrivial_drop_0_slot.Get();
  void* nontrivial_drop_cabi[] = {&nontrivial_drop_cabi_0};
  return __crubit_internal::__crubit_thunk_param_unontrivial_udrop_uin_utuple(
      nontrivial_drop_cabi);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_param_uoption_uin_utuple(void**);
}
inline void param_option_in_tuple(
    ::std::tuple<::std::optional<::std::int32_t>> opt) {
  auto&& opt_0 = ::std::get<0>(opt);
  unsigned char opt_0_buffer
      [::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>::kSize];
  ::crubit::internal::Encode<
      ::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>>(
      ::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>(
          ::crubit::TransmuteAbi<::std::int32_t>()),
      opt_0_buffer, opt_0);
  auto&& opt_cabi_0 = opt_0_buffer;
  void* opt_cabi[] = {&opt_cabi_0};
  return __crubit_internal::__crubit_thunk_param_uoption_uin_utuple(opt_cabi);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_param_utriply_unested_utuple(void**);
}
inline void param_triply_nested_tuple(
    ::std::tuple<::std::tuple<::std::tuple<::std::int32_t>>> v) {
  auto&& v_0 = ::std::get<0>(v);
  auto&& v_0_0 = ::std::get<0>(v_0);
  auto&& v_0_0_0 = ::std::get<0>(v_0_0);
  auto&& v_0_0_cabi_0 = v_0_0_0;
  void* v_0_0_cabi[] = {&v_0_0_cabi_0};
  auto* v_0_cabi_0 = &v_0_0_cabi;
  void* v_0_cabi[] = {&v_0_cabi_0};
  auto* v_cabi_0 = &v_0_cabi;
  void* v_cabi[] = {&v_cabi_0};
  return __crubit_internal::__crubit_thunk_param_utriply_unested_utuple(v_cabi);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uadt_uin_utuple(void** __ret_ptr);
}
inline ::std::tuple<::tuples::AdtHoldingFiveAndSix> return_adt_in_tuple() {
  crubit::Slot<::tuples::AdtHoldingFiveAndSix> __return_value_0_ret_val_holder;
  auto* __return_value_0_storage = __return_value_0_ret_val_holder.Get();
  void* __return_value_storage[] = {__return_value_0_storage};
  __crubit_internal::__crubit_thunk_return_uadt_uin_utuple(
      __return_value_storage);
  return ::std::make_tuple(
      ::std::move(__return_value_0_ret_val_holder).AssumeInitAndTakeValue());
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uc_uabi_ucompatible_ufive_uin_utuple(
    void** __ret_ptr);
}
inline ::std::tuple<::std::int32_t> return_c_abi_compatible_five_in_tuple() {
  ::std::int32_t __return_value_0_ret_val_holder;
  ::std::int32_t* __return_value_0_storage = &__return_value_0_ret_val_holder;
  void* __return_value_storage[] = {__return_value_0_storage};
  __crubit_internal::__crubit_thunk_return_uc_uabi_ucompatible_ufive_uin_utuple(
      __return_value_storage);
  return ::std::make_tuple(*__return_value_0_storage);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uffi_ualias_uin_utuple(void** __ret_ptr);
}
inline ::std::tuple<::std::int8_t> return_ffi_alias_in_tuple() {
  ::std::int8_t __return_value_0_ret_val_holder;
  ::std::int8_t* __return_value_0_storage = &__return_value_0_ret_val_holder;
  void* __return_value_storage[] = {__return_value_0_storage};
  __crubit_internal::__crubit_thunk_return_uffi_ualias_uin_utuple(
      __return_value_storage);
  return ::std::make_tuple(*__return_value_0_storage);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_unested_utuples(void** __ret_ptr);
}
inline ::std::tuple<::std::tuple<::std::int32_t, ::std::int32_t>,
                    ::std::int32_t>
return_nested_tuples() {
  ::std::int32_t __return_value_0_0_ret_val_holder;
  ::std::int32_t* __return_value_0_0_storage =
      &__return_value_0_0_ret_val_holder;
  ::std::int32_t __return_value_0_1_ret_val_holder;
  ::std::int32_t* __return_value_0_1_storage =
      &__return_value_0_1_ret_val_holder;
  void* __return_value_0_storage[] = {__return_value_0_0_storage,
                                      __return_value_0_1_storage};
  ::std::int32_t __return_value_1_ret_val_holder;
  ::std::int32_t* __return_value_1_storage = &__return_value_1_ret_val_holder;
  void* __return_value_storage[] = {__return_value_0_storage,
                                    __return_value_1_storage};
  __crubit_internal::__crubit_thunk_return_unested_utuples(
      __return_value_storage);
  return ::std::make_tuple(::std::make_tuple(*__return_value_0_0_storage,
                                             *__return_value_0_1_storage),
                           *__return_value_1_storage);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_unew_unontrivial_udrop_uin_utuple(
    void** __ret_ptr);
}
inline ::std::tuple<::tuples::NontrivialDrop>
return_new_nontrivial_drop_in_tuple() {
  crubit::Slot<::tuples::NontrivialDrop> __return_value_0_ret_val_holder;
  auto* __return_value_0_storage = __return_value_0_ret_val_holder.Get();
  void* __return_value_storage[] = {__return_value_0_storage};
  __crubit_internal::__crubit_thunk_return_unew_unontrivial_udrop_uin_utuple(
      __return_value_storage);
  return ::std::make_tuple(
      ::std::move(__return_value_0_ret_val_holder).AssumeInitAndTakeValue());
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uoption_uin_utuple(void** __ret_ptr);
}
inline ::std::tuple<::std::optional<::std::int32_t>> return_option_in_tuple() {
  unsigned char __return_value_0_storage
      [::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>::kSize];
  void* __return_value_storage[] = {__return_value_0_storage};
  __crubit_internal::__crubit_thunk_return_uoption_uin_utuple(
      __return_value_storage);
  return ::std::make_tuple(
      ::crubit::internal::Decode<
          ::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>>(
          ::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>(
              ::crubit::TransmuteAbi<::std::int32_t>()),
          __return_value_0_storage));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uoption_uin_utuple_uref(
    rs_std::Tuple<rs_std::Option<::std::int32_t>> const&,
    unsigned char* __ret_ptr);
}
inline ::std::optional<::std::int32_t> return_option_in_tuple_ref(
    rs_std::Tuple<rs_std::Option<::std::int32_t>> const& opt) {
  unsigned char __return_value_storage
      [::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>::kSize];
  __crubit_internal::__crubit_thunk_return_uoption_uin_utuple_uref(
      opt, __return_value_storage);
  return ::crubit::internal::Decode<
      ::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>>(
      ::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>(
          ::crubit::TransmuteAbi<::std::int32_t>()),
      __return_value_storage);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_utriply_unested_utuple(void** __ret_ptr);
}
inline ::std::tuple<::std::tuple<::std::tuple<::std::int32_t>>>
return_triply_nested_tuple() {
  ::std::int32_t __return_value_0_0_0_ret_val_holder;
  ::std::int32_t* __return_value_0_0_0_storage =
      &__return_value_0_0_0_ret_val_holder;
  void* __return_value_0_0_storage[] = {__return_value_0_0_0_storage};
  void* __return_value_0_storage[] = {__return_value_0_0_storage};
  void* __return_value_storage[] = {__return_value_0_storage};
  __crubit_internal::__crubit_thunk_return_utriply_unested_utuple(
      __return_value_storage);
  return ::std::make_tuple(
      ::std::make_tuple(::std::make_tuple(*__return_value_0_0_0_storage)));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uunit_uis_unot_utuple();
}
inline void return_unit_is_not_tuple() {
  return __crubit_internal::__crubit_thunk_return_uunit_uis_unot_utuple();
}

namespace __crubit_internal {
extern "C" ::std::uint8_t __crubit_thunk_take_utuple_uclone_uno_udefault_u2(
    rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault> const&);
}
inline ::std::uint8_t take_tuple_clone_no_default_2(
    rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault> const& r) {
  return __crubit_internal::__crubit_thunk_take_utuple_uclone_uno_udefault_u2(
      r);
}

namespace __crubit_internal {
extern "C" ::std::uint8_t __crubit_thunk_take_utuple_ucopy_uno_udefault_u1(
    rs_std::Tuple<::tuples::CopyNoDefault, ::std::uint8_t> const&);
}
inline ::std::uint8_t take_tuple_copy_no_default_1(
    rs_std::Tuple<::tuples::CopyNoDefault, ::std::uint8_t> const& r) {
  return __crubit_internal::__crubit_thunk_take_utuple_ucopy_uno_udefault_u1(r);
}

namespace __crubit_internal {
extern "C" rs_std::StrRef __crubit_thunk_take_utuple_uhas_udefault(
    rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t> const* $(__anon1)
        crubit_nonnull);
}
inline rs_std::StrRef take_tuple_has_default(
    rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t> const* $(__anon1)
        crubit_nonnull r CRUBIT_LIFETIME_BOUND) {
  return __crubit_internal::__crubit_thunk_take_utuple_uhas_udefault(r);
}

}  // namespace tuples

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    rs_std::Tuple<rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                                ::std::uint32_t>,
                  ::std::uint32_t>* __ret_ptr);
}
inline ::rs_std::Tuple<
    rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                  ::std::uint32_t>,
    ::std::uint32_t>::Tuple() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(::std::is_trivially_copy_constructible_v<::rs_std::Tuple<
                  rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                                ::std::uint32_t>,
                  ::std::uint32_t>>);
static_assert(::std::is_trivially_copy_assignable_v<::rs_std::Tuple<
                  rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                                ::std::uint32_t>,
                  ::std::uint32_t>>);
static_assert(::std::is_trivially_move_constructible_v<::rs_std::Tuple<
                  rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                                ::std::uint32_t>,
                  ::std::uint32_t>>);
static_assert(::std::is_trivially_move_assignable_v<::rs_std::Tuple<
                  rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                                ::std::uint32_t>,
                  ::std::uint32_t>>);
inline rs_std::Tuple<
    rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                  ::std::uint32_t>,
    ::std::uint32_t>::
    Tuple(std::tuple<
          rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                        ::std::uint32_t>,
          ::std::uint32_t>&& tuple) noexcept {
  std::construct_at(
      reinterpret_cast<rs_std::Tuple<
          rs_std::Tuple<::std::uint32_t, ::std::uint32_t>, ::std::uint32_t>*>(
          storage_ + 0),
      std::move(std::get<0>(tuple)));
  std::construct_at(reinterpret_cast<::std::uint32_t*>(storage_ + 12),
                    std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<
    rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                  ::std::uint32_t>,
    ::std::uint32_t>::
operator std::tuple<
    rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                  ::std::uint32_t>,
    ::std::uint32_t>() && noexcept {
  return std::tuple<
      rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                    ::std::uint32_t>,
      ::std::uint32_t>(
      std::move(*reinterpret_cast<
                rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                              ::std::uint32_t>*>(storage_ + 0)),
      std::move(*reinterpret_cast<::std::uint32_t*>(storage_ + 12)));
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                  ::std::uint32_t>* __ret_ptr);
}
inline ::rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                       ::std::uint32_t>::Tuple() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(
    ::std::is_trivially_copy_constructible_v<::rs_std::Tuple<
        rs_std::Tuple<::std::uint32_t, ::std::uint32_t>, ::std::uint32_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::rs_std::Tuple<
        rs_std::Tuple<::std::uint32_t, ::std::uint32_t>, ::std::uint32_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<::rs_std::Tuple<
        rs_std::Tuple<::std::uint32_t, ::std::uint32_t>, ::std::uint32_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<::rs_std::Tuple<
        rs_std::Tuple<::std::uint32_t, ::std::uint32_t>, ::std::uint32_t>>);
inline rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                     ::std::uint32_t>::
    Tuple(std::tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                     ::std::uint32_t>&& tuple) noexcept {
  std::construct_at(
      reinterpret_cast<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>*>(
          storage_ + 0),
      std::move(std::get<0>(tuple)));
  std::construct_at(reinterpret_cast<::std::uint32_t*>(storage_ + 8),
                    std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                     ::std::uint32_t>::
operator std::tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                    ::std::uint32_t>() && noexcept {
  return std::tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                    ::std::uint32_t>(
      std::move(
          *reinterpret_cast<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>*>(
              storage_ + 0)),
      std::move(*reinterpret_cast<::std::uint32_t*>(storage_ + 8)));
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    rs_std::Tuple<::std::int32_t>* __ret_ptr);
}
inline ::rs_std::Tuple<::std::int32_t>::Tuple() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(
    ::std::is_trivially_copy_constructible_v<::rs_std::Tuple<::std::int32_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::rs_std::Tuple<::std::int32_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<::rs_std::Tuple<::std::int32_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<::rs_std::Tuple<::std::int32_t>>);
inline rs_std::Tuple<::std::int32_t>::Tuple(
    std::tuple<::std::int32_t>&& tuple) noexcept {
  std::construct_at(reinterpret_cast<::std::int32_t*>(storage_ + 0),
                    std::move(std::get<0>(tuple)));
}
inline rs_std::Tuple<::std::int32_t>::operator std::tuple<
    ::std::int32_t>() && noexcept {
  return std::tuple<::std::int32_t>(
      std::move(*reinterpret_cast<::std::int32_t*>(storage_ + 0)));
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020intptr_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020intptr_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    rs_std::Tuple<::std::int8_t, ::std::intptr_t>* __ret_ptr);
}
inline ::rs_std::Tuple<::std::int8_t, ::std::intptr_t>::Tuple() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs_std::Tuple<::std::int8_t, ::std::intptr_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs_std::Tuple<::std::int8_t, ::std::intptr_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::std::int8_t, ::std::intptr_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::std::int8_t, ::std::intptr_t>>);
inline rs_std::Tuple<::std::int8_t, ::std::intptr_t>::Tuple(
    std::tuple<::std::int8_t, ::std::intptr_t>&& tuple) noexcept {
  std::construct_at(reinterpret_cast<::std::int8_t*>(storage_ + 0),
                    std::move(std::get<0>(tuple)));
  std::construct_at(reinterpret_cast<::std::intptr_t*>(storage_ + 8),
                    std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::int8_t, ::std::intptr_t>::operator std::tuple<
    ::std::int8_t, ::std::intptr_t>() && noexcept {
  return std::tuple<::std::int8_t, ::std::intptr_t>(
      std::move(*reinterpret_cast<::std::int8_t*>(storage_ + 0)),
      std::move(*reinterpret_cast<::std::intptr_t*>(storage_ + 8)));
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020intptr_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020intptr_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    rs_std::Tuple<::std::intptr_t, ::std::int8_t>* __ret_ptr);
}
inline ::rs_std::Tuple<::std::intptr_t, ::std::int8_t>::Tuple() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs_std::Tuple<::std::intptr_t, ::std::int8_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs_std::Tuple<::std::intptr_t, ::std::int8_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::std::intptr_t, ::std::int8_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::std::intptr_t, ::std::int8_t>>);
inline rs_std::Tuple<::std::intptr_t, ::std::int8_t>::Tuple(
    std::tuple<::std::intptr_t, ::std::int8_t>&& tuple) noexcept {
  std::construct_at(reinterpret_cast<::std::intptr_t*>(storage_ + 0),
                    std::move(std::get<0>(tuple)));
  std::construct_at(reinterpret_cast<::std::int8_t*>(storage_ + 8),
                    std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::intptr_t, ::std::int8_t>::operator std::tuple<
    ::std::intptr_t, ::std::int8_t>() && noexcept {
  return std::tuple<::std::intptr_t, ::std::int8_t>(
      std::move(*reinterpret_cast<::std::intptr_t*>(storage_ + 0)),
      std::move(*reinterpret_cast<::std::int8_t*>(storage_ + 8)));
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    rs_std::Tuple<
        rs_std::Option<::std::int32_t>,
        rs_std::Result<::std::int32_t, ::rs::alloc::string::String>> const&,
    rs_std::Tuple<rs_std::Option<::std::int32_t>,
                  rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>*
        __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(
    rs_std::Tuple<rs_std::Option<::std::int32_t>,
                  rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>&,
    rs_std::Tuple<
        rs_std::Option<::std::int32_t>,
        rs_std::Result<::std::int32_t, ::rs::alloc::string::String>> const&);
}
inline ::rs_std::Tuple<
    rs_std::Option<::std::int32_t>,
    rs_std::Result<::std::int32_t,
                   ::rs::alloc::string::String>>::Tuple(const Tuple& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline ::rs_std::Tuple<
    rs_std::Option<::std::int32_t>,
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>& ::
rs_std::Tuple<rs_std::Option<::std::int32_t>,
              rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>::
operator=(const Tuple& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
inline rs_std::Tuple<
    rs_std::Option<::std::int32_t>,
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>::
    Tuple(std::tuple<
          rs_std::Option<::std::int32_t>,
          rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>&&
              tuple) noexcept {
  std::construct_at(
      reinterpret_cast<rs_std::Option<::std::int32_t>*>(storage_ + 0),
      std::move(std::get<0>(tuple)));
  std::construct_at(
      reinterpret_cast<
          rs_std::Result<::std::int32_t, ::rs::alloc::string::String>*>(
          storage_ + 8),
      std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<
    rs_std::Option<::std::int32_t>,
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>::
operator std::tuple<
    rs_std::Option<::std::int32_t>,
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>() && noexcept {
  return std::tuple<
      rs_std::Option<::std::int32_t>,
      rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>(
      std::move(
          *reinterpret_cast<rs_std::Option<::std::int32_t>*>(storage_ + 0)),
      std::move(*reinterpret_cast<
                rs_std::Result<::std::int32_t, ::rs::alloc::string::String>*>(
          storage_ + 8)));
}
inline rs_std::Tuple<
    rs_std::Option<::std::int32_t>,
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>::~Tuple() {
  std::destroy_at(
      reinterpret_cast<rs_std::Option<::std::int32_t>*>(storage_ + 0));
  std::destroy_at(reinterpret_cast<
                  rs_std::Result<::std::int32_t, ::rs::alloc::string::String>*>(
      storage_ + 8));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    rs_std::Tuple<rs_std::Option<::std::int32_t>>* __ret_ptr);
}
inline ::rs_std::Tuple<rs_std::Option<::std::int32_t>>::Tuple() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs_std::Tuple<rs_std::Option<::std::int32_t>>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs_std::Tuple<rs_std::Option<::std::int32_t>>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<rs_std::Option<::std::int32_t>>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<rs_std::Option<::std::int32_t>>>);
inline rs_std::Tuple<rs_std::Option<::std::int32_t>>::Tuple(
    std::tuple<rs_std::Option<::std::int32_t>>&& tuple) noexcept {
  std::construct_at(
      reinterpret_cast<rs_std::Option<::std::int32_t>*>(storage_ + 0),
      std::move(std::get<0>(tuple)));
}
inline rs_std::Tuple<rs_std::Option<::std::int32_t>>::operator std::tuple<
    rs_std::Option<::std::int32_t>>() && noexcept {
  return std::tuple<rs_std::Option<::std::int32_t>>(std::move(
      *reinterpret_cast<rs_std::Option<::std::int32_t>*>(storage_ + 0)));
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t> const&,
    rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(
    rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t>&,
    rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t> const&);
}
inline ::rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t>::Tuple(
    const Tuple& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline ::rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t>& ::rs_std::
    Tuple<::tuples::CloneNoDefault, ::std::uint8_t>::operator=(
        const Tuple& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t>>);
inline rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t>::Tuple(
    std::tuple<::tuples::CloneNoDefault, ::std::uint8_t>&& tuple) noexcept {
  std::construct_at(reinterpret_cast<::tuples::CloneNoDefault*>(storage_ + 0),
                    std::move(std::get<0>(tuple)));
  std::construct_at(reinterpret_cast<::std::uint8_t*>(storage_ + 1),
                    std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t>::operator std::
    tuple<::tuples::CloneNoDefault, ::std::uint8_t>() && noexcept {
  return std::tuple<::tuples::CloneNoDefault, ::std::uint8_t>(
      std::move(*reinterpret_cast<::tuples::CloneNoDefault*>(storage_ + 0)),
      std::move(*reinterpret_cast<::std::uint8_t*>(storage_ + 1)));
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs_std::Tuple<::tuples::CopyNoDefault, ::std::uint8_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs_std::Tuple<::tuples::CopyNoDefault, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::tuples::CopyNoDefault, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::tuples::CopyNoDefault, ::std::uint8_t>>);
inline rs_std::Tuple<::tuples::CopyNoDefault, ::std::uint8_t>::Tuple(
    std::tuple<::tuples::CopyNoDefault, ::std::uint8_t>&& tuple) noexcept {
  std::construct_at(reinterpret_cast<::tuples::CopyNoDefault*>(storage_ + 0),
                    std::move(std::get<0>(tuple)));
  std::construct_at(reinterpret_cast<::std::uint8_t*>(storage_ + 1),
                    std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::tuples::CopyNoDefault, ::std::uint8_t>::operator std::
    tuple<::tuples::CopyNoDefault, ::std::uint8_t>() && noexcept {
  return std::tuple<::tuples::CopyNoDefault, ::std::uint8_t>(
      std::move(*reinterpret_cast<::tuples::CopyNoDefault*>(storage_ + 0)),
      std::move(*reinterpret_cast<::std::uint8_t*>(storage_ + 1)));
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t>* __ret_ptr);
}
inline ::rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t>::Tuple() {
  __crubit_internal::__crubit_thunk_default(this);
}
inline ::rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t>::Tuple(
    Tuple&& other)
    : Tuple() {
  *this = ::std::move(other);
}
inline ::rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t>& ::rs_std::Tuple<
    ::tuples::HasDefault, ::std::uint8_t>::operator=(Tuple&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t>::Tuple(
    std::tuple<::tuples::HasDefault, ::std::uint8_t>&& tuple) noexcept {
  std::construct_at(reinterpret_cast<::tuples::HasDefault*>(storage_ + 0),
                    std::move(std::get<0>(tuple)));
  std::construct_at(reinterpret_cast<::std::uint8_t*>(storage_ + 24),
                    std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t>::operator std::tuple<
    ::tuples::HasDefault, ::std::uint8_t>() && noexcept {
  return std::tuple<::tuples::HasDefault, ::std::uint8_t>(
      std::move(*reinterpret_cast<::tuples::HasDefault*>(storage_ + 0)),
      std::move(*reinterpret_cast<::std::uint8_t*>(storage_ + 24)));
}
inline rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t>::~Tuple() {
  std::destroy_at(reinterpret_cast<::tuples::HasDefault*>(storage_ + 0));
  std::destroy_at(reinterpret_cast<::std::uint8_t*>(storage_ + 24));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e

inline rs_std::Tuple<::tuples::HasNoDefault, ::std::uint8_t>::~Tuple() {
  std::destroy_at(reinterpret_cast<::tuples::HasNoDefault*>(storage_ + 0));
  std::destroy_at(reinterpret_cast<::std::uint8_t*>(storage_ + 24));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    rs_std::Tuple<
        ::std::uint32_t,
        rs_std::Tuple<::std::uint32_t,
                      rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>*
        __ret_ptr);
}
inline ::rs_std::Tuple<
    ::std::uint32_t,
    rs_std::Tuple<::std::uint32_t,
                  rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>::Tuple() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(
    ::std::is_trivially_copy_constructible_v<::rs_std::Tuple<
        ::std::uint32_t,
        rs_std::Tuple<::std::uint32_t,
                      rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::rs_std::Tuple<
        ::std::uint32_t,
        rs_std::Tuple<::std::uint32_t,
                      rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>>);
static_assert(
    ::std::is_trivially_move_constructible_v<::rs_std::Tuple<
        ::std::uint32_t,
        rs_std::Tuple<::std::uint32_t,
                      rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>>);
static_assert(
    ::std::is_trivially_move_assignable_v<::rs_std::Tuple<
        ::std::uint32_t,
        rs_std::Tuple<::std::uint32_t,
                      rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>>);
inline rs_std::Tuple<
    ::std::uint32_t,
    rs_std::Tuple<::std::uint32_t,
                  rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>::
    Tuple(std::tuple<
          ::std::uint32_t,
          rs_std::Tuple<::std::uint32_t,
                        rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>&&
              tuple) noexcept {
  std::construct_at(reinterpret_cast<::std::uint32_t*>(storage_ + 0),
                    std::move(std::get<0>(tuple)));
  std::construct_at(
      reinterpret_cast<rs_std::Tuple<
          ::std::uint32_t, rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>*>(
          storage_ + 4),
      std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<
    ::std::uint32_t,
    rs_std::Tuple<::std::uint32_t,
                  rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>::
operator std::tuple<
    ::std::uint32_t,
    rs_std::Tuple<
        ::std::uint32_t,
        rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>() && noexcept {
  return std::tuple<
      ::std::uint32_t,
      rs_std::Tuple<::std::uint32_t,
                    rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>(
      std::move(*reinterpret_cast<::std::uint32_t*>(storage_ + 0)),
      std::move(
          *reinterpret_cast<
              rs_std::Tuple<::std::uint32_t,
                            rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>*>(
              storage_ + 4)));
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    rs_std::Tuple<::std::uint32_t,
                  rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>* __ret_ptr);
}
inline ::rs_std::Tuple<
    ::std::uint32_t, rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>::Tuple() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(
    ::std::is_trivially_copy_constructible_v<::rs_std::Tuple<
        ::std::uint32_t, rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::rs_std::Tuple<
        ::std::uint32_t, rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>);
static_assert(
    ::std::is_trivially_move_constructible_v<::rs_std::Tuple<
        ::std::uint32_t, rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>);
static_assert(
    ::std::is_trivially_move_assignable_v<::rs_std::Tuple<
        ::std::uint32_t, rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>);
inline rs_std::Tuple<::std::uint32_t,
                     rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>::
    Tuple(std::tuple<::std::uint32_t,
                     rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>&&
              tuple) noexcept {
  std::construct_at(reinterpret_cast<::std::uint32_t*>(storage_ + 0),
                    std::move(std::get<0>(tuple)));
  std::construct_at(
      reinterpret_cast<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>*>(
          storage_ + 4),
      std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::uint32_t,
                     rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>::
operator std::tuple<
    ::std::uint32_t,
    rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>() && noexcept {
  return std::tuple<::std::uint32_t,
                    rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>(
      std::move(*reinterpret_cast<::std::uint32_t*>(storage_ + 0)),
      std::move(
          *reinterpret_cast<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>*>(
              storage_ + 4)));
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    rs_std::Tuple<::std::uint32_t, ::std::uint32_t>* __ret_ptr);
}
inline ::rs_std::Tuple<::std::uint32_t, ::std::uint32_t>::Tuple() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>);
inline rs_std::Tuple<::std::uint32_t, ::std::uint32_t>::Tuple(
    std::tuple<::std::uint32_t, ::std::uint32_t>&& tuple) noexcept {
  std::construct_at(reinterpret_cast<::std::uint32_t*>(storage_ + 0),
                    std::move(std::get<0>(tuple)));
  std::construct_at(reinterpret_cast<::std::uint32_t*>(storage_ + 4),
                    std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::uint32_t, ::std::uint32_t>::operator std::tuple<
    ::std::uint32_t, ::std::uint32_t>() && noexcept {
  return std::tuple<::std::uint32_t, ::std::uint32_t>(
      std::move(*reinterpret_cast<::std::uint32_t*>(storage_ + 0)),
      std::move(*reinterpret_cast<::std::uint32_t*>(storage_ + 4)));
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault> const&,
    rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(
    rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault>&,
    rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault> const&);
}
inline ::rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault>::Tuple(
    const Tuple& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline ::rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault>& ::rs_std::
    Tuple<::std::uint8_t, ::tuples::CloneNoDefault>::operator=(
        const Tuple& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault>>);
inline rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault>::Tuple(
    std::tuple<::std::uint8_t, ::tuples::CloneNoDefault>&& tuple) noexcept {
  std::construct_at(reinterpret_cast<::std::uint8_t*>(storage_ + 0),
                    std::move(std::get<0>(tuple)));
  std::construct_at(reinterpret_cast<::tuples::CloneNoDefault*>(storage_ + 1),
                    std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault>::operator std::
    tuple<::std::uint8_t, ::tuples::CloneNoDefault>() && noexcept {
  return std::tuple<::std::uint8_t, ::tuples::CloneNoDefault>(
      std::move(*reinterpret_cast<::std::uint8_t*>(storage_ + 0)),
      std::move(*reinterpret_cast<::tuples::CloneNoDefault*>(storage_ + 1)));
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs_std::Tuple<::std::uint8_t, ::tuples::CopyNoDefault>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs_std::Tuple<::std::uint8_t, ::tuples::CopyNoDefault>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::std::uint8_t, ::tuples::CopyNoDefault>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::std::uint8_t, ::tuples::CopyNoDefault>>);
inline rs_std::Tuple<::std::uint8_t, ::tuples::CopyNoDefault>::Tuple(
    std::tuple<::std::uint8_t, ::tuples::CopyNoDefault>&& tuple) noexcept {
  std::construct_at(reinterpret_cast<::std::uint8_t*>(storage_ + 0),
                    std::move(std::get<0>(tuple)));
  std::construct_at(reinterpret_cast<::tuples::CopyNoDefault*>(storage_ + 1),
                    std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::uint8_t, ::tuples::CopyNoDefault>::operator std::
    tuple<::std::uint8_t, ::tuples::CopyNoDefault>() && noexcept {
  return std::tuple<::std::uint8_t, ::tuples::CopyNoDefault>(
      std::move(*reinterpret_cast<::std::uint8_t*>(storage_ + 0)),
      std::move(*reinterpret_cast<::tuples::CopyNoDefault*>(storage_ + 1)));
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    rs_std::Tuple<::std::uint8_t, ::tuples::HasDefault>* __ret_ptr);
}
inline ::rs_std::Tuple<::std::uint8_t, ::tuples::HasDefault>::Tuple() {
  __crubit_internal::__crubit_thunk_default(this);
}
inline ::rs_std::Tuple<::std::uint8_t, ::tuples::HasDefault>::Tuple(
    Tuple&& other)
    : Tuple() {
  *this = ::std::move(other);
}
inline ::rs_std::Tuple<::std::uint8_t, ::tuples::HasDefault>& ::rs_std::Tuple<
    ::std::uint8_t, ::tuples::HasDefault>::operator=(Tuple&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline rs_std::Tuple<::std::uint8_t, ::tuples::HasDefault>::Tuple(
    std::tuple<::std::uint8_t, ::tuples::HasDefault>&& tuple) noexcept {
  std::construct_at(reinterpret_cast<::std::uint8_t*>(storage_ + 0),
                    std::move(std::get<0>(tuple)));
  std::construct_at(reinterpret_cast<::tuples::HasDefault*>(storage_ + 8),
                    std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::uint8_t, ::tuples::HasDefault>::operator std::tuple<
    ::std::uint8_t, ::tuples::HasDefault>() && noexcept {
  return std::tuple<::std::uint8_t, ::tuples::HasDefault>(
      std::move(*reinterpret_cast<::std::uint8_t*>(storage_ + 0)),
      std::move(*reinterpret_cast<::tuples::HasDefault*>(storage_ + 8)));
}
inline rs_std::Tuple<::std::uint8_t, ::tuples::HasDefault>::~Tuple() {
  std::destroy_at(reinterpret_cast<::std::uint8_t*>(storage_ + 0));
  std::destroy_at(reinterpret_cast<::tuples::HasDefault*>(storage_ + 8));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e

inline rs_std::Tuple<::std::uint8_t, ::tuples::HasNoDefault>::~Tuple() {
  std::destroy_at(reinterpret_cast<::std::uint8_t*>(storage_ + 0));
  std::destroy_at(reinterpret_cast<::tuples::HasNoDefault*>(storage_ + 8));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uintptr_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uintptr_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    rs_std::Tuple<::std::uint8_t, ::std::uintptr_t>* __ret_ptr);
}
inline ::rs_std::Tuple<::std::uint8_t, ::std::uintptr_t>::Tuple() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs_std::Tuple<::std::uint8_t, ::std::uintptr_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs_std::Tuple<::std::uint8_t, ::std::uintptr_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::std::uint8_t, ::std::uintptr_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::std::uint8_t, ::std::uintptr_t>>);
inline rs_std::Tuple<::std::uint8_t, ::std::uintptr_t>::Tuple(
    std::tuple<::std::uint8_t, ::std::uintptr_t>&& tuple) noexcept {
  std::construct_at(reinterpret_cast<::std::uint8_t*>(storage_ + 0),
                    std::move(std::get<0>(tuple)));
  std::construct_at(reinterpret_cast<::std::uintptr_t*>(storage_ + 8),
                    std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::uint8_t, ::std::uintptr_t>::operator std::tuple<
    ::std::uint8_t, ::std::uintptr_t>() && noexcept {
  return std::tuple<::std::uint8_t, ::std::uintptr_t>(
      std::move(*reinterpret_cast<::std::uint8_t*>(storage_ + 0)),
      std::move(*reinterpret_cast<::std::uintptr_t*>(storage_ + 8)));
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uintptr_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uintptr_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    rs_std::Tuple<::std::uintptr_t, ::std::uint8_t>* __ret_ptr);
}
inline ::rs_std::Tuple<::std::uintptr_t, ::std::uint8_t>::Tuple() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs_std::Tuple<::std::uintptr_t, ::std::uint8_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs_std::Tuple<::std::uintptr_t, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::std::uintptr_t, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::std::uintptr_t, ::std::uint8_t>>);
inline rs_std::Tuple<::std::uintptr_t, ::std::uint8_t>::Tuple(
    std::tuple<::std::uintptr_t, ::std::uint8_t>&& tuple) noexcept {
  std::construct_at(reinterpret_cast<::std::uintptr_t*>(storage_ + 0),
                    std::move(std::get<0>(tuple)));
  std::construct_at(reinterpret_cast<::std::uint8_t*>(storage_ + 8),
                    std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::uintptr_t, ::std::uint8_t>::operator std::tuple<
    ::std::uintptr_t, ::std::uint8_t>() && noexcept {
  return std::tuple<::std::uintptr_t, ::std::uint8_t>(
      std::move(*reinterpret_cast<::std::uintptr_t*>(storage_ + 0)),
      std::move(*reinterpret_cast<::std::uint8_t*>(storage_ + 8)));
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<rs_std::Option<::std::int32_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<rs_std::Option<::std::int32_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<rs_std::Option<::std::int32_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<rs_std::Option<::std::int32_t>>);
inline constexpr rs_std::Option<::std::int32_t>::Option() { set_tag(0); }
inline constexpr rs_std::Option<::std::int32_t>::Option(
    ::std::nullopt_t) noexcept {
  set_tag(0);
}
inline constexpr rs_std::Option<::std::int32_t>&
rs_std::Option<::std::int32_t>::operator=(::std::nullopt_t) noexcept {
  if (tag() != 0) {
    ::std::destroy_at(reinterpret_cast<::std::int32_t*>(storage_ + 4));
  }
  set_tag(0);
  return *this;
}
inline rs_std::Option<::std::int32_t>::Option(::std::int32_t&& value) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::std::int32_t*>(storage_ + 4),
                      ::std::move(value));
}
inline rs_std::Option<::std::int32_t>&
rs_std::Option<::std::int32_t>::operator=(::std::int32_t&& value) noexcept {
  if (tag() != 0) {
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::int32_t*>(storage_ + 4), ::std::move(value));
  } else {
    set_tag(1);
    ::std::construct_at(reinterpret_cast<::std::int32_t*>(storage_ + 4),
                        ::std::move(value));
  }
  return *this;
}
inline rs_std::Option<::std::int32_t>::Option(
    ::std::optional<::std::int32_t>&& value) noexcept {
  if (value.has_value()) {
    set_tag(1);
    ::std::int32_t* some = reinterpret_cast<::std::int32_t*>(storage_ + 4);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(0);
  }
}
inline rs_std::Option<::std::int32_t>&
rs_std::Option<::std::int32_t>::operator=(
    ::std::optional<::std::int32_t>&& value) noexcept {
  if (tag() != 0) {
    ::std::destroy_at(reinterpret_cast<::std::int32_t*>(storage_ + 4));
  }
  if (value.has_value()) {
    set_tag(1);
    ::std::int32_t* some = reinterpret_cast<::std::int32_t*>(storage_ + 4);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(0);
  }
  return *this;
}
template <typename... Args>
inline rs_std::Option<::std::int32_t>::Option(::std::in_place_t,
                                              Args&&... args) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::std::int32_t*>(storage_ + 4),
                      ::std::forward<Args>(args)...);
}
static_assert(
    ::std::is_trivially_destructible_v<rs_std::Option<::std::int32_t>>);
inline rs_std::Option<::std::int32_t>::operator ::std::optional<
    ::std::int32_t>() && noexcept {
  if (tag() == 0) {
    return ::std::nullopt;
  } else {
    ::std::int32_t& value = *reinterpret_cast<::std::int32_t*>(storage_ + 4);
    ::std::optional<::std::int32_t> return_value(::std::move(value));
    ::std::destroy_at(&value);
    set_tag(0);
    return return_value;
  }
}
inline bool rs_std::Option<::std::int32_t>::has_value() noexcept {
  return tag() != 0;
}
inline constexpr ::std::uint32_t rs_std::Option<::std::int32_t>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void rs_std::Option<::std::int32_t>::set_tag(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String> const&,
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>&,
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String> const&);
}
inline rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::Result(
    const Result& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline rs_std::Result<::std::int32_t, ::rs::alloc::string::String>&
rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::operator=(
    const Result& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
inline rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::Result(
    ::std::int32_t&& ok) noexcept {
  set_tag(UINT64_C(18446744073709551615));
  ::std::construct_at(reinterpret_cast<::std::int32_t*>(__storage + 8),
                      ::std::move(ok));
}
inline rs_std::Result<::std::int32_t, ::rs::alloc::string::String>&
rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::operator=(
    ::std::int32_t&& ok) noexcept {
  if (!has_value()) {
    ::std::destroy_at(
        reinterpret_cast<::rs::alloc::string::String*>(__storage));
    set_tag(UINT64_C(18446744073709551615));
    ::std::construct_at(reinterpret_cast<::std::int32_t*>(__storage + 8),
                        ::std::move(ok));
  } else {
    set_tag(UINT64_C(18446744073709551615));
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::int32_t*>(__storage + 8), ::std::move(ok));
  }
  return *this;
}

inline rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::Result(
    rs_std::unexpected<::rs::alloc::string::String>&& err) noexcept {
  ::std::construct_at(reinterpret_cast<::rs::alloc::string::String*>(__storage),
                      ::std::move(err.error()));
}
inline rs_std::Result<::std::int32_t, ::rs::alloc::string::String>&
rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::operator=(
    rs_std::unexpected<::rs::alloc::string::String>&& err) noexcept {
  if (has_value()) {
    ::std::destroy_at(__storage + 8);
    ::std::construct_at(
        reinterpret_cast<::rs::alloc::string::String*>(__storage),
        ::std::move(err.error()));
  } else {
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::rs::alloc::string::String*>(__storage),
        ::std::move(err.error()));
  }
  return *this;
}

template <typename... Args>
inline rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::Result(
    std::in_place_t, Args&&... args) {
  set_tag(UINT64_C(18446744073709551615));
  std::construct_at(__storage + 8, std::forward<Args>(args)...);
}
template <typename... Args>
inline rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::Result(
    rs_std::unexpect_t, Args&&... args) {
  std::construct_at(__storage, std::forward<Args>(args)...);
}
inline constexpr rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::
operator bool() const noexcept {
  return has_value();
}
inline constexpr bool rs_std::Result<
    ::std::int32_t, ::rs::alloc::string::String>::has_value() const noexcept {
  return tag() == UINT64_C(18446744073709551615);
}
inline ::std::int32_t&
rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::value() & {
  check_has_ok();
  return *reinterpret_cast<::std::int32_t*>(__storage + 8);
}
inline ::std::int32_t&&
rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::value() && {
  check_has_ok();
  return ::std::move(*reinterpret_cast<::std::int32_t*>(__storage + 8));
}
inline ::rs::alloc::string::String&
rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::err() & {
  check_has_err();
  return *reinterpret_cast<::rs::alloc::string::String*>(__storage);
}
inline ::rs::alloc::string::String&&
rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::err() && {
  check_has_err();
  return ::std::move(
      *reinterpret_cast<::rs::alloc::string::String*>(__storage));
}
inline rs_std::Result<::std::int32_t,
                      ::rs::alloc::string::String>::~Result() noexcept {
  if (has_value()) {
    ::std::destroy_at(reinterpret_cast<::std::int32_t*>(__storage + 8));
  } else {
    ::std::destroy_at(
        reinterpret_cast<::rs::alloc::string::String*>(__storage));
  }
}
inline constexpr ::std::uint64_t rs_std::Result<
    ::std::int32_t, ::rs::alloc::string::String>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void
rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint64_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

inline void
rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::check_has_ok() {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs_std::Result";
}
inline void
rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::check_has_err() {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs_std::Result";
}
#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TUPLES_TUPLES_GOLDEN
