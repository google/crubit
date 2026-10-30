// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate_golden

// clang-format off
#ifndef EXAMPLES_RUST_ENUM_EXAMPLE_CRATE_GOLDEN
#define EXAMPLES_RUST_ENUM_EXAMPLE_CRATE_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#pragma clang diagnostic ignored "-Wshadow"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/rs_std/enum.h"

#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

namespace example_crate {

struct CRUBIT_INTERNAL_RUST_TYPE(":: example_crate_golden :: Color") alignas(1)
    [[clang::trivial_abi]] Color final {
 public:
  // Default::default
  Color();

  // No custom `Drop` impl and no custom "drop glue" required
  ~Color() = default;
  Color(Color&&) = default;
  Color& operator=(Color&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Color(const Color&) = default;
  Color& operator=(const Color&) = default;
  Color(::crubit::UnsafeRelocateTag, Color&& value);

 public:
  using Discriminant = ::std::uint8_t;

  struct Red
      : public ::crubit::internal::Variant<Red, ::example_crate::Color, 0> {
   public:
    constexpr explicit Red(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    Red(const Red&) = default;
    Red(Red&&) = default;
    Red& operator=(const Red&) = default;
    Red& operator=(Red&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 0;
  };

  struct Blue
      : public ::crubit::internal::Variant<Blue, ::example_crate::Color, 1> {
   public:
    constexpr explicit Blue(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    Blue(const Blue&) = default;
    Blue(Blue&&) = default;
    Blue& operator=(const Blue&) = default;
    Blue& operator=(Blue&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 1;
  };

  struct Green
      : public ::crubit::internal::Variant<Green, ::example_crate::Color, 2> {
   public:
    constexpr explicit Green(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    Green(const Green&) = default;
    Green(Green&&) = default;
    Green& operator=(const Green&) = default;
    Green& operator=(Green&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 2;
  };

  // Constructs the enum holding the variant `__Variant`, built in place from
  // `__args`. `__Variant::Make(__args...)` is shorthand for this.
  template <typename __Variant, typename... __Args>
    requires ::crubit::internal::ConstructibleVariantOf<
        __Variant, ::example_crate::Color, __Args...>
  constexpr explicit Color(::std::in_place_type_t<__Variant> __variant,
                           __Args&&... __args)
      : storage_(__variant, ::std::forward<__Args>(__args)...) {}

 private:
  friend struct ::crubit::internal::EnumAccess;
  ::crubit::internal::VariadicUnionStorage<
      ::crubit::internal::tag_encoding::Direct<::std::uint8_t, 0>, Red, Blue,
      Green>
      storage_;
};

static_assert(
    sizeof(::example_crate::Color) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::example_crate::Color) == 1,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_uexample_ucrate_ugolden_x0000003a_x0000003aColor(
    ::example_crate::Color* __ret_ptr);
}
inline ::example_crate::Color::Color() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_uexample_ucrate_ugolden_x0000003a_x0000003aColor(
          this);
}
static_assert(::std::is_trivially_destructible_v<::example_crate::Color>);
static_assert(::std::is_trivially_move_constructible_v<::example_crate::Color>);
static_assert(::std::is_trivially_move_assignable_v<::example_crate::Color>);
static_assert(::std::is_trivially_copy_constructible_v<::example_crate::Color>);
static_assert(::std::is_trivially_copy_assignable_v<::example_crate::Color>);
inline ::example_crate::Color::Color(::crubit::UnsafeRelocateTag,
                                     Color&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

}  // namespace example_crate

#pragma clang diagnostic pop
#endif  // EXAMPLES_RUST_ENUM_EXAMPLE_CRATE_GOLDEN
