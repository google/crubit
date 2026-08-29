// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// tuples_golden

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
#include "support/internal/memswap.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/movable.h"
#include "support/rs_std/option.h"
#include "support/rs_std/result.h"
#include "support/rs_std/str_ref.h"
#include "support/rs_std/tuple.h"

#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <memory>
#include <optional>
#include <tuple>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_alloc.h"

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
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
                       AdtHoldingFiveAndSix&& value);

 private:
  union {
    ::std::int32_t five;
  };
  union {
    ::std::int32_t six;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: CloneNoDefault") alignas(
    1) [[clang::trivial_abi]] CloneNoDefault final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::tuples::CloneNoDefault new_(::std::uint8_t val);

  ::std::uint8_t val = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: CopyNoDefault") alignas(1)
    [[clang::trivial_abi]] CopyNoDefault final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::tuples::CopyNoDefault new_(::std::uint8_t val);

  ::std::uint8_t val = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: HasDefault") alignas(8)
    [[clang::trivial_abi]] HasDefault final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::tuples::HasDefault new_(rs_std::StrRef val);

  // CRUBIT_ANNOTATE: must_bind=
  rs_std::StrRef val() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  ::rs::alloc::string::String val_ = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: HasNoDefault") alignas(8)
    [[clang::trivial_abi]] HasNoDefault final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  rs_std::StrRef val() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  ::rs::alloc::string::String val_ = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//  The same as NontrivialDrop, but without a C++ move operation. This can be
