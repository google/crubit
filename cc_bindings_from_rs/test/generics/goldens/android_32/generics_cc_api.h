// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// generics_golden

// clang-format off
#ifndef CC_BINDINGS_FROM_RS_TEST_GENERICS_GENERICS_GOLDEN
#define CC_BINDINGS_FROM_RS_TEST_GENERICS_GENERICS_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#pragma clang diagnostic ignored "-Wshadow"
#include "support/annotations_internal.h"
#include "support/internal/check.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/movable.h"
#include "support/rs_std/vec.h"

#include <array>
#include <compare>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <functional>
#include <memory>
#include <new>
#include <ostream>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_alloc.h"

namespace generics {
template <typename T>
struct GenericStruct;
struct SelfReferentialViaPhantom;
}  // namespace generics

namespace generics {
template <typename T>
struct RenamedSubGeneric;
}

namespace generics {

using AliasedGenericI64 CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: AliasedGenericI64") =
    ::generics::GenericStruct<::std::int64_t>;

// Error generating bindings for struct `generics_golden::ConstGenericStruct`:
// crubit.rs/errors/unsupported_type: `const`-generic ADTs are not supported yet
// (b/259749095)

template <typename T>
struct DerivedGeneric {
  static_assert(false, "This template can only be used via a specialization");
};

template <typename T>
struct GenericEnum {
  static_assert(false, "This template can only be used via a specialization");
};

template <typename T>
struct GenericStruct {
  static_assert(false, "This template can only be used via a specialization");
};

template <typename T>
union GenericUnion {
  static_assert(false, "This template can only be used via a specialization");
};

template <typename T>
struct GenericWithDrop {
  static_assert(false, "This template can only be used via a specialization");
};

template <typename T>
struct LifetimeAndTypeGeneric {
  static_assert(false, "This template can only be used via a specialization");
};

template <typename T, typename U>
struct MultiGenericStruct {
  static_assert(false, "This template can only be used via a specialization");
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: NotPartialEqOrDefault") alignas(4)
    [[clang::trivial_abi]] NotPartialEqOrDefault final {
 public:
  ::std::int32_t __field0{};

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: PartialEqAndDefault") alignas(4)
    [[clang::trivial_abi]] PartialEqAndDefault final {
 public:
  bool operator==(::generics::PartialEqAndDefault const& other) const;

  ::std::int32_t __field0{};

 private:
  static void __crubit_field_offset_assertions();
};

template <typename T>
struct PhantomGeneric {
  static_assert(false, "This template can only be used via a specialization");
};

template <typename T>
struct Ping {
  static_assert(false, "This template can only be used via a specialization");
};

template <typename T>
struct Pong {
  static_assert(false, "This template can only be used via a specialization");
};

template <typename T>
struct Wrapper {
  static_assert(false, "This template can only be used via a specialization");
};

// Error generating bindings for struct `generics_golden::ZeroSizedUnboundType`:
// Zero-sized types (ZSTs) are not supported (b/258259459)

void accept_generic_with_drop_unmovable(
    ::rs::Movable<
        ::generics::GenericWithDrop<::generics::NotPartialEqOrDefault>>
        _x);

// Error generating bindings for function
// `generics_golden::instantiate_const_generic`: Error formatting function
// return type `generics_golden::ConstGenericStruct<i32, 4>`:
// crubit.rs/errors/unsupported_type: `const`-generic ADTs are not supported yet
// (b/259749095)

::generics::DerivedGeneric<::std::int32_t> instantiate_derived_generic_at_i32();

::generics::DerivedGeneric<::generics::NotPartialEqOrDefault>
instantiate_derived_generic_at_not_partial_eq_or_default();

::generics::GenericEnum<::std::int32_t> instantiate_generic_enum();

::generics::GenericStruct<::generics::PartialEqAndDefault>
instantiate_generic_struct_at_i32();

::generics::GenericStruct<::generics::NotPartialEqOrDefault>
instantiate_generic_struct_at_not_partial_eq_or_default();

::generics::GenericStruct<::std::int32_t>
instantiate_generic_struct_at_primitive_i32();

::generics::GenericUnion<::std::uint32_t> instantiate_generic_union();

::generics::GenericWithDrop<::std::int32_t>
instantiate_generic_with_drop_movable();

::generics::GenericWithDrop<::generics::NotPartialEqOrDefault>
instantiate_generic_with_drop_unmovable();

::generics::LifetimeAndTypeGeneric<::std::int32_t>
instantiate_lifetime_and_type_generic(
    ::std::int32_t const* $(__anon1) crubit_nonnull x CRUBIT_LIFETIME_BOUND);

::generics::MultiGenericStruct<bool, double>
instantiate_multi_generic_at_bool_f64();

::generics::MultiGenericStruct<bool, ::std::int32_t>
instantiate_multi_generic_at_bool_i32();

::generics::Ping<::std::int32_t> instantiate_ping();

::generics::SelfReferentialViaPhantom instantiate_self_referential_phantom();

::generics::RenamedSubGeneric<::std::int32_t> instantiate_sub_generic();

::generics::GenericStruct<::std::array<::std::int32_t, 4>>
instantiate_with_array();

// Error generating bindings for function
// `generics_golden::instantiate_with_i128`: Error formatting function return
// type `generics_golden::GenericStruct<i128>`: C++ doesn't have a standard
// equivalent of `i128` (b/254094650)

// Error generating bindings for function
// `generics_golden::instantiate_with_static_ref`: Error formatting function
// return type `generics_golden::GenericStruct<&'static i32>`: Generic type
// arguments containing references or lifetimes are not supported yet
// (b/259749095)

// Error generating bindings for function
// `generics_golden::instantiate_with_unbound_zst`: Error formatting function
// return type
// `generics_golden::GenericStruct<generics_golden::ZeroSizedUnboundType>`:
// Failed to format type for the definition of
// `generics_golden::ZeroSizedUnboundType`: Zero-sized types (ZSTs) are not
// supported (b/258259459)

}  // namespace generics

namespace generics {

template <typename T>
struct RenamedSubGeneric {
  static_assert(false, "This template can only be used via a specialization");
};

}  // namespace generics

namespace generics {

::generics::GenericStruct<::std::int64_t> use_aliased_generic(
    ::generics::GenericStruct<::std::int64_t> x);

}

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020DerivedGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020NotPartialEqOrDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020DerivedGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020NotPartialEqOrDefault_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: DerivedGeneric < :: generics_golden :: "
    "NotPartialEqOrDefault >") alignas(4)
    [[clang::trivial_abi]] ::generics::DerivedGeneric<
        ::generics::NotPartialEqOrDefault> final {
 public:
  ::generics::NotPartialEqOrDefault value{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020DerivedGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020DerivedGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: DerivedGeneric < i32 >") alignas(4)
    [[clang::trivial_abi]] ::generics::DerivedGeneric<::std::int32_t> final {
 public:
  bool operator==(
      ::generics::DerivedGeneric<::std::int32_t> const& other) const;

  // AbslHashValue and std::hash support via core::hash::Hash
  template <typename H>
  friend H AbslHashValue(
      H h, const ::generics::DerivedGeneric<::std::int32_t>& self);

  ::std::strong_ordering operator<=>(const DerivedGeneric& other) const;

  ::std::int32_t value{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020hash_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020DerivedGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020hash_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020DerivedGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" ::std::uint64_t
__crubit_thunk_Hash_uhash_ugenerics_ugolden_x0000003a_x0000003aDerivedGeneric_x0000003ci32_x0000003e(
    const ::generics::DerivedGeneric<::std::int32_t>&);
}
namespace std {
template <>
struct hash<::generics::DerivedGeneric<::std::int32_t>> {
  ::std::size_t operator()(
      const ::generics::DerivedGeneric<::std::int32_t>& self) const {
    return static_cast<::std::size_t>(
        __crubit_internal::
            __crubit_thunk_Hash_uhash_ugenerics_ugolden_x0000003a_x0000003aDerivedGeneric_x0000003ci32_x0000003e(
                self));
  }
};
}  // namespace std
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericEnum_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericEnum_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: GenericEnum < i32 >") alignas(4)
    [[clang::trivial_abi]] ::generics::GenericEnum<::std::int32_t> final {
 public:
  // `generics_golden::GenericEnum` doesn't implement the `Default` trait
  GenericEnum() = delete;

  static constexpr GenericEnum MakeEmpty();

  static ::generics::GenericEnum<::std::int32_t> MakeValue(
      ::std::int32_t __param_0);

  // No custom `Drop` impl and no custom "drop glue" required
  ~GenericEnum() = default;
  GenericEnum(GenericEnum&&) = default;
  GenericEnum& operator=(GenericEnum&&) = default;

  // `generics_golden::GenericEnum` doesn't implement the `Clone` trait
  GenericEnum(const GenericEnum&) = delete;
  GenericEnum& operator=(const GenericEnum&) = delete;
  GenericEnum(::crubit::UnsafeRelocateTag, GenericEnum&& value);

  // Variant Empty has no size, so no struct is generated.

  struct alignas(4) __crubit_Value_struct {
   public:
    ::std::int32_t __field0;
  };

  enum class Tag : ::std::uint32_t {
    Empty = 0,
    Value = 1,
  };

 public:
  Tag tag;

 public:
  union {
    __crubit_Value_struct Value;
  };

 private:
  struct PrivateTagCtorTag {};
  constexpr GenericEnum(PrivateTagCtorTag, Tag tag) : tag(tag) {}

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020array_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x000000204_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020array_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x000000204_x00000020_x0000003e_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: GenericStruct < [i32 ; 4] >") alignas(4)
    [[clang::trivial_abi]] ::generics::GenericStruct<
        ::std::array<::std::int32_t, 4>> final {
 public:
  static ::generics::GenericStruct<::std::array<::std::int32_t, 4>> new_(
      ::std::array<::std::int32_t, 4> value);

  ::std::array<::std::int32_t, 4> const& $(__anon1)
      get_ref() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  void set(::std::array<::std::int32_t, 4> value);

  ::std::array<::std::int32_t, 4> into_inner() &&;
  static constexpr ::std::uintptr_t VALUE_SIZE = 16;

  static ::std::uintptr_t type_size();

  bool is_default() const;

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<[i32; 4]>::t_iter`: Inherent methods with
  // their own predicates are not supported

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<[i32; 4]>::map`: Generic methods are not
  // supported

  ::std::array<::std::int32_t, 4> const& $(__anon1)
      borrow_with_explicit_lt() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  ::generics::MultiGenericStruct<::std::array<::std::int32_t, 4>,
                                 ::std::array<::std::int32_t, 4>>
  into_multi() const;

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<[i32; 4]>::to_unbound`: Error formatting
  // function return type
  // `generics_golden::GenericStruct<generics_golden::ZeroSizedUnboundType>`:
  // Failed to format type for the definition of
  // `generics_golden::ZeroSizedUnboundType`: Zero-sized types (ZSTs) are not
  // supported (b/258259459)

  // Constructor from `<generics_golden::GenericStruct<T> as
  // std::convert::From<T>>` is suppressed because `GenericStruct` is a C++
  // aggregate.

  ::std::array<::std::int32_t, 4> value{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020NotPartialEqOrDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020NotPartialEqOrDefault_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: GenericStruct < :: generics_golden :: "
    "NotPartialEqOrDefault >") alignas(4)
    [[clang::trivial_abi]] ::generics::GenericStruct<
        ::generics::NotPartialEqOrDefault> final {
 public:
  static ::generics::GenericStruct<::generics::NotPartialEqOrDefault> new_(
      ::generics::NotPartialEqOrDefault value);

  ::generics::NotPartialEqOrDefault const& $(__anon1)
      get_ref() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  void set(::generics::NotPartialEqOrDefault value);

  ::generics::NotPartialEqOrDefault into_inner() &&;
  static constexpr ::std::uintptr_t VALUE_SIZE = 4;

  static ::std::uintptr_t type_size();

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<generics_golden::NotPartialEqOrDefault>::t_iter`:
  // Inherent methods with their own predicates are not supported

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<generics_golden::NotPartialEqOrDefault>::map`:
  // Generic methods are not supported

  ::generics::NotPartialEqOrDefault const& $(__anon1)
      borrow_with_explicit_lt() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<generics_golden::NotPartialEqOrDefault>::to_unbound`:
  // Error formatting function return type
  // `generics_golden::GenericStruct<generics_golden::ZeroSizedUnboundType>`:
  // Failed to format type for the definition of
  // `generics_golden::ZeroSizedUnboundType`: Zero-sized types (ZSTs) are not
  // supported (b/258259459)

  // Constructor from `<generics_golden::GenericStruct<T> as
  // std::convert::From<T>>` is suppressed because `GenericStruct` is a C++
  // aggregate.

  ::generics::NotPartialEqOrDefault value{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020PartialEqAndDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020PartialEqAndDefault_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: GenericStruct < :: generics_golden :: "
    "PartialEqAndDefault >") alignas(4)
    [[clang::trivial_abi]] ::generics::GenericStruct<
        ::generics::PartialEqAndDefault> final {
 public:
  static ::generics::GenericStruct<::generics::PartialEqAndDefault> new_(
      ::generics::PartialEqAndDefault value);

  ::generics::PartialEqAndDefault const& $(__anon1)
      get_ref() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  void set(::generics::PartialEqAndDefault value);

  ::generics::PartialEqAndDefault into_inner() &&;
  static constexpr ::std::uintptr_t VALUE_SIZE = 4;

  static ::std::uintptr_t type_size();

  bool is_default() const;

  ::std::int32_t const& $(__anon1)
      deref_value() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<generics_golden::PartialEqAndDefault>::t_iter`:
  // Inherent methods with their own predicates are not supported

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<generics_golden::PartialEqAndDefault>::map`:
  // Generic methods are not supported

  ::generics::PartialEqAndDefault const& $(__anon1)
      borrow_with_explicit_lt() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<generics_golden::PartialEqAndDefault>::to_unbound`:
  // Error formatting function return type
  // `generics_golden::GenericStruct<generics_golden::ZeroSizedUnboundType>`:
  // Failed to format type for the definition of
  // `generics_golden::ZeroSizedUnboundType`: Zero-sized types (ZSTs) are not
  // supported (b/258259459)

  // Constructor from `<generics_golden::GenericStruct<T> as
  // std::convert::From<T>>` is suppressed because `GenericStruct` is a C++
  // aggregate.

  ::generics::PartialEqAndDefault value{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: GenericStruct < i32 >") alignas(4)
    [[clang::trivial_abi]] ::generics::GenericStruct<::std::int32_t> final {
 public:
  static ::generics::GenericStruct<::std::int32_t> new_(::std::int32_t value);

  ::std::int32_t const& $(__anon1)
      get_ref() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  void set(::std::int32_t value);

  ::std::int32_t into_inner() &&;
  static constexpr ::std::uintptr_t VALUE_SIZE = 4;

  static ::std::uintptr_t type_size();

  bool is_default() const;
  static const ::generics::GenericStruct<::std::int32_t> ZERO;

  ::std::int32_t add_one() const;

  static ::std::int32_t concrete_id();

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<i32>::t_iter`: Inherent methods with
  // their own predicates are not supported

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<i32>::map`: Generic methods are not
  // supported

  ::std::int32_t const& $(__anon1)
      borrow_with_explicit_lt() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  ::generics::MultiGenericStruct<::std::int32_t, ::std::int32_t> into_multi()
      const;

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<i32>::to_unbound`: Error formatting
  // function return type
  // `generics_golden::GenericStruct<generics_golden::ZeroSizedUnboundType>`:
  // Failed to format type for the definition of
  // `generics_golden::ZeroSizedUnboundType`: Zero-sized types (ZSTs) are not
  // supported (b/258259459)

  ::generics::GenericStruct<::std::int32_t> operator+(
      ::generics::GenericStruct<::std::int32_t> rhs) &&;

  // Constructor from `<generics_golden::GenericStruct<T> as
  // std::convert::From<T>>` is suppressed because `GenericStruct` is a C++
  // aggregate.

  // AbslStringify and std::ostream support via std::fmt::Display
  template <typename Sink, typename Str = ::rs::alloc::string::String>
  friend void AbslStringify(Sink& sink, const GenericStruct& self) {
    crubit::Slot<Str> s;
    __crubit_thunk_ToString_uto_ustring_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003ci32_x0000003e(
        self, s.Get());
    AbslStringify(sink, ::std::move(s).AssumeInitAndTakeValue().as_str());
  }
  template <typename Str = ::rs::alloc::string::String>
  friend ::std::ostream& operator<<(::std::ostream& os,
                                    const GenericStruct& self) {
    crubit::Slot<Str> s;
    __crubit_thunk_ToString_uto_ustring_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003ci32_x0000003e(
        self, s.Get());
    return os << ::std::string_view(
               ::std::move(s).AssumeInitAndTakeValue().as_str());
  }

  ::std::int32_t value{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: GenericStruct < i64 >") alignas(4)
    [[clang::trivial_abi]] ::generics::GenericStruct<::std::int64_t> final {
 public:
  static ::generics::GenericStruct<::std::int64_t> new_(::std::int64_t value);

  ::std::int64_t const& $(__anon1)
      get_ref() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  void set(::std::int64_t value);

  ::std::int64_t into_inner() &&;
  static constexpr ::std::uintptr_t VALUE_SIZE = 8;

  static ::std::uintptr_t type_size();

  bool is_default() const;

  static ::std::int32_t concrete_id();

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<i64>::t_iter`: Inherent methods with
  // their own predicates are not supported

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<i64>::map`: Generic methods are not
  // supported

  ::std::int64_t const& $(__anon1)
      borrow_with_explicit_lt() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  ::generics::MultiGenericStruct<::std::int64_t, ::std::int64_t> into_multi()
      const;

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<i64>::to_unbound`: Error formatting
  // function return type
  // `generics_golden::GenericStruct<generics_golden::ZeroSizedUnboundType>`:
  // Failed to format type for the definition of
  // `generics_golden::ZeroSizedUnboundType`: Zero-sized types (ZSTs) are not
  // supported (b/258259459)

  ::generics::GenericStruct<::std::int64_t> operator+(
      ::generics::GenericStruct<::std::int64_t> rhs) &&;

  // Constructor from `<generics_golden::GenericStruct<T> as
  // std::convert::From<T>>` is suppressed because `GenericStruct` is a C++
  // aggregate.

  // AbslStringify and std::ostream support via std::fmt::Display
  template <typename Sink, typename Str = ::rs::alloc::string::String>
  friend void AbslStringify(Sink& sink, const GenericStruct& self) {
    crubit::Slot<Str> s;
    __crubit_thunk_ToString_uto_ustring_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003ci64_x0000003e(
        self, s.Get());
    AbslStringify(sink, ::std::move(s).AssumeInitAndTakeValue().as_str());
  }
  template <typename Str = ::rs::alloc::string::String>
  friend ::std::ostream& operator<<(::std::ostream& os,
                                    const GenericStruct& self) {
    crubit::Slot<Str> s;
    __crubit_thunk_ToString_uto_ustring_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003ci64_x0000003e(
        self, s.Get());
    return os << ::std::string_view(
               ::std::move(s).AssumeInitAndTakeValue().as_str());
  }

  ::std::int64_t value{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: GenericStruct < u32 >") alignas(4)
    [[clang::trivial_abi]] ::generics::GenericStruct<::std::uint32_t> final {
 public:
  static ::generics::GenericStruct<::std::uint32_t> new_(::std::uint32_t value);

  ::std::uint32_t const& $(__anon1)
      get_ref() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  void set(::std::uint32_t value);

  ::std::uint32_t into_inner() &&;
  static constexpr ::std::uintptr_t VALUE_SIZE = 4;

  static ::std::uintptr_t type_size();

  bool is_default() const;

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<u32>::t_iter`: Inherent methods with
  // their own predicates are not supported

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<u32>::map`: Generic methods are not
  // supported

  ::std::uint32_t const& $(__anon1)
      borrow_with_explicit_lt() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  ::generics::MultiGenericStruct<::std::uint32_t, ::std::uint32_t> into_multi()
      const;

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<u32>::to_unbound`: Error formatting
  // function return type
  // `generics_golden::GenericStruct<generics_golden::ZeroSizedUnboundType>`:
  // Failed to format type for the definition of
  // `generics_golden::ZeroSizedUnboundType`: Zero-sized types (ZSTs) are not
  // supported (b/258259459)

  ::generics::GenericStruct<::std::uint32_t> operator+(
      ::generics::GenericStruct<::std::uint32_t> rhs) &&;

  // Constructor from `<generics_golden::GenericStruct<T> as
  // std::convert::From<T>>` is suppressed because `GenericStruct` is a C++
  // aggregate.

  // AbslStringify and std::ostream support via std::fmt::Display
  template <typename Sink, typename Str = ::rs::alloc::string::String>
  friend void AbslStringify(Sink& sink, const GenericStruct& self) {
    crubit::Slot<Str> s;
    __crubit_thunk_ToString_uto_ustring_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
        self, s.Get());
    AbslStringify(sink, ::std::move(s).AssumeInitAndTakeValue().as_str());
  }
  template <typename Str = ::rs::alloc::string::String>
  friend ::std::ostream& operator<<(::std::ostream& os,
                                    const GenericStruct& self) {
    crubit::Slot<Str> s;
    __crubit_thunk_ToString_uto_ustring_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
        self, s.Get());
    return os << ::std::string_view(
               ::std::move(s).AssumeInitAndTakeValue().as_str());
  }

  ::std::uint32_t value{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: GenericStruct < :: generics_golden :: GenericStruct "
    "< u32 > >") alignas(4)
    [[clang::trivial_abi]] ::generics::GenericStruct<
        ::generics::GenericStruct<::std::uint32_t>> final {
 public:
  static ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>
  new_(::generics::GenericStruct<::std::uint32_t> value);

  ::generics::GenericStruct<::std::uint32_t> const& $(__anon1)
      get_ref() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  void set(::generics::GenericStruct<::std::uint32_t> value);

  ::generics::GenericStruct<::std::uint32_t> into_inner() &&;
  static constexpr ::std::uintptr_t VALUE_SIZE = 4;

  static ::std::uintptr_t type_size();

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<generics_golden::GenericStruct<u32>>::t_iter`:
  // Inherent methods with their own predicates are not supported

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<generics_golden::GenericStruct<u32>>::map`:
  // Generic methods are not supported

  ::generics::GenericStruct<::std::uint32_t> const& $(__anon1)
      borrow_with_explicit_lt() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  // Error generating bindings for associated function
  // `generics_golden::GenericStruct::<generics_golden::GenericStruct<u32>>::to_unbound`:
  // Error formatting function return type
  // `generics_golden::GenericStruct<generics_golden::ZeroSizedUnboundType>`:
  // Failed to format type for the definition of
  // `generics_golden::ZeroSizedUnboundType`: Zero-sized types (ZSTs) are not
  // supported (b/258259459)

  ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>
  operator+(
      ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>
          rhs) &&;

  // Constructor from `<generics_golden::GenericStruct<T> as
  // std::convert::From<T>>` is suppressed because `GenericStruct` is a C++
  // aggregate.

  // AbslStringify and std::ostream support via std::fmt::Display
  template <typename Sink, typename Str = ::rs::alloc::string::String>
  friend void AbslStringify(Sink& sink, const GenericStruct& self) {
    crubit::Slot<Str> s;
    __crubit_thunk_ToString_uto_ustring_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cgenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e_x0000003e(
        self, s.Get());
    AbslStringify(sink, ::std::move(s).AssumeInitAndTakeValue().as_str());
  }
  template <typename Str = ::rs::alloc::string::String>
  friend ::std::ostream& operator<<(::std::ostream& os,
                                    const GenericStruct& self) {
    crubit::Slot<Str> s;
    __crubit_thunk_ToString_uto_ustring_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cgenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e_x0000003e(
        self, s.Get());
    return os << ::std::string_view(
               ::std::move(s).AssumeInitAndTakeValue().as_str());
  }

  ::generics::GenericStruct<::std::uint32_t> value{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

namespace generics {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: OuterWithNestedGenericField") alignas(4)
    [[clang::trivial_abi]] OuterWithNestedGenericField final {
 public:
  ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>> field{};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace generics

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericUnion_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericUnion_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e

template <>
union CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: GenericUnion < u32 >") alignas(4)
    [[clang::trivial_abi]] ::generics::GenericUnion<::std::uint32_t> final {
 public:
  // `generics_golden::GenericUnion` doesn't implement the `Default` trait
  GenericUnion() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~GenericUnion() = default;
  GenericUnion(GenericUnion&&) = default;
  GenericUnion& operator=(GenericUnion&&) = default;

  // `generics_golden::GenericUnion` doesn't implement the `Clone` trait
  GenericUnion(const GenericUnion&) = delete;
  GenericUnion& operator=(const GenericUnion&) = delete;
  GenericUnion(::crubit::UnsafeRelocateTag, GenericUnion&& value);

  ::std::uint32_t val;
  ::std::uint32_t raw;

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericWithDrop_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020NotPartialEqOrDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericWithDrop_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020NotPartialEqOrDefault_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: GenericWithDrop < :: generics_golden :: "
    "NotPartialEqOrDefault >") alignas(4)
    [[clang::trivial_abi]] ::generics::GenericWithDrop<
        ::generics::NotPartialEqOrDefault> final {
 public:
  // Type is not a C++ aggregate: Type implements `Drop`

  // `generics_golden::GenericWithDrop` doesn't implement the `Default` trait
  GenericWithDrop() = delete;

  // Drop::drop
  ~GenericWithDrop();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  GenericWithDrop(GenericWithDrop&&) = delete;
  ::generics::GenericWithDrop<::generics::NotPartialEqOrDefault>& operator=(
      GenericWithDrop&&) = delete;
  // `generics_golden::GenericWithDrop` doesn't implement the `Clone` trait
  GenericWithDrop(const GenericWithDrop&) = delete;
  GenericWithDrop& operator=(const GenericWithDrop&) = delete;
  GenericWithDrop(::crubit::UnsafeRelocateTag, GenericWithDrop&& value);

  explicit GenericWithDrop(
      ::generics::Wrapper<::generics::NotPartialEqOrDefault> value);

  union {
    ::generics::NotPartialEqOrDefault value;
  };

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericWithDrop_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericWithDrop_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: GenericWithDrop < i32 >") alignas(4)
    [[clang::trivial_abi]] ::generics::GenericWithDrop<::std::int32_t> final {
 public:
  // Type is not a C++ aggregate: Type implements `Drop`

  // Default::default
  GenericWithDrop();

  // Drop::drop
  ~GenericWithDrop();

  GenericWithDrop(GenericWithDrop&&) noexcept;
  ::generics::GenericWithDrop<::std::int32_t>& operator=(
      GenericWithDrop&&) noexcept;

  // `generics_golden::GenericWithDrop` doesn't implement the `Clone` trait
  GenericWithDrop(const GenericWithDrop&) = delete;
  GenericWithDrop& operator=(const GenericWithDrop&) = delete;
  GenericWithDrop(::crubit::UnsafeRelocateTag, GenericWithDrop&& value);

  explicit operator ::generics::Wrapper<::std::int32_t>();

  explicit GenericWithDrop(::generics::Wrapper<::std::int32_t> value);

  union {
    ::std::int32_t value;
  };

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020LifetimeAndTypeGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020LifetimeAndTypeGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: LifetimeAndTypeGeneric < 'static , i32 >") alignas(4)
    [[clang::trivial_abi]] ::generics::LifetimeAndTypeGeneric<
        ::std::int32_t> final {
 public:
  // Type is not a C++ aggregate: Field `reference` is not default-constructible
  // in C++

  // `generics_golden::LifetimeAndTypeGeneric` doesn't implement the `Default`
  // trait
  LifetimeAndTypeGeneric() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~LifetimeAndTypeGeneric() = default;
  LifetimeAndTypeGeneric(LifetimeAndTypeGeneric&&) = default;
  LifetimeAndTypeGeneric& operator=(LifetimeAndTypeGeneric&&) = default;

  // `generics_golden::LifetimeAndTypeGeneric` doesn't implement the `Clone`
  // trait
  LifetimeAndTypeGeneric(const LifetimeAndTypeGeneric&) = delete;
  LifetimeAndTypeGeneric& operator=(const LifetimeAndTypeGeneric&) = delete;
  LifetimeAndTypeGeneric(::crubit::UnsafeRelocateTag,
                         LifetimeAndTypeGeneric&& value);

  union {
    ::std::int32_t const* $static crubit_nonnull reference;
  };

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020PhantomGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020SelfReferentialViaPhantom_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020PhantomGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020SelfReferentialViaPhantom_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: PhantomGeneric < :: generics_golden :: "
    "SelfReferentialViaPhantom >") alignas(4)
    [[clang::trivial_abi]] ::generics::PhantomGeneric<
        ::generics::SelfReferentialViaPhantom> final {
 public:
  // Type is not a C++ aggregate: Field `marker` has a type unsupported in C++

  // `generics_golden::PhantomGeneric` doesn't implement the `Default` trait
  PhantomGeneric() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~PhantomGeneric() = default;
  PhantomGeneric(PhantomGeneric&&) = default;
  PhantomGeneric& operator=(PhantomGeneric&&) = default;

  // `generics_golden::PhantomGeneric` doesn't implement the `Clone` trait
  PhantomGeneric(const PhantomGeneric&) = delete;
  PhantomGeneric& operator=(const PhantomGeneric&) = delete;
  PhantomGeneric(::crubit::UnsafeRelocateTag, PhantomGeneric&& value);

  union {
    ::std::int32_t id;
  };
  // Field `marker` omitted: C++ does not support zero-sized types.
 private:
  static void __crubit_field_offset_assertions();
};

#endif

namespace generics {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: SelfReferentialViaPhantom") alignas(4)
    [[clang::trivial_abi]] SelfReferentialViaPhantom final {
 public:
  // Type is not a C++ aggregate: Field `phantom` is not default-constructible
  // in C++

  // `generics_golden::SelfReferentialViaPhantom` doesn't implement the
  // `Default` trait
  SelfReferentialViaPhantom() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~SelfReferentialViaPhantom() = default;
  SelfReferentialViaPhantom(SelfReferentialViaPhantom&&) = default;
  SelfReferentialViaPhantom& operator=(SelfReferentialViaPhantom&&) = default;

  // `generics_golden::SelfReferentialViaPhantom` doesn't implement the `Clone`
  // trait
  SelfReferentialViaPhantom(const SelfReferentialViaPhantom&) = delete;
  SelfReferentialViaPhantom& operator=(const SelfReferentialViaPhantom&) =
      delete;
  SelfReferentialViaPhantom(::crubit::UnsafeRelocateTag,
                            SelfReferentialViaPhantom&& value);

  union {
    ::generics::PhantomGeneric<::generics::SelfReferentialViaPhantom> phantom;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace generics

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Ping_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Ping_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(":: generics_golden :: Ping < i32 >") alignas(
    4) [[clang::trivial_abi]] ::generics::Ping<::std::int32_t> final {
 public:
  ::generics::Pong<::generics::Wrapper<::std::int32_t>> to_pong() const;

  ::std::int32_t __field0{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020RenamedSubGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020RenamedSubGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: RenamedSubGeneric < i32 >") alignas(4)
    [[clang::trivial_abi]] ::generics::RenamedSubGeneric<::std::int32_t> final {
 public:
  ::std::int32_t item{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Wrapper_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020NotPartialEqOrDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Wrapper_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020NotPartialEqOrDefault_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: Wrapper < :: generics_golden :: "
    "NotPartialEqOrDefault >") alignas(4)
    [[clang::trivial_abi]] ::generics::Wrapper<
        ::generics::NotPartialEqOrDefault> final {
 public:
  explicit
  operator ::generics::GenericWithDrop<::generics::NotPartialEqOrDefault>();

  ::generics::NotPartialEqOrDefault __field0{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Wrapper_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Wrapper_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: Wrapper < i32 >") alignas(4)
    [[clang::trivial_abi]] ::generics::Wrapper<::std::int32_t> final {
 public:
  explicit operator ::generics::GenericWithDrop<::std::int32_t>();

  // Constructor from `<generics_golden::Wrapper<T> as
  // std::convert::From<generics_golden::GenericWithDrop<T>>>` is suppressed
  // because `Wrapper` is a C++ aggregate.

  ::std::int32_t __field0{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Pong_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Wrapper_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Pong_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Wrapper_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: Pong < :: generics_golden :: Wrapper < i32 > "
    ">") alignas(4)
    [[clang::trivial_abi]] ::generics::Pong<
        ::generics::Wrapper<::std::int32_t>> final {
 public:
  ::generics::Wrapper<::std::int32_t> __field0{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020array_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x000000204_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020array_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x000000204_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(4)
    CRUBIT_INTERNAL_RUST_TYPE(":: alloc :: vec :: Vec < [i32 ; 4] >")::rs::Vec<
        ::std::array<::std::int32_t, 4>>
    : public ::rs::VecBase<::std::array<::std::int32_t, 4>> {
 public:
  // Default::default
  Vec() noexcept;

  // Clone::clone
  Vec(const Vec&) noexcept;

  // Clone::clone_from
  ::rs::Vec<::std::array<::std::int32_t, 4>>& operator=(const Vec&) noexcept;

  Vec(Vec&&) noexcept;
  ::rs::Vec<::std::array<::std::int32_t, 4>>& operator=(Vec&&) noexcept;
  Vec(::crubit::UnsafeRelocateTag, Vec&& value);

  ~Vec() noexcept;

 private:
  friend class ::rs::VecBase<::std::array<::std::int32_t, 4>>;
  static constexpr ::std::size_t kPtrOffset = 4;
  static constexpr ::std::size_t kCapOffset = 0;
  static constexpr ::std::size_t kLenOffset = 8;
  unsigned char storage_[12];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020array_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x000000204_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020array_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x000000204_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020array_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x000000204_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020array_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x000000204_x00000020_x0000003e_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: MultiGenericStruct < [i32 ; 4] , [i32 ; 4] "
    ">") alignas(4)
    [[clang::trivial_abi]] ::generics::MultiGenericStruct<
        ::std::array<::std::int32_t, 4>,
        ::std::array<::std::int32_t, 4>> final {
 public:
  ::rs::Vec<::std::array<::std::int32_t, 4>> seconds{};
  ::std::array<::std::int32_t, 4> first{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020double_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020double_x00000020_x0000003e
template <>
struct alignas(4)
    CRUBIT_INTERNAL_RUST_TYPE(":: alloc :: vec :: Vec < f64 >")::rs::Vec<double>
    : public ::rs::VecBase<double> {
 public:
  // Default::default
  Vec() noexcept;

  // Clone::clone
  Vec(const Vec&) noexcept;

  // Clone::clone_from
  ::rs::Vec<double>& operator=(const Vec&) noexcept;

  Vec(Vec&&) noexcept;
  ::rs::Vec<double>& operator=(Vec&&) noexcept;
  Vec(::crubit::UnsafeRelocateTag, Vec&& value);

  ~Vec() noexcept;

 private:
  friend class ::rs::VecBase<double>;
  static constexpr ::std::size_t kPtrOffset = 4;
  static constexpr ::std::size_t kCapOffset = 0;
  static constexpr ::std::size_t kLenOffset = 8;
  unsigned char storage_[12];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020bool_x00000020_x0000002c_x00000020double_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020bool_x00000020_x0000002c_x00000020double_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: MultiGenericStruct < bool , f64 >") alignas(4)
    [[clang::trivial_abi]] ::generics::MultiGenericStruct<bool, double> final {
 public:
  ::rs::Vec<double> seconds{};
  bool first{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    ":: alloc :: vec :: Vec < i32 >")::rs::Vec<::std::int32_t>
    : public ::rs::VecBase<::std::int32_t> {
 public:
  // Default::default
  Vec() noexcept;

  // Clone::clone
  Vec(const Vec&) noexcept;

  // Clone::clone_from
  ::rs::Vec<::std::int32_t>& operator=(const Vec&) noexcept;

  Vec(Vec&&) noexcept;
  ::rs::Vec<::std::int32_t>& operator=(Vec&&) noexcept;
  Vec(::crubit::UnsafeRelocateTag, Vec&& value);

  ~Vec() noexcept;

 private:
  friend class ::rs::VecBase<::std::int32_t>;
  static constexpr ::std::size_t kPtrOffset = 4;
  static constexpr ::std::size_t kCapOffset = 0;
  static constexpr ::std::size_t kLenOffset = 8;
  unsigned char storage_[12];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020bool_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020bool_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: MultiGenericStruct < bool , i32 >") alignas(4)
    [[clang::trivial_abi]] ::generics::MultiGenericStruct<
        bool, ::std::int32_t> final {
 public:
  ::std::tuple<bool, ::std::uintptr_t> first_and_len() const;

  ::rs::Vec<::std::int32_t> seconds{};
  bool first{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: MultiGenericStruct < i32 , i32 >") alignas(4)
    [[clang::trivial_abi]] ::generics::MultiGenericStruct<
        ::std::int32_t, ::std::int32_t> final {
 public:
  ::std::tuple<::std::int32_t, ::std::uintptr_t> first_and_len() const;

  ::rs::Vec<::std::int32_t> seconds{};
  ::std::int32_t first{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    ":: alloc :: vec :: Vec < i64 >")::rs::Vec<::std::int64_t>
    : public ::rs::VecBase<::std::int64_t> {
 public:
  // Default::default
  Vec() noexcept;

  // Clone::clone
  Vec(const Vec&) noexcept;

  // Clone::clone_from
  ::rs::Vec<::std::int64_t>& operator=(const Vec&) noexcept;

  Vec(Vec&&) noexcept;
  ::rs::Vec<::std::int64_t>& operator=(Vec&&) noexcept;
  Vec(::crubit::UnsafeRelocateTag, Vec&& value);

  ~Vec() noexcept;

 private:
  friend class ::rs::VecBase<::std::int64_t>;
  static constexpr ::std::size_t kPtrOffset = 4;
  static constexpr ::std::size_t kCapOffset = 0;
  static constexpr ::std::size_t kLenOffset = 8;
  unsigned char storage_[12];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: MultiGenericStruct < i64 , i64 >") alignas(4)
    [[clang::trivial_abi]] ::generics::MultiGenericStruct<
        ::std::int64_t, ::std::int64_t> final {
 public:
  ::rs::Vec<::std::int64_t> seconds{};
  ::std::int64_t first{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    ":: alloc :: vec :: Vec < u32 >")::rs::Vec<::std::uint32_t>
    : public ::rs::VecBase<::std::uint32_t> {
 public:
  // Default::default
  Vec() noexcept;

  // Clone::clone
  Vec(const Vec&) noexcept;

  // Clone::clone_from
  ::rs::Vec<::std::uint32_t>& operator=(const Vec&) noexcept;

  Vec(Vec&&) noexcept;
  ::rs::Vec<::std::uint32_t>& operator=(Vec&&) noexcept;
  Vec(::crubit::UnsafeRelocateTag, Vec&& value);

  ~Vec() noexcept;

 private:
  friend class ::rs::VecBase<::std::uint32_t>;
  static constexpr ::std::size_t kPtrOffset = 4;
  static constexpr ::std::size_t kCapOffset = 0;
  static constexpr ::std::size_t kLenOffset = 8;
  unsigned char storage_[12];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e

template <>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generics_golden :: MultiGenericStruct < u32 , u32 >") alignas(4)
    [[clang::trivial_abi]] ::generics::MultiGenericStruct<
        ::std::uint32_t, ::std::uint32_t> final {
 public:
  ::rs::Vec<::std::uint32_t> seconds{};
  ::std::uint32_t first{};

 private:
  static void __crubit_field_offset_assertions();
};

#endif

namespace generics {

static_assert(
    sizeof(::generics::NotPartialEqOrDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::NotPartialEqOrDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    ::std::is_trivially_destructible_v<::generics::NotPartialEqOrDefault>);
static_assert(::std::is_trivially_move_constructible_v<
              ::generics::NotPartialEqOrDefault>);
static_assert(
    ::std::is_trivially_move_assignable_v<::generics::NotPartialEqOrDefault>);
inline void ::generics::NotPartialEqOrDefault::
    __crubit_field_offset_assertions() {
  using __crubit_assert_type = ::generics::NotPartialEqOrDefault;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
static_assert(
    sizeof(::generics::OuterWithNestedGenericField) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::OuterWithNestedGenericField) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::generics::OuterWithNestedGenericField>);
static_assert(::std::is_trivially_move_constructible_v<
              ::generics::OuterWithNestedGenericField>);
static_assert(::std::is_trivially_move_assignable_v<
              ::generics::OuterWithNestedGenericField>);
inline void ::generics::OuterWithNestedGenericField::
    __crubit_field_offset_assertions() {
  using __crubit_assert_type = ::generics::OuterWithNestedGenericField;
  static_assert(0 == offsetof(__crubit_assert_type, field));
}
static_assert(
    sizeof(::generics::PartialEqAndDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::PartialEqAndDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    ::std::is_trivially_destructible_v<::generics::PartialEqAndDefault>);
static_assert(
    ::std::is_trivially_move_constructible_v<::generics::PartialEqAndDefault>);
static_assert(
    ::std::is_trivially_move_assignable_v<::generics::PartialEqAndDefault>);
namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
    ::generics::PartialEqAndDefault const&,
    ::generics::PartialEqAndDefault const&);
}
inline bool(PartialEqAndDefault::operator==)(
    ::generics::PartialEqAndDefault const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
          self, other);
}
inline void ::generics::PartialEqAndDefault::
    __crubit_field_offset_assertions() {
  using __crubit_assert_type = ::generics::PartialEqAndDefault;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
static_assert(
    sizeof(::generics::SelfReferentialViaPhantom) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::SelfReferentialViaPhantom) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    ::std::is_trivially_destructible_v<::generics::SelfReferentialViaPhantom>);
static_assert(::std::is_trivially_move_constructible_v<
              ::generics::SelfReferentialViaPhantom>);
static_assert(::std::is_trivially_move_assignable_v<
              ::generics::SelfReferentialViaPhantom>);
inline ::generics::SelfReferentialViaPhantom::SelfReferentialViaPhantom(
    ::crubit::UnsafeRelocateTag, SelfReferentialViaPhantom&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::generics::SelfReferentialViaPhantom::
    __crubit_field_offset_assertions() {
  using __crubit_assert_type = ::generics::SelfReferentialViaPhantom;
  static_assert(0 == offsetof(__crubit_assert_type, phantom));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_accept_ugeneric_uwith_udrop_uunmovable(
    ::generics::GenericWithDrop<::generics::NotPartialEqOrDefault>*);
}
inline void accept_generic_with_drop_unmovable(
    ::rs::Movable<
        ::generics::GenericWithDrop<::generics::NotPartialEqOrDefault>>
        _x) {
  crubit::Slot<::generics::GenericWithDrop<::generics::NotPartialEqOrDefault>>
      _x_slot;
  ::std::move(_x).MoveToSlot(_x_slot);
  return __crubit_internal::
      __crubit_thunk_accept_ugeneric_uwith_udrop_uunmovable(_x_slot.Get());
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_instantiate_uderived_ugeneric_uat_ui32(
    ::generics::DerivedGeneric<::std::int32_t>* __ret_ptr);
}
inline ::generics::DerivedGeneric<::std::int32_t>
instantiate_derived_generic_at_i32() {
  crubit::Slot<::generics::DerivedGeneric<::std::int32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_instantiate_uderived_ugeneric_uat_ui32(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_instantiate_uderived_ugeneric_uat_unot_upartial_ueq_uor_udefault(
    ::generics::DerivedGeneric<::generics::NotPartialEqOrDefault>* __ret_ptr);
}
inline ::generics::DerivedGeneric<::generics::NotPartialEqOrDefault>
instantiate_derived_generic_at_not_partial_eq_or_default() {
  crubit::Slot<::generics::DerivedGeneric<::generics::NotPartialEqOrDefault>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_instantiate_uderived_ugeneric_uat_unot_upartial_ueq_uor_udefault(
          __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_instantiate_ugeneric_uenum(
    ::generics::GenericEnum<::std::int32_t>* __ret_ptr);
}
inline ::generics::GenericEnum<::std::int32_t> instantiate_generic_enum() {
  crubit::Slot<::generics::GenericEnum<::std::int32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_instantiate_ugeneric_uenum(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_instantiate_ugeneric_ustruct_uat_ui32(
    ::generics::GenericStruct<::generics::PartialEqAndDefault>* __ret_ptr);
}
inline ::generics::GenericStruct<::generics::PartialEqAndDefault>
instantiate_generic_struct_at_i32() {
  crubit::Slot<::generics::GenericStruct<::generics::PartialEqAndDefault>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_instantiate_ugeneric_ustruct_uat_ui32(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_instantiate_ugeneric_ustruct_uat_unot_upartial_ueq_uor_udefault(
    ::generics::GenericStruct<::generics::NotPartialEqOrDefault>* __ret_ptr);
}
inline ::generics::GenericStruct<::generics::NotPartialEqOrDefault>
instantiate_generic_struct_at_not_partial_eq_or_default() {
  crubit::Slot<::generics::GenericStruct<::generics::NotPartialEqOrDefault>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_instantiate_ugeneric_ustruct_uat_unot_upartial_ueq_uor_udefault(
          __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_instantiate_ugeneric_ustruct_uat_uprimitive_ui32(
    ::generics::GenericStruct<::std::int32_t>* __ret_ptr);
}
inline ::generics::GenericStruct<::std::int32_t>
instantiate_generic_struct_at_primitive_i32() {
  crubit::Slot<::generics::GenericStruct<::std::int32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_instantiate_ugeneric_ustruct_uat_uprimitive_ui32(
          __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_instantiate_ugeneric_uunion(
    ::generics::GenericUnion<::std::uint32_t>* __ret_ptr);
}
inline ::generics::GenericUnion<::std::uint32_t> instantiate_generic_union() {
  crubit::Slot<::generics::GenericUnion<::std::uint32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_instantiate_ugeneric_uunion(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_instantiate_ugeneric_uwith_udrop_umovable(
    ::generics::GenericWithDrop<::std::int32_t>* __ret_ptr);
}
inline ::generics::GenericWithDrop<::std::int32_t>
instantiate_generic_with_drop_movable() {
  crubit::Slot<::generics::GenericWithDrop<::std::int32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_instantiate_ugeneric_uwith_udrop_umovable(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_instantiate_ugeneric_uwith_udrop_uunmovable(
    ::generics::GenericWithDrop<::generics::NotPartialEqOrDefault>* __ret_ptr);
}
inline ::generics::GenericWithDrop<::generics::NotPartialEqOrDefault>
instantiate_generic_with_drop_unmovable() {
  crubit::Slot<::generics::GenericWithDrop<::generics::NotPartialEqOrDefault>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_instantiate_ugeneric_uwith_udrop_uunmovable(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_instantiate_ulifetime_uand_utype_ugeneric(
    ::std::int32_t const* $(__anon1) crubit_nonnull,
    ::generics::LifetimeAndTypeGeneric<::std::int32_t>* __ret_ptr);
}
inline ::generics::LifetimeAndTypeGeneric<::std::int32_t>
instantiate_lifetime_and_type_generic(
    ::std::int32_t const* $(__anon1) crubit_nonnull x CRUBIT_LIFETIME_BOUND) {
  crubit::Slot<::generics::LifetimeAndTypeGeneric<::std::int32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_instantiate_ulifetime_uand_utype_ugeneric(
      x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_instantiate_umulti_ugeneric_uat_ubool_uf64(
    ::generics::MultiGenericStruct<bool, double>* __ret_ptr);
}
inline ::generics::MultiGenericStruct<bool, double>
instantiate_multi_generic_at_bool_f64() {
  crubit::Slot<::generics::MultiGenericStruct<bool, double>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_instantiate_umulti_ugeneric_uat_ubool_uf64(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_instantiate_umulti_ugeneric_uat_ubool_ui32(
    ::generics::MultiGenericStruct<bool, ::std::int32_t>* __ret_ptr);
}
inline ::generics::MultiGenericStruct<bool, ::std::int32_t>
instantiate_multi_generic_at_bool_i32() {
  crubit::Slot<::generics::MultiGenericStruct<bool, ::std::int32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_instantiate_umulti_ugeneric_uat_ubool_ui32(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_instantiate_uping(
    ::generics::Ping<::std::int32_t>* __ret_ptr);
}
inline ::generics::Ping<::std::int32_t> instantiate_ping() {
  crubit::Slot<::generics::Ping<::std::int32_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_instantiate_uping(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_instantiate_uself_ureferential_uphantom(
    ::generics::SelfReferentialViaPhantom* __ret_ptr);
}
inline ::generics::SelfReferentialViaPhantom
instantiate_self_referential_phantom() {
  crubit::Slot<::generics::SelfReferentialViaPhantom>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_instantiate_uself_ureferential_uphantom(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_instantiate_usub_ugeneric(
    ::generics::RenamedSubGeneric<::std::int32_t>* __ret_ptr);
}
inline ::generics::RenamedSubGeneric<::std::int32_t> instantiate_sub_generic() {
  crubit::Slot<::generics::RenamedSubGeneric<::std::int32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_instantiate_usub_ugeneric(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_instantiate_uwith_uarray(
    ::generics::GenericStruct<::std::array<::std::int32_t, 4>>* __ret_ptr);
}
inline ::generics::GenericStruct<::std::array<::std::int32_t, 4>>
instantiate_with_array() {
  crubit::Slot<::generics::GenericStruct<::std::array<::std::int32_t, 4>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_instantiate_uwith_uarray(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace generics

namespace generics {

template <typename T>
using SubGeneric = ::generics::RenamedSubGeneric<T>;
}

namespace generics::submodule {

template <typename T>
using SubGeneric = ::generics::RenamedSubGeneric<T>;
}

namespace generics {

namespace __crubit_internal {
extern "C" void __crubit_thunk_use_ualiased_ugeneric(
    ::generics::GenericStruct<::std::int64_t>*,
    ::generics::GenericStruct<::std::int64_t>* __ret_ptr);
}
inline ::generics::GenericStruct<::std::int64_t> use_aliased_generic(
    ::generics::GenericStruct<::std::int64_t> x) {
  crubit::Slot<::generics::GenericStruct<::std::int64_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_use_ualiased_ugeneric(
      &x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace generics

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020DerivedGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020NotPartialEqOrDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020DerivedGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020NotPartialEqOrDefault_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::DerivedGeneric<::generics::NotPartialEqOrDefault>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::DerivedGeneric<::generics::NotPartialEqOrDefault>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::generics::DerivedGeneric<::generics::NotPartialEqOrDefault>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::generics::DerivedGeneric<::generics::NotPartialEqOrDefault>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::generics::DerivedGeneric<::generics::NotPartialEqOrDefault>>);
inline void ::generics::DerivedGeneric<
    ::generics::NotPartialEqOrDefault>::__crubit_field_offset_assertions() {
  using __crubit_assert_type =
      ::generics::DerivedGeneric<::generics::NotPartialEqOrDefault>;
  static_assert(0 == offsetof(__crubit_assert_type, value));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020DerivedGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020DerivedGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::DerivedGeneric<::std::int32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::DerivedGeneric<::std::int32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::generics::DerivedGeneric<::std::int32_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::generics::DerivedGeneric<::std::int32_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::generics::DerivedGeneric<::std::int32_t>>);
static_assert(::std::is_trivially_copy_constructible_v<
              ::generics::DerivedGeneric<::std::int32_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::generics::DerivedGeneric<::std::int32_t>>);
namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_ugenerics_ugolden_x0000003a_x0000003aDerivedGeneric_x0000003cT_x0000003e_ugenerics_ugolden_x0000003a_x0000003aDerivedGeneric_x0000003cT_x0000003e_ui32(
    ::generics::DerivedGeneric<::std::int32_t> const&,
    ::generics::DerivedGeneric<::std::int32_t> const&);
}
inline bool(::generics::DerivedGeneric<::std::int32_t>::operator==)(
    ::generics::DerivedGeneric<::std::int32_t> const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_ugenerics_ugolden_x0000003a_x0000003aDerivedGeneric_x0000003cT_x0000003e_ugenerics_ugolden_x0000003a_x0000003aDerivedGeneric_x0000003cT_x0000003e_ui32(
          self, other);
}
namespace __crubit_internal {
extern "C" ::std::uint64_t
__crubit_thunk_Hash_uhash_ugenerics_ugolden_x0000003a_x0000003aDerivedGeneric_x0000003ci32_x0000003e(
    ::generics::DerivedGeneric<::std::int32_t> const&);
}
template <typename H>
inline H AbslHashValue(H h,
                       const ::generics::DerivedGeneric<::std::int32_t>& self) {
  return H::combine(
      ::std::move(h),
      __crubit_internal::
          __crubit_thunk_Hash_uhash_ugenerics_ugolden_x0000003a_x0000003aDerivedGeneric_x0000003ci32_x0000003e(
              self));
}
namespace __crubit_internal {
extern "C" ::std::int8_t
__crubit_thunk_Ord_ucmp_ugenerics_ugolden_x0000003a_x0000003aDerivedGeneric_x0000003ci32_x0000003e(
    ::generics::DerivedGeneric<::std::int32_t> const&,
    ::generics::DerivedGeneric<::std::int32_t> const&);
}
inline ::std::strong_ordering(
    ::generics::DerivedGeneric<::std::int32_t>::operator<=>)(
    const DerivedGeneric& other) const {
  auto val = __crubit_internal::
      __crubit_thunk_Ord_ucmp_ugenerics_ugolden_x0000003a_x0000003aDerivedGeneric_x0000003ci32_x0000003e(
          *this, other);
  switch (val) {
    case -1:
      return ::std::strong_ordering::less;
    case 0:
      return ::std::strong_ordering::equal;
    case 1:
      return ::std::strong_ordering::greater;
    default:
      CRUBIT_UNREACHABLE();
  }
}
inline void ::generics::DerivedGeneric<
    ::std::int32_t>::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::generics::DerivedGeneric<::std::int32_t>;
  static_assert(0 == offsetof(__crubit_assert_type, value));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericEnum_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericEnum_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::GenericEnum<::std::int32_t>) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::GenericEnum<::std::int32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");

// `static` constructor
inline constexpr ::generics::GenericEnum<::std::int32_t>(
    ::generics::GenericEnum<::std::int32_t>::MakeEmpty)() {
  return GenericEnum(PrivateTagCtorTag{}, Tag{INT64_C(0)});
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_Value_ui32(
    ::std::int32_t, ::generics::GenericEnum<::std::int32_t>* __ret_ptr);
}
inline ::generics::GenericEnum<::std::int32_t>(
    ::generics::GenericEnum<::std::int32_t>::MakeValue)(
    ::std::int32_t __param_0) {
  crubit::Slot<::generics::GenericEnum<::std::int32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_Value_ui32(__param_0,
                                               __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
static_assert(::std::is_trivially_destructible_v<
              ::generics::GenericEnum<::std::int32_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::generics::GenericEnum<::std::int32_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::generics::GenericEnum<::std::int32_t>>);
inline ::generics::GenericEnum<::std::int32_t>::GenericEnum(
    ::crubit::UnsafeRelocateTag, GenericEnum&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::generics::GenericEnum<
    ::std::int32_t>::__crubit_field_offset_assertions() {
  {
    using __crubit_assert_type = ::generics::GenericEnum<::std::int32_t>;
    static_assert(4 == offsetof(__crubit_assert_type, Value));
  }
  {
    using __crubit_assert_variant_type =
        ::generics::GenericEnum<::std::int32_t>::__crubit_Value_struct;
    static_assert(0 == offsetof(__crubit_assert_variant_type, __field0));
  }
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020array_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x000000204_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020array_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x000000204_x00000020_x0000003e_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::GenericStruct<::std::array<::std::int32_t, 4>>) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::GenericStruct<::std::array<::std::int32_t, 4>>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::generics::GenericStruct<::std::array<::std::int32_t, 4>>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::generics::GenericStruct<::std::array<::std::int32_t, 4>>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::generics::GenericStruct<::std::array<::std::int32_t, 4>>>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_new_u_x0000005bi32_x0000003b_x000000204_x0000005d(
    void*,
    ::generics::GenericStruct<::std::array<::std::int32_t, 4>>* __ret_ptr);
}
inline ::generics::GenericStruct<::std::array<::std::int32_t, 4>>(
    ::generics::GenericStruct<::std::array<::std::int32_t, 4>>::new_)(
    ::std::array<::std::int32_t, 4> value) {
  crubit::Slot<::generics::GenericStruct<::std::array<::std::int32_t, 4>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_new_u_x0000005bi32_x0000003b_x000000204_x0000005d(
          &value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::array<::std::int32_t, 4> const& $(__anon1)
    __crubit_thunk_get_uref_u_x0000005bi32_x0000003b_x000000204_x0000005d(
        ::generics::GenericStruct<::std::array<::std::int32_t, 4>> const&);
}
inline ::std::array<::std::int32_t, 4> const& $(__anon1)(
    ::generics::GenericStruct<::std::array<::std::int32_t, 4>>::get_ref)()
    const& $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_get_uref_u_x0000005bi32_x0000003b_x000000204_x0000005d(
          self);
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_set_u_x0000005bi32_x0000003b_x000000204_x0000005d(
    ::generics::GenericStruct<::std::array<::std::int32_t, 4>>&, void*);
}
inline void(::generics::GenericStruct<::std::array<::std::int32_t, 4>>::set)(
    ::std::array<::std::int32_t, 4> value) {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_set_u_x0000005bi32_x0000003b_x000000204_x0000005d(self,
                                                                       &value);
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_into_uinner_u_x0000005bi32_x0000003b_x000000204_x0000005d(
    ::generics::GenericStruct<::std::array<::std::int32_t, 4>>*,
    void* __ret_ptr);
}
inline ::std::array<::std::int32_t, 4>(
    ::generics::GenericStruct<
        ::std::array<::std::int32_t, 4>>::into_inner)() && {
  auto&& self = *this;
  crubit::Slot<::std::array<::std::int32_t, 4>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_into_uinner_u_x0000005bi32_x0000003b_x000000204_x0000005d(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t
__crubit_thunk_type_usize_u_x0000005bi32_x0000003b_x000000204_x0000005d();
}
inline ::std::uintptr_t(
    ::generics::GenericStruct<::std::array<::std::int32_t, 4>>::type_size)() {
  return __crubit_internal::
      __crubit_thunk_type_usize_u_x0000005bi32_x0000003b_x000000204_x0000005d();
}

namespace __crubit_internal {
extern "C" bool
__crubit_thunk_is_udefault_u_x0000005bi32_x0000003b_x000000204_x0000005d(
    ::generics::GenericStruct<::std::array<::std::int32_t, 4>> const&);
}
inline bool(
    ::generics::GenericStruct<::std::array<::std::int32_t, 4>>::is_default)()
    const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_is_udefault_u_x0000005bi32_x0000003b_x000000204_x0000005d(
          self);
}

namespace __crubit_internal {
extern "C" ::std::array<::std::int32_t, 4> const& $(__anon1)
    __crubit_thunk_borrow_uwith_uexplicit_ult_u_x0000005bi32_x0000003b_x000000204_x0000005d(
        ::generics::GenericStruct<::std::array<::std::int32_t, 4>> const&);
}
inline ::std::array<::std::int32_t, 4> const& $(__anon1)(
    ::generics::GenericStruct<::std::array<::std::int32_t, 4>>::
        borrow_with_explicit_lt)() const& $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_borrow_uwith_uexplicit_ult_u_x0000005bi32_x0000003b_x000000204_x0000005d(
          self);
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_into_umulti_u_x0000005bi32_x0000003b_x000000204_x0000005d(
    ::generics::GenericStruct<::std::array<::std::int32_t, 4>> const&,
    ::generics::MultiGenericStruct<::std::array<::std::int32_t, 4>,
                                   ::std::array<::std::int32_t, 4>>* __ret_ptr);
}
inline ::generics::MultiGenericStruct<::std::array<::std::int32_t, 4>,
                                      ::std::array<::std::int32_t, 4>>(
    ::generics::GenericStruct<::std::array<::std::int32_t, 4>>::into_multi)()
    const {
  auto&& self = *this;
  crubit::Slot<::generics::MultiGenericStruct<::std::array<::std::int32_t, 4>,
                                              ::std::array<::std::int32_t, 4>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_into_umulti_u_x0000005bi32_x0000003b_x000000204_x0000005d(
          self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::generics::GenericStruct<
    ::std::array<::std::int32_t, 4>>::__crubit_field_offset_assertions() {
  using __crubit_assert_type =
      ::generics::GenericStruct<::std::array<::std::int32_t, 4>>;
  static_assert(0 == offsetof(__crubit_assert_type, value));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::GenericStruct<
           ::generics::GenericStruct<::std::uint32_t>>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::GenericStruct<
            ::generics::GenericStruct<::std::uint32_t>>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    ::std::is_trivially_destructible_v<
        ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>>);
static_assert(
    ::std::is_trivially_move_constructible_v<
        ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>>);
static_assert(
    ::std::is_trivially_move_assignable_v<
        ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_new_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
    ::generics::GenericStruct<::std::uint32_t>*,
    ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>*
        __ret_ptr);
}
inline ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>(
    ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>::
        new_)(::generics::GenericStruct<::std::uint32_t> value) {
  crubit::Slot<
      ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_new_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
          &value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::generics::GenericStruct<::std::uint32_t> const& $(__anon1)
    __crubit_thunk_get_uref_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
        ::generics::GenericStruct<
            ::generics::GenericStruct<::std::uint32_t>> const&);
}
inline ::generics::GenericStruct<::std::uint32_t> const& $(__anon1)(
    ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>::
        get_ref)() const& $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_get_uref_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
          self);
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_set_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
    ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>&,
    ::generics::GenericStruct<::std::uint32_t>*);
}
inline void(
    ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>::set)(
    ::generics::GenericStruct<::std::uint32_t> value) {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_set_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
          self, &value);
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_into_uinner_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
    ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>*,
    ::generics::GenericStruct<::std::uint32_t>* __ret_ptr);
}
inline ::generics::GenericStruct<::std::uint32_t>(
    ::generics::GenericStruct<
        ::generics::GenericStruct<::std::uint32_t>>::into_inner)() && {
  auto&& self = *this;
  crubit::Slot<::generics::GenericStruct<::std::uint32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_into_uinner_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t
__crubit_thunk_type_usize_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e();
}
inline ::std::uintptr_t(
    ::generics::GenericStruct<
        ::generics::GenericStruct<::std::uint32_t>>::type_size)() {
  return __crubit_internal::
      __crubit_thunk_type_usize_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e();
}

namespace __crubit_internal {
extern "C" ::generics::GenericStruct<::std::uint32_t> const& $(__anon1)
    __crubit_thunk_borrow_uwith_uexplicit_ult_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
        ::generics::GenericStruct<
            ::generics::GenericStruct<::std::uint32_t>> const&);
}
inline ::generics::GenericStruct<::std::uint32_t> const& $(__anon1)(
    ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>::
        borrow_with_explicit_lt)() const& $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_borrow_uwith_uexplicit_ult_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
          self);
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_Add_uadd_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
    ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>*,
    ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>*,
    ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>*
        __ret_ptr);
}
inline ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>(
    ::generics::GenericStruct<
        ::generics::GenericStruct<::std::uint32_t>>::operator+)(
    ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>
        rhs) && {
  auto&& self = *this;
  crubit::Slot<
      ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Add_uadd_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
          &self, &rhs, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
extern "C" void
__crubit_thunk_ToString_uto_ustring_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cgenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e_x0000003e(
    ::generics::GenericStruct<
        ::generics::GenericStruct<::std::uint32_t>> const&,
    ::rs::alloc::string::String* __ret_ptr);
inline void ::generics::GenericStruct<::generics::GenericStruct<
    ::std::uint32_t>>::__crubit_field_offset_assertions() {
  using __crubit_assert_type =
      ::generics::GenericStruct<::generics::GenericStruct<::std::uint32_t>>;
  static_assert(0 == offsetof(__crubit_assert_type, value));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020NotPartialEqOrDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020NotPartialEqOrDefault_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::GenericStruct<::generics::NotPartialEqOrDefault>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::GenericStruct<::generics::NotPartialEqOrDefault>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::generics::GenericStruct<::generics::NotPartialEqOrDefault>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::generics::GenericStruct<::generics::NotPartialEqOrDefault>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::generics::GenericStruct<::generics::NotPartialEqOrDefault>>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_new_ugenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault(
    ::generics::NotPartialEqOrDefault*,
    ::generics::GenericStruct<::generics::NotPartialEqOrDefault>* __ret_ptr);
}
inline ::generics::GenericStruct<::generics::NotPartialEqOrDefault>(
    ::generics::GenericStruct<::generics::NotPartialEqOrDefault>::new_)(
    ::generics::NotPartialEqOrDefault value) {
  crubit::Slot<::generics::GenericStruct<::generics::NotPartialEqOrDefault>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_new_ugenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault(
          &value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::generics::NotPartialEqOrDefault const& $(__anon1)
    __crubit_thunk_get_uref_ugenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault(
        ::generics::GenericStruct<::generics::NotPartialEqOrDefault> const&);
}
inline ::generics::NotPartialEqOrDefault const& $(__anon1)(
    ::generics::GenericStruct<::generics::NotPartialEqOrDefault>::get_ref)()
    const& $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_get_uref_ugenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault(
          self);
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_set_ugenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault(
    ::generics::GenericStruct<::generics::NotPartialEqOrDefault>&,
    ::generics::NotPartialEqOrDefault*);
}
inline void(::generics::GenericStruct<::generics::NotPartialEqOrDefault>::set)(
    ::generics::NotPartialEqOrDefault value) {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_set_ugenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault(
          self, &value);
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_into_uinner_ugenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault(
    ::generics::GenericStruct<::generics::NotPartialEqOrDefault>*,
    ::generics::NotPartialEqOrDefault* __ret_ptr);
}
inline ::generics::NotPartialEqOrDefault(
    ::generics::GenericStruct<
        ::generics::NotPartialEqOrDefault>::into_inner)() && {
  auto&& self = *this;
  crubit::Slot<::generics::NotPartialEqOrDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_into_uinner_ugenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t
__crubit_thunk_type_usize_ugenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault();
}
inline ::std::uintptr_t(
    ::generics::GenericStruct<::generics::NotPartialEqOrDefault>::type_size)() {
  return __crubit_internal::
      __crubit_thunk_type_usize_ugenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault();
}

namespace __crubit_internal {
extern "C" ::generics::NotPartialEqOrDefault const& $(__anon1)
    __crubit_thunk_borrow_uwith_uexplicit_ult_ugenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault(
        ::generics::GenericStruct<::generics::NotPartialEqOrDefault> const&);
}
inline ::generics::NotPartialEqOrDefault const& $(__anon1)(
    ::generics::GenericStruct<::generics::NotPartialEqOrDefault>::
        borrow_with_explicit_lt)() const& $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_borrow_uwith_uexplicit_ult_ugenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault(
          self);
}
inline void ::generics::GenericStruct<
    ::generics::NotPartialEqOrDefault>::__crubit_field_offset_assertions() {
  using __crubit_assert_type =
      ::generics::GenericStruct<::generics::NotPartialEqOrDefault>;
  static_assert(0 == offsetof(__crubit_assert_type, value));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020PartialEqAndDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020PartialEqAndDefault_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::GenericStruct<::generics::PartialEqAndDefault>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::GenericStruct<::generics::PartialEqAndDefault>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::generics::GenericStruct<::generics::PartialEqAndDefault>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::generics::GenericStruct<::generics::PartialEqAndDefault>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::generics::GenericStruct<::generics::PartialEqAndDefault>>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_new_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
    ::generics::PartialEqAndDefault*,
    ::generics::GenericStruct<::generics::PartialEqAndDefault>* __ret_ptr);
}
inline ::generics::GenericStruct<::generics::PartialEqAndDefault>(
    ::generics::GenericStruct<::generics::PartialEqAndDefault>::new_)(
    ::generics::PartialEqAndDefault value) {
  crubit::Slot<::generics::GenericStruct<::generics::PartialEqAndDefault>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_new_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
          &value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::generics::PartialEqAndDefault const& $(__anon1)
    __crubit_thunk_get_uref_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
        ::generics::GenericStruct<::generics::PartialEqAndDefault> const&);
}
inline ::generics::PartialEqAndDefault const& $(__anon1)(
    ::generics::GenericStruct<::generics::PartialEqAndDefault>::get_ref)()
    const& $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_get_uref_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
          self);
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_set_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
    ::generics::GenericStruct<::generics::PartialEqAndDefault>&,
    ::generics::PartialEqAndDefault*);
}
inline void(::generics::GenericStruct<::generics::PartialEqAndDefault>::set)(
    ::generics::PartialEqAndDefault value) {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_set_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
          self, &value);
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_into_uinner_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
    ::generics::GenericStruct<::generics::PartialEqAndDefault>*,
    ::generics::PartialEqAndDefault* __ret_ptr);
}
inline ::generics::PartialEqAndDefault(
    ::generics::GenericStruct<
        ::generics::PartialEqAndDefault>::into_inner)() && {
  auto&& self = *this;
  crubit::Slot<::generics::PartialEqAndDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_into_uinner_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t
__crubit_thunk_type_usize_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault();
}
inline ::std::uintptr_t(
    ::generics::GenericStruct<::generics::PartialEqAndDefault>::type_size)() {
  return __crubit_internal::
      __crubit_thunk_type_usize_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault();
}

namespace __crubit_internal {
extern "C" bool
__crubit_thunk_is_udefault_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
    ::generics::GenericStruct<::generics::PartialEqAndDefault> const&);
}
inline bool(
    ::generics::GenericStruct<::generics::PartialEqAndDefault>::is_default)()
    const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_is_udefault_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
          self);
}

namespace __crubit_internal {
extern "C" ::std::int32_t const& $(__anon1)
    __crubit_thunk_deref_uvalue_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
        ::generics::GenericStruct<::generics::PartialEqAndDefault> const&);
}
inline ::std::int32_t const& $(__anon1)(
    ::generics::GenericStruct<::generics::PartialEqAndDefault>::deref_value)()
    const& $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_deref_uvalue_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
          self);
}

namespace __crubit_internal {
extern "C" ::generics::PartialEqAndDefault const& $(__anon1)
    __crubit_thunk_borrow_uwith_uexplicit_ult_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
        ::generics::GenericStruct<::generics::PartialEqAndDefault> const&);
}
inline ::generics::PartialEqAndDefault const& $(__anon1)(
    ::generics::GenericStruct<::generics::PartialEqAndDefault>::
        borrow_with_explicit_lt)() const& $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_borrow_uwith_uexplicit_ult_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
          self);
}
inline void ::generics::GenericStruct<
    ::generics::PartialEqAndDefault>::__crubit_field_offset_assertions() {
  using __crubit_assert_type =
      ::generics::GenericStruct<::generics::PartialEqAndDefault>;
  static_assert(0 == offsetof(__crubit_assert_type, value));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::GenericStruct<::std::int32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::GenericStruct<::std::int32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::generics::GenericStruct<::std::int32_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::generics::GenericStruct<::std::int32_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::generics::GenericStruct<::std::int32_t>>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new_ui32(
    ::std::int32_t, ::generics::GenericStruct<::std::int32_t>* __ret_ptr);
}
inline ::generics::GenericStruct<::std::int32_t>(
    ::generics::GenericStruct<::std::int32_t>::new_)(::std::int32_t value) {
  crubit::Slot<::generics::GenericStruct<::std::int32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new_ui32(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t const& $(__anon1) __crubit_thunk_get_uref_ui32(
    ::generics::GenericStruct<::std::int32_t> const&);
}
inline ::std::int32_t const& $(__anon1)(
    ::generics::GenericStruct<::std::int32_t>::get_ref)() const& $(__anon1)
    CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_uref_ui32(self);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_set_ui32(
    ::generics::GenericStruct<::std::int32_t>&, ::std::int32_t);
}
inline void(::generics::GenericStruct<::std::int32_t>::set)(
    ::std::int32_t value) {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_set_ui32(self, value);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_into_uinner_ui32(
    ::generics::GenericStruct<::std::int32_t>*);
}
inline ::std::int32_t(
    ::generics::GenericStruct<::std::int32_t>::into_inner)() && {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_into_uinner_ui32(&self);
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t __crubit_thunk_type_usize_ui32();
}
inline ::std::uintptr_t(
    ::generics::GenericStruct<::std::int32_t>::type_size)() {
  return __crubit_internal::__crubit_thunk_type_usize_ui32();
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_udefault_ui32(
    ::generics::GenericStruct<::std::int32_t> const&);
}
inline bool(::generics::GenericStruct<::std::int32_t>::is_default)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_udefault_ui32(self);
}
constexpr ::generics::GenericStruct<::std::int32_t>(
    ::generics::GenericStruct<::std::int32_t>::ZERO) =
    ::generics::GenericStruct<::std::int32_t>{.value = INT32_C(0)};
namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_add_uone(
    ::generics::GenericStruct<::std::int32_t> const&);
}
inline ::std::int32_t(
    ::generics::GenericStruct<::std::int32_t>::add_one)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_add_uone(self);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_concrete_uid();
}
inline ::std::int32_t(
    ::generics::GenericStruct<::std::int32_t>::concrete_id)() {
  return __crubit_internal::__crubit_thunk_concrete_uid();
}

namespace __crubit_internal {
extern "C" ::std::int32_t const& $(__anon1)
    __crubit_thunk_borrow_uwith_uexplicit_ult_ui32(
        ::generics::GenericStruct<::std::int32_t> const&);
}
inline ::std::int32_t const& $(__anon1)(
    ::generics::GenericStruct<::std::int32_t>::borrow_with_explicit_lt)()
    const& $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_borrow_uwith_uexplicit_ult_ui32(
      self);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_into_umulti_ui32(
    ::generics::GenericStruct<::std::int32_t> const&,
    ::generics::MultiGenericStruct<::std::int32_t, ::std::int32_t>* __ret_ptr);
}
inline ::generics::MultiGenericStruct<::std::int32_t, ::std::int32_t>(
    ::generics::GenericStruct<::std::int32_t>::into_multi)() const {
  auto&& self = *this;
  crubit::Slot<::generics::MultiGenericStruct<::std::int32_t, ::std::int32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_into_umulti_ui32(self,
                                                     __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_Add_uadd_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ui32(
    ::generics::GenericStruct<::std::int32_t>*,
    ::generics::GenericStruct<::std::int32_t>*,
    ::generics::GenericStruct<::std::int32_t>* __ret_ptr);
}
inline ::generics::GenericStruct<::std::int32_t>(
    ::generics::GenericStruct<::std::int32_t>::operator+)(
    ::generics::GenericStruct<::std::int32_t> rhs) && {
  auto&& self = *this;
  crubit::Slot<::generics::GenericStruct<::std::int32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Add_uadd_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ui32(
          &self, &rhs, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
extern "C" void
__crubit_thunk_ToString_uto_ustring_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003ci32_x0000003e(
    ::generics::GenericStruct<::std::int32_t> const&,
    ::rs::alloc::string::String* __ret_ptr);
inline void ::generics::GenericStruct<
    ::std::int32_t>::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::generics::GenericStruct<::std::int32_t>;
  static_assert(0 == offsetof(__crubit_assert_type, value));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::GenericStruct<::std::int64_t>) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::GenericStruct<::std::int64_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::generics::GenericStruct<::std::int64_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::generics::GenericStruct<::std::int64_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::generics::GenericStruct<::std::int64_t>>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new_ui64(
    ::std::int64_t, ::generics::GenericStruct<::std::int64_t>* __ret_ptr);
}
inline ::generics::GenericStruct<::std::int64_t>(
    ::generics::GenericStruct<::std::int64_t>::new_)(::std::int64_t value) {
  crubit::Slot<::generics::GenericStruct<::std::int64_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new_ui64(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int64_t const& $(__anon1) __crubit_thunk_get_uref_ui64(
    ::generics::GenericStruct<::std::int64_t> const&);
}
inline ::std::int64_t const& $(__anon1)(
    ::generics::GenericStruct<::std::int64_t>::get_ref)() const& $(__anon1)
    CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_uref_ui64(self);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_set_ui64(
    ::generics::GenericStruct<::std::int64_t>&, ::std::int64_t);
}
inline void(::generics::GenericStruct<::std::int64_t>::set)(
    ::std::int64_t value) {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_set_ui64(self, value);
}

namespace __crubit_internal {
extern "C" ::std::int64_t __crubit_thunk_into_uinner_ui64(
    ::generics::GenericStruct<::std::int64_t>*);
}
inline ::std::int64_t(
    ::generics::GenericStruct<::std::int64_t>::into_inner)() && {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_into_uinner_ui64(&self);
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t __crubit_thunk_type_usize_ui64();
}
inline ::std::uintptr_t(
    ::generics::GenericStruct<::std::int64_t>::type_size)() {
  return __crubit_internal::__crubit_thunk_type_usize_ui64();
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_udefault_ui64(
    ::generics::GenericStruct<::std::int64_t> const&);
}
inline bool(::generics::GenericStruct<::std::int64_t>::is_default)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_udefault_ui64(self);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_concrete_uid();
}
inline ::std::int32_t(
    ::generics::GenericStruct<::std::int64_t>::concrete_id)() {
  return __crubit_internal::__crubit_thunk_concrete_uid();
}

namespace __crubit_internal {
extern "C" ::std::int64_t const& $(__anon1)
    __crubit_thunk_borrow_uwith_uexplicit_ult_ui64(
        ::generics::GenericStruct<::std::int64_t> const&);
}
inline ::std::int64_t const& $(__anon1)(
    ::generics::GenericStruct<::std::int64_t>::borrow_with_explicit_lt)()
    const& $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_borrow_uwith_uexplicit_ult_ui64(
      self);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_into_umulti_ui64(
    ::generics::GenericStruct<::std::int64_t> const&,
    ::generics::MultiGenericStruct<::std::int64_t, ::std::int64_t>* __ret_ptr);
}
inline ::generics::MultiGenericStruct<::std::int64_t, ::std::int64_t>(
    ::generics::GenericStruct<::std::int64_t>::into_multi)() const {
  auto&& self = *this;
  crubit::Slot<::generics::MultiGenericStruct<::std::int64_t, ::std::int64_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_into_umulti_ui64(self,
                                                     __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_Add_uadd_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ui64(
    ::generics::GenericStruct<::std::int64_t>*,
    ::generics::GenericStruct<::std::int64_t>*,
    ::generics::GenericStruct<::std::int64_t>* __ret_ptr);
}
inline ::generics::GenericStruct<::std::int64_t>(
    ::generics::GenericStruct<::std::int64_t>::operator+)(
    ::generics::GenericStruct<::std::int64_t> rhs) && {
  auto&& self = *this;
  crubit::Slot<::generics::GenericStruct<::std::int64_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Add_uadd_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ui64(
          &self, &rhs, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
extern "C" void
__crubit_thunk_ToString_uto_ustring_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003ci64_x0000003e(
    ::generics::GenericStruct<::std::int64_t> const&,
    ::rs::alloc::string::String* __ret_ptr);
inline void ::generics::GenericStruct<
    ::std::int64_t>::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::generics::GenericStruct<::std::int64_t>;
  static_assert(0 == offsetof(__crubit_assert_type, value));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::GenericStruct<::std::uint32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::GenericStruct<::std::uint32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::generics::GenericStruct<::std::uint32_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::generics::GenericStruct<::std::uint32_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::generics::GenericStruct<::std::uint32_t>>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new_uu32(
    ::std::uint32_t, ::generics::GenericStruct<::std::uint32_t>* __ret_ptr);
}
inline ::generics::GenericStruct<::std::uint32_t>(
    ::generics::GenericStruct<::std::uint32_t>::new_)(::std::uint32_t value) {
  crubit::Slot<::generics::GenericStruct<::std::uint32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new_uu32(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::uint32_t const& $(__anon1) __crubit_thunk_get_uref_uu32(
    ::generics::GenericStruct<::std::uint32_t> const&);
}
inline ::std::uint32_t const& $(__anon1)(
    ::generics::GenericStruct<::std::uint32_t>::get_ref)() const& $(__anon1)
    CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_uref_uu32(self);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_set_uu32(
    ::generics::GenericStruct<::std::uint32_t>&, ::std::uint32_t);
}
inline void(::generics::GenericStruct<::std::uint32_t>::set)(
    ::std::uint32_t value) {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_set_uu32(self, value);
}

namespace __crubit_internal {
extern "C" ::std::uint32_t __crubit_thunk_into_uinner_uu32(
    ::generics::GenericStruct<::std::uint32_t>*);
}
inline ::std::uint32_t(
    ::generics::GenericStruct<::std::uint32_t>::into_inner)() && {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_into_uinner_uu32(&self);
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t __crubit_thunk_type_usize_uu32();
}
inline ::std::uintptr_t(
    ::generics::GenericStruct<::std::uint32_t>::type_size)() {
  return __crubit_internal::__crubit_thunk_type_usize_uu32();
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_udefault_uu32(
    ::generics::GenericStruct<::std::uint32_t> const&);
}
inline bool(::generics::GenericStruct<::std::uint32_t>::is_default)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_udefault_uu32(self);
}

namespace __crubit_internal {
extern "C" ::std::uint32_t const& $(__anon1)
    __crubit_thunk_borrow_uwith_uexplicit_ult_uu32(
        ::generics::GenericStruct<::std::uint32_t> const&);
}
inline ::std::uint32_t const& $(__anon1)(
    ::generics::GenericStruct<::std::uint32_t>::borrow_with_explicit_lt)()
    const& $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_borrow_uwith_uexplicit_ult_uu32(
      self);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_into_umulti_uu32(
    ::generics::GenericStruct<::std::uint32_t> const&,
    ::generics::MultiGenericStruct<::std::uint32_t, ::std::uint32_t>*
        __ret_ptr);
}
inline ::generics::MultiGenericStruct<::std::uint32_t, ::std::uint32_t>(
    ::generics::GenericStruct<::std::uint32_t>::into_multi)() const {
  auto&& self = *this;
  crubit::Slot<::generics::MultiGenericStruct<::std::uint32_t, ::std::uint32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_into_umulti_uu32(self,
                                                     __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_Add_uadd_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_uu32(
    ::generics::GenericStruct<::std::uint32_t>*,
    ::generics::GenericStruct<::std::uint32_t>*,
    ::generics::GenericStruct<::std::uint32_t>* __ret_ptr);
}
inline ::generics::GenericStruct<::std::uint32_t>(
    ::generics::GenericStruct<::std::uint32_t>::operator+)(
    ::generics::GenericStruct<::std::uint32_t> rhs) && {
  auto&& self = *this;
  crubit::Slot<::generics::GenericStruct<::std::uint32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Add_uadd_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_uu32(
          &self, &rhs, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
extern "C" void
__crubit_thunk_ToString_uto_ustring_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
    ::generics::GenericStruct<::std::uint32_t> const&,
    ::rs::alloc::string::String* __ret_ptr);
inline void ::generics::GenericStruct<
    ::std::uint32_t>::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::generics::GenericStruct<::std::uint32_t>;
  static_assert(0 == offsetof(__crubit_assert_type, value));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericUnion_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericUnion_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::GenericUnion<::std::uint32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::GenericUnion<::std::uint32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::generics::GenericUnion<::std::uint32_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::generics::GenericUnion<::std::uint32_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::generics::GenericUnion<::std::uint32_t>>);
inline ::generics::GenericUnion<::std::uint32_t>::GenericUnion(
    ::crubit::UnsafeRelocateTag, GenericUnion&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::generics::GenericUnion<
    ::std::uint32_t>::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::generics::GenericUnion<::std::uint32_t>;
  static_assert(0 == offsetof(__crubit_assert_type, val));
  static_assert(0 == offsetof(__crubit_assert_type, raw));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericWithDrop_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020NotPartialEqOrDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericWithDrop_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020NotPartialEqOrDefault_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::GenericWithDrop<::generics::NotPartialEqOrDefault>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::GenericWithDrop<::generics::NotPartialEqOrDefault>) ==
        4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003cgenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault_x0000003e(
    ::generics::GenericWithDrop<::generics::NotPartialEqOrDefault>&);
}
inline ::generics::GenericWithDrop<
    ::generics::NotPartialEqOrDefault>::~GenericWithDrop() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003cgenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault_x0000003e(
          *this);
}
inline ::generics::GenericWithDrop<::generics::NotPartialEqOrDefault>::
    GenericWithDrop(::crubit::UnsafeRelocateTag, GenericWithDrop&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_ugenerics_ugolden_x0000003a_x0000003aWrapper_x0000003cgenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003cgenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault_x0000003e(
    ::generics::Wrapper<::generics::NotPartialEqOrDefault>*,
    ::generics::GenericWithDrop<::generics::NotPartialEqOrDefault>* __ret_ptr);
}
inline ::generics::GenericWithDrop<::generics::NotPartialEqOrDefault>::
    GenericWithDrop(
        ::generics::Wrapper<::generics::NotPartialEqOrDefault> value) {
  __crubit_internal::
      __crubit_thunk_Into_uinto_ugenerics_ugolden_x0000003a_x0000003aWrapper_x0000003cgenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003cgenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault_x0000003e(
          &value, this);
}
inline void ::generics::GenericWithDrop<
    ::generics::NotPartialEqOrDefault>::__crubit_field_offset_assertions() {
  using __crubit_assert_type =
      ::generics::GenericWithDrop<::generics::NotPartialEqOrDefault>;
  static_assert(0 == offsetof(__crubit_assert_type, value));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericWithDrop_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020GenericWithDrop_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::GenericWithDrop<::std::int32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::GenericWithDrop<::std::int32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003ci32_x0000003e(
    ::generics::GenericWithDrop<::std::int32_t>* __ret_ptr);
}
inline ::generics::GenericWithDrop<::std::int32_t>::GenericWithDrop() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003ci32_x0000003e(
          this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003ci32_x0000003e(
    ::generics::GenericWithDrop<::std::int32_t>&);
}
inline ::generics::GenericWithDrop<::std::int32_t>::~GenericWithDrop() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003ci32_x0000003e(
          *this);
}
inline ::generics::GenericWithDrop<::std::int32_t>::GenericWithDrop(
    GenericWithDrop&& other) noexcept
    : GenericWithDrop() {
  *this = ::std::move(other);
}
inline ::generics::GenericWithDrop<::std::int32_t>& ::generics::GenericWithDrop<
    ::std::int32_t>::operator=(GenericWithDrop&& other) noexcept {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::generics::GenericWithDrop<::std::int32_t>::GenericWithDrop(
    ::crubit::UnsafeRelocateTag, GenericWithDrop&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003ci32_x0000003e_ugenerics_ugolden_x0000003a_x0000003aWrapper_x0000003ci32_x0000003e(
    ::generics::GenericWithDrop<::std::int32_t>*,
    ::generics::Wrapper<::std::int32_t>* __ret_ptr);
}
inline ::generics::GenericWithDrop<
    ::std::int32_t>::operator ::generics::Wrapper<::std::int32_t>() {
  auto&& self = *this;
  crubit::Slot self_slot((::std::move(self)));
  crubit::Slot<::generics::Wrapper<::std::int32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Into_uinto_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003ci32_x0000003e_ugenerics_ugolden_x0000003a_x0000003aWrapper_x0000003ci32_x0000003e(
          self_slot.Get(), __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_ugenerics_ugolden_x0000003a_x0000003aWrapper_x0000003ci32_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003ci32_x0000003e(
    ::generics::Wrapper<::std::int32_t>*,
    ::generics::GenericWithDrop<::std::int32_t>* __ret_ptr);
}
inline ::generics::GenericWithDrop<::std::int32_t>::GenericWithDrop(
    ::generics::Wrapper<::std::int32_t> value) {
  __crubit_internal::
      __crubit_thunk_Into_uinto_ugenerics_ugolden_x0000003a_x0000003aWrapper_x0000003ci32_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003ci32_x0000003e(
          &value, this);
}
inline void ::generics::GenericWithDrop<
    ::std::int32_t>::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::generics::GenericWithDrop<::std::int32_t>;
  static_assert(0 == offsetof(__crubit_assert_type, value));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020LifetimeAndTypeGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020LifetimeAndTypeGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::LifetimeAndTypeGeneric<::std::int32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::LifetimeAndTypeGeneric<::std::int32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::generics::LifetimeAndTypeGeneric<::std::int32_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::generics::LifetimeAndTypeGeneric<::std::int32_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::generics::LifetimeAndTypeGeneric<::std::int32_t>>);
inline ::generics::LifetimeAndTypeGeneric<
    ::std::int32_t>::LifetimeAndTypeGeneric(::crubit::UnsafeRelocateTag,
                                            LifetimeAndTypeGeneric&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::generics::LifetimeAndTypeGeneric<
    ::std::int32_t>::__crubit_field_offset_assertions() {
  using __crubit_assert_type =
      ::generics::LifetimeAndTypeGeneric<::std::int32_t>;
  static_assert(0 == offsetof(__crubit_assert_type, reference));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020bool_x00000020_x0000002c_x00000020double_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020bool_x00000020_x0000002c_x00000020double_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::MultiGenericStruct<bool, double>) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::MultiGenericStruct<bool, double>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
inline void ::generics::MultiGenericStruct<
    bool, double>::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::generics::MultiGenericStruct<bool, double>;
  static_assert(0 == offsetof(__crubit_assert_type, seconds));
  static_assert(12 == offsetof(__crubit_assert_type, first));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020bool_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020bool_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::MultiGenericStruct<bool, ::std::int32_t>) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::MultiGenericStruct<bool, ::std::int32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_first_uand_ulen_ubool(
    ::generics::MultiGenericStruct<bool, ::std::int32_t> const&,
    void** __ret_ptr);
}
inline ::std::tuple<bool, ::std::uintptr_t>(
    ::generics::MultiGenericStruct<bool, ::std::int32_t>::first_and_len)()
    const {
  auto&& self = *this;
  bool __return_value_0_ret_val_holder;
  bool* __return_value_0_storage = &__return_value_0_ret_val_holder;
  ::std::uintptr_t __return_value_1_ret_val_holder;
  ::std::uintptr_t* __return_value_1_storage = &__return_value_1_ret_val_holder;
  void* __return_value_storage[] = {__return_value_0_storage,
                                    __return_value_1_storage};
  __crubit_internal::__crubit_thunk_first_uand_ulen_ubool(
      self, __return_value_storage);
  return ::std::make_tuple(*__return_value_0_storage,
                           *__return_value_1_storage);
}
inline void ::generics::MultiGenericStruct<
    bool, ::std::int32_t>::__crubit_field_offset_assertions() {
  using __crubit_assert_type =
      ::generics::MultiGenericStruct<bool, ::std::int32_t>;
  static_assert(0 == offsetof(__crubit_assert_type, seconds));
  static_assert(12 == offsetof(__crubit_assert_type, first));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020PhantomGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020SelfReferentialViaPhantom_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020PhantomGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020SelfReferentialViaPhantom_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::PhantomGeneric<::generics::SelfReferentialViaPhantom>) ==
        4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(
        ::generics::PhantomGeneric<::generics::SelfReferentialViaPhantom>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    ::std::is_trivially_destructible_v<
        ::generics::PhantomGeneric<::generics::SelfReferentialViaPhantom>>);
static_assert(
    ::std::is_trivially_move_constructible_v<
        ::generics::PhantomGeneric<::generics::SelfReferentialViaPhantom>>);
static_assert(
    ::std::is_trivially_move_assignable_v<
        ::generics::PhantomGeneric<::generics::SelfReferentialViaPhantom>>);
inline ::generics::PhantomGeneric<::generics::SelfReferentialViaPhantom>::
    PhantomGeneric(::crubit::UnsafeRelocateTag, PhantomGeneric&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::generics::PhantomGeneric<
    ::generics::SelfReferentialViaPhantom>::__crubit_field_offset_assertions() {
  using __crubit_assert_type =
      ::generics::PhantomGeneric<::generics::SelfReferentialViaPhantom>;
  static_assert(0 == offsetof(__crubit_assert_type, id));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Ping_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Ping_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::Ping<::std::int32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::Ping<::std::int32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    ::std::is_trivially_destructible_v<::generics::Ping<::std::int32_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<::generics::Ping<::std::int32_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<::generics::Ping<::std::int32_t>>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_to_upong_ui32(
    ::generics::Ping<::std::int32_t> const&,
    ::generics::Pong<::generics::Wrapper<::std::int32_t>>* __ret_ptr);
}
inline ::generics::Pong<::generics::Wrapper<::std::int32_t>>(
    ::generics::Ping<::std::int32_t>::to_pong)() const {
  auto&& self = *this;
  crubit::Slot<::generics::Pong<::generics::Wrapper<::std::int32_t>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_to_upong_ui32(self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::generics::Ping<
    ::std::int32_t>::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::generics::Ping<::std::int32_t>;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020RenamedSubGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020RenamedSubGeneric_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::RenamedSubGeneric<::std::int32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::RenamedSubGeneric<::std::int32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::generics::RenamedSubGeneric<::std::int32_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::generics::RenamedSubGeneric<::std::int32_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::generics::RenamedSubGeneric<::std::int32_t>>);
inline void ::generics::RenamedSubGeneric<
    ::std::int32_t>::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::generics::RenamedSubGeneric<::std::int32_t>;
  static_assert(0 == offsetof(__crubit_assert_type, item));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020array_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x000000204_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020array_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x000000204_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020array_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x000000204_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020array_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x000000204_x00000020_x0000003e_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::MultiGenericStruct<::std::array<::std::int32_t, 4>,
                                          ::std::array<::std::int32_t, 4>>) ==
        28,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::MultiGenericStruct<::std::array<::std::int32_t, 4>,
                                           ::std::array<::std::int32_t, 4>>) ==
        4,
    "Verify that ADT layout didn't change since this header got generated");
inline void ::generics::MultiGenericStruct<
    ::std::array<::std::int32_t, 4>,
    ::std::array<::std::int32_t, 4>>::__crubit_field_offset_assertions() {
  using __crubit_assert_type =
      ::generics::MultiGenericStruct<::std::array<::std::int32_t, 4>,
                                     ::std::array<::std::int32_t, 4>>;
  static_assert(0 == offsetof(__crubit_assert_type, seconds));
  static_assert(12 == offsetof(__crubit_assert_type, first));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::MultiGenericStruct<::std::int32_t, ::std::int32_t>) ==
        16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::MultiGenericStruct<::std::int32_t, ::std::int32_t>) ==
        4,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_first_uand_ulen_ui32(
    ::generics::MultiGenericStruct<::std::int32_t, ::std::int32_t> const&,
    void** __ret_ptr);
}
inline ::std::tuple<::std::int32_t, ::std::uintptr_t>(
    ::generics::MultiGenericStruct<::std::int32_t,
                                   ::std::int32_t>::first_and_len)() const {
  auto&& self = *this;
  ::std::int32_t __return_value_0_ret_val_holder;
  ::std::int32_t* __return_value_0_storage = &__return_value_0_ret_val_holder;
  ::std::uintptr_t __return_value_1_ret_val_holder;
  ::std::uintptr_t* __return_value_1_storage = &__return_value_1_ret_val_holder;
  void* __return_value_storage[] = {__return_value_0_storage,
                                    __return_value_1_storage};
  __crubit_internal::__crubit_thunk_first_uand_ulen_ui32(
      self, __return_value_storage);
  return ::std::make_tuple(*__return_value_0_storage,
                           *__return_value_1_storage);
}
inline void ::generics::MultiGenericStruct<
    ::std::int32_t, ::std::int32_t>::__crubit_field_offset_assertions() {
  using __crubit_assert_type =
      ::generics::MultiGenericStruct<::std::int32_t, ::std::int32_t>;
  static_assert(0 == offsetof(__crubit_assert_type, seconds));
  static_assert(12 == offsetof(__crubit_assert_type, first));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::MultiGenericStruct<::std::int64_t, ::std::int64_t>) ==
        20,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::MultiGenericStruct<::std::int64_t, ::std::int64_t>) ==
        4,
    "Verify that ADT layout didn't change since this header got generated");
inline void ::generics::MultiGenericStruct<
    ::std::int64_t, ::std::int64_t>::__crubit_field_offset_assertions() {
  using __crubit_assert_type =
      ::generics::MultiGenericStruct<::std::int64_t, ::std::int64_t>;
  static_assert(0 == offsetof(__crubit_assert_type, seconds));
  static_assert(12 == offsetof(__crubit_assert_type, first));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020MultiGenericStruct_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::MultiGenericStruct<::std::uint32_t, ::std::uint32_t>) ==
        16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::MultiGenericStruct<::std::uint32_t, ::std::uint32_t>) ==
        4,
    "Verify that ADT layout didn't change since this header got generated");
inline void ::generics::MultiGenericStruct<
    ::std::uint32_t, ::std::uint32_t>::__crubit_field_offset_assertions() {
  using __crubit_assert_type =
      ::generics::MultiGenericStruct<::std::uint32_t, ::std::uint32_t>;
  static_assert(0 == offsetof(__crubit_assert_type, seconds));
  static_assert(12 == offsetof(__crubit_assert_type, first));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Pong_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Wrapper_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Pong_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Wrapper_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::Pong<::generics::Wrapper<::std::int32_t>>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::Pong<::generics::Wrapper<::std::int32_t>>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::generics::Pong<::generics::Wrapper<::std::int32_t>>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::generics::Pong<::generics::Wrapper<::std::int32_t>>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::generics::Pong<::generics::Wrapper<::std::int32_t>>>);
inline void ::generics::Pong<
    ::generics::Wrapper<::std::int32_t>>::__crubit_field_offset_assertions() {
  using __crubit_assert_type =
      ::generics::Pong<::generics::Wrapper<::std::int32_t>>;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Wrapper_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020NotPartialEqOrDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Wrapper_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020NotPartialEqOrDefault_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::Wrapper<::generics::NotPartialEqOrDefault>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::Wrapper<::generics::NotPartialEqOrDefault>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::generics::Wrapper<::generics::NotPartialEqOrDefault>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::generics::Wrapper<::generics::NotPartialEqOrDefault>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::generics::Wrapper<::generics::NotPartialEqOrDefault>>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_ugenerics_ugolden_x0000003a_x0000003aWrapper_x0000003cgenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003cgenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault_x0000003e(
    ::generics::Wrapper<::generics::NotPartialEqOrDefault>*,
    ::generics::GenericWithDrop<::generics::NotPartialEqOrDefault>* __ret_ptr);
}
inline ::generics::Wrapper<::generics::NotPartialEqOrDefault>::operator ::
    generics::GenericWithDrop<::generics::NotPartialEqOrDefault>() {
  auto&& self = *this;
  crubit::Slot<::generics::GenericWithDrop<::generics::NotPartialEqOrDefault>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Into_uinto_ugenerics_ugolden_x0000003a_x0000003aWrapper_x0000003cgenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003cgenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault_x0000003e(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::generics::Wrapper<
    ::generics::NotPartialEqOrDefault>::__crubit_field_offset_assertions() {
  using __crubit_assert_type =
      ::generics::Wrapper<::generics::NotPartialEqOrDefault>;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Wrapper_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020generics_x00000020_x0000003a_x0000003a_x00000020Wrapper_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e

namespace generics {

static_assert(
    sizeof(::generics::Wrapper<::std::int32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::generics::Wrapper<::std::int32_t>) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    ::std::is_trivially_destructible_v<::generics::Wrapper<::std::int32_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::generics::Wrapper<::std::int32_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<::generics::Wrapper<::std::int32_t>>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_ugenerics_ugolden_x0000003a_x0000003aWrapper_x0000003ci32_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003ci32_x0000003e(
    ::generics::Wrapper<::std::int32_t>*,
    ::generics::GenericWithDrop<::std::int32_t>* __ret_ptr);
}
inline ::generics::Wrapper<
    ::std::int32_t>::operator ::generics::GenericWithDrop<::std::int32_t>() {
  auto&& self = *this;
  crubit::Slot<::generics::GenericWithDrop<::std::int32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Into_uinto_ugenerics_ugolden_x0000003a_x0000003aWrapper_x0000003ci32_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003ci32_x0000003e(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::generics::Wrapper<
    ::std::int32_t>::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::generics::Wrapper<::std::int32_t>;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
}  // namespace generics

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020double_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020double_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cf64_x0000003e(
    ::rs::Vec<double> const&, ::rs::Vec<double>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cf64_x0000003e(
    ::rs::Vec<double>&, ::rs::Vec<double> const&);
}
inline ::rs::Vec<double>::Vec(const Vec& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cf64_x0000003e(
          other, this);
}
inline ::rs::Vec<double>& ::rs::Vec<double>::operator=(
    const Vec& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cf64_x0000003e(
            *this, other);
  }
  return *this;
}
inline ::rs::Vec<double>::Vec(Vec&& other) noexcept : storage_{} {
  ::std::memcpy(storage_, other.storage_, sizeof(storage_));
  other.init_empty();
}
inline ::rs::Vec<double>& ::rs::Vec<double>::operator=(Vec&& other) noexcept {
  if (this != &other) {
    destroy();
    crubit::MemSwap(*this, other);
  }
  return *this;
}
inline ::rs::Vec<double>::Vec(::crubit::UnsafeRelocateTag, Vec&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

inline ::rs::Vec<double>::Vec() noexcept : storage_{} { init_empty(); }
inline ::rs::Vec<double>::~Vec() noexcept { destroy(); }
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
    ::rs::Vec<::std::int32_t> const&, ::rs::Vec<::std::int32_t>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
    ::rs::Vec<::std::int32_t>&, ::rs::Vec<::std::int32_t> const&);
}
inline ::rs::Vec<::std::int32_t>::Vec(const Vec& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
          other, this);
}
inline ::rs::Vec<::std::int32_t>& ::rs::Vec<::std::int32_t>::operator=(
    const Vec& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
            *this, other);
  }
  return *this;
}
inline ::rs::Vec<::std::int32_t>::Vec(Vec&& other) noexcept : storage_{} {
  ::std::memcpy(storage_, other.storage_, sizeof(storage_));
  other.init_empty();
}
inline ::rs::Vec<::std::int32_t>& ::rs::Vec<::std::int32_t>::operator=(
    Vec&& other) noexcept {
  if (this != &other) {
    destroy();
    crubit::MemSwap(*this, other);
  }
  return *this;
}
inline ::rs::Vec<::std::int32_t>::Vec(::crubit::UnsafeRelocateTag,
                                      Vec&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

inline ::rs::Vec<::std::int32_t>::Vec() noexcept : storage_{} { init_empty(); }
inline ::rs::Vec<::std::int32_t>::~Vec() noexcept { destroy(); }
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020array_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x000000204_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020array_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x000000204_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003c_x0000005bi32_x0000003b_x000000204_x0000005d_x0000003e(
    ::rs::Vec<::std::array<::std::int32_t, 4>> const&,
    ::rs::Vec<::std::array<::std::int32_t, 4>>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003c_x0000005bi32_x0000003b_x000000204_x0000005d_x0000003e(
    ::rs::Vec<::std::array<::std::int32_t, 4>>&,
    ::rs::Vec<::std::array<::std::int32_t, 4>> const&);
}
inline ::rs::Vec<::std::array<::std::int32_t, 4>>::Vec(
    const Vec& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003c_x0000005bi32_x0000003b_x000000204_x0000005d_x0000003e(
          other, this);
}
inline ::rs::Vec<::std::array<::std::int32_t, 4>>& ::rs::Vec<
    ::std::array<::std::int32_t, 4>>::operator=(const Vec& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003c_x0000005bi32_x0000003b_x000000204_x0000005d_x0000003e(
            *this, other);
  }
  return *this;
}
inline ::rs::Vec<::std::array<::std::int32_t, 4>>::Vec(Vec&& other) noexcept
    : storage_{} {
  ::std::memcpy(storage_, other.storage_, sizeof(storage_));
  other.init_empty();
}
inline ::rs::Vec<::std::array<::std::int32_t, 4>>& ::rs::Vec<
    ::std::array<::std::int32_t, 4>>::operator=(Vec&& other) noexcept {
  if (this != &other) {
    destroy();
    crubit::MemSwap(*this, other);
  }
  return *this;
}
inline ::rs::Vec<::std::array<::std::int32_t, 4>>::Vec(
    ::crubit::UnsafeRelocateTag, Vec&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

inline ::rs::Vec<::std::array<::std::int32_t, 4>>::Vec() noexcept : storage_{} {
  init_empty();
}
inline ::rs::Vec<::std::array<::std::int32_t, 4>>::~Vec() noexcept {
  destroy();
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci64_x0000003e(
    ::rs::Vec<::std::int64_t> const&, ::rs::Vec<::std::int64_t>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci64_x0000003e(
    ::rs::Vec<::std::int64_t>&, ::rs::Vec<::std::int64_t> const&);
}
inline ::rs::Vec<::std::int64_t>::Vec(const Vec& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci64_x0000003e(
          other, this);
}
inline ::rs::Vec<::std::int64_t>& ::rs::Vec<::std::int64_t>::operator=(
    const Vec& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci64_x0000003e(
            *this, other);
  }
  return *this;
}
inline ::rs::Vec<::std::int64_t>::Vec(Vec&& other) noexcept : storage_{} {
  ::std::memcpy(storage_, other.storage_, sizeof(storage_));
  other.init_empty();
}
inline ::rs::Vec<::std::int64_t>& ::rs::Vec<::std::int64_t>::operator=(
    Vec&& other) noexcept {
  if (this != &other) {
    destroy();
    crubit::MemSwap(*this, other);
  }
  return *this;
}
inline ::rs::Vec<::std::int64_t>::Vec(::crubit::UnsafeRelocateTag,
                                      Vec&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

inline ::rs::Vec<::std::int64_t>::Vec() noexcept : storage_{} { init_empty(); }
inline ::rs::Vec<::std::int64_t>::~Vec() noexcept { destroy(); }
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu32_x0000003e(
    ::rs::Vec<::std::uint32_t> const&, ::rs::Vec<::std::uint32_t>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu32_x0000003e(
    ::rs::Vec<::std::uint32_t>&, ::rs::Vec<::std::uint32_t> const&);
}
inline ::rs::Vec<::std::uint32_t>::Vec(const Vec& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu32_x0000003e(
          other, this);
}
inline ::rs::Vec<::std::uint32_t>& ::rs::Vec<::std::uint32_t>::operator=(
    const Vec& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu32_x0000003e(
            *this, other);
  }
  return *this;
}
inline ::rs::Vec<::std::uint32_t>::Vec(Vec&& other) noexcept : storage_{} {
  ::std::memcpy(storage_, other.storage_, sizeof(storage_));
  other.init_empty();
}
inline ::rs::Vec<::std::uint32_t>& ::rs::Vec<::std::uint32_t>::operator=(
    Vec&& other) noexcept {
  if (this != &other) {
    destroy();
    crubit::MemSwap(*this, other);
  }
  return *this;
}
inline ::rs::Vec<::std::uint32_t>::Vec(::crubit::UnsafeRelocateTag,
                                       Vec&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

inline ::rs::Vec<::std::uint32_t>::Vec() noexcept : storage_{} { init_empty(); }
inline ::rs::Vec<::std::uint32_t>::~Vec() noexcept { destroy(); }
#endif

#pragma clang diagnostic pop
#endif  // CC_BINDINGS_FROM_RS_TEST_GENERICS_GENERICS_GOLDEN
