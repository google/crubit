// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate_golden

// clang-format off
#ifndef EXAMPLES_RUST_ENUM_WITH_PAYLOAD_EXAMPLE_CRATE_GOLDEN
#define EXAMPLES_RUST_ENUM_WITH_PAYLOAD_EXAMPLE_CRATE_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#pragma clang diagnostic ignored "-Wshadow"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/rs_std/enum.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

namespace example_crate {

struct CRUBIT_INTERNAL_RUST_TYPE(":: example_crate_golden :: Color") alignas(1)
    [[clang::trivial_abi]] Color final {
 public:
  // `example_crate_golden::Color` doesn't implement the `Default` trait
  Color() = delete;

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

  //  A completely transparent color (no payload)
  struct Transparent
      : public ::crubit::internal::Variant<Transparent, ::example_crate::Color,
                                           0> {
   public:
    constexpr explicit Transparent(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    Transparent(const Transparent&) = default;
    Transparent(Transparent&&) = default;
    Transparent& operator=(const Transparent&) = default;
    Transparent& operator=(Transparent&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 0;
  };

  //  A grayscale value from 0 to 255
  struct Grayscale
      : public ::crubit::internal::Variant<Grayscale, ::example_crate::Color,
                                           1> {
   public:
    constexpr explicit Grayscale(::crubit::internal::VariantKey,
                                 ::std::uint8_t __field0)
        : __field0(::std::move(__field0)) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    Grayscale(const Grayscale&) = default;
    Grayscale(Grayscale&&) = default;
    Grayscale& operator=(const Grayscale&) = default;
    Grayscale& operator=(Grayscale&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 1;

   public:
    ::std::uint8_t __field0;
  };

  //  Red, Green, and Blue values from 0 to 255
  struct Rgb
      : public ::crubit::internal::Variant<Rgb, ::example_crate::Color, 2> {
   public:
    constexpr explicit Rgb(::crubit::internal::VariantKey,
                           ::std::uint8_t __field0, ::std::uint8_t __field1,
                           ::std::uint8_t __field2)
        : __field0(::std::move(__field0)),
          __field1(::std::move(__field1)),
          __field2(::std::move(__field2)) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    Rgb(const Rgb&) = default;
    Rgb(Rgb&&) = default;
    Rgb& operator=(const Rgb&) = default;
    Rgb& operator=(Rgb&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 2;

   public:
    ::std::uint8_t __field0;
    ::std::uint8_t __field1;
    ::std::uint8_t __field2;
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
      ::crubit::internal::tag_encoding::Direct<::std::uint8_t, 0>, Transparent,
      Grayscale, Rgb>
      storage_;

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(::example_crate::Color) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::example_crate::Color) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::example_crate::Color>);
static_assert(::std::is_trivially_move_constructible_v<::example_crate::Color>);
static_assert(::std::is_trivially_move_assignable_v<::example_crate::Color>);
static_assert(::std::is_trivially_copy_constructible_v<::example_crate::Color>);
static_assert(::std::is_trivially_copy_assignable_v<::example_crate::Color>);
inline ::example_crate::Color::Color(::crubit::UnsafeRelocateTag,
                                     Color&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::example_crate::Color::__crubit_field_offset_assertions() {
  {
    using __crubit_assert_type = ::example_crate::Color::Grayscale;
    static_assert(1 == offsetof(__crubit_assert_type, __field0));
  }
  {
    using __crubit_assert_type = ::example_crate::Color::Rgb;
    static_assert(1 == offsetof(__crubit_assert_type, __field0));
  }
  {
    using __crubit_assert_type = ::example_crate::Color::Rgb;
    static_assert(2 == offsetof(__crubit_assert_type, __field1));
  }
  {
    using __crubit_assert_type = ::example_crate::Color::Rgb;
    static_assert(3 == offsetof(__crubit_assert_type, __field2));
  }
}
}  // namespace example_crate

#pragma clang diagnostic pop
#endif  // EXAMPLES_RUST_ENUM_WITH_PAYLOAD_EXAMPLE_CRATE_GOLDEN
