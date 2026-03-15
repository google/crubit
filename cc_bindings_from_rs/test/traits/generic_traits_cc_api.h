// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// generic_traits_golden
// Features: assume_lifetimes, assume_this_lifetimes, callables,
// check_default_initialized, experimental, fmt, supported, types, unsafe_view,
// wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_GENERIC_TRAITS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_GENERIC_TRAITS_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/rs_std/traits.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

namespace generic_traits {

// Generated from:
// cc_bindings_from_rs/test/traits/generic_traits.rs;l=37
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
  ::generic_traits::AnotherStruct& operator=(AnotherStruct&&) = default;

  // `generic_traits_golden::AnotherStruct` doesn't implement the `Clone` trait
  AnotherStruct(const AnotherStruct&) = delete;
  AnotherStruct& operator=(const AnotherStruct&) = delete;
  AnotherStruct(::crubit::UnsafeRelocateTag, AnotherStruct&& value) {
    std::memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/traits/generic_traits.rs;l=38
    std::int32_t y;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/traits/generic_traits.rs;l=9
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
  ::generic_traits::StructGeneric& operator=(StructGeneric&&) = default;

  // `generic_traits_golden::StructGeneric` doesn't implement the `Clone` trait
  StructGeneric(const StructGeneric&) = delete;
  StructGeneric& operator=(const StructGeneric&) = delete;
  StructGeneric(::crubit::UnsafeRelocateTag, StructGeneric&& value) {
    std::memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/traits/generic_traits.rs;l=14
  static ::generic_traits::StructGeneric new_(std::int32_t x);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/traits/generic_traits.rs;l=10
    std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for `generic_traits_golden::TraitWithConst` defined
// at
// cc_bindings_from_rs/test/traits/generic_traits.rs;l=49:
// Trait is not yet supported

// Generated from:
// cc_bindings_from_rs/test/traits/generic_traits.rs;l=5
template <typename T0>
struct CRUBIT_INTERNAL_RUST_TYPE(":: generic_traits_golden :: TraitWithGeneric")
    TraitWithGeneric {
  template <typename T>
  using impl = rs_std::impl<T, TraitWithGeneric<T0>>;
};

// Generated from:
// cc_bindings_from_rs/test/traits/generic_traits.rs;l=26
template <typename T0, typename T1>
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: generic_traits_golden :: TraitWithTwoGenerics") TraitWithTwoGenerics {
  template <typename T>
  using impl = rs_std::impl<T, TraitWithTwoGenerics<T0, T1>>;
};

static_assert(
    sizeof(AnotherStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(AnotherStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<AnotherStruct>);
static_assert(
    std::is_trivially_move_constructible_v<::generic_traits::AnotherStruct>);
static_assert(
    std::is_trivially_move_assignable_v<::generic_traits::AnotherStruct>);
inline void AnotherStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(AnotherStruct, y));
}
static_assert(
    sizeof(StructGeneric) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructGeneric) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<StructGeneric>);
static_assert(
    std::is_trivially_move_constructible_v<::generic_traits::StructGeneric>);
static_assert(
    std::is_trivially_move_assignable_v<::generic_traits::StructGeneric>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(std::int32_t,
                                   ::generic_traits::StructGeneric* __ret_ptr);
}
inline ::generic_traits::StructGeneric StructGeneric::new_(std::int32_t x) {
  crubit::Slot<::generic_traits::StructGeneric> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void StructGeneric::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructGeneric, x));
}
}  // namespace generic_traits

template <>
struct rs_std::impl<::generic_traits::StructGeneric,
                    ::generic_traits::TraitWithGeneric<std::int32_t>> {
  static constexpr bool kIsImplemented = true;

  // Generated from:
  // cc_bindings_from_rs/test/traits/generic_traits.rs;l=21
  static std::int32_t foo(::generic_traits::StructGeneric const& self,
                          std::int32_t t);
};

template <>
struct rs_std::impl<
    ::generic_traits::StructGeneric,
    ::generic_traits::TraitWithTwoGenerics<std::int32_t, std::int32_t>> {
  static constexpr bool kIsImplemented = true;

  // Generated from:
  // cc_bindings_from_rs/test/traits/generic_traits.rs;l=32
  static std::int32_t bar(::generic_traits::StructGeneric const& self,
                          std::int32_t t, std::int32_t u);
};

// Error generating bindings for `<generic_traits_golden::AnotherStruct as
// generic_traits_golden::TraitWithTwoGenerics<i32, U>>` defined at
// cc_bindings_from_rs/test/traits/generic_traits.rs;l=42:
// The following Rust type is not supported yet: U

namespace generic_traits {
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_TraitWithGeneric_ufoo(
    ::generic_traits::StructGeneric const&, std::int32_t);
}
}  // namespace generic_traits
inline std::int32_t
rs_std::impl<::generic_traits::StructGeneric,
             ::generic_traits::TraitWithGeneric<std::int32_t>>::
    foo(::generic_traits::StructGeneric const& self, std::int32_t t) {
  return generic_traits::__crubit_internal::
      __crubit_thunk_TraitWithGeneric_ufoo(self, t);
}

namespace generic_traits {
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_TraitWithTwoGenerics_ubar(
    ::generic_traits::StructGeneric const&, std::int32_t, std::int32_t);
}
}  // namespace generic_traits
inline std::int32_t rs_std::impl<
    ::generic_traits::StructGeneric,
    ::generic_traits::TraitWithTwoGenerics<std::int32_t, std::int32_t>>::
    bar(::generic_traits::StructGeneric const& self, std::int32_t t,
        std::int32_t u) {
  return generic_traits::__crubit_internal::
      __crubit_thunk_TraitWithTwoGenerics_ubar(self, t, u);
}

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_GENERIC_TRAITS_GOLDEN