//  returned by value, even inside a tuple!
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: NonCppMovable") alignas(1)
    [[clang::trivial_abi]] NonCppMovable final {
 public:
  // Type is not a C++ aggregate: Type implements `Drop`

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
  NonCppMovable(::crubit::UnsafeRelocateTag, NonCppMovable&& value);

  union {
    ::std::uint8_t value;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
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
  NontrivialDrop(::crubit::UnsafeRelocateTag, NontrivialDrop&& value);

 private:
  union {
    ::std::uint8_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for constant `tuples_golden::TUPLE_CONSTANT`
// defined at
// cc_bindings_from_rs/test/tuples/tuples.rs;l=200:
// const of type `(i32,)` cannot be generated as only scalars, string
// references, and simple aggregate types are supported.

// CRUBIT_ANNOTATE: must_bind=
void assert_non_cpp_movable_drop_count(::std::uint8_t drop_count);

// CRUBIT_ANNOTATE: must_bind=
void assert_nontrivial_drop_count(::std::uint8_t drop_count);

// CRUBIT_ANNOTATE: must_bind=
void param_adt_in_tuple(::std::tuple<::tuples::AdtHoldingFiveAndSix> adt);

// CRUBIT_ANNOTATE: must_bind=
void param_c_abi_compatible_five_in_tuple(::std::tuple<::std::int32_t> five);

// CRUBIT_ANNOTATE: must_bind=
void param_ffi_alias_in_tuple(::std::tuple<::std::int8_t> five);

// CRUBIT_ANNOTATE: must_bind=
void param_nested_tuple_with_non_cpp_movable_at_2nd(
    ::std::tuple<
        ::std::int32_t,
        ::std::tuple<::std::int32_t, ::rs::Movable<::tuples::NonCppMovable>>>
        v);

// CRUBIT_ANNOTATE: must_bind=
void param_nested_tuples(
    ::std::tuple<::std::tuple<::std::int32_t, ::std::int32_t>, ::std::int32_t>
        v);

// CRUBIT_ANNOTATE: must_bind=
void param_non_cpp_movable_at_1st(
    ::std::tuple<::rs::Movable<::tuples::NonCppMovable>, ::std::int32_t,
                 ::std::int32_t>
        v);

// CRUBIT_ANNOTATE: must_bind=
void param_non_cpp_movable_at_2nd(
    ::std::tuple<::std::int32_t, ::rs::Movable<::tuples::NonCppMovable>,
                 ::std::int32_t>
        v);

// CRUBIT_ANNOTATE: must_bind=
void param_non_cpp_movable_at_3rd(
    ::std::tuple<::std::int32_t, ::std::int32_t,
                 ::rs::Movable<::tuples::NonCppMovable>>
        v);

// CRUBIT_ANNOTATE: must_bind=
void param_non_cpp_movable_in_tuple(
    ::std::tuple<::rs::Movable<::tuples::NonCppMovable>> v);

// CRUBIT_ANNOTATE: must_bind=
void param_non_cpp_movable_multi(
    ::std::tuple<::rs::Movable<::tuples::NonCppMovable>, ::std::int32_t,
                 ::rs::Movable<::tuples::NonCppMovable>>
        v);

// CRUBIT_ANNOTATE: must_bind=
void param_nontrivial_drop_in_tuple(
    ::std::tuple<::tuples::NontrivialDrop> nontrivial_drop);

// Error generating bindings for function `tuples_golden::param_option_in_tuple`
// defined at
// cc_bindings_from_rs/test/tuples/tuples.rs;l=370:
// Error handling parameter #0 of type `(std::option::Option<i32>,)`:
// crubit.rs/errors/bridge_compound_type: Tuples containing bridged type
// `std::option::Option<i32>` are not supported. Pass `std::option::Option<i32>`
// directly as a parameter or return value instead of inside a tuple.

// CRUBIT_ANNOTATE: must_bind=
void param_triply_nested_tuple(
    ::std::tuple<::std::tuple<::std::tuple<::std::int32_t>>> v);

// CRUBIT_ANNOTATE: must_bind=
void reset_non_cpp_movable_drop_count();

// CRUBIT_ANNOTATE: must_bind=
::std::tuple<::tuples::AdtHoldingFiveAndSix> return_adt_in_tuple();

// CRUBIT_ANNOTATE: must_bind=
::std::tuple<::std::int32_t> return_c_abi_compatible_five_in_tuple();

// CRUBIT_ANNOTATE: must_bind=
::std::tuple<::std::int8_t> return_ffi_alias_in_tuple();

// CRUBIT_ANNOTATE: must_bind=
::std::tuple<
    ::std::int32_t,
    ::std::tuple<::std::int32_t, ::rs::Movable<::tuples::NonCppMovable>>>
return_nested_tuple_with_non_cpp_movable_at_2nd();

// CRUBIT_ANNOTATE: must_bind=
::std::tuple<::std::tuple<::std::int32_t, ::std::int32_t>, ::std::int32_t>
return_nested_tuples();

// CRUBIT_ANNOTATE: must_bind=
::std::tuple<::std::tuple<::rs::Movable<::tuples::NonCppMovable>>>
return_new_non_cpp_movable_in_nested_tuple();

// CRUBIT_ANNOTATE: must_bind=
::std::tuple<::rs::Movable<::tuples::NonCppMovable>>
return_new_non_cpp_movable_in_tuple();

// CRUBIT_ANNOTATE: must_bind=
::std::tuple<::tuples::NontrivialDrop> return_new_nontrivial_drop_in_tuple();

// CRUBIT_ANNOTATE: must_bind=
::std::tuple<::rs::Movable<::tuples::NonCppMovable>, ::std::int32_t,
             ::std::int32_t>
return_non_cpp_movable_at_1st();

// CRUBIT_ANNOTATE: must_bind=
::std::tuple<::std::int32_t, ::rs::Movable<::tuples::NonCppMovable>,
             ::std::int32_t>
return_non_cpp_movable_at_2nd();

// CRUBIT_ANNOTATE: must_bind=
::std::tuple<::std::int32_t, ::std::int32_t,
             ::rs::Movable<::tuples::NonCppMovable>>
return_non_cpp_movable_at_3rd();

// CRUBIT_ANNOTATE: must_bind=
::std::tuple<::rs::Movable<::tuples::NonCppMovable>, ::std::int32_t,
             ::rs::Movable<::tuples::NonCppMovable>>
return_non_cpp_movable_multi();

// Error generating bindings for function
// `tuples_golden::return_option_in_tuple` defined at
// cc_bindings_from_rs/test/tuples/tuples.rs;l=366:
// Error formatting function return type `(std::option::Option<i32>,)`:
// crubit.rs/errors/bridge_compound_type: Tuples containing bridged type
// `std::option::Option<i32>` are not supported. Pass `std::option::Option<i32>`
// directly as a parameter or return value instead of inside a tuple.

// CRUBIT_ANNOTATE: must_bind=
::std::optional<::std::int32_t> return_option_in_tuple_ref(
    rs_std::Tuple<rs_std::Option<::std::int32_t>> const& opt);

// CRUBIT_ANNOTATE: must_bind=
::std::tuple<::std::tuple<::std::tuple<::std::int32_t>>>
return_triply_nested_tuple();

// CRUBIT_ANNOTATE: must_bind=
void return_unit_is_not_tuple();

// CRUBIT_ANNOTATE: must_bind=
::std::uint8_t take_tuple_clone_no_default_2(
    rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault> const& r);

// CRUBIT_ANNOTATE: must_bind=
::std::uint8_t take_tuple_copy_no_default_1(
    rs_std::Tuple<::tuples::CopyNoDefault, ::std::uint8_t> const& r);

// CRUBIT_ANNOTATE: must_bind=
rs_std::StrRef take_tuple_has_default(
    rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t> const* $(__anon1)
        crubit_nonnull r CRUBIT_LIFETIME_BOUND);

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

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value);
  Tuple(std::tuple<::std::int32_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::int32_t>() && noexcept;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 1, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 1, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 1, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 1, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: TupleStruct") alignas(4)
    [[clang::trivial_abi]] TupleStruct final {
 public:
  // Type is not a C++ aggregate: Field `empty_tuple_field` has a type
  // unsupported in C++

  // `tuples_golden::TupleStruct` doesn't implement the `Default` trait
  TupleStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~TupleStruct() = default;
  TupleStruct(TupleStruct&&) = default;
  TupleStruct& operator=(TupleStruct&&) = default;

  // `tuples_golden::TupleStruct` doesn't implement the `Clone` trait
  TupleStruct(const TupleStruct&) = delete;
  TupleStruct& operator=(const TupleStruct&) = delete;
  TupleStruct(::crubit::UnsafeRelocateTag, TupleStruct&& value);

  // Error generating bindings for associated function
  // `tuples_golden::TupleStruct::tuple_not_by_value` defined at
  // cc_bindings_from_rs/test/tuples/tuples.rs;l=195:
  // Error formatting function return type `*const ()`: Failed to format the
  // pointee of the pointer type `*const ()`: Tuple type `()` is not supported
  // in this context

  union {
    rs_std::Tuple<::std::int32_t> tuple_field;
  };
  // Field `empty_tuple_field` omitted: C++ does not support zero-sized types.
 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace tuples

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "(i8 , isize ,)") rs_std::Tuple<::std::int8_t, ::std::int64_t> {
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
  Tuple(std::tuple<::std::int8_t, ::std::int64_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::int8_t, ::std::int64_t>() && noexcept;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    ::std::int8_t __field0;
  };

 private:
  unsigned char __padding0[7];

 public:
  union {
    ::std::intptr_t __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "(isize , i8 ,)") rs_std::Tuple<::std::int64_t, ::std::int8_t> {
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
  Tuple(std::tuple<::std::int64_t, ::std::int8_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::int64_t, ::std::int8_t>() && noexcept;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    ::std::intptr_t __field0;
  };
  union {
    ::std::int8_t __field1;
  };

 private:
  unsigned char __padding1[7];

 private:
  static void __crubit_field_offset_assertions();
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

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value);
  Tuple(std::tuple<::tuples::CloneNoDefault, ::std::uint8_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::tuples::CloneNoDefault, ::std::uint8_t>() && noexcept;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    ::tuples::CloneNoDefault __field0;
  };
  union {
    ::std::uint8_t __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
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

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value);
  Tuple(std::tuple<::tuples::CopyNoDefault, ::std::uint8_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::tuples::CopyNoDefault, ::std::uint8_t>() && noexcept;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    ::tuples::CopyNoDefault __field0;
  };
  union {
    ::std::uint8_t __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};
#endif

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
  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value);
  Tuple(std::tuple<::tuples::HasDefault, ::std::uint8_t>&& tuple) noexcept;
  ~Tuple();
  operator std::tuple<::tuples::HasDefault, ::std::uint8_t>() && noexcept;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    ::tuples::HasDefault __field0;
  };
  union {
    ::std::uint8_t __field1;
  };

 private:
  unsigned char __padding1[7];

 private:
  static void __crubit_field_offset_assertions();
};
#endif

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
  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value);
  Tuple(std::tuple<::tuples::HasNoDefault, ::std::uint8_t>&& tuple) = delete;
  ~Tuple();
  operator std::tuple<::tuples::HasNoDefault, ::std::uint8_t>() && = delete;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    ::tuples::HasNoDefault __field0;
  };
  union {
    ::std::uint8_t __field1;
  };

 private:
  unsigned char __padding1[7];

 private:
  static void __crubit_field_offset_assertions();
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

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value);
  Tuple(std::tuple<::std::uint32_t, ::std::uint32_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::uint32_t, ::std::uint32_t>() && noexcept;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    ::std::uint32_t __field0;
  };
  union {
    ::std::uint32_t __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: GetsTuple") alignas(4)
    [[clang::trivial_abi]] GetsTuple final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::tuples::GetsTuple new_(::std::uint32_t val);

  rs_std::Tuple<::std::uint32_t, ::std::uint32_t> value = {};

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

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value);
  Tuple(std::tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                   ::std::uint32_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                      ::std::uint32_t>() && noexcept;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    rs_std::Tuple<::std::uint32_t, ::std::uint32_t> __field0;
  };
  union {
    ::std::uint32_t __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
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

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value);
  Tuple(
      std::tuple<rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                               ::std::uint32_t>,
                 ::std::uint32_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<
      rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                    ::std::uint32_t>,
      ::std::uint32_t>() && noexcept;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                  ::std::uint32_t>
        __field0;
  };
  union {
    ::std::uint32_t __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
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

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value);
  Tuple(std::tuple<::std::uint32_t,
                   rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>&&
            tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<
      ::std::uint32_t,
      rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>() && noexcept;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    ::std::uint32_t __field0;
  };
  union {
    rs_std::Tuple<::std::uint32_t, ::std::uint32_t> __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuples_golden :: NestedTupleIntermediate1") alignas(4)
    [[clang::trivial_abi]] NestedTupleIntermediate1 final {
 public:
  rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                ::std::uint32_t>
      v1 = {};
  rs_std::Tuple<::std::uint32_t,
                rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>
      v2 = {};

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

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value);
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
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    ::std::uint32_t __field0;
  };
  union {
    rs_std::Tuple<::std::uint32_t,
                  rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>
        __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuples_golden :: NestedTupleIntermediate2") alignas(4)
    [[clang::trivial_abi]] NestedTupleIntermediate2 final {
 public:
  rs_std::Tuple<rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                              ::std::uint32_t>,
                ::std::uint32_t>
      v1 = {};
  rs_std::Tuple<::std::uint32_t,
                rs_std::Tuple<::std::uint32_t,
                              rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>
      v2 = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuples_golden :: NestedTupleStruct") alignas(4) [[clang::trivial_abi]]
NestedTupleStruct final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::tuples::NestedTupleStruct new_(::std::uint32_t val);

  rs_std::Tuple<rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                              ::std::uint32_t>,
                ::std::uint32_t>
      in_tuple1 = {};
  rs_std::Tuple<::std::uint32_t,
                rs_std::Tuple<::std::uint32_t,
                              rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>
      in_tuple2 = {};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace tuples

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "(u64 , u64 ,)") rs_std::Tuple<::std::uint64_t, ::std::uint64_t> {
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
  Tuple(std::tuple<::std::uint64_t, ::std::uint64_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::uint64_t, ::std::uint64_t>() && noexcept;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    ::std::uint64_t __field0;
  };
  union {
    ::std::uint64_t __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};
#endif

namespace tuples {

struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: TuplesWithU64") alignas(8)
    [[clang::trivial_abi]] TuplesWithU64 final {
 public:
  rs_std::Tuple<::std::uint64_t, ::std::uint64_t> u64_in_tuple1 = {};

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

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value);
  Tuple(std::tuple<::std::uint8_t, ::tuples::CloneNoDefault>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::uint8_t, ::tuples::CloneNoDefault>() && noexcept;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    ::std::uint8_t __field0;
  };
  union {
    ::tuples::CloneNoDefault __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuples_golden :: CloneNoDefaultTuple") alignas(1) [[clang::trivial_abi]]
CloneNoDefaultTuple final {
 public:
  // Type is not a C++ aggregate: Field `in_tuple1` is not default-constructible
  // in C++

  // `tuples_golden::CloneNoDefaultTuple` doesn't implement the `Default` trait
  CloneNoDefaultTuple() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~CloneNoDefaultTuple() = default;
  CloneNoDefaultTuple(CloneNoDefaultTuple&&) = default;
  CloneNoDefaultTuple& operator=(CloneNoDefaultTuple&&) = default;

  // `tuples_golden::CloneNoDefaultTuple` doesn't implement the `Clone` trait
  CloneNoDefaultTuple(const CloneNoDefaultTuple&) = delete;
  CloneNoDefaultTuple& operator=(const CloneNoDefaultTuple&) = delete;
  CloneNoDefaultTuple(::crubit::UnsafeRelocateTag, CloneNoDefaultTuple&& value);

  // CRUBIT_ANNOTATE: must_bind=
  static ::tuples::CloneNoDefaultTuple new_(::std::uint8_t val);

  union {
    rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t> in_tuple1;
  };
  union {
    rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault> in_tuple2;
  };

 private:
  static void __crubit_field_offset_assertions();
};

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

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value);
  Tuple(std::tuple<::std::uint8_t, ::tuples::CopyNoDefault>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::uint8_t, ::tuples::CopyNoDefault>() && noexcept;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    ::std::uint8_t __field0;
  };
  union {
    ::tuples::CopyNoDefault __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuples_golden :: CopyNoDefaultTuple") alignas(1) [[clang::trivial_abi]]
CopyNoDefaultTuple final {
 public:
  // Type is not a C++ aggregate: Field `in_tuple1` is not default-constructible
  // in C++

  // `tuples_golden::CopyNoDefaultTuple` doesn't implement the `Default` trait
  CopyNoDefaultTuple() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~CopyNoDefaultTuple() = default;
  CopyNoDefaultTuple(CopyNoDefaultTuple&&) = default;
  CopyNoDefaultTuple& operator=(CopyNoDefaultTuple&&) = default;

  // `tuples_golden::CopyNoDefaultTuple` doesn't implement the `Clone` trait
  CopyNoDefaultTuple(const CopyNoDefaultTuple&) = delete;
  CopyNoDefaultTuple& operator=(const CopyNoDefaultTuple&) = delete;
  CopyNoDefaultTuple(::crubit::UnsafeRelocateTag, CopyNoDefaultTuple&& value);

  // CRUBIT_ANNOTATE: must_bind=
  static ::tuples::CopyNoDefaultTuple new_(::std::uint8_t val);

  union {
    rs_std::Tuple<::tuples::CopyNoDefault, ::std::uint8_t> in_tuple1;
  };
  union {
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
  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value);
  Tuple(std::tuple<::std::uint8_t, ::tuples::HasDefault>&& tuple) noexcept;
  ~Tuple();
  operator std::tuple<::std::uint8_t, ::tuples::HasDefault>() && noexcept;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    ::std::uint8_t __field0;
  };

 private:
  unsigned char __padding0[7];

 public:
  union {
    ::tuples::HasDefault __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: HasDefaultTuple") alignas(
    8) [[clang::trivial_abi]] HasDefaultTuple final {
 public:
  // Type is not a C++ aggregate: Multiple fields require drop glue (annotate
  // with `#[crubit_annotate::field_drop_order_does_not_matter]` if field drop
  // order does not matter)

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
  HasDefaultTuple(::crubit::UnsafeRelocateTag, HasDefaultTuple&& value);

  // CRUBIT_ANNOTATE: must_bind=
  static ::tuples::HasDefaultTuple new_(rs_std::StrRef val);

  union {
    rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t> in_tuple1;
  };
  union {
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
  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value);
  Tuple(std::tuple<::std::uint8_t, ::tuples::HasNoDefault>&& tuple) = delete;
  ~Tuple();
  operator std::tuple<::std::uint8_t, ::tuples::HasNoDefault>() && = delete;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    ::std::uint8_t __field0;
  };

 private:
  unsigned char __padding0[7];

 public:
  union {
    ::tuples::HasNoDefault __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuples_golden :: HasNoDefaultTuple") alignas(8) [[clang::trivial_abi]]
HasNoDefaultTuple final {
 public:
  // Type is not a C++ aggregate: Field `in_tuple1` is not default-constructible
  // in C++

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
  HasNoDefaultTuple(::crubit::UnsafeRelocateTag, HasNoDefaultTuple&& value);

  // CRUBIT_ANNOTATE: must_bind=
  static ::tuples::HasNoDefaultTuple new_(rs_std::StrRef val);

  union {
    rs_std::Tuple<::tuples::HasNoDefault, ::std::uint8_t> in_tuple1;
  };
  union {
    rs_std::Tuple<::std::uint8_t, ::tuples::HasNoDefault> in_tuple2;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace tuples

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "(u8 , usize ,)") rs_std::Tuple<::std::uint8_t, ::std::uint64_t> {
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
  Tuple(std::tuple<::std::uint8_t, ::std::uint64_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::uint8_t, ::std::uint64_t>() && noexcept;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    ::std::uint8_t __field0;
  };

 private:
  unsigned char __padding0[7];

 public:
  union {
    ::std::uintptr_t __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "(usize , u8 ,)") rs_std::Tuple<::std::uint64_t, ::std::uint8_t> {
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
  Tuple(std::tuple<::std::uint64_t, ::std::uint8_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::uint64_t, ::std::uint8_t>() && noexcept;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    ::std::uintptr_t __field0;
  };
  union {
    ::std::uint8_t __field1;
  };

 private:
  unsigned char __padding1[7];

 private:
  static void __crubit_field_offset_assertions();
};
#endif

namespace tuples {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuples_golden :: TupleWithSizeTypes") alignas(8) [[clang::trivial_abi]]
TupleWithSizeTypes final {
 public:
  rs_std::Tuple<::std::uint64_t, ::std::uint8_t> uval_in_tuple1 = {};
  rs_std::Tuple<::std::uint8_t, ::std::uint64_t> uval_in_tuple2 = {};
  rs_std::Tuple<::std::int64_t, ::std::int8_t> ival_in_tuple1 = {};
  rs_std::Tuple<::std::int8_t, ::std::int64_t> ival_in_tuple2 = {};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace tuples

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < i32 >") rs_std::Option<::std::int32_t>
    : public rs_std::OptionBase<rs_std::Option<::std::int32_t>,
                                ::std::int32_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      rs_std::OptionBase<rs_std::Option<::std::int32_t>, ::std::int32_t>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option, ::std::int32_t, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option, ::std::int32_t, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::std::int32_t, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::std::int32_t, Opt>)
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint32_t;
  static constexpr tag_type kNoneVal = 0;
  ::std::int32_t* some_ptr() noexcept {
    return reinterpret_cast<::std::int32_t*>(storage_ + 4);
  }
  ::std::int32_t const* some_const_ptr() const noexcept {
    return reinterpret_cast<::std::int32_t const*>(storage_ + 4);
  }
  void set_some_tag() noexcept { set_tag(1); }
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
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

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value);
  Tuple(std::tuple<rs_std::Option<::std::int32_t>>&& tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<rs_std::Option<::std::int32_t>>() && noexcept;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 1, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 1, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 1, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 1, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    rs_std::Option<::std::int32_t> __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < i32 , :: alloc :: string :: String >")
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>
    : public rs_std::ResultBase<
          rs_std::Result<::std::int32_t, ::rs::alloc::string::String>,
          ::std::int32_t, ::rs::alloc::string::String> {
 public:
  // Clone::clone
  Result(const Result&);

  // Clone::clone_from
  rs_std::Result<::std::int32_t, ::rs::alloc::string::String>& operator=(
      const Result&);

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type = rs_std::ResultBase<
      rs_std::Result<::std::int32_t, ::rs::alloc::string::String>,
      ::std::int32_t, ::rs::alloc::string::String>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::int32_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::int32_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(
        rs_std::ResultUnexpectedConstructible<::rs::alloc::string::String, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(
        rs_std::ResultUnexpectedConstructible<::rs::alloc::string::String, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept;

 private:
  friend base_type;
  bool has_value_impl() const noexcept {
    return tag() == UINT64_C(18446744073709551615);
  }
  ::std::int32_t* ok_ptr() noexcept {
    return reinterpret_cast<::std::int32_t*>(__storage + 8);
  }
  ::std::int32_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::std::int32_t const*>(__storage + 8);
  }
  ::rs::alloc::string::String* err_ptr() noexcept {
    return reinterpret_cast<::rs::alloc::string::String*>(__storage);
  }
  ::rs::alloc::string::String const* err_const_ptr() const noexcept {
    return reinterpret_cast<::rs::alloc::string::String const*>(__storage);
  }
  void set_ok_tag() noexcept { set_tag(UINT64_C(18446744073709551615)); }
  void set_err_tag() noexcept {}
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

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

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value);
  Tuple(
      std::tuple<rs_std::Option<::std::int32_t>,
                 rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>&&
          tuple) noexcept;
  ~Tuple();
  operator std::tuple<
      rs_std::Option<::std::int32_t>,
      rs_std::Result<::std::int32_t,
                     ::rs::alloc::string::String>>() && noexcept;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    rs_std::Option<::std::int32_t> __field0;
  };
  union {
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String> __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};
#endif

namespace tuples {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuples_golden :: StructWithOptionTuple") alignas(8)
    [[clang::trivial_abi]] StructWithOptionTuple final {
 public:
  // Type is not a C++ aggregate: Field `opt_tuple` is not default-constructible
  // in C++

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
                        StructWithOptionTuple&& value);

  // CRUBIT_ANNOTATE: must_bind=
  static ::tuples::StructWithOptionTuple new_(::std::int32_t val);

  union {
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
inline ::tuples::AdtHoldingFiveAndSix::AdtHoldingFiveAndSix(
    ::crubit::UnsafeRelocateTag, AdtHoldingFiveAndSix&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
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
inline ::tuples::CloneNoDefaultTuple::CloneNoDefaultTuple(
    ::crubit::UnsafeRelocateTag, CloneNoDefaultTuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

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
inline ::tuples::CopyNoDefaultTuple::CopyNoDefaultTuple(
    ::crubit::UnsafeRelocateTag, CopyNoDefaultTuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

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
extern "C" void
__crubit_thunk_Drop_udrop_utuples_ugolden_x0000003a_x0000003aHasDefaultTuple(
    ::tuples::HasDefaultTuple&);
}
inline HasDefaultTuple::~HasDefaultTuple() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_utuples_ugolden_x0000003a_x0000003aHasDefaultTuple(
          *this);
}
inline ::tuples::HasDefaultTuple::HasDefaultTuple(::crubit::UnsafeRelocateTag,
                                                  HasDefaultTuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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
extern "C" void
__crubit_thunk_Drop_udrop_utuples_ugolden_x0000003a_x0000003aHasNoDefaultTuple(
    ::tuples::HasNoDefaultTuple&);
}
inline HasNoDefaultTuple::~HasNoDefaultTuple() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_utuples_ugolden_x0000003a_x0000003aHasNoDefaultTuple(
          *this);
}
inline ::tuples::HasNoDefaultTuple::HasNoDefaultTuple(
    ::crubit::UnsafeRelocateTag, HasNoDefaultTuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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
extern "C" void
__crubit_thunk_Drop_udrop_utuples_ugolden_x0000003a_x0000003aNonCppMovable(
    ::tuples::NonCppMovable&);
}
inline NonCppMovable::~NonCppMovable() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_utuples_ugolden_x0000003a_x0000003aNonCppMovable(
          *this);
}
inline ::tuples::NonCppMovable::NonCppMovable(::crubit::UnsafeRelocateTag,
                                              NonCppMovable&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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
extern "C" void
__crubit_thunk_Default_udefault_utuples_ugolden_x0000003a_x0000003aNontrivialDrop(
    ::tuples::NontrivialDrop* __ret_ptr);
}
inline ::tuples::NontrivialDrop::NontrivialDrop() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_utuples_ugolden_x0000003a_x0000003aNontrivialDrop(
          this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_utuples_ugolden_x0000003a_x0000003aNontrivialDrop(
    ::tuples::NontrivialDrop&);
}
inline NontrivialDrop::~NontrivialDrop() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_utuples_ugolden_x0000003a_x0000003aNontrivialDrop(
          *this);
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
inline ::tuples::NontrivialDrop::NontrivialDrop(::crubit::UnsafeRelocateTag,
                                                NontrivialDrop&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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
extern "C" void
__crubit_thunk_Drop_udrop_utuples_ugolden_x0000003a_x0000003aStructWithOptionTuple(
    ::tuples::StructWithOptionTuple&);
}
inline StructWithOptionTuple::~StructWithOptionTuple() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_utuples_ugolden_x0000003a_x0000003aStructWithOptionTuple(
          *this);
}
inline ::tuples::StructWithOptionTuple::StructWithOptionTuple(
    ::crubit::UnsafeRelocateTag, StructWithOptionTuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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
inline ::tuples::TupleStruct::TupleStruct(::crubit::UnsafeRelocateTag,
                                          TupleStruct&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
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
static_assert(
    sizeof(TuplesWithU64) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TuplesWithU64) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<TuplesWithU64>);
static_assert(
    ::std::is_trivially_move_constructible_v<::tuples::TuplesWithU64>);
static_assert(::std::is_trivially_move_assignable_v<::tuples::TuplesWithU64>);
inline void TuplesWithU64::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TuplesWithU64, u64_in_tuple1));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_assert_unon_ucpp_umovable_udrop_ucount(
    ::std::uint8_t);
}
inline void assert_non_cpp_movable_drop_count(::std::uint8_t drop_count) {
  return __crubit_internal::
      __crubit_thunk_assert_unon_ucpp_umovable_udrop_ucount(drop_count);
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
extern "C" void
__crubit_thunk_param_unested_utuple_uwith_unon_ucpp_umovable_uat_u2nd(void**);
}
inline void param_nested_tuple_with_non_cpp_movable_at_2nd(
    ::std::tuple<
        ::std::int32_t,
        ::std::tuple<::std::int32_t, ::rs::Movable<::tuples::NonCppMovable>>>
        v) {
  auto&& v_0 = ::std::get<0>(v);
  auto&& v_cabi_0 = v_0;
  auto&& v_1 = ::std::get<1>(v);
  auto&& v_1_0 = ::std::get<0>(v_1);
  auto&& v_1_cabi_0 = v_1_0;
  auto&& v_1_1 = ::std::get<1>(v_1);
  crubit::Slot<::tuples::NonCppMovable> v_1_1_slot;
  ::std::move(v_1_1).MoveToSlot(v_1_1_slot);
  auto&& v_1_cabi_1 = v_1_1_slot.Get();
  void* v_1_cabi[] = {&v_1_cabi_0, &v_1_cabi_1};
  auto* v_cabi_1 = &v_1_cabi;
  void* v_cabi[] = {&v_cabi_0, &v_cabi_1};
  return __crubit_internal::
      __crubit_thunk_param_unested_utuple_uwith_unon_ucpp_umovable_uat_u2nd(
          v_cabi);
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
extern "C" void __crubit_thunk_param_unon_ucpp_umovable_uat_u1st(void**);
}
inline void param_non_cpp_movable_at_1st(
    ::std::tuple<::rs::Movable<::tuples::NonCppMovable>, ::std::int32_t,
                 ::std::int32_t>
        v) {
  auto&& v_0 = ::std::get<0>(v);
  crubit::Slot<::tuples::NonCppMovable> v_0_slot;
  ::std::move(v_0).MoveToSlot(v_0_slot);
  auto&& v_cabi_0 = v_0_slot.Get();
  auto&& v_1 = ::std::get<1>(v);
  auto&& v_cabi_1 = v_1;
  auto&& v_2 = ::std::get<2>(v);
  auto&& v_cabi_2 = v_2;
  void* v_cabi[] = {&v_cabi_0, &v_cabi_1, &v_cabi_2};
  return __crubit_internal::__crubit_thunk_param_unon_ucpp_umovable_uat_u1st(
      v_cabi);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_param_unon_ucpp_umovable_uat_u2nd(void**);
}
inline void param_non_cpp_movable_at_2nd(
    ::std::tuple<::std::int32_t, ::rs::Movable<::tuples::NonCppMovable>,
                 ::std::int32_t>
        v) {
  auto&& v_0 = ::std::get<0>(v);
  auto&& v_cabi_0 = v_0;
  auto&& v_1 = ::std::get<1>(v);
  crubit::Slot<::tuples::NonCppMovable> v_1_slot;
  ::std::move(v_1).MoveToSlot(v_1_slot);
  auto&& v_cabi_1 = v_1_slot.Get();
  auto&& v_2 = ::std::get<2>(v);
  auto&& v_cabi_2 = v_2;
  void* v_cabi[] = {&v_cabi_0, &v_cabi_1, &v_cabi_2};
  return __crubit_internal::__crubit_thunk_param_unon_ucpp_umovable_uat_u2nd(
      v_cabi);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_param_unon_ucpp_umovable_uat_u3rd(void**);
}
inline void param_non_cpp_movable_at_3rd(
    ::std::tuple<::std::int32_t, ::std::int32_t,
                 ::rs::Movable<::tuples::NonCppMovable>>
        v) {
  auto&& v_0 = ::std::get<0>(v);
  auto&& v_cabi_0 = v_0;
  auto&& v_1 = ::std::get<1>(v);
  auto&& v_cabi_1 = v_1;
  auto&& v_2 = ::std::get<2>(v);
  crubit::Slot<::tuples::NonCppMovable> v_2_slot;
  ::std::move(v_2).MoveToSlot(v_2_slot);
  auto&& v_cabi_2 = v_2_slot.Get();
  void* v_cabi[] = {&v_cabi_0, &v_cabi_1, &v_cabi_2};
  return __crubit_internal::__crubit_thunk_param_unon_ucpp_umovable_uat_u3rd(
      v_cabi);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_param_unon_ucpp_umovable_uin_utuple(void**);
}
inline void param_non_cpp_movable_in_tuple(
    ::std::tuple<::rs::Movable<::tuples::NonCppMovable>> v) {
  auto&& v_0 = ::std::get<0>(v);
  crubit::Slot<::tuples::NonCppMovable> v_0_slot;
  ::std::move(v_0).MoveToSlot(v_0_slot);
  auto&& v_cabi_0 = v_0_slot.Get();
  void* v_cabi[] = {&v_cabi_0};
  return __crubit_internal::__crubit_thunk_param_unon_ucpp_umovable_uin_utuple(
      v_cabi);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_param_unon_ucpp_umovable_umulti(void**);
}
inline void param_non_cpp_movable_multi(
    ::std::tuple<::rs::Movable<::tuples::NonCppMovable>, ::std::int32_t,
                 ::rs::Movable<::tuples::NonCppMovable>>
        v) {
  auto&& v_0 = ::std::get<0>(v);
  crubit::Slot<::tuples::NonCppMovable> v_0_slot;
  ::std::move(v_0).MoveToSlot(v_0_slot);
  auto&& v_cabi_0 = v_0_slot.Get();
  auto&& v_1 = ::std::get<1>(v);
  auto&& v_cabi_1 = v_1;
  auto&& v_2 = ::std::get<2>(v);
  crubit::Slot<::tuples::NonCppMovable> v_2_slot;
  ::std::move(v_2).MoveToSlot(v_2_slot);
  auto&& v_cabi_2 = v_2_slot.Get();
  void* v_cabi[] = {&v_cabi_0, &v_cabi_1, &v_cabi_2};
  return __crubit_internal::__crubit_thunk_param_unon_ucpp_umovable_umulti(
      v_cabi);
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
extern "C" void __crubit_thunk_reset_unon_ucpp_umovable_udrop_ucount();
}
inline void reset_non_cpp_movable_drop_count() {
  return __crubit_internal::
      __crubit_thunk_reset_unon_ucpp_umovable_udrop_ucount();
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
extern "C" void
__crubit_thunk_return_unested_utuple_uwith_unon_ucpp_umovable_uat_u2nd(
    void** __ret_ptr);
}
inline ::std::tuple<
    ::std::int32_t,
    ::std::tuple<::std::int32_t, ::rs::Movable<::tuples::NonCppMovable>>>
return_nested_tuple_with_non_cpp_movable_at_2nd() {
  ::std::int32_t __return_value_0_ret_val_holder;
  ::std::int32_t* __return_value_0_storage = &__return_value_0_ret_val_holder;
  ::std::int32_t __return_value_1_0_ret_val_holder;
  ::std::int32_t* __return_value_1_0_storage =
      &__return_value_1_0_ret_val_holder;
  crubit::Slot<::tuples::NonCppMovable> __return_value_1_1_ret_val_holder;
  auto* __return_value_1_1_storage = __return_value_1_1_ret_val_holder.Get();
  void* __return_value_1_storage[] = {__return_value_1_0_storage,
                                      __return_value_1_1_storage};
  void* __return_value_storage[] = {__return_value_0_storage,
                                    __return_value_1_storage};
  __crubit_internal::
      __crubit_thunk_return_unested_utuple_uwith_unon_ucpp_umovable_uat_u2nd(
          __return_value_storage);
  return ::std::make_tuple(
      *__return_value_0_storage,
      ::std::make_tuple(*__return_value_1_0_storage,
                        ::rs::Movable<::tuples::NonCppMovable>::TakeFromSlot(
                            ::std::move(__return_value_1_1_ret_val_holder))));
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
extern "C" void
__crubit_thunk_return_unew_unon_ucpp_umovable_uin_unested_utuple(
    void** __ret_ptr);
}
inline ::std::tuple<::std::tuple<::rs::Movable<::tuples::NonCppMovable>>>
return_new_non_cpp_movable_in_nested_tuple() {
  crubit::Slot<::tuples::NonCppMovable> __return_value_0_0_ret_val_holder;
  auto* __return_value_0_0_storage = __return_value_0_0_ret_val_holder.Get();
  void* __return_value_0_storage[] = {__return_value_0_0_storage};
  void* __return_value_storage[] = {__return_value_0_storage};
  __crubit_internal::
      __crubit_thunk_return_unew_unon_ucpp_umovable_uin_unested_utuple(
          __return_value_storage);
  return ::std::make_tuple(
      ::std::make_tuple(::rs::Movable<::tuples::NonCppMovable>::TakeFromSlot(
          ::std::move(__return_value_0_0_ret_val_holder))));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_unew_unon_ucpp_umovable_uin_utuple(
    void** __ret_ptr);
}
inline ::std::tuple<::rs::Movable<::tuples::NonCppMovable>>
return_new_non_cpp_movable_in_tuple() {
  crubit::Slot<::tuples::NonCppMovable> __return_value_0_ret_val_holder;
  auto* __return_value_0_storage = __return_value_0_ret_val_holder.Get();
  void* __return_value_storage[] = {__return_value_0_storage};
  __crubit_internal::__crubit_thunk_return_unew_unon_ucpp_umovable_uin_utuple(
      __return_value_storage);
  return ::std::make_tuple(::rs::Movable<::tuples::NonCppMovable>::TakeFromSlot(
      ::std::move(__return_value_0_ret_val_holder)));
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
extern "C" void __crubit_thunk_return_unon_ucpp_umovable_uat_u1st(
    void** __ret_ptr);
}
inline ::std::tuple<::rs::Movable<::tuples::NonCppMovable>, ::std::int32_t,
                    ::std::int32_t>
return_non_cpp_movable_at_1st() {
  crubit::Slot<::tuples::NonCppMovable> __return_value_0_ret_val_holder;
  auto* __return_value_0_storage = __return_value_0_ret_val_holder.Get();
  ::std::int32_t __return_value_1_ret_val_holder;
  ::std::int32_t* __return_value_1_storage = &__return_value_1_ret_val_holder;
  ::std::int32_t __return_value_2_ret_val_holder;
  ::std::int32_t* __return_value_2_storage = &__return_value_2_ret_val_holder;
  void* __return_value_storage[] = {__return_value_0_storage,
                                    __return_value_1_storage,
                                    __return_value_2_storage};
  __crubit_internal::__crubit_thunk_return_unon_ucpp_umovable_uat_u1st(
      __return_value_storage);
  return ::std::make_tuple(::rs::Movable<::tuples::NonCppMovable>::TakeFromSlot(
                               ::std::move(__return_value_0_ret_val_holder)),
                           *__return_value_1_storage,
                           *__return_value_2_storage);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_unon_ucpp_umovable_uat_u2nd(
    void** __ret_ptr);
}
inline ::std::tuple<::std::int32_t, ::rs::Movable<::tuples::NonCppMovable>,
                    ::std::int32_t>
return_non_cpp_movable_at_2nd() {
  ::std::int32_t __return_value_0_ret_val_holder;
  ::std::int32_t* __return_value_0_storage = &__return_value_0_ret_val_holder;
  crubit::Slot<::tuples::NonCppMovable> __return_value_1_ret_val_holder;
  auto* __return_value_1_storage = __return_value_1_ret_val_holder.Get();
  ::std::int32_t __return_value_2_ret_val_holder;
  ::std::int32_t* __return_value_2_storage = &__return_value_2_ret_val_holder;
  void* __return_value_storage[] = {__return_value_0_storage,
                                    __return_value_1_storage,
                                    __return_value_2_storage};
  __crubit_internal::__crubit_thunk_return_unon_ucpp_umovable_uat_u2nd(
      __return_value_storage);
  return ::std::make_tuple(*__return_value_0_storage,
                           ::rs::Movable<::tuples::NonCppMovable>::TakeFromSlot(
                               ::std::move(__return_value_1_ret_val_holder)),
                           *__return_value_2_storage);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_unon_ucpp_umovable_uat_u3rd(
    void** __ret_ptr);
}
inline ::std::tuple<::std::int32_t, ::std::int32_t,
                    ::rs::Movable<::tuples::NonCppMovable>>
return_non_cpp_movable_at_3rd() {
  ::std::int32_t __return_value_0_ret_val_holder;
  ::std::int32_t* __return_value_0_storage = &__return_value_0_ret_val_holder;
  ::std::int32_t __return_value_1_ret_val_holder;
  ::std::int32_t* __return_value_1_storage = &__return_value_1_ret_val_holder;
  crubit::Slot<::tuples::NonCppMovable> __return_value_2_ret_val_holder;
  auto* __return_value_2_storage = __return_value_2_ret_val_holder.Get();
  void* __return_value_storage[] = {__return_value_0_storage,
                                    __return_value_1_storage,
                                    __return_value_2_storage};
  __crubit_internal::__crubit_thunk_return_unon_ucpp_umovable_uat_u3rd(
      __return_value_storage);
  return ::std::make_tuple(*__return_value_0_storage, *__return_value_1_storage,
                           ::rs::Movable<::tuples::NonCppMovable>::TakeFromSlot(
                               ::std::move(__return_value_2_ret_val_holder)));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_unon_ucpp_umovable_umulti(
    void** __ret_ptr);
}
inline ::std::tuple<::rs::Movable<::tuples::NonCppMovable>, ::std::int32_t,
                    ::rs::Movable<::tuples::NonCppMovable>>
return_non_cpp_movable_multi() {
  crubit::Slot<::tuples::NonCppMovable> __return_value_0_ret_val_holder;
  auto* __return_value_0_storage = __return_value_0_ret_val_holder.Get();
  ::std::int32_t __return_value_1_ret_val_holder;
  ::std::int32_t* __return_value_1_storage = &__return_value_1_ret_val_holder;
  crubit::Slot<::tuples::NonCppMovable> __return_value_2_ret_val_holder;
  auto* __return_value_2_storage = __return_value_2_ret_val_holder.Get();
  void* __return_value_storage[] = {__return_value_0_storage,
                                    __return_value_1_storage,
                                    __return_value_2_storage};
  __crubit_internal::__crubit_thunk_return_unon_ucpp_umovable_umulti(
      __return_value_storage);
  return ::std::make_tuple(::rs::Movable<::tuples::NonCppMovable>::TakeFromSlot(
                               ::std::move(__return_value_0_ret_val_holder)),
                           *__return_value_1_storage,
                           ::rs::Movable<::tuples::NonCppMovable>::TakeFromSlot(
                               ::std::move(__return_value_2_ret_val_holder)));
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
extern "C" void
__crubit_thunk_Default_udefault_u_x00000028_x00000028_x00000028u32_x0000002c_x00000020u32_x00000029_x0000002c_x00000020u32_x00000029_x0000002c_x00000020u32_x00000029(
    rs_std::Tuple<rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                                ::std::uint32_t>,
                  ::std::uint32_t>* __ret_ptr);
}
inline ::rs_std::Tuple<
    rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                  ::std::uint32_t>,
    ::std::uint32_t>::Tuple() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_u_x00000028_x00000028_x00000028u32_x0000002c_x00000020u32_x00000029_x0000002c_x00000020u32_x00000029_x0000002c_x00000020u32_x00000029(
          this);
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
inline ::rs_std::Tuple<
    rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                  ::std::uint32_t>,
    ::std::uint32_t>::Tuple(::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<
    rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                  ::std::uint32_t>,
    ::std::uint32_t>::
    Tuple(std::tuple<
          rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                        ::std::uint32_t>,
          ::std::uint32_t>&& tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
  std::construct_at(&this->__field1, std::move(std::get<1>(tuple)));
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
      ::std::uint32_t>(std::move(this->__field0), std::move(this->__field1));
}

inline void ::rs_std::Tuple<
    rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                  ::std::uint32_t>,
    ::std::uint32_t>::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(12 == offsetof(Tuple, __field1));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_u_x00000028_x00000028u32_x0000002c_x00000020u32_x00000029_x0000002c_x00000020u32_x00000029(
    rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                  ::std::uint32_t>* __ret_ptr);
}
inline ::rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                       ::std::uint32_t>::Tuple() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_u_x00000028_x00000028u32_x0000002c_x00000020u32_x00000029_x0000002c_x00000020u32_x00000029(
          this);
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
inline ::rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                       ::std::uint32_t>::Tuple(::crubit::UnsafeRelocateTag,
                                               Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                     ::std::uint32_t>::
    Tuple(std::tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                     ::std::uint32_t>&& tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
  std::construct_at(&this->__field1, std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                     ::std::uint32_t>::
operator std::tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                    ::std::uint32_t>() && noexcept {
  return std::tuple<rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
                    ::std::uint32_t>(std::move(this->__field0),
                                     std::move(this->__field1));
}

inline void ::rs_std::Tuple<
    rs_std::Tuple<::std::uint32_t, ::std::uint32_t>,
    ::std::uint32_t>::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(8 == offsetof(Tuple, __field1));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_u_x00000028i32_x0000002c_x00000029(
    rs_std::Tuple<::std::int32_t>* __ret_ptr);
}
inline ::rs_std::Tuple<::std::int32_t>::Tuple() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_u_x00000028i32_x0000002c_x00000029(this);
}
static_assert(
    ::std::is_trivially_copy_constructible_v<::rs_std::Tuple<::std::int32_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::rs_std::Tuple<::std::int32_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<::rs_std::Tuple<::std::int32_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<::rs_std::Tuple<::std::int32_t>>);
inline ::rs_std::Tuple<::std::int32_t>::Tuple(::crubit::UnsafeRelocateTag,
                                              Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<::std::int32_t>::Tuple(
    std::tuple<::std::int32_t>&& tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
}
inline rs_std::Tuple<::std::int32_t>::operator std::tuple<
    ::std::int32_t>() && noexcept {
  return std::tuple<::std::int32_t>(std::move(this->__field0));
}

inline void ::rs_std::Tuple<
    ::std::int32_t>::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_u_x00000028i8_x0000002c_x00000020isize_x00000029(
    rs_std::Tuple<::std::int8_t, ::std::int64_t>* __ret_ptr);
}
inline ::rs_std::Tuple<::std::int8_t, ::std::int64_t>::Tuple() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_u_x00000028i8_x0000002c_x00000020isize_x00000029(
          this);
}
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs_std::Tuple<::std::int8_t, ::std::int64_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs_std::Tuple<::std::int8_t, ::std::int64_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::std::int8_t, ::std::int64_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::std::int8_t, ::std::int64_t>>);
inline ::rs_std::Tuple<::std::int8_t, ::std::int64_t>::Tuple(
    ::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<::std::int8_t, ::std::int64_t>::Tuple(
    std::tuple<::std::int8_t, ::std::int64_t>&& tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
  std::construct_at(&this->__field1, std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::int8_t, ::std::int64_t>::operator std::tuple<
    ::std::int8_t, ::std::int64_t>() && noexcept {
  return std::tuple<::std::int8_t, ::std::int64_t>(std::move(this->__field0),
                                                   std::move(this->__field1));
}

inline void ::rs_std::Tuple<
    ::std::int8_t, ::std::int64_t>::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(8 == offsetof(Tuple, __field1));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_u_x00000028isize_x0000002c_x00000020i8_x00000029(
    rs_std::Tuple<::std::int64_t, ::std::int8_t>* __ret_ptr);
}
inline ::rs_std::Tuple<::std::int64_t, ::std::int8_t>::Tuple() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_u_x00000028isize_x0000002c_x00000020i8_x00000029(
          this);
}
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs_std::Tuple<::std::int64_t, ::std::int8_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs_std::Tuple<::std::int64_t, ::std::int8_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::std::int64_t, ::std::int8_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::std::int64_t, ::std::int8_t>>);
inline ::rs_std::Tuple<::std::int64_t, ::std::int8_t>::Tuple(
    ::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<::std::int64_t, ::std::int8_t>::Tuple(
    std::tuple<::std::int64_t, ::std::int8_t>&& tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
  std::construct_at(&this->__field1, std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::int64_t, ::std::int8_t>::operator std::tuple<
    ::std::int64_t, ::std::int8_t>() && noexcept {
  return std::tuple<::std::int64_t, ::std::int8_t>(std::move(this->__field0),
                                                   std::move(this->__field1));
}

inline void ::rs_std::Tuple<::std::int64_t,
                            ::std::int8_t>::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(8 == offsetof(Tuple, __field1));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_u_x00000028std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x00000029(
    rs_std::Tuple<
        rs_std::Option<::std::int32_t>,
        rs_std::Result<::std::int32_t, ::rs::alloc::string::String>> const&,
    rs_std::Tuple<rs_std::Option<::std::int32_t>,
                  rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>*
        __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_u_x00000028std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x00000029(
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
  __crubit_internal::
      __crubit_thunk_Clone_uclone_u_x00000028std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x00000029(
          other, this);
}
inline ::rs_std::Tuple<
    rs_std::Option<::std::int32_t>,
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>& ::
rs_std::Tuple<rs_std::Option<::std::int32_t>,
              rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>::
operator=(const Tuple& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_u_x00000028std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x00000029(
            *this, other);
  }
  return *this;
}
inline ::rs_std::Tuple<
    rs_std::Option<::std::int32_t>,
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>::
    Tuple(::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<
    rs_std::Option<::std::int32_t>,
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>::
    Tuple(std::tuple<
          rs_std::Option<::std::int32_t>,
          rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>&&
              tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
  std::construct_at(&this->__field1, std::move(std::get<1>(tuple)));
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
      std::move(this->__field0), std::move(this->__field1));
}
inline rs_std::Tuple<
    rs_std::Option<::std::int32_t>,
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>::~Tuple() {
  std::destroy_at(&this->__field1);
}
inline void ::rs_std::Tuple<
    rs_std::Option<::std::int32_t>,
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>::
    __crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(8 == offsetof(Tuple, __field1));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_u_x00000028std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000029(
    rs_std::Tuple<rs_std::Option<::std::int32_t>>* __ret_ptr);
}
inline ::rs_std::Tuple<rs_std::Option<::std::int32_t>>::Tuple() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_u_x00000028std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000029(
          this);
}
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs_std::Tuple<rs_std::Option<::std::int32_t>>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs_std::Tuple<rs_std::Option<::std::int32_t>>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<rs_std::Option<::std::int32_t>>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<rs_std::Option<::std::int32_t>>>);
inline ::rs_std::Tuple<rs_std::Option<::std::int32_t>>::Tuple(
    ::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<rs_std::Option<::std::int32_t>>::Tuple(
    std::tuple<rs_std::Option<::std::int32_t>>&& tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
}
inline rs_std::Tuple<rs_std::Option<::std::int32_t>>::operator std::tuple<
    rs_std::Option<::std::int32_t>>() && noexcept {
  return std::tuple<rs_std::Option<::std::int32_t>>(std::move(this->__field0));
}

inline void ::rs_std::Tuple<
    rs_std::Option<::std::int32_t>>::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_u_x00000028tuples_ugolden_x0000003a_x0000003aCloneNoDefault_x0000002c_x00000020u8_x00000029(
    rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t> const&,
    rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_u_x00000028tuples_ugolden_x0000003a_x0000003aCloneNoDefault_x0000002c_x00000020u8_x00000029(
    rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t>&,
    rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t> const&);
}
inline ::rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t>::Tuple(
    const Tuple& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_u_x00000028tuples_ugolden_x0000003a_x0000003aCloneNoDefault_x0000002c_x00000020u8_x00000029(
          other, this);
}
inline ::rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t>& ::rs_std::
    Tuple<::tuples::CloneNoDefault, ::std::uint8_t>::operator=(
        const Tuple& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_u_x00000028tuples_ugolden_x0000003a_x0000003aCloneNoDefault_x0000002c_x00000020u8_x00000029(
            *this, other);
  }
  return *this;
}
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t>>);
inline ::rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t>::Tuple(
    ::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t>::Tuple(
    std::tuple<::tuples::CloneNoDefault, ::std::uint8_t>&& tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
  std::construct_at(&this->__field1, std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t>::operator std::
    tuple<::tuples::CloneNoDefault, ::std::uint8_t>() && noexcept {
  return std::tuple<::tuples::CloneNoDefault, ::std::uint8_t>(
      std::move(this->__field0), std::move(this->__field1));
}

inline void ::rs_std::Tuple<::tuples::CloneNoDefault, ::std::uint8_t>::
    __crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(1 == offsetof(Tuple, __field1));
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
inline ::rs_std::Tuple<::tuples::CopyNoDefault, ::std::uint8_t>::Tuple(
    ::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<::tuples::CopyNoDefault, ::std::uint8_t>::Tuple(
    std::tuple<::tuples::CopyNoDefault, ::std::uint8_t>&& tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
  std::construct_at(&this->__field1, std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::tuples::CopyNoDefault, ::std::uint8_t>::operator std::
    tuple<::tuples::CopyNoDefault, ::std::uint8_t>() && noexcept {
  return std::tuple<::tuples::CopyNoDefault, ::std::uint8_t>(
      std::move(this->__field0), std::move(this->__field1));
}

inline void ::rs_std::Tuple<::tuples::CopyNoDefault, ::std::uint8_t>::
    __crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(1 == offsetof(Tuple, __field1));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_u_x00000028tuples_ugolden_x0000003a_x0000003aHasDefault_x0000002c_x00000020u8_x00000029(
    rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t>* __ret_ptr);
}
inline ::rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t>::Tuple() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_u_x00000028tuples_ugolden_x0000003a_x0000003aHasDefault_x0000002c_x00000020u8_x00000029(
          this);
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
inline ::rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t>::Tuple(
    ::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t>::Tuple(
    std::tuple<::tuples::HasDefault, ::std::uint8_t>&& tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
  std::construct_at(&this->__field1, std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t>::operator std::tuple<
    ::tuples::HasDefault, ::std::uint8_t>() && noexcept {
  return std::tuple<::tuples::HasDefault, ::std::uint8_t>(
      std::move(this->__field0), std::move(this->__field1));
}
inline rs_std::Tuple<::tuples::HasDefault, ::std::uint8_t>::~Tuple() {
  std::destroy_at(&this->__field0);
}
inline void ::rs_std::Tuple<
    ::tuples::HasDefault, ::std::uint8_t>::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(24 == offsetof(Tuple, __field1));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
inline ::rs_std::Tuple<::tuples::HasNoDefault, ::std::uint8_t>::Tuple(
    ::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

inline rs_std::Tuple<::tuples::HasNoDefault, ::std::uint8_t>::~Tuple() {
  std::destroy_at(&this->__field0);
}
inline void ::rs_std::Tuple<::tuples::HasNoDefault, ::std::uint8_t>::
    __crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(24 == offsetof(Tuple, __field1));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_u_x00000028u32_x0000002c_x00000020_x00000028u32_x0000002c_x00000020_x00000028u32_x0000002c_x00000020u32_x00000029_x00000029_x00000029(
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
  __crubit_internal::
      __crubit_thunk_Default_udefault_u_x00000028u32_x0000002c_x00000020_x00000028u32_x0000002c_x00000020_x00000028u32_x0000002c_x00000020u32_x00000029_x00000029_x00000029(
          this);
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
inline ::rs_std::Tuple<
    ::std::uint32_t,
    rs_std::Tuple<::std::uint32_t,
                  rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>::
    Tuple(::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<
    ::std::uint32_t,
    rs_std::Tuple<::std::uint32_t,
                  rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>::
    Tuple(std::tuple<
          ::std::uint32_t,
          rs_std::Tuple<::std::uint32_t,
                        rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>&&
              tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
  std::construct_at(&this->__field1, std::move(std::get<1>(tuple)));
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
      std::move(this->__field0), std::move(this->__field1));
}

inline void ::rs_std::Tuple<
    ::std::uint32_t,
    rs_std::Tuple<::std::uint32_t,
                  rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>>::
    __crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(4 == offsetof(Tuple, __field1));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_u_x00000028u32_x0000002c_x00000020_x00000028u32_x0000002c_x00000020u32_x00000029_x00000029(
    rs_std::Tuple<::std::uint32_t,
                  rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>* __ret_ptr);
}
inline ::rs_std::Tuple<
    ::std::uint32_t, rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>::Tuple() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_u_x00000028u32_x0000002c_x00000020_x00000028u32_x0000002c_x00000020u32_x00000029_x00000029(
          this);
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
inline ::rs_std::Tuple<::std::uint32_t,
                       rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>::
    Tuple(::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<::std::uint32_t,
                     rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>::
    Tuple(std::tuple<::std::uint32_t,
                     rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>&&
              tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
  std::construct_at(&this->__field1, std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::uint32_t,
                     rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>::
operator std::tuple<
    ::std::uint32_t,
    rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>() && noexcept {
  return std::tuple<::std::uint32_t,
                    rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>(
      std::move(this->__field0), std::move(this->__field1));
}

inline void ::rs_std::Tuple<::std::uint32_t,
                            rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>::
    __crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(4 == offsetof(Tuple, __field1));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_u_x00000028u32_x0000002c_x00000020u32_x00000029(
    rs_std::Tuple<::std::uint32_t, ::std::uint32_t>* __ret_ptr);
}
inline ::rs_std::Tuple<::std::uint32_t, ::std::uint32_t>::Tuple() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_u_x00000028u32_x0000002c_x00000020u32_x00000029(
          this);
}
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::std::uint32_t, ::std::uint32_t>>);
inline ::rs_std::Tuple<::std::uint32_t, ::std::uint32_t>::Tuple(
    ::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<::std::uint32_t, ::std::uint32_t>::Tuple(
    std::tuple<::std::uint32_t, ::std::uint32_t>&& tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
  std::construct_at(&this->__field1, std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::uint32_t, ::std::uint32_t>::operator std::tuple<
    ::std::uint32_t, ::std::uint32_t>() && noexcept {
  return std::tuple<::std::uint32_t, ::std::uint32_t>(
      std::move(this->__field0), std::move(this->__field1));
}

inline void ::rs_std::Tuple<
    ::std::uint32_t, ::std::uint32_t>::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(4 == offsetof(Tuple, __field1));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_u_x00000028u64_x0000002c_x00000020u64_x00000029(
    rs_std::Tuple<::std::uint64_t, ::std::uint64_t>* __ret_ptr);
}
inline ::rs_std::Tuple<::std::uint64_t, ::std::uint64_t>::Tuple() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_u_x00000028u64_x0000002c_x00000020u64_x00000029(
          this);
}
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs_std::Tuple<::std::uint64_t, ::std::uint64_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs_std::Tuple<::std::uint64_t, ::std::uint64_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::std::uint64_t, ::std::uint64_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::std::uint64_t, ::std::uint64_t>>);
inline ::rs_std::Tuple<::std::uint64_t, ::std::uint64_t>::Tuple(
    ::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<::std::uint64_t, ::std::uint64_t>::Tuple(
    std::tuple<::std::uint64_t, ::std::uint64_t>&& tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
  std::construct_at(&this->__field1, std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::uint64_t, ::std::uint64_t>::operator std::tuple<
    ::std::uint64_t, ::std::uint64_t>() && noexcept {
  return std::tuple<::std::uint64_t, ::std::uint64_t>(
      std::move(this->__field0), std::move(this->__field1));
}

inline void ::rs_std::Tuple<
    ::std::uint64_t, ::std::uint64_t>::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(8 == offsetof(Tuple, __field1));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_u_x00000028u8_x0000002c_x00000020tuples_ugolden_x0000003a_x0000003aCloneNoDefault_x00000029(
    rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault> const&,
    rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_u_x00000028u8_x0000002c_x00000020tuples_ugolden_x0000003a_x0000003aCloneNoDefault_x00000029(
    rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault>&,
    rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault> const&);
}
inline ::rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault>::Tuple(
    const Tuple& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_u_x00000028u8_x0000002c_x00000020tuples_ugolden_x0000003a_x0000003aCloneNoDefault_x00000029(
          other, this);
}
inline ::rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault>& ::rs_std::
    Tuple<::std::uint8_t, ::tuples::CloneNoDefault>::operator=(
        const Tuple& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_u_x00000028u8_x0000002c_x00000020tuples_ugolden_x0000003a_x0000003aCloneNoDefault_x00000029(
            *this, other);
  }
  return *this;
}
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault>>);
inline ::rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault>::Tuple(
    ::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault>::Tuple(
    std::tuple<::std::uint8_t, ::tuples::CloneNoDefault>&& tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
  std::construct_at(&this->__field1, std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault>::operator std::
    tuple<::std::uint8_t, ::tuples::CloneNoDefault>() && noexcept {
  return std::tuple<::std::uint8_t, ::tuples::CloneNoDefault>(
      std::move(this->__field0), std::move(this->__field1));
}

inline void ::rs_std::Tuple<::std::uint8_t, ::tuples::CloneNoDefault>::
    __crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(1 == offsetof(Tuple, __field1));
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
inline ::rs_std::Tuple<::std::uint8_t, ::tuples::CopyNoDefault>::Tuple(
    ::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<::std::uint8_t, ::tuples::CopyNoDefault>::Tuple(
    std::tuple<::std::uint8_t, ::tuples::CopyNoDefault>&& tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
  std::construct_at(&this->__field1, std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::uint8_t, ::tuples::CopyNoDefault>::operator std::
    tuple<::std::uint8_t, ::tuples::CopyNoDefault>() && noexcept {
  return std::tuple<::std::uint8_t, ::tuples::CopyNoDefault>(
      std::move(this->__field0), std::move(this->__field1));
}

inline void ::rs_std::Tuple<::std::uint8_t, ::tuples::CopyNoDefault>::
    __crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(1 == offsetof(Tuple, __field1));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_u_x00000028u8_x0000002c_x00000020tuples_ugolden_x0000003a_x0000003aHasDefault_x00000029(
    rs_std::Tuple<::std::uint8_t, ::tuples::HasDefault>* __ret_ptr);
}
inline ::rs_std::Tuple<::std::uint8_t, ::tuples::HasDefault>::Tuple() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_u_x00000028u8_x0000002c_x00000020tuples_ugolden_x0000003a_x0000003aHasDefault_x00000029(
          this);
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
inline ::rs_std::Tuple<::std::uint8_t, ::tuples::HasDefault>::Tuple(
    ::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<::std::uint8_t, ::tuples::HasDefault>::Tuple(
    std::tuple<::std::uint8_t, ::tuples::HasDefault>&& tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
  std::construct_at(&this->__field1, std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::uint8_t, ::tuples::HasDefault>::operator std::tuple<
    ::std::uint8_t, ::tuples::HasDefault>() && noexcept {
  return std::tuple<::std::uint8_t, ::tuples::HasDefault>(
      std::move(this->__field0), std::move(this->__field1));
}
inline rs_std::Tuple<::std::uint8_t, ::tuples::HasDefault>::~Tuple() {
  std::destroy_at(&this->__field1);
}
inline void ::rs_std::Tuple<
    ::std::uint8_t, ::tuples::HasDefault>::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(8 == offsetof(Tuple, __field1));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020tuples_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
inline ::rs_std::Tuple<::std::uint8_t, ::tuples::HasNoDefault>::Tuple(
    ::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

inline rs_std::Tuple<::std::uint8_t, ::tuples::HasNoDefault>::~Tuple() {
  std::destroy_at(&this->__field1);
}
inline void ::rs_std::Tuple<::std::uint8_t, ::tuples::HasNoDefault>::
    __crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(8 == offsetof(Tuple, __field1));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_u_x00000028u8_x0000002c_x00000020usize_x00000029(
    rs_std::Tuple<::std::uint8_t, ::std::uint64_t>* __ret_ptr);
}
inline ::rs_std::Tuple<::std::uint8_t, ::std::uint64_t>::Tuple() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_u_x00000028u8_x0000002c_x00000020usize_x00000029(
          this);
}
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs_std::Tuple<::std::uint8_t, ::std::uint64_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs_std::Tuple<::std::uint8_t, ::std::uint64_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::std::uint8_t, ::std::uint64_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::std::uint8_t, ::std::uint64_t>>);
inline ::rs_std::Tuple<::std::uint8_t, ::std::uint64_t>::Tuple(
    ::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<::std::uint8_t, ::std::uint64_t>::Tuple(
    std::tuple<::std::uint8_t, ::std::uint64_t>&& tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
  std::construct_at(&this->__field1, std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::uint8_t, ::std::uint64_t>::operator std::tuple<
    ::std::uint8_t, ::std::uint64_t>() && noexcept {
  return std::tuple<::std::uint8_t, ::std::uint64_t>(std::move(this->__field0),
                                                     std::move(this->__field1));
}

inline void ::rs_std::Tuple<
    ::std::uint8_t, ::std::uint64_t>::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(8 == offsetof(Tuple, __field1));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_u_x00000028usize_x0000002c_x00000020u8_x00000029(
    rs_std::Tuple<::std::uint64_t, ::std::uint8_t>* __ret_ptr);
}
inline ::rs_std::Tuple<::std::uint64_t, ::std::uint8_t>::Tuple() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_u_x00000028usize_x0000002c_x00000020u8_x00000029(
          this);
}
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs_std::Tuple<::std::uint64_t, ::std::uint8_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs_std::Tuple<::std::uint64_t, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::std::uint64_t, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::std::uint64_t, ::std::uint8_t>>);
inline ::rs_std::Tuple<::std::uint64_t, ::std::uint8_t>::Tuple(
    ::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<::std::uint64_t, ::std::uint8_t>::Tuple(
    std::tuple<::std::uint64_t, ::std::uint8_t>&& tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
  std::construct_at(&this->__field1, std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::uint64_t, ::std::uint8_t>::operator std::tuple<
    ::std::uint64_t, ::std::uint8_t>() && noexcept {
  return std::tuple<::std::uint64_t, ::std::uint8_t>(std::move(this->__field0),
                                                     std::move(this->__field1));
}

inline void ::rs_std::Tuple<
    ::std::uint64_t, ::std::uint8_t>::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(8 == offsetof(Tuple, __field1));
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
inline rs_std::Option<::std::int32_t>::Option(::crubit::UnsafeRelocateTag,
                                              Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<rs_std::Option<::std::int32_t>>);
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

inline constexpr rs_std::Option<::std::int32_t>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<::std::int32_t>&
rs_std::Option<::std::int32_t>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<rs_std::Option<::std::int32_t>,
                                              ::std::int32_t, U>)
inline rs_std::Option<::std::int32_t>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<rs_std::Option<::std::int32_t>,
                                              ::std::int32_t, U>)
inline rs_std::Option<::std::int32_t>&
rs_std::Option<::std::int32_t>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::std::int32_t, Opt>)
inline rs_std::Option<::std::int32_t>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::std::int32_t, Opt>)
inline rs_std::Option<::std::int32_t>&
rs_std::Option<::std::int32_t>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<::std::int32_t>::Option(::std::in_place_t ip,
                                              Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e(
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String> const&,
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e(
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>&,
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String> const&);
}
inline rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::Result(
    const Result& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e(
          other, this);
}
inline rs_std::Result<::std::int32_t, ::rs::alloc::string::String>&
rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::operator=(
    const Result& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e(
            *this, other);
  }
  return *this;
}
inline rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Result<::std::int32_t,
                      ::rs::alloc::string::String>::~Result() noexcept {
  this->Reset();
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

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::int32_t, ::rs::alloc::string::String>,
           ::std::int32_t, U>)
inline constexpr rs_std::Result<
    ::std::int32_t, ::rs::alloc::string::String>::Result(U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::int32_t, ::rs::alloc::string::String>,
           ::std::int32_t, U>)
inline constexpr rs_std::Result<::std::int32_t, ::rs::alloc::string::String>&
rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::operator=(
    U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(
      rs_std::ResultUnexpectedConstructible<::rs::alloc::string::String, F>)
inline constexpr rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::
    Result(rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(
      rs_std::ResultUnexpectedConstructible<::rs::alloc::string::String, F>)
inline constexpr rs_std::Result<::std::int32_t, ::rs::alloc::string::String>&
rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::
    Result(::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<::std::int32_t, ::rs::alloc::string::String>::
    Result(rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TUPLES_TUPLES_GOLDEN
