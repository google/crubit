// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// generic_traits_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_GENERIC_TRAITS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_GENERIC_TRAITS_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/offsetof.h"
#include "support/internal/slot.h"
#include "support/rs_std/traits.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

namespace generic_traits {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generic_traits_golden :: AnotherStruct") alignas(4)
    [[clang::trivial_abi]] AnotherStruct final {
 public:
  // `generic_traits_golden::AnotherStruct` doesn't implement the `Default`
  // trait
  AnotherStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~AnotherStruct() = default;
  AnotherStruct(AnotherStruct&&) = default;
  AnotherStruct& operator=(AnotherStruct&&) = default;

  // `generic_traits_golden::AnotherStruct` doesn't implement the `Clone` trait
  AnotherStruct(const AnotherStruct&) = delete;
  AnotherStruct& operator=(const AnotherStruct&) = delete;
  AnotherStruct(::crubit::UnsafeRelocateTag, AnotherStruct&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::int32_t y;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generic_traits_golden :: StructGeneric") alignas(4)
    [[clang::trivial_abi]] StructGeneric final {
 public:
  // `generic_traits_golden::StructGeneric` doesn't implement the `Default`
  // trait
  StructGeneric() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~StructGeneric() = default;
  StructGeneric(StructGeneric&&) = default;
  StructGeneric& operator=(StructGeneric&&) = default;

  // `generic_traits_golden::StructGeneric` doesn't implement the `Clone` trait
  StructGeneric(const StructGeneric&) = delete;
  StructGeneric& operator=(const StructGeneric&) = delete;
  StructGeneric(::crubit::UnsafeRelocateTag, StructGeneric&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  static ::generic_traits::StructGeneric new_(::std::int32_t x);

  union {
    ::std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for struct `generic_traits_golden::StructWithAlias`
// defined at
// cc_bindings_from_rs/test/traits/generic_traits.rs;l=60:
// Zero-sized types (ZSTs) are not supported (b/258259459)

// Error generating bindings for trait `generic_traits_golden::TraitWithConst`
// defined at
// cc_bindings_from_rs/test/traits/generic_traits.rs;l=49:
// Trait is not yet supported

template <typename T0>
struct CRUBIT_INTERNAL_RUST_TYPE(":: generic_traits_golden :: TraitWithGeneric")
    TraitWithGeneric {
  template <typename T>
  using impl = rs_std::impl<T, TraitWithGeneric<T0>>;
};

template <typename T0, typename T1>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generic_traits_golden :: TraitWithTwoGenerics") TraitWithTwoGenerics {
  template <typename T>
  using impl = rs_std::impl<T, TraitWithTwoGenerics<T0, T1>>;
};

}  // namespace generic_traits

// Error generating bindings for implementation
// `<generic_traits_golden::AnotherStruct as
// generic_traits_golden::TraitWithTwoGenerics<i32, U>>` defined at
// cc_bindings_from_rs/test/traits/generic_traits.rs;l=42:
// Implementation of traits must specify all types to receive bindings.

template <>
struct rs_std::impl<::generic_traits::StructGeneric,
                    ::generic_traits::TraitWithGeneric<::std::int32_t>> {
  static constexpr bool kIsImplemented = true;

  static ::std::int32_t foo(::generic_traits::StructGeneric const& self,
                            ::std::int32_t t);
};

template <>
struct rs_std::impl<
    ::generic_traits::StructGeneric,
    ::generic_traits::TraitWithTwoGenerics<::std::int32_t, ::std::int32_t>> {
  static constexpr bool kIsImplemented = true;

  static ::std::int32_t bar(::generic_traits::StructGeneric const& self,
                            ::std::int32_t t, ::std::int32_t u);
};

namespace generic_traits {

static_assert(
    sizeof(AnotherStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(AnotherStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<AnotherStruct>);
static_assert(
    ::std::is_trivially_move_constructible_v<::generic_traits::AnotherStruct>);
static_assert(
    ::std::is_trivially_move_assignable_v<::generic_traits::AnotherStruct>);
inline void AnotherStruct::__crubit_field_offset_assertions() {
  CRUBIT_WARNING_PUSH("-Wno-invalid-offsetof")
  static_assert(0 == offsetof(AnotherStruct, y));
  CRUBIT_WARNING_POP
}
static_assert(
    sizeof(StructGeneric) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructGeneric) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<StructGeneric>);
static_assert(
    ::std::is_trivially_move_constructible_v<::generic_traits::StructGeneric>);
static_assert(
    ::std::is_trivially_move_assignable_v<::generic_traits::StructGeneric>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::int32_t,
                                   ::generic_traits::StructGeneric* __ret_ptr);
}
inline ::generic_traits::StructGeneric StructGeneric::new_(::std::int32_t x) {
  crubit::Slot<::generic_traits::StructGeneric> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void StructGeneric::__crubit_field_offset_assertions() {
  CRUBIT_WARNING_PUSH("-Wno-invalid-offsetof")
  static_assert(0 == offsetof(StructGeneric, x));
  CRUBIT_WARNING_POP
}

// Error generating bindings for trait `generic_traits_golden::TraitWithGeneric`
// defined at
// cc_bindings_from_rs/test/traits/generic_traits.rs;l=5:
// Aliases to generic trait `generic_traits_golden::TraitWithGeneric` are not
// supported.

}  // namespace generic_traits

namespace generic_traits {
namespace __crubit_internal {
extern "C" ::std::int32_t
__crubit_thunk_TraitWithGeneric_ufoo_ugeneric_utraits_ugolden_x0000003a_x0000003aStructGeneric_ui32(
    ::generic_traits::StructGeneric const&, ::std::int32_t);
}
}  // namespace generic_traits
inline ::std::int32_t
rs_std::impl<::generic_traits::StructGeneric,
             ::generic_traits::TraitWithGeneric<::std::int32_t>>::
    foo(::generic_traits::StructGeneric const& self, ::std::int32_t t) {
  return generic_traits::__crubit_internal::
      __crubit_thunk_TraitWithGeneric_ufoo_ugeneric_utraits_ugolden_x0000003a_x0000003aStructGeneric_ui32(
          self, t);
}

namespace generic_traits {
namespace __crubit_internal {
extern "C" ::std::int32_t
__crubit_thunk_TraitWithTwoGenerics_ubar_ugeneric_utraits_ugolden_x0000003a_x0000003aStructGeneric_ui32_ui32(
    ::generic_traits::StructGeneric const&, ::std::int32_t, ::std::int32_t);
}
}  // namespace generic_traits
inline ::std::int32_t rs_std::impl<
    ::generic_traits::StructGeneric,
    ::generic_traits::TraitWithTwoGenerics<::std::int32_t, ::std::int32_t>>::
    bar(::generic_traits::StructGeneric const& self, ::std::int32_t t,
        ::std::int32_t u) {
  return generic_traits::__crubit_internal::
      __crubit_thunk_TraitWithTwoGenerics_ubar_ugeneric_utraits_ugolden_x0000003a_x0000003aStructGeneric_ui32_ui32(
          self, t, u);
}

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_GENERIC_TRAITS_GOLDEN
