// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// option_golden

// clang-format off
#ifndef CC_BINDINGS_FROM_RS_TEST_ENUMS_OPTION_GOLDEN
#define CC_BINDINGS_FROM_RS_TEST_ENUMS_OPTION_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#pragma clang diagnostic ignored "-Wshadow"
#include "support/annotations_internal.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/movable.h"
#include "support/rs_std/enum.h"
#include "support/rs_std/option.h"
#include "support/rs_std/result.h"
#include "support/rs_std/str_ref.h"
#include "support/rs_std/unit.h"

#include <array>
#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <optional>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_alloc.h"

namespace option {
struct HasOptions;

// Error generating bindings for struct `option_golden::BridgedType`: Type
// bindings for option_golden::BridgedType suppressed due to being mapped to an
// existing C++ type (decltype(char(0)))

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: CloneNoDefault") alignas(
    1) [[clang::trivial_abi]] CloneNoDefault final {
 public:
  ::std::uint8_t val{};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: CopyNoDefault") alignas(1)
    [[clang::trivial_abi]] CopyNoDefault final {
 public:
  ::std::uint8_t val{};

 private:
  static void __crubit_field_offset_assertions();
};

using FreeFunc CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: FreeFunc") =
    crubit::type_identity_t<void(void*, void*)>*;
// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: HasDefault") alignas(4)
    [[clang::trivial_abi]] HasDefault final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::option::HasDefault new_(::rs::StrRef s);

  // CRUBIT_ANNOTATE: must_bind=
  ::rs::StrRef get_string_inside_option() const& $(__anon1)
      CRUBIT_LIFETIME_BOUND;

  ::rs::alloc::string::String foo{};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: HasNoDefault") alignas(4)
    [[clang::trivial_abi]] HasNoDefault final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::rs::Movable<::option::HasNoDefault> new_(::rs::StrRef s);

  // CRUBIT_ANNOTATE: must_bind=
  ::rs::StrRef get_string_inside_option() const& $(__anon1)
      CRUBIT_LIFETIME_BOUND;

  ::rs::alloc::string::String foo{};
  ::std::uint32_t a{};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: LessThan20U8") alignas(1)
    [[clang::trivial_abi]] LessThan20U8 final {
 public:
  // `option_golden::LessThan20U8` doesn't implement the `Default` trait
  LessThan20U8() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~LessThan20U8() = default;
  LessThan20U8(LessThan20U8&&) = default;
  LessThan20U8& operator=(LessThan20U8&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  LessThan20U8(const LessThan20U8&) = default;
  LessThan20U8& operator=(const LessThan20U8&) = default;
  LessThan20U8(::crubit::UnsafeRelocateTag, LessThan20U8&& value);

  // CRUBIT_ANNOTATE: must_bind=
  static ::rs::Option<::option::LessThan20U8> new_(::std::uint8_t value);

  // CRUBIT_ANNOTATE: must_bind=
  ::std::uint8_t value() const;

 public:
  using Discriminant = ::std::uint8_t;

  struct N0
      : public ::crubit::internal::Variant<N0, ::option::LessThan20U8, 0> {
   public:
    constexpr explicit N0(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N0(const N0&) = default;
    N0(N0&&) = default;
    N0& operator=(const N0&) = default;
    N0& operator=(N0&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 0;
  };

  struct N1
      : public ::crubit::internal::Variant<N1, ::option::LessThan20U8, 1> {
   public:
    constexpr explicit N1(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N1(const N1&) = default;
    N1(N1&&) = default;
    N1& operator=(const N1&) = default;
    N1& operator=(N1&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 1;
  };

  struct N2
      : public ::crubit::internal::Variant<N2, ::option::LessThan20U8, 2> {
   public:
    constexpr explicit N2(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N2(const N2&) = default;
    N2(N2&&) = default;
    N2& operator=(const N2&) = default;
    N2& operator=(N2&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 2;
  };

  struct N3
      : public ::crubit::internal::Variant<N3, ::option::LessThan20U8, 3> {
   public:
    constexpr explicit N3(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N3(const N3&) = default;
    N3(N3&&) = default;
    N3& operator=(const N3&) = default;
    N3& operator=(N3&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 3;
  };

  struct N4
      : public ::crubit::internal::Variant<N4, ::option::LessThan20U8, 4> {
   public:
    constexpr explicit N4(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N4(const N4&) = default;
    N4(N4&&) = default;
    N4& operator=(const N4&) = default;
    N4& operator=(N4&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 4;
  };

  struct N5
      : public ::crubit::internal::Variant<N5, ::option::LessThan20U8, 5> {
   public:
    constexpr explicit N5(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N5(const N5&) = default;
    N5(N5&&) = default;
    N5& operator=(const N5&) = default;
    N5& operator=(N5&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 5;
  };

  struct N6
      : public ::crubit::internal::Variant<N6, ::option::LessThan20U8, 6> {
   public:
    constexpr explicit N6(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N6(const N6&) = default;
    N6(N6&&) = default;
    N6& operator=(const N6&) = default;
    N6& operator=(N6&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 6;
  };

  struct N7
      : public ::crubit::internal::Variant<N7, ::option::LessThan20U8, 7> {
   public:
    constexpr explicit N7(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N7(const N7&) = default;
    N7(N7&&) = default;
    N7& operator=(const N7&) = default;
    N7& operator=(N7&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 7;
  };

  struct N8
      : public ::crubit::internal::Variant<N8, ::option::LessThan20U8, 8> {
   public:
    constexpr explicit N8(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N8(const N8&) = default;
    N8(N8&&) = default;
    N8& operator=(const N8&) = default;
    N8& operator=(N8&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 8;
  };

  struct N9
      : public ::crubit::internal::Variant<N9, ::option::LessThan20U8, 9> {
   public:
    constexpr explicit N9(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N9(const N9&) = default;
    N9(N9&&) = default;
    N9& operator=(const N9&) = default;
    N9& operator=(N9&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 9;
  };

  struct N10
      : public ::crubit::internal::Variant<N10, ::option::LessThan20U8, 10> {
   public:
    constexpr explicit N10(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N10(const N10&) = default;
    N10(N10&&) = default;
    N10& operator=(const N10&) = default;
    N10& operator=(N10&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 10;
  };

  struct N11
      : public ::crubit::internal::Variant<N11, ::option::LessThan20U8, 11> {
   public:
    constexpr explicit N11(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N11(const N11&) = default;
    N11(N11&&) = default;
    N11& operator=(const N11&) = default;
    N11& operator=(N11&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 11;
  };

  struct N12
      : public ::crubit::internal::Variant<N12, ::option::LessThan20U8, 12> {
   public:
    constexpr explicit N12(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N12(const N12&) = default;
    N12(N12&&) = default;
    N12& operator=(const N12&) = default;
    N12& operator=(N12&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 12;
  };

  struct N13
      : public ::crubit::internal::Variant<N13, ::option::LessThan20U8, 13> {
   public:
    constexpr explicit N13(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N13(const N13&) = default;
    N13(N13&&) = default;
    N13& operator=(const N13&) = default;
    N13& operator=(N13&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 13;
  };

  struct N14
      : public ::crubit::internal::Variant<N14, ::option::LessThan20U8, 14> {
   public:
    constexpr explicit N14(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N14(const N14&) = default;
    N14(N14&&) = default;
    N14& operator=(const N14&) = default;
    N14& operator=(N14&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 14;
  };

  struct N15
      : public ::crubit::internal::Variant<N15, ::option::LessThan20U8, 15> {
   public:
    constexpr explicit N15(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N15(const N15&) = default;
    N15(N15&&) = default;
    N15& operator=(const N15&) = default;
    N15& operator=(N15&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 15;
  };

  struct N16
      : public ::crubit::internal::Variant<N16, ::option::LessThan20U8, 16> {
   public:
    constexpr explicit N16(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N16(const N16&) = default;
    N16(N16&&) = default;
    N16& operator=(const N16&) = default;
    N16& operator=(N16&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 16;
  };

  struct N17
      : public ::crubit::internal::Variant<N17, ::option::LessThan20U8, 17> {
   public:
    constexpr explicit N17(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N17(const N17&) = default;
    N17(N17&&) = default;
    N17& operator=(const N17&) = default;
    N17& operator=(N17&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 17;
  };

  struct N18
      : public ::crubit::internal::Variant<N18, ::option::LessThan20U8, 18> {
   public:
    constexpr explicit N18(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N18(const N18&) = default;
    N18(N18&&) = default;
    N18& operator=(const N18&) = default;
    N18& operator=(N18&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 18;
  };

  struct N19
      : public ::crubit::internal::Variant<N19, ::option::LessThan20U8, 19> {
   public:
    constexpr explicit N19(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    N19(const N19&) = default;
    N19(N19&&) = default;
    N19& operator=(const N19&) = default;
    N19& operator=(N19&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 19;
  };

  // Constructs the enum holding the variant `__Variant`, built in place from
  // `__args`. `__Variant::Make(__args...)` is shorthand for this.
  template <typename __Variant, typename... __Args>
    requires ::crubit::internal::ConstructibleVariantOf<
        __Variant, ::option::LessThan20U8, __Args...>
  constexpr explicit LessThan20U8(::std::in_place_type_t<__Variant> __variant,
                                  __Args&&... __args)
      : storage_(__variant, ::std::forward<__Args>(__args)...) {}

 private:
  friend struct ::crubit::internal::EnumAccess;
  ::crubit::internal::VariadicUnionStorage<
      ::crubit::internal::tag_encoding::Direct<::std::uint8_t, 0>, N0, N1, N2,
      N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18,
      N19>
      storage_;
};

// Error generating bindings for struct `option_golden::OptUninhabited`:
// Zero-sized types (ZSTs) are not supported (b/258259459)

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: OptZst") alignas(1)
    [[clang::trivial_abi]] OptZst final {
 public:
  // Type is not a C++ aggregate: Field `val` has a type unsupported in C++

  // Default::default
  OptZst();

  // No custom `Drop` impl and no custom "drop glue" required
  ~OptZst() = default;
  OptZst(OptZst&&) = default;
  OptZst& operator=(OptZst&&) = default;

  // `option_golden::OptZst` doesn't implement the `Clone` trait
  OptZst(const OptZst&) = delete;
  OptZst& operator=(const OptZst&) = delete;
  OptZst(::crubit::UnsafeRelocateTag, OptZst&& value);

 private:
  // Field type has been replaced with a blob of bytes: Failed to format type
  // for the definition of `option_golden::Unit`: Zero-sized types (ZSTs) are
  // not supported (b/258259459)
  ::std::array<unsigned char, 1> val;

 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for enum `option_golden::UninhabitedEnum`:
// Zero-sized types (ZSTs) are not supported (b/258259459)

// Error generating bindings for struct `option_golden::Unit`: Zero-sized types
// (ZSTs) are not supported (b/258259459)

using Voidpf CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: Voidpf") = void*;

// CRUBIT_ANNOTATE: must_bind=
::rs::Option<::std::int32_t const*> pass_option_ptr(
    ::rs::Option<::std::int32_t const*> x);

// CRUBIT_ANNOTATE: must_bind=
::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>
return_option_result();

::rs::Option<::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>
return_option_result_unmovable();

// CRUBIT_ANNOTATE: must_bind=
::rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>>
stress_testing_nested_types();

// CRUBIT_ANNOTATE: must_bind=
::rs::Option<::std::uint32_t> stringify_len(
    ::rs::Option<::option::HasDefault> const& x);

::std::uint8_t take_option_bridged(::rs::Option<decltype(char(0))> x);

void take_option_result_unmovable(
    ::rs::Option<
        ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>
        _x);

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < () >")::rs::Option<::rs::unit_t>
    : public ::rs::OptionBase<::rs::Option<::rs::unit_t>, ::rs::unit_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = ::rs::OptionBase<::rs::Option<::rs::unit_t>, ::rs::unit_t>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::rs::unit_t, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::rs::unit_t, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::rs::unit_t, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::rs::unit_t, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint8_t;
  static constexpr tag_type kNoneVal = 0;
  ::rs::unit_t* some_ptr() noexcept {
    return reinterpret_cast<::rs::unit_t*>(storage_ + 1);
  }
  ::rs::unit_t const* some_const_ptr() const noexcept {
    return reinterpret_cast<::rs::unit_t const*>(storage_ + 1);
  }
  void set_some_tag() noexcept { set_tag(1); }
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char storage_[1];
};
#endif

namespace option {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: UnitOptionField") alignas(
    1) [[clang::trivial_abi]] UnitOptionField final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::option::UnitOptionField new_with_some();

  ::rs::Option<::rs::unit_t> unit{};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020_x0000003e
template <>
struct alignas(4)
    CRUBIT_INTERNAL_RUST_TYPE("core :: option :: Option < * const i32 >")::rs::
        Option<::std::int32_t const*>
    : public ::rs::OptionBase<::rs::Option<::std::int32_t const*>,
                              ::std::int32_t const*> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = ::rs::OptionBase<::rs::Option<::std::int32_t const*>,
                                     ::std::int32_t const*>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::std::int32_t const*, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::std::int32_t const*, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::std::int32_t const*, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::std::int32_t const*, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint32_t;
  static constexpr tag_type kNoneVal = 0;
  ::std::int32_t const** some_ptr() noexcept {
    return reinterpret_cast<::std::int32_t const**>(storage_ + 4);
  }
  ::std::int32_t const* const* some_const_ptr() const noexcept {
    return reinterpret_cast<::std::int32_t const* const*>(storage_ + 4);
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

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < i32 >")::rs::Option<::std::int32_t>
    : public ::rs::OptionBase<::rs::Option<::std::int32_t>, ::std::int32_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      ::rs::OptionBase<::rs::Option<::std::int32_t>, ::std::int32_t>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::std::int32_t, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::std::int32_t, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::std::int32_t, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::std::int32_t, Opt>
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

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < isize >")::rs::Option<::std::int32_t>
    : public ::rs::OptionBase<::rs::Option<::std::int32_t>, ::std::int32_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      ::rs::OptionBase<::rs::Option<::std::int32_t>, ::std::int32_t>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::std::int32_t, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::std::int32_t, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::std::int32_t, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::std::int32_t, Opt>
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

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020decltype_x00000020_x00000028char_x00000020_x000000280_x00000029_x00000029_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020decltype_x00000020_x00000028char_x00000020_x000000280_x00000029_x00000029_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < :: option_golden :: BridgedType >")::rs::
    Option<decltype(char(0))>
    : public ::rs::OptionBase<::rs::Option<decltype(char(0))>,
                              decltype(char(0))> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      ::rs::OptionBase<::rs::Option<decltype(char(0))>, decltype(char(0))>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, decltype(char(0)), U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, decltype(char(0)), U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<decltype(char(0)), Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<decltype(char(0)), Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint8_t;
  static constexpr tag_type kNoneVal = 0;
  decltype(char(0))* some_ptr() noexcept {
    return reinterpret_cast<decltype(char(0))*>(storage_ + 1);
  }
  decltype(char(0)) const* some_const_ptr() const noexcept {
    return reinterpret_cast<decltype(char(0)) const*>(storage_ + 1);
  }
  void set_some_tag() noexcept { set_tag(1); }
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char storage_[2];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < :: option_golden :: CloneNoDefault >")::rs::
    Option<::option::CloneNoDefault>
    : public ::rs::OptionBase<::rs::Option<::option::CloneNoDefault>,
                              ::option::CloneNoDefault> {
 public:
  // Clone::clone
  Option(const Option&) noexcept;

  // Clone::clone_from
  ::rs::Option<::option::CloneNoDefault>& operator=(const Option&) noexcept;

  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = ::rs::OptionBase<::rs::Option<::option::CloneNoDefault>,
                                     ::option::CloneNoDefault>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::option::CloneNoDefault,
                                              U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::option::CloneNoDefault,
                                              U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::option::CloneNoDefault, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::option::CloneNoDefault, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint8_t;
  static constexpr tag_type kNoneVal = 0;
  ::option::CloneNoDefault* some_ptr() noexcept {
    return reinterpret_cast<::option::CloneNoDefault*>(storage_ + 1);
  }
  ::option::CloneNoDefault const* some_const_ptr() const noexcept {
    return reinterpret_cast<::option::CloneNoDefault const*>(storage_ + 1);
  }
  void set_some_tag() noexcept { set_tag(1); }
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char storage_[2];
};
#endif

namespace option {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: option_golden :: OptCloneNoDefault") alignas(1) [[clang::trivial_abi]]
OptCloneNoDefault final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::option::OptCloneNoDefault new_(::std::uint8_t x);

  ::rs::Option<::option::CloneNoDefault> val{};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < :: option_golden :: CopyNoDefault >")::rs::
    Option<::option::CopyNoDefault>
    : public ::rs::OptionBase<::rs::Option<::option::CopyNoDefault>,
                              ::option::CopyNoDefault> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = ::rs::OptionBase<::rs::Option<::option::CopyNoDefault>,
                                     ::option::CopyNoDefault>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::option::CopyNoDefault,
                                              U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::option::CopyNoDefault,
                                              U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::option::CopyNoDefault, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::option::CopyNoDefault, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint8_t;
  static constexpr tag_type kNoneVal = 0;
  ::option::CopyNoDefault* some_ptr() noexcept {
    return reinterpret_cast<::option::CopyNoDefault*>(storage_ + 1);
  }
  ::option::CopyNoDefault const* some_const_ptr() const noexcept {
    return reinterpret_cast<::option::CopyNoDefault const*>(storage_ + 1);
  }
  void set_some_tag() noexcept { set_tag(1); }
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char storage_[2];
};
#endif

namespace option {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: option_golden :: OptCopyNoDefault") alignas(1) [[clang::trivial_abi]]
OptCopyNoDefault final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::option::OptCopyNoDefault new_(::std::uint8_t x);

  ::rs::Option<::option::CopyNoDefault> val{};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < :: option_golden :: HasDefault >")::rs::
    Option<::option::HasDefault>
    : public ::rs::OptionBase<::rs::Option<::option::HasDefault>,
                              ::option::HasDefault> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&) noexcept;
  ::rs::Option<::option::HasDefault>& operator=(Option&&) noexcept;
  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = ::rs::OptionBase<::rs::Option<::option::HasDefault>,
                                     ::option::HasDefault>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::option::HasDefault, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::option::HasDefault, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::option::HasDefault, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::option::HasDefault, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  constexpr ~Option() noexcept;

 private:
  friend base_type;
  using tag_type = ::std::uint32_t;
  static constexpr tag_type kNoneVal = UINT32_C(4294967295);
  ::option::HasDefault* some_ptr() noexcept {
    return reinterpret_cast<::option::HasDefault*>(storage_);
  }
  ::option::HasDefault const* some_const_ptr() const noexcept {
    return reinterpret_cast<::option::HasDefault const*>(storage_);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;

 private:
  unsigned char storage_[12];
};
#endif

namespace option {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: option_golden :: OptDefaultWithDrop") alignas(4) [[clang::trivial_abi]]
OptDefaultWithDrop final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::rs::Movable<::option::OptDefaultWithDrop> new_(::rs::StrRef s);

  ::rs::Option<::option::HasDefault> opt{};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < :: option_golden :: HasNoDefault >")::rs::
    Option<::option::HasNoDefault>
    : public ::rs::OptionBase<::rs::Option<::option::HasNoDefault>,
                              ::option::HasNoDefault> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&) noexcept;
  ::rs::Option<::option::HasNoDefault>& operator=(Option&&) noexcept;
  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = ::rs::OptionBase<::rs::Option<::option::HasNoDefault>,
                                     ::option::HasNoDefault>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::option::HasNoDefault, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::option::HasNoDefault, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::option::HasNoDefault, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::option::HasNoDefault, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  constexpr ~Option() noexcept;

 private:
  friend base_type;
  using tag_type = ::std::uint32_t;
  static constexpr tag_type kNoneVal = UINT32_C(4294967295);
  ::option::HasNoDefault* some_ptr() noexcept {
    return reinterpret_cast<::option::HasNoDefault*>(storage_);
  }
  ::option::HasNoDefault const* some_const_ptr() const noexcept {
    return reinterpret_cast<::option::HasNoDefault const*>(storage_);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;

 private:
  unsigned char storage_[16];
};
#endif

namespace option {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: option_golden :: OptNoDefaultWithDrop") alignas(4)
    [[clang::trivial_abi]] OptNoDefaultWithDrop final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::rs::Movable<::option::OptNoDefaultWithDrop> new_(::rs::StrRef s);

  // CRUBIT_ANNOTATE: must_bind=
  ::rs::StrRef get_string_inside_option() const& $(__anon1)
      CRUBIT_LIFETIME_BOUND;

  ::rs::Option<::option::HasNoDefault> val{};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasOptions_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasOptions_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < :: option_golden :: HasOptions >")::rs::
    Option<::option::HasOptions>
    : public ::rs::OptionBase<::rs::Option<::option::HasOptions>,
                              ::option::HasOptions> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = ::rs::OptionBase<::rs::Option<::option::HasOptions>,
                                     ::option::HasOptions>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::option::HasOptions, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::option::HasOptions, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::option::HasOptions, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::option::HasOptions, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint8_t;
  static constexpr tag_type kNoneVal = 2;
  ::option::HasOptions* some_ptr() noexcept {
    return reinterpret_cast<::option::HasOptions*>(storage_);
  }
  ::option::HasOptions const* some_const_ptr() const noexcept {
    return reinterpret_cast<::option::HasOptions const*>(storage_);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char storage_[4];
};
#endif

namespace option {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: HasHasOptions") alignas(1)
    [[clang::trivial_abi]] HasHasOptions final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::option::HasHasOptions new_(::std::uint8_t value);

  ::rs::Option<::option::HasOptions> me{};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < :: option_golden :: LessThan20U8 >")::rs::
    Option<::option::LessThan20U8>
    : public ::rs::OptionBase<::rs::Option<::option::LessThan20U8>,
                              ::option::LessThan20U8> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = ::rs::OptionBase<::rs::Option<::option::LessThan20U8>,
                                     ::option::LessThan20U8>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::option::LessThan20U8, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::option::LessThan20U8, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::option::LessThan20U8, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::option::LessThan20U8, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint8_t;
  static constexpr tag_type kNoneVal = 255;
  ::option::LessThan20U8* some_ptr() noexcept {
    return reinterpret_cast<::option::LessThan20U8*>(storage_);
  }
  ::option::LessThan20U8 const* some_const_ptr() const noexcept {
    return reinterpret_cast<::option::LessThan20U8 const*>(storage_);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char storage_[1];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < :: core :: option :: Option < :: option_golden "
    ":: LessThan20U8 > >")::rs::Option<::rs::Option<::option::LessThan20U8>>
    : public ::rs::OptionBase<
          ::rs::Option<::rs::Option<::option::LessThan20U8>>,
          ::rs::Option<::option::LessThan20U8>> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      ::rs::OptionBase<::rs::Option<::rs::Option<::option::LessThan20U8>>,
                       ::rs::Option<::option::LessThan20U8>>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<
        Option, ::rs::Option<::option::LessThan20U8>, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<
        Option, ::rs::Option<::option::LessThan20U8>, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::rs::Option<::option::LessThan20U8>,
                                         Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::rs::Option<::option::LessThan20U8>,
                                         Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint8_t;
  static constexpr tag_type kNoneVal = 254;
  ::rs::Option<::option::LessThan20U8>* some_ptr() noexcept {
    return reinterpret_cast<::rs::Option<::option::LessThan20U8>*>(storage_);
  }
  ::rs::Option<::option::LessThan20U8> const* some_const_ptr() const noexcept {
    return reinterpret_cast<::rs::Option<::option::LessThan20U8> const*>(
        storage_);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char storage_[1];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < :: core :: result :: Result < i32 , :: alloc "
    ":: string :: String > >")::rs::
    Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>
    : public ::rs::OptionBase<
          ::rs::Option<
              ::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
          ::rs::Result<::std::int32_t, ::rs::alloc::string::String>> {
 public:
  // Clone::clone
  Option(const Option&) noexcept;

  // Clone::clone_from
  ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>&
  operator=(const Option&) noexcept;

  Option(Option&&) noexcept;
  ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>&
  operator=(Option&&) noexcept;
  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = ::rs::OptionBase<
      ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      ::rs::Result<::std::int32_t, ::rs::alloc::string::String>>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<
        Option, ::rs::Result<::std::int32_t, ::rs::alloc::string::String>, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<
        Option, ::rs::Result<::std::int32_t, ::rs::alloc::string::String>, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<
        ::rs::Result<::std::int32_t, ::rs::alloc::string::String>, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<
        ::rs::Result<::std::int32_t, ::rs::alloc::string::String>, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  constexpr ~Option() noexcept;

 private:
  friend base_type;
  using tag_type = ::std::uint32_t;
  static constexpr tag_type kNoneVal = UINT32_C(4294967294);
  ::rs::Result<::std::int32_t, ::rs::alloc::string::String>*
  some_ptr() noexcept {
    return reinterpret_cast<
        ::rs::Result<::std::int32_t, ::rs::alloc::string::String>*>(storage_);
  }
  ::rs::Result<::std::int32_t, ::rs::alloc::string::String> const*
  some_const_ptr() const noexcept {
    return reinterpret_cast<
        ::rs::Result<::std::int32_t, ::rs::alloc::string::String> const*>(
        storage_);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;

 private:
  unsigned char storage_[12];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < :: core :: result :: Result < :: option_golden "
    ":: HasNoDefault , :: alloc :: string :: String > >")::rs::
    Option<::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>
    : public ::rs::OptionBase<
          ::rs::Option<::rs::Result<::option::HasNoDefault,
                                    ::rs::alloc::string::String>>,
          ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&) noexcept;
  ::rs::Option<
      ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>&
  operator=(Option&&) noexcept;
  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = ::rs::OptionBase<
      ::rs::Option<
          ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>,
      ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<
        Option,
        ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<
        Option,
        ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<
        ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<
        ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  constexpr ~Option() noexcept;

 private:
  friend base_type;
  using tag_type = ::std::uint32_t;
  static constexpr tag_type kNoneVal = UINT32_C(4294967294);
  ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>*
  some_ptr() noexcept {
    return reinterpret_cast<
        ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>*>(
        storage_);
  }
  ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String> const*
  some_const_ptr() const noexcept {
    return reinterpret_cast<::rs::Result<::option::HasNoDefault,
                                         ::rs::alloc::string::String> const*>(
        storage_);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;

 private:
  unsigned char storage_[16];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < :: core :: result :: Result < :: core :: "
    "option :: Option < :: core :: result :: Result < i32 , :: alloc :: string "
    ":: String > > , :: core :: result :: Result < :: core :: option :: Option "
    "< i32 > , :: core :: option :: Option < i32 > > > >")::rs::
    Option<::rs::Result<
        ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        ::rs::Result<::rs::Option<::std::int32_t>,
                     ::rs::Option<::std::int32_t>>>>
    : public ::rs::OptionBase<
          ::rs::Option<
              ::rs::Result<::rs::Option<::rs::Result<
                               ::std::int32_t, ::rs::alloc::string::String>>,
                           ::rs::Result<::rs::Option<::std::int32_t>,
                                        ::rs::Option<::std::int32_t>>>>,
          ::rs::Result<::rs::Option<::rs::Result<::std::int32_t,
                                                 ::rs::alloc::string::String>>,
                       ::rs::Result<::rs::Option<::std::int32_t>,
                                    ::rs::Option<::std::int32_t>>>> {
 public:
  // Clone::clone
  Option(const Option&) noexcept;

  // Clone::clone_from
  ::rs::Option<::rs::Result<
      ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      ::rs::Result<::rs::Option<::std::int32_t>,
                   ::rs::Option<::std::int32_t>>>>&
  operator=(const Option&) noexcept;

  Option(Option&&) noexcept;
  ::rs::Option<::rs::Result<
      ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      ::rs::Result<::rs::Option<::std::int32_t>,
                   ::rs::Option<::std::int32_t>>>>&
  operator=(Option&&) noexcept;
  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = ::rs::OptionBase<
      ::rs::Option<
          ::rs::Result<::rs::Option<::rs::Result<::std::int32_t,
                                                 ::rs::alloc::string::String>>,
                       ::rs::Result<::rs::Option<::std::int32_t>,
                                    ::rs::Option<::std::int32_t>>>>,
      ::rs::Result<::rs::Option<::rs::Result<::std::int32_t,
                                             ::rs::alloc::string::String>>,
                   ::rs::Result<::rs::Option<::std::int32_t>,
                                ::rs::Option<::std::int32_t>>>>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<
        Option,
        ::rs::Result<::rs::Option<::rs::Result<::std::int32_t,
                                               ::rs::alloc::string::String>>,
                     ::rs::Result<::rs::Option<::std::int32_t>,
                                  ::rs::Option<::std::int32_t>>>,
        U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<
        Option,
        ::rs::Result<::rs::Option<::rs::Result<::std::int32_t,
                                               ::rs::alloc::string::String>>,
                     ::rs::Result<::rs::Option<::std::int32_t>,
                                  ::rs::Option<::std::int32_t>>>,
        U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<
        ::rs::Result<::rs::Option<::rs::Result<::std::int32_t,
                                               ::rs::alloc::string::String>>,
                     ::rs::Result<::rs::Option<::std::int32_t>,
                                  ::rs::Option<::std::int32_t>>>,
        Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<
        ::rs::Result<::rs::Option<::rs::Result<::std::int32_t,
                                               ::rs::alloc::string::String>>,
                     ::rs::Result<::rs::Option<::std::int32_t>,
                                  ::rs::Option<::std::int32_t>>>,
        Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  constexpr ~Option() noexcept;

 private:
  friend base_type;
  using tag_type = ::std::uint32_t;
  static constexpr tag_type kNoneVal = 2;
  ::rs::Result<
      ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>*
  some_ptr() noexcept {
    return reinterpret_cast<::rs::Result<
        ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        ::rs::Result<::rs::Option<::std::int32_t>,
                     ::rs::Option<::std::int32_t>>>*>(storage_);
  }
  ::rs::Result<
      ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      ::rs::Result<::rs::Option<::std::int32_t>,
                   ::rs::Option<::std::int32_t>>> const*
  some_const_ptr() const noexcept {
    return reinterpret_cast<::rs::Result<
        ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        ::rs::Result<::rs::Option<::std::int32_t>,
                     ::rs::Option<::std::int32_t>>> const*>(storage_);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;

 private:
  unsigned char storage_[16];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < u32 >")::rs::Option<::std::uint32_t>
    : public ::rs::OptionBase<::rs::Option<::std::uint32_t>, ::std::uint32_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      ::rs::OptionBase<::rs::Option<::std::uint32_t>, ::std::uint32_t>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::std::uint32_t, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::std::uint32_t, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::std::uint32_t, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::std::uint32_t, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint32_t;
  static constexpr tag_type kNoneVal = 0;
  ::std::uint32_t* some_ptr() noexcept {
    return reinterpret_cast<::std::uint32_t*>(storage_ + 4);
  }
  ::std::uint32_t const* some_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint32_t const*>(storage_ + 4);
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

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < u8 >")::rs::Option<::std::uint8_t>
    : public ::rs::OptionBase<::rs::Option<::std::uint8_t>, ::std::uint8_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      ::rs::OptionBase<::rs::Option<::std::uint8_t>, ::std::uint8_t>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::std::uint8_t, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::std::uint8_t, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::std::uint8_t, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::std::uint8_t, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint8_t;
  static constexpr tag_type kNoneVal = 0;
  ::std::uint8_t* some_ptr() noexcept {
    return reinterpret_cast<::std::uint8_t*>(storage_ + 1);
  }
  ::std::uint8_t const* some_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint8_t const*>(storage_ + 1);
  }
  void set_some_tag() noexcept { set_tag(1); }
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char storage_[2];
};
#endif

namespace option {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: HasOptions") alignas(1)
    [[clang::trivial_abi]] HasOptions final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::option::HasOptions new_(::std::uint8_t value);

  // CRUBIT_ANNOTATE: must_bind=
  static ::option::HasOptions with_option(::rs::Option<::std::uint8_t> value);

  // CRUBIT_ANNOTATE: must_bind=
  static ::option::HasOptions from_ref(
      ::rs::Option<::std::uint8_t> const& value);

  // CRUBIT_ANNOTATE: must_bind=
  static ::option::HasOptions with_none();

  ::rs::Option<::std::uint8_t> direct{};
  ::rs::Option<::option::LessThan20U8> niche{};
  ::rs::Option<::rs::Option<::option::LessThan20U8>> nested{};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < unsafe extern \"C\" fn (* mut :: core :: ffi "
    ":: c_void , * mut :: core :: ffi :: c_void) >")::rs::
    Option<crubit::type_identity_t<void(void*, void*)>*>
    : public ::rs::OptionBase<
          ::rs::Option<crubit::type_identity_t<void(void*, void*)>*>,
          crubit::type_identity_t<void(void*, void*)>*> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = ::rs::OptionBase<
      ::rs::Option<crubit::type_identity_t<void(void*, void*)>*>,
      crubit::type_identity_t<void(void*, void*)>*>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<
        Option, crubit::type_identity_t<void(void*, void*)>*, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<
        Option, crubit::type_identity_t<void(void*, void*)>*, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<
        crubit::type_identity_t<void(void*, void*)>*, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<
        crubit::type_identity_t<void(void*, void*)>*, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint32_t;
  static constexpr tag_type kNoneVal = 0;
  crubit::type_identity_t<void(void*, void*)>** some_ptr() noexcept {
    return reinterpret_cast<crubit::type_identity_t<void(void*, void*)>**>(
        storage_);
  }
  crubit::type_identity_t<void(void*, void*)>* const* some_const_ptr()
      const noexcept {
    return reinterpret_cast<
        crubit::type_identity_t<void(void*, void*)>* const*>(storage_);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;

 private:
  unsigned char storage_[4];
};
#endif

namespace option {

struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: ZStream") alignas(4)
    [[clang::trivial_abi]] ZStream final {
 public:
  ::rs::Option<crubit::type_identity_t<void(void*, void*)>*> zfree{};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < usize >")::rs::Option<::std::uint32_t>
    : public ::rs::OptionBase<::rs::Option<::std::uint32_t>, ::std::uint32_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      ::rs::OptionBase<::rs::Option<::std::uint32_t>, ::std::uint32_t>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::std::uint32_t, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::std::uint32_t, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::std::uint32_t, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::std::uint32_t, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint32_t;
  static constexpr tag_type kNoneVal = 0;
  ::std::uint32_t* some_ptr() noexcept {
    return reinterpret_cast<::std::uint32_t*>(storage_ + 4);
  }
  ::std::uint32_t const* some_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint32_t const*>(storage_ + 4);
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

namespace option {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: option_golden :: OptionWithSizeTypes") alignas(4) [[clang::trivial_abi]]
OptionWithSizeTypes final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::option::OptionWithSizeTypes new_(::rs::Option<::std::uint32_t> uval,
                                            ::rs::Option<::std::int32_t> ival);

  ::rs::Option<::std::uint32_t> uval{};
  ::rs::Option<::std::int32_t> ival{};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: result :: Result < i32 , :: alloc :: string :: String >")::rs::
    Result<::std::int32_t, ::rs::alloc::string::String>
    : public ::rs::ResultBase<
          ::rs::Result<::std::int32_t, ::rs::alloc::string::String>,
          ::std::int32_t, ::rs::alloc::string::String> {
 public:
  // Clone::clone
  Result(const Result&) noexcept;

  // Clone::clone_from
  ::rs::Result<::std::int32_t, ::rs::alloc::string::String>& operator=(
      const Result&) noexcept;

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type = ::rs::ResultBase<
      ::rs::Result<::std::int32_t, ::rs::alloc::string::String>, ::std::int32_t,
      ::rs::alloc::string::String>;
  template <typename U>
    requires(::rs::ResultForwardConstructible<Result, ::std::int32_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(::rs::ResultForwardConstructible<Result, ::std::int32_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(
        ::rs::ResultUnexpectedConstructible<::rs::alloc::string::String, F>)
  explicit constexpr Result(::rs::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(
        ::rs::ResultUnexpectedConstructible<::rs::alloc::string::String, F>)
  constexpr Result& operator=(::rs::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(::rs::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == UINT32_C(4294967295); }
  ::std::int32_t* ok_ptr() noexcept {
    return reinterpret_cast<::std::int32_t*>(__storage + 4);
  }
  ::std::int32_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::std::int32_t const*>(__storage + 4);
  }
  ::rs::alloc::string::String* err_ptr() noexcept {
    return reinterpret_cast<::rs::alloc::string::String*>(__storage);
  }
  ::rs::alloc::string::String const* err_const_ptr() const noexcept {
    return reinterpret_cast<::rs::alloc::string::String const*>(__storage);
  }
  void set_ok_tag() noexcept { set_tag(UINT32_C(4294967295)); }
  void set_err_tag() noexcept {}
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;

 private:
  unsigned char __storage[12];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: result :: Result < :: option_golden :: HasNoDefault , :: alloc :: "
    "string :: String >")::rs::Result<::option::HasNoDefault,
                                      ::rs::alloc::string::String>
    : public ::rs::ResultBase<
          ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>,
          ::option::HasNoDefault, ::rs::alloc::string::String> {
 public:
  // `core::result::Result` doesn't implement the `Clone` trait
  Result(const Result&) = delete;
  Result& operator=(const Result&) = delete;
  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  Result(Result&&) = delete;
  ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>& operator=(
      Result&&) = delete;
  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type = ::rs::ResultBase<
      ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>,
      ::option::HasNoDefault, ::rs::alloc::string::String>;
  template <typename U>
    requires(
        ::rs::ResultForwardConstructible<Result, ::option::HasNoDefault, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(
        ::rs::ResultForwardConstructible<Result, ::option::HasNoDefault, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(
        ::rs::ResultUnexpectedConstructible<::rs::alloc::string::String, F>)
  explicit constexpr Result(::rs::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(
        ::rs::ResultUnexpectedConstructible<::rs::alloc::string::String, F>)
  constexpr Result& operator=(::rs::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(::rs::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() != UINT32_C(4294967295); }
  ::option::HasNoDefault* ok_ptr() noexcept {
    return reinterpret_cast<::option::HasNoDefault*>(__storage);
  }
  ::option::HasNoDefault const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::option::HasNoDefault const*>(__storage);
  }
  ::rs::alloc::string::String* err_ptr() noexcept {
    return reinterpret_cast<::rs::alloc::string::String*>(__storage + 4);
  }
  ::rs::alloc::string::String const* err_const_ptr() const noexcept {
    return reinterpret_cast<::rs::alloc::string::String const*>(__storage + 4);
  }
  void set_ok_tag() noexcept {}
  void set_err_tag() noexcept { set_tag(UINT32_C(4294967295)); }
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;

 private:
  unsigned char __storage[16];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: result :: Result < :: core :: option :: Option < i32 > , :: core "
    ":: option :: Option < i32 > >")::rs::Result<::rs::Option<::std::int32_t>,
                                                 ::rs::Option<::std::int32_t>>
    : public ::rs::ResultBase<::rs::Result<::rs::Option<::std::int32_t>,
                                           ::rs::Option<::std::int32_t>>,
                              ::rs::Option<::std::int32_t>,
                              ::rs::Option<::std::int32_t>> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Result(const Result&) = default;
  Result& operator=(const Result&) = default;
  Result(Result&&) = default;
  Result& operator=(Result&&) = default;

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type = ::rs::ResultBase<
      ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>,
      ::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>;
  template <typename U>
    requires(::rs::ResultForwardConstructible<Result,
                                              ::rs::Option<::std::int32_t>, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(::rs::ResultForwardConstructible<Result,
                                              ::rs::Option<::std::int32_t>, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(
        ::rs::ResultUnexpectedConstructible<::rs::Option<::std::int32_t>, F>)
  explicit constexpr Result(::rs::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(
        ::rs::ResultUnexpectedConstructible<::rs::Option<::std::int32_t>, F>)
  constexpr Result& operator=(::rs::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(::rs::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  ::rs::Option<::std::int32_t>* ok_ptr() noexcept {
    return reinterpret_cast<::rs::Option<::std::int32_t>*>(__storage + 4);
  }
  ::rs::Option<::std::int32_t> const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::rs::Option<::std::int32_t> const*>(__storage + 4);
  }
  ::rs::Option<::std::int32_t>* err_ptr() noexcept {
    return reinterpret_cast<::rs::Option<::std::int32_t>*>(__storage + 4);
  }
  ::rs::Option<::std::int32_t> const* err_const_ptr() const noexcept {
    return reinterpret_cast<::rs::Option<::std::int32_t> const*>(__storage + 4);
  }
  void set_ok_tag() noexcept { set_tag(0); }
  void set_err_tag() noexcept { set_tag(1); }
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;

 private:
  unsigned char __storage[12];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: result :: Result < :: core :: option :: Option < :: core :: "
    "result :: Result < i32 , :: alloc :: string :: String > > , :: core :: "
    "result :: Result < :: core :: option :: Option < i32 > , :: core :: "
    "option :: Option < i32 > > >")::rs::
    Result<
        ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        ::rs::Result<::rs::Option<::std::int32_t>,
                     ::rs::Option<::std::int32_t>>>
    : public ::rs::ResultBase<
          ::rs::Result<::rs::Option<::rs::Result<::std::int32_t,
                                                 ::rs::alloc::string::String>>,
                       ::rs::Result<::rs::Option<::std::int32_t>,
                                    ::rs::Option<::std::int32_t>>>,
          ::rs::Option<
              ::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
          ::rs::Result<::rs::Option<::std::int32_t>,
                       ::rs::Option<::std::int32_t>>> {
 public:
  // Clone::clone
  Result(const Result&) noexcept;

  // Clone::clone_from
  ::rs::Result<
      ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>&
  operator=(const Result&) noexcept;

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type = ::rs::ResultBase<
      ::rs::Result<::rs::Option<::rs::Result<::std::int32_t,
                                             ::rs::alloc::string::String>>,
                   ::rs::Result<::rs::Option<::std::int32_t>,
                                ::rs::Option<::std::int32_t>>>,
      ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>;
  template <typename U>
    requires(::rs::ResultForwardConstructible<
             Result,
             ::rs::Option<
                 ::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
             U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(::rs::ResultForwardConstructible<
             Result,
             ::rs::Option<
                 ::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
             U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(::rs::ResultUnexpectedConstructible<
             ::rs::Result<::rs::Option<::std::int32_t>,
                          ::rs::Option<::std::int32_t>>,
             F>)
  explicit constexpr Result(::rs::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(::rs::ResultUnexpectedConstructible<
             ::rs::Result<::rs::Option<::std::int32_t>,
                          ::rs::Option<::std::int32_t>>,
             F>)
  constexpr Result& operator=(::rs::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(::rs::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>*
  ok_ptr() noexcept {
    return reinterpret_cast<::rs::Option<
        ::rs::Result<::std::int32_t, ::rs::alloc::string::String>>*>(__storage +
                                                                     4);
  }
  ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>> const*
  ok_const_ptr() const noexcept {
    return reinterpret_cast<::rs::Option<
        ::rs::Result<::std::int32_t, ::rs::alloc::string::String>> const*>(
        __storage + 4);
  }
  ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>*
  err_ptr() noexcept {
    return reinterpret_cast<::rs::Result<::rs::Option<::std::int32_t>,
                                         ::rs::Option<::std::int32_t>>*>(
        __storage + 4);
  }
  ::rs::Result<::rs::Option<::std::int32_t>,
               ::rs::Option<::std::int32_t>> const*
  err_const_ptr() const noexcept {
    return reinterpret_cast<::rs::Result<::rs::Option<::std::int32_t>,
                                         ::rs::Option<::std::int32_t>> const*>(
        __storage + 4);
  }
  void set_ok_tag() noexcept { set_tag(0); }
  void set_err_tag() noexcept { set_tag(1); }
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;

 private:
  unsigned char __storage[16];
};
#endif

namespace option {

static_assert(
    sizeof(::option::CloneNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::option::CloneNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::option::CloneNoDefault>);
static_assert(
    ::std::is_trivially_move_constructible_v<::option::CloneNoDefault>);
static_assert(::std::is_trivially_move_assignable_v<::option::CloneNoDefault>);
inline void ::option::CloneNoDefault::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::option::CloneNoDefault;
  static_assert(0 == offsetof(__crubit_assert_type, val));
}
static_assert(
    sizeof(::option::CopyNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::option::CopyNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::option::CopyNoDefault>);
static_assert(
    ::std::is_trivially_move_constructible_v<::option::CopyNoDefault>);
static_assert(::std::is_trivially_move_assignable_v<::option::CopyNoDefault>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::option::CopyNoDefault>);
static_assert(::std::is_trivially_copy_assignable_v<::option::CopyNoDefault>);
inline void ::option::CopyNoDefault::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::option::CopyNoDefault;
  static_assert(0 == offsetof(__crubit_assert_type, val));
}
static_assert(
    sizeof(::option::HasDefault) == 12,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::option::HasDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::rs::StrRef*,
                                   ::option::HasDefault* __ret_ptr);
}
inline ::option::HasDefault(HasDefault::new_)(::rs::StrRef s) {
  crubit::Slot<::option::HasDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(&s, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_get_ustring_uinside_uoption(
    ::option::HasDefault const&, ::rs::StrRef* __ret_ptr);
}
inline ::rs::StrRef(HasDefault::get_string_inside_option)() const& $(__anon1)
    CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  crubit::Slot<::rs::StrRef> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_get_ustring_uinside_uoption(
      self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::option::HasDefault::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::option::HasDefault;
  static_assert(0 == offsetof(__crubit_assert_type, foo));
}
static_assert(
    sizeof(::option::HasHasOptions) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::option::HasHasOptions) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::option::HasHasOptions>);
static_assert(
    ::std::is_trivially_move_constructible_v<::option::HasHasOptions>);
static_assert(::std::is_trivially_move_assignable_v<::option::HasHasOptions>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uint8_t,
                                   ::option::HasHasOptions* __ret_ptr);
}
inline ::option::HasHasOptions(HasHasOptions::new_)(::std::uint8_t value) {
  crubit::Slot<::option::HasHasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::option::HasHasOptions::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::option::HasHasOptions;
  static_assert(0 == offsetof(__crubit_assert_type, me));
}
static_assert(
    sizeof(::option::HasNoDefault) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::option::HasNoDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::rs::StrRef*,
                                   ::option::HasNoDefault* __ret_ptr);
}
inline ::rs::Movable<::option::HasNoDefault>(HasNoDefault::new_)(
    ::rs::StrRef s) {
  crubit::Slot<::option::HasNoDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(&s, __return_value_storage);
  return ::rs::Movable<::option::HasNoDefault>::TakeFromSlot(
      ::std::move(__return_value_ret_val_holder));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_get_ustring_uinside_uoption(
    ::option::HasNoDefault const&, ::rs::StrRef* __ret_ptr);
}
inline ::rs::StrRef(HasNoDefault::get_string_inside_option)() const& $(__anon1)
    CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  crubit::Slot<::rs::StrRef> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_get_ustring_uinside_uoption(
      self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::option::HasNoDefault::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::option::HasNoDefault;
  static_assert(0 == offsetof(__crubit_assert_type, foo));
  static_assert(12 == offsetof(__crubit_assert_type, a));
}
static_assert(
    sizeof(::option::HasOptions) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::option::HasOptions) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::option::HasOptions>);
static_assert(::std::is_trivially_move_constructible_v<::option::HasOptions>);
static_assert(::std::is_trivially_move_assignable_v<::option::HasOptions>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uint8_t,
                                   ::option::HasOptions* __ret_ptr);
}
inline ::option::HasOptions(HasOptions::new_)(::std::uint8_t value) {
  crubit::Slot<::option::HasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_with_uoption(::rs::Option<::std::uint8_t>*,
                                            ::option::HasOptions* __ret_ptr);
}
inline ::option::HasOptions(HasOptions::with_option)(
    ::rs::Option<::std::uint8_t> value) {
  crubit::Slot<::option::HasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_with_uoption(&value,
                                                 __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_from_uref(::rs::Option<::std::uint8_t> const&,
                                         ::option::HasOptions* __ret_ptr);
}
inline ::option::HasOptions(HasOptions::from_ref)(
    ::rs::Option<::std::uint8_t> const& value) {
  crubit::Slot<::option::HasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_from_uref(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_with_unone(::option::HasOptions* __ret_ptr);
}
inline ::option::HasOptions(HasOptions::with_none)() {
  crubit::Slot<::option::HasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_with_unone(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::option::HasOptions::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::option::HasOptions;
  static_assert(0 == offsetof(__crubit_assert_type, direct));
  static_assert(2 == offsetof(__crubit_assert_type, niche));
  static_assert(3 == offsetof(__crubit_assert_type, nested));
}
static_assert(
    sizeof(::option::LessThan20U8) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::option::LessThan20U8) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::option::LessThan20U8>);
static_assert(::std::is_trivially_move_constructible_v<::option::LessThan20U8>);
static_assert(::std::is_trivially_move_assignable_v<::option::LessThan20U8>);
static_assert(::std::is_trivially_copy_constructible_v<::option::LessThan20U8>);
static_assert(::std::is_trivially_copy_assignable_v<::option::LessThan20U8>);
inline ::option::LessThan20U8::LessThan20U8(::crubit::UnsafeRelocateTag,
                                            LessThan20U8&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(
    ::std::uint8_t, ::rs::Option<::option::LessThan20U8>* __ret_ptr);
}
inline ::rs::Option<::option::LessThan20U8>(LessThan20U8::new_)(
    ::std::uint8_t value) {
  crubit::Slot<::rs::Option<::option::LessThan20U8>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::uint8_t __crubit_thunk_value(::option::LessThan20U8*);
}
inline ::std::uint8_t(LessThan20U8::value)() const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_value(&self);
}

static_assert(
    sizeof(::option::OptCloneNoDefault) == 2,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::option::OptCloneNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::option::OptCloneNoDefault>);
static_assert(
    ::std::is_trivially_move_constructible_v<::option::OptCloneNoDefault>);
static_assert(
    ::std::is_trivially_move_assignable_v<::option::OptCloneNoDefault>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uint8_t,
                                   ::option::OptCloneNoDefault* __ret_ptr);
}
inline ::option::OptCloneNoDefault(OptCloneNoDefault::new_)(::std::uint8_t x) {
  crubit::Slot<::option::OptCloneNoDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::option::OptCloneNoDefault::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::option::OptCloneNoDefault;
  static_assert(0 == offsetof(__crubit_assert_type, val));
}
static_assert(
    sizeof(::option::OptCopyNoDefault) == 2,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::option::OptCopyNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::option::OptCopyNoDefault>);
static_assert(
    ::std::is_trivially_move_constructible_v<::option::OptCopyNoDefault>);
static_assert(
    ::std::is_trivially_move_assignable_v<::option::OptCopyNoDefault>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::option::OptCopyNoDefault>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::option::OptCopyNoDefault>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uint8_t,
                                   ::option::OptCopyNoDefault* __ret_ptr);
}
inline ::option::OptCopyNoDefault(OptCopyNoDefault::new_)(::std::uint8_t x) {
  crubit::Slot<::option::OptCopyNoDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::option::OptCopyNoDefault::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::option::OptCopyNoDefault;
  static_assert(0 == offsetof(__crubit_assert_type, val));
}
static_assert(
    sizeof(::option::OptDefaultWithDrop) == 12,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::option::OptDefaultWithDrop) == 4,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::rs::StrRef*,
                                   ::option::OptDefaultWithDrop* __ret_ptr);
}
inline ::rs::Movable<::option::OptDefaultWithDrop>(OptDefaultWithDrop::new_)(
    ::rs::StrRef s) {
  crubit::Slot<::option::OptDefaultWithDrop> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(&s, __return_value_storage);
  return ::rs::Movable<::option::OptDefaultWithDrop>::TakeFromSlot(
      ::std::move(__return_value_ret_val_holder));
}
inline void ::option::OptDefaultWithDrop::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::option::OptDefaultWithDrop;
  static_assert(0 == offsetof(__crubit_assert_type, opt));
}
static_assert(
    sizeof(::option::OptNoDefaultWithDrop) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::option::OptNoDefaultWithDrop) == 4,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::rs::StrRef*,
                                   ::option::OptNoDefaultWithDrop* __ret_ptr);
}
inline ::rs::Movable<::option::OptNoDefaultWithDrop>(
    OptNoDefaultWithDrop::new_)(::rs::StrRef s) {
  crubit::Slot<::option::OptNoDefaultWithDrop> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(&s, __return_value_storage);
  return ::rs::Movable<::option::OptNoDefaultWithDrop>::TakeFromSlot(
      ::std::move(__return_value_ret_val_holder));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_get_ustring_uinside_uoption(
    ::option::OptNoDefaultWithDrop const&, ::rs::StrRef* __ret_ptr);
}
inline ::rs::StrRef(OptNoDefaultWithDrop::get_string_inside_option)() const& $(
    __anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  crubit::Slot<::rs::StrRef> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_get_ustring_uinside_uoption(
      self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::option::OptNoDefaultWithDrop::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::option::OptNoDefaultWithDrop;
  static_assert(0 == offsetof(__crubit_assert_type, val));
}
static_assert(
    sizeof(::option::OptZst) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::option::OptZst) == 1,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_uoption_ugolden_x0000003a_x0000003aOptZst(
    ::option::OptZst* __ret_ptr);
}
inline ::option::OptZst::OptZst() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_uoption_ugolden_x0000003a_x0000003aOptZst(
          this);
}
static_assert(::std::is_trivially_destructible_v<::option::OptZst>);
static_assert(::std::is_trivially_move_constructible_v<::option::OptZst>);
static_assert(::std::is_trivially_move_assignable_v<::option::OptZst>);
inline ::option::OptZst::OptZst(::crubit::UnsafeRelocateTag, OptZst&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::option::OptZst::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::option::OptZst;
  static_assert(0 == offsetof(__crubit_assert_type, val));
}
static_assert(
    sizeof(::option::OptionWithSizeTypes) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::option::OptionWithSizeTypes) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    ::std::is_trivially_destructible_v<::option::OptionWithSizeTypes>);
static_assert(
    ::std::is_trivially_move_constructible_v<::option::OptionWithSizeTypes>);
static_assert(
    ::std::is_trivially_move_assignable_v<::option::OptionWithSizeTypes>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::rs::Option<::std::uint32_t>*,
                                   ::rs::Option<::std::int32_t>*,
                                   ::option::OptionWithSizeTypes* __ret_ptr);
}
inline ::option::OptionWithSizeTypes(OptionWithSizeTypes::new_)(
    ::rs::Option<::std::uint32_t> uval, ::rs::Option<::std::int32_t> ival) {
  crubit::Slot<::option::OptionWithSizeTypes> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(&uval, &ival, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::option::OptionWithSizeTypes::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::option::OptionWithSizeTypes;
  static_assert(0 == offsetof(__crubit_assert_type, uval));
  static_assert(8 == offsetof(__crubit_assert_type, ival));
}
static_assert(
    sizeof(::option::UnitOptionField) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::option::UnitOptionField) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::option::UnitOptionField>);
static_assert(
    ::std::is_trivially_move_constructible_v<::option::UnitOptionField>);
static_assert(::std::is_trivially_move_assignable_v<::option::UnitOptionField>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new_uwith_usome(
    ::option::UnitOptionField* __ret_ptr);
}
inline ::option::UnitOptionField(UnitOptionField::new_with_some)() {
  crubit::Slot<::option::UnitOptionField> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new_uwith_usome(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::option::UnitOptionField::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::option::UnitOptionField;
  static_assert(0 == offsetof(__crubit_assert_type, unit));
}
static_assert(
    sizeof(::option::ZStream) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::option::ZStream) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::option::ZStream>);
static_assert(::std::is_trivially_move_constructible_v<::option::ZStream>);
static_assert(::std::is_trivially_move_assignable_v<::option::ZStream>);
inline void ::option::ZStream::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::option::ZStream;
  static_assert(0 == offsetof(__crubit_assert_type, zfree));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_pass_uoption_uptr(
    ::rs::Option<::std::int32_t const*>*,
    ::rs::Option<::std::int32_t const*>* __ret_ptr);
}
inline ::rs::Option<::std::int32_t const*> pass_option_ptr(
    ::rs::Option<::std::int32_t const*> x) {
  crubit::Slot<::rs::Option<::std::int32_t const*>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_pass_uoption_uptr(&x,
                                                      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uoption_uresult(
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>*
        __ret_ptr);
}
inline ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>
return_option_result() {
  crubit::Slot<
      ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_return_uoption_uresult(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uoption_uresult_uunmovable(
    ::rs::Option<::rs::Result<::option::HasNoDefault,
                              ::rs::alloc::string::String>>* __ret_ptr);
}
inline ::rs::Option<
    ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>
return_option_result_unmovable() {
  crubit::Slot<::rs::Option<
      ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_return_uoption_uresult_uunmovable(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_stress_utesting_unested_utypes(
    ::rs::Option<::rs::Result<
        ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        ::rs::Result<::rs::Option<::std::int32_t>,
                     ::rs::Option<::std::int32_t>>>>* __ret_ptr);
}
inline ::rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>>
stress_testing_nested_types() {
  crubit::Slot<::rs::Option<::rs::Result<
      ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      ::rs::Result<::rs::Option<::std::int32_t>,
                   ::rs::Option<::std::int32_t>>>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_stress_utesting_unested_utypes(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_stringify_ulen(
    ::rs::Option<::option::HasDefault> const&,
    ::rs::Option<::std::uint32_t>* __ret_ptr);
}
inline ::rs::Option<::std::uint32_t> stringify_len(
    ::rs::Option<::option::HasDefault> const& x) {
  crubit::Slot<::rs::Option<::std::uint32_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_stringify_ulen(x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::uint8_t __crubit_thunk_take_uoption_ubridged(
    ::rs::Option<decltype(char(0))>*);
}
inline ::std::uint8_t take_option_bridged(::rs::Option<decltype(char(0))> x) {
  return __crubit_internal::__crubit_thunk_take_uoption_ubridged(&x);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_take_uoption_uresult_uunmovable(
    ::rs::Option<
        ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>*);
}
inline void take_option_result_unmovable(
    ::rs::Option<
        ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>
        _x) {
  crubit::Slot _x_slot((::std::move(_x)));
  return __crubit_internal::__crubit_thunk_take_uoption_uresult_uunmovable(
      _x_slot.Get());
}

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<::rs::Option<::rs::unit_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::rs::Option<::rs::unit_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<::rs::Option<::rs::unit_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<::rs::Option<::rs::unit_t>>);
inline ::rs::Option<::rs::unit_t>::Option(::crubit::UnsafeRelocateTag,
                                          Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<::rs::Option<::rs::unit_t>>);
inline constexpr ::std::uint8_t(
    ::rs::Option<::rs::unit_t>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void(::rs::Option<::rs::unit_t>::set_tag)(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::rs::unit_t>::Option(::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::rs::unit_t>& ::rs::Option<
    ::rs::unit_t>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::rs::unit_t>,
                                            ::rs::unit_t, U>
inline ::rs::Option<::rs::unit_t>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::rs::unit_t>,
                                            ::rs::unit_t, U>
inline ::rs::Option<::rs::unit_t>& ::rs::Option<::rs::unit_t>::operator=(
    U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::rs::unit_t, Opt>
inline ::rs::Option<::rs::unit_t>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::rs::unit_t, Opt>
inline ::rs::Option<::rs::unit_t>& ::rs::Option<::rs::unit_t>::operator=(
    Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::rs::unit_t>::Option(::std::in_place_t ip,
                                          Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs::Option<::std::int32_t const*>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::rs::Option<::std::int32_t const*>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs::Option<::std::int32_t const*>>);
static_assert(
    ::std::is_trivially_move_assignable_v<::rs::Option<::std::int32_t const*>>);
inline ::rs::Option<::std::int32_t const*>::Option(::crubit::UnsafeRelocateTag,
                                                   Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<::rs::Option<::std::int32_t const*>>);
inline constexpr ::std::uint32_t(
    ::rs::Option<::std::int32_t const*>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void(::rs::Option<::std::int32_t const*>::set_tag)(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::std::int32_t const*>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::std::int32_t const*>& ::rs::Option<
    ::std::int32_t const*>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::std::int32_t const*>,
                                            ::std::int32_t const*, U>
inline ::rs::Option<::std::int32_t const*>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::std::int32_t const*>,
                                            ::std::int32_t const*, U>
inline ::rs::Option<::std::int32_t const*>& ::rs::Option<
    ::std::int32_t const*>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::std::int32_t const*, Opt>
inline ::rs::Option<::std::int32_t const*>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::std::int32_t const*, Opt>
inline ::rs::Option<::std::int32_t const*>& ::rs::Option<
    ::std::int32_t const*>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::std::int32_t const*>::Option(::std::in_place_t ip,
                                                   Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<::rs::Option<::std::int32_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::rs::Option<::std::int32_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<::rs::Option<::std::int32_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<::rs::Option<::std::int32_t>>);
inline ::rs::Option<::std::int32_t>::Option(::crubit::UnsafeRelocateTag,
                                            Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<::rs::Option<::std::int32_t>>);
inline constexpr ::std::uint32_t(
    ::rs::Option<::std::int32_t>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void(::rs::Option<::std::int32_t>::set_tag)(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::std::int32_t>::Option(::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::std::int32_t>& ::rs::Option<
    ::std::int32_t>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::std::int32_t>,
                                            ::std::int32_t, U>
inline ::rs::Option<::std::int32_t>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::std::int32_t>,
                                            ::std::int32_t, U>
inline ::rs::Option<::std::int32_t>& ::rs::Option<::std::int32_t>::operator=(
    U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::std::int32_t, Opt>
inline ::rs::Option<::std::int32_t>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::std::int32_t, Opt>
inline ::rs::Option<::std::int32_t>& ::rs::Option<::std::int32_t>::operator=(
    Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::std::int32_t>::Option(::std::in_place_t ip,
                                            Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<::rs::Option<::std::int32_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::rs::Option<::std::int32_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<::rs::Option<::std::int32_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<::rs::Option<::std::int32_t>>);
inline ::rs::Option<::std::int32_t>::Option(::crubit::UnsafeRelocateTag,
                                            Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<::rs::Option<::std::int32_t>>);
inline constexpr ::std::uint32_t(
    ::rs::Option<::std::int32_t>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void(::rs::Option<::std::int32_t>::set_tag)(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::std::int32_t>::Option(::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::std::int32_t>& ::rs::Option<
    ::std::int32_t>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::std::int32_t>,
                                            ::std::int32_t, U>
inline ::rs::Option<::std::int32_t>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::std::int32_t>,
                                            ::std::int32_t, U>
inline ::rs::Option<::std::int32_t>& ::rs::Option<::std::int32_t>::operator=(
    U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::std::int32_t, Opt>
inline ::rs::Option<::std::int32_t>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::std::int32_t, Opt>
inline ::rs::Option<::std::int32_t>& ::rs::Option<::std::int32_t>::operator=(
    Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::std::int32_t>::Option(::std::in_place_t ip,
                                            Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020decltype_x00000020_x00000028char_x00000020_x000000280_x00000029_x00000029_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020decltype_x00000020_x00000028char_x00000020_x000000280_x00000029_x00000029_x00000020_x0000003e
static_assert(
    ::std::is_trivially_move_constructible_v<::rs::Option<decltype(char(0))>>);
static_assert(
    ::std::is_trivially_move_assignable_v<::rs::Option<decltype(char(0))>>);
inline ::rs::Option<decltype(char(0))>::Option(::crubit::UnsafeRelocateTag,
                                               Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<::rs::Option<decltype(char(0))>>);
inline constexpr ::std::uint8_t(
    ::rs::Option<decltype(char(0))>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void(::rs::Option<decltype(char(0))>::set_tag)(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<decltype(char(0))>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<decltype(char(0))>& ::rs::Option<
    decltype(char(0))>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<decltype(char(0))>,
                                            decltype(char(0)), U>
inline ::rs::Option<decltype(char(0))>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<decltype(char(0))>,
                                            decltype(char(0)), U>
inline ::rs::Option<decltype(char(0))>& ::rs::Option<
    decltype(char(0))>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<decltype(char(0)), Opt>
inline ::rs::Option<decltype(char(0))>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<decltype(char(0)), Opt>
inline ::rs::Option<decltype(char(0))>& ::rs::Option<
    decltype(char(0))>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<decltype(char(0))>::Option(::std::in_place_t ip,
                                               Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e(
    ::rs::Option<::option::CloneNoDefault> const&,
    ::rs::Option<::option::CloneNoDefault>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e(
    ::rs::Option<::option::CloneNoDefault>&,
    ::rs::Option<::option::CloneNoDefault> const&);
}
inline ::rs::Option<::option::CloneNoDefault>::Option(
    const Option& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e(
          other, this);
}
inline ::rs::Option<::option::CloneNoDefault>& ::rs::Option<
    ::option::CloneNoDefault>::operator=(const Option& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e(
            *this, other);
  }
  return *this;
}
static_assert(::std::is_trivially_move_constructible_v<
              ::rs::Option<::option::CloneNoDefault>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs::Option<::option::CloneNoDefault>>);
inline ::rs::Option<::option::CloneNoDefault>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<::rs::Option<::option::CloneNoDefault>>);
inline constexpr ::std::uint8_t(
    ::rs::Option<::option::CloneNoDefault>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void(::rs::Option<::option::CloneNoDefault>::set_tag)(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::option::CloneNoDefault>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::option::CloneNoDefault>& ::rs::Option<
    ::option::CloneNoDefault>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::option::CloneNoDefault>, ::option::CloneNoDefault, U>
inline ::rs::Option<::option::CloneNoDefault>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::option::CloneNoDefault>, ::option::CloneNoDefault, U>
inline ::rs::Option<::option::CloneNoDefault>& ::rs::Option<
    ::option::CloneNoDefault>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::option::CloneNoDefault, Opt>
inline ::rs::Option<::option::CloneNoDefault>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::option::CloneNoDefault, Opt>
inline ::rs::Option<::option::CloneNoDefault>& ::rs::Option<
    ::option::CloneNoDefault>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::option::CloneNoDefault>::Option(::std::in_place_t ip,
                                                      Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs::Option<::option::CopyNoDefault>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs::Option<::option::CopyNoDefault>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs::Option<::option::CopyNoDefault>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs::Option<::option::CopyNoDefault>>);
inline ::rs::Option<::option::CopyNoDefault>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<::rs::Option<::option::CopyNoDefault>>);
inline constexpr ::std::uint8_t(
    ::rs::Option<::option::CopyNoDefault>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void(::rs::Option<::option::CopyNoDefault>::set_tag)(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::option::CopyNoDefault>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::option::CopyNoDefault>& ::rs::Option<
    ::option::CopyNoDefault>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::option::CopyNoDefault>, ::option::CopyNoDefault, U>
inline ::rs::Option<::option::CopyNoDefault>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::option::CopyNoDefault>, ::option::CopyNoDefault, U>
inline ::rs::Option<::option::CopyNoDefault>& ::rs::Option<
    ::option::CopyNoDefault>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::option::CopyNoDefault, Opt>
inline ::rs::Option<::option::CopyNoDefault>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::option::CopyNoDefault, Opt>
inline ::rs::Option<::option::CopyNoDefault>& ::rs::Option<
    ::option::CopyNoDefault>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::option::CopyNoDefault>::Option(::std::in_place_t ip,
                                                     Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
inline ::rs::Option<::option::HasDefault>::Option(Option&& other) noexcept
    : Option() {
  *this = ::std::move(other);
}
inline ::rs::Option<::option::HasDefault>& ::rs::Option<
    ::option::HasDefault>::operator=(Option&& other) noexcept {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::rs::Option<::option::HasDefault>::Option(::crubit::UnsafeRelocateTag,
                                                  Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline constexpr ::rs::Option<::option::HasDefault>::~Option() noexcept {
  this->reset();
}
inline constexpr ::std::uint32_t(
    ::rs::Option<::option::HasDefault>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void(::rs::Option<::option::HasDefault>::set_tag)(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::option::HasDefault>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::option::HasDefault>& ::rs::Option<
    ::option::HasDefault>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::option::HasDefault>,
                                            ::option::HasDefault, U>
inline ::rs::Option<::option::HasDefault>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::option::HasDefault>,
                                            ::option::HasDefault, U>
inline ::rs::Option<::option::HasDefault>& ::rs::Option<
    ::option::HasDefault>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::option::HasDefault, Opt>
inline ::rs::Option<::option::HasDefault>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::option::HasDefault, Opt>
inline ::rs::Option<::option::HasDefault>& ::rs::Option<
    ::option::HasDefault>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::option::HasDefault>::Option(::std::in_place_t ip,
                                                  Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
inline ::rs::Option<::option::HasNoDefault>::Option(Option&& other) noexcept
    : Option() {
  *this = ::std::move(other);
}
inline ::rs::Option<::option::HasNoDefault>& ::rs::Option<
    ::option::HasNoDefault>::operator=(Option&& other) noexcept {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::rs::Option<::option::HasNoDefault>::Option(::crubit::UnsafeRelocateTag,
                                                    Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline constexpr ::rs::Option<::option::HasNoDefault>::~Option() noexcept {
  this->reset();
}
inline constexpr ::std::uint32_t(
    ::rs::Option<::option::HasNoDefault>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void(::rs::Option<::option::HasNoDefault>::set_tag)(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::option::HasNoDefault>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::option::HasNoDefault>& ::rs::Option<
    ::option::HasNoDefault>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::option::HasNoDefault>, ::option::HasNoDefault, U>
inline ::rs::Option<::option::HasNoDefault>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::option::HasNoDefault>, ::option::HasNoDefault, U>
inline ::rs::Option<::option::HasNoDefault>& ::rs::Option<
    ::option::HasNoDefault>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::option::HasNoDefault, Opt>
inline ::rs::Option<::option::HasNoDefault>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::option::HasNoDefault, Opt>
inline ::rs::Option<::option::HasNoDefault>& ::rs::Option<
    ::option::HasNoDefault>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::option::HasNoDefault>::Option(::std::in_place_t ip,
                                                    Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasOptions_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasOptions_x00000020_x0000003e
static_assert(::std::is_trivially_move_constructible_v<
              ::rs::Option<::option::HasOptions>>);
static_assert(
    ::std::is_trivially_move_assignable_v<::rs::Option<::option::HasOptions>>);
inline ::rs::Option<::option::HasOptions>::Option(::crubit::UnsafeRelocateTag,
                                                  Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<::rs::Option<::option::HasOptions>>);
inline constexpr ::std::uint8_t(
    ::rs::Option<::option::HasOptions>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void(::rs::Option<::option::HasOptions>::set_tag)(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::option::HasOptions>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::option::HasOptions>& ::rs::Option<
    ::option::HasOptions>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::option::HasOptions>,
                                            ::option::HasOptions, U>
inline ::rs::Option<::option::HasOptions>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::option::HasOptions>,
                                            ::option::HasOptions, U>
inline ::rs::Option<::option::HasOptions>& ::rs::Option<
    ::option::HasOptions>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::option::HasOptions, Opt>
inline ::rs::Option<::option::HasOptions>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::option::HasOptions, Opt>
inline ::rs::Option<::option::HasOptions>& ::rs::Option<
    ::option::HasOptions>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::option::HasOptions>::Option(::std::in_place_t ip,
                                                  Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs::Option<::option::LessThan20U8>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs::Option<::option::LessThan20U8>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs::Option<::option::LessThan20U8>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs::Option<::option::LessThan20U8>>);
inline ::rs::Option<::option::LessThan20U8>::Option(::crubit::UnsafeRelocateTag,
                                                    Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<::rs::Option<::option::LessThan20U8>>);
inline constexpr ::std::uint8_t(
    ::rs::Option<::option::LessThan20U8>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void(::rs::Option<::option::LessThan20U8>::set_tag)(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::option::LessThan20U8>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::option::LessThan20U8>& ::rs::Option<
    ::option::LessThan20U8>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::option::LessThan20U8>, ::option::LessThan20U8, U>
inline ::rs::Option<::option::LessThan20U8>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::option::LessThan20U8>, ::option::LessThan20U8, U>
inline ::rs::Option<::option::LessThan20U8>& ::rs::Option<
    ::option::LessThan20U8>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::option::LessThan20U8, Opt>
inline ::rs::Option<::option::LessThan20U8>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::option::LessThan20U8, Opt>
inline ::rs::Option<::option::LessThan20U8>& ::rs::Option<
    ::option::LessThan20U8>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::option::LessThan20U8>::Option(::std::in_place_t ip,
                                                    Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs::Option<::rs::Option<::option::LessThan20U8>>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs::Option<::rs::Option<::option::LessThan20U8>>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs::Option<::rs::Option<::option::LessThan20U8>>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs::Option<::rs::Option<::option::LessThan20U8>>>);
inline ::rs::Option<::rs::Option<::option::LessThan20U8>>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              ::rs::Option<::rs::Option<::option::LessThan20U8>>>);
inline constexpr ::std::uint8_t(
    ::rs::Option<::rs::Option<::option::LessThan20U8>>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void(
    ::rs::Option<::rs::Option<::option::LessThan20U8>>::set_tag)(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::rs::Option<::option::LessThan20U8>>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::rs::Option<::option::LessThan20U8>>& ::rs::
    Option<::rs::Option<::option::LessThan20U8>>::operator=(
        ::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::rs::Option<::option::LessThan20U8>>,
      ::rs::Option<::option::LessThan20U8>, U>
inline ::rs::Option<::rs::Option<::option::LessThan20U8>>::Option(
    U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::rs::Option<::option::LessThan20U8>>,
      ::rs::Option<::option::LessThan20U8>, U>
inline ::rs::Option<::rs::Option<::option::LessThan20U8>>& ::rs::Option<
    ::rs::Option<::option::LessThan20U8>>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::rs::Option<::option::LessThan20U8>,
                                       Opt>
inline ::rs::Option<::rs::Option<::option::LessThan20U8>>::Option(
    Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::rs::Option<::option::LessThan20U8>,
                                       Opt>
inline ::rs::Option<::rs::Option<::option::LessThan20U8>>& ::rs::Option<
    ::rs::Option<::option::LessThan20U8>>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::rs::Option<::option::LessThan20U8>>::Option(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e(
    ::rs::Option<
        ::rs::Result<::std::int32_t, ::rs::alloc::string::String>> const&,
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>*
        __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e(
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>&,
    ::rs::Option<
        ::rs::Result<::std::int32_t, ::rs::alloc::string::String>> const&);
}
inline ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>::
    Option(const Option& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e(
          other, this);
}
inline ::rs::Option<
    ::rs::Result<::std::int32_t, ::rs::alloc::string::String>>& ::
rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>::
operator=(const Option& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e(
            *this, other);
  }
  return *this;
}
inline ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>::
    Option(Option&& other) noexcept
    : Option() {
  *this = ::std::move(other);
}
inline ::rs::Option<
    ::rs::Result<::std::int32_t, ::rs::alloc::string::String>>& ::
rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>::
operator=(Option&& other) noexcept {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>::
    Option(::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline constexpr ::rs::Option<::rs::Result<
    ::std::int32_t, ::rs::alloc::string::String>>::~Option() noexcept {
  this->reset();
}
inline constexpr ::std::uint32_t(
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>::
        tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void(
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>::
        set_tag)(::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::
    Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>::Option(
        ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<
    ::rs::Result<::std::int32_t, ::rs::alloc::string::String>>& ::
rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>::
operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      ::rs::Result<::std::int32_t, ::rs::alloc::string::String>, U>
inline ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>::
    Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      ::rs::Result<::std::int32_t, ::rs::alloc::string::String>, U>
inline ::rs::Option<
    ::rs::Result<::std::int32_t, ::rs::alloc::string::String>>& ::
rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>::
operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<
      ::rs::Result<::std::int32_t, ::rs::alloc::string::String>, Opt>
inline ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>::
    Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<
      ::rs::Result<::std::int32_t, ::rs::alloc::string::String>, Opt>
inline ::rs::Option<
    ::rs::Result<::std::int32_t, ::rs::alloc::string::String>>& ::
rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>::
operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>::
    Option(::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
inline ::rs::Option<
    ::rs::Result<::option::HasNoDefault,
                 ::rs::alloc::string::String>>::Option(Option&& other) noexcept
    : Option() {
  *this = ::std::move(other);
}
inline ::rs::Option<
    ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>& ::
rs::Option<::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
operator=(Option&& other) noexcept {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::rs::Option<
    ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
    Option(::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline constexpr ::rs::Option<::rs::Result<
    ::option::HasNoDefault, ::rs::alloc::string::String>>::~Option() noexcept {
  this->reset();
}
inline constexpr ::std::uint32_t(
    ::rs::Option<::rs::Result<::option::HasNoDefault,
                              ::rs::alloc::string::String>>::tag)()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void(
    ::rs::Option<::rs::Result<::option::HasNoDefault,
                              ::rs::alloc::string::String>>::set_tag)(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<
    ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
    Option(::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<
    ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>& ::
rs::Option<::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<
          ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>,
      ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>, U>
inline ::rs::Option<
    ::rs::Result<::option::HasNoDefault,
                 ::rs::alloc::string::String>>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<
          ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>,
      ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>, U>
inline ::rs::Option<
    ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>& ::
rs::Option<::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<
      ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>, Opt>
inline ::rs::Option<
    ::rs::Result<::option::HasNoDefault,
                 ::rs::alloc::string::String>>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<
      ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>, Opt>
inline ::rs::Option<
    ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>& ::
rs::Option<::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<
    ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
    Option(::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e_x0000003e(
    ::rs::Option<::rs::Result<
        ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        ::rs::Result<::rs::Option<::std::int32_t>,
                     ::rs::Option<::std::int32_t>>>> const&,
    ::rs::Option<::rs::Result<
        ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        ::rs::Result<::rs::Option<::std::int32_t>,
                     ::rs::Option<::std::int32_t>>>>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e_x0000003e(
    ::rs::Option<::rs::Result<
        ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        ::rs::Result<::rs::Option<::std::int32_t>,
                     ::rs::Option<::std::int32_t>>>>&,
    ::rs::Option<::rs::Result<
        ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        ::rs::Result<::rs::Option<::std::int32_t>,
                     ::rs::Option<::std::int32_t>>>> const&);
}
inline ::rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>>::
    Option(const Option& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e_x0000003e(
          other, this);
}
inline ::rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>,
                 ::rs::Option<::std::int32_t>>>>& ::
rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>>::
operator=(const Option& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e_x0000003e(
            *this, other);
  }
  return *this;
}
inline ::rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>>::
    Option(Option&& other) noexcept
    : Option() {
  *this = ::std::move(other);
}
inline ::rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>,
                 ::rs::Option<::std::int32_t>>>>& ::
rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>>::
operator=(Option&& other) noexcept {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>>::
    Option(::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline constexpr ::rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>>::
    ~Option() noexcept {
  this->reset();
}
inline constexpr ::std::uint32_t(
    ::rs::Option<::rs::Result<
        ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        ::rs::Result<::rs::Option<::std::int32_t>,
                     ::rs::Option<::std::int32_t>>>>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void(
    ::rs::Option<::rs::Result<
        ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        ::rs::Result<::rs::Option<::std::int32_t>,
                     ::rs::Option<::std::int32_t>>>>::set_tag)(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>>::
    Option(::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>,
                 ::rs::Option<::std::int32_t>>>>& ::
rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>>::
operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<
          ::rs::Result<::rs::Option<::rs::Result<::std::int32_t,
                                                 ::rs::alloc::string::String>>,
                       ::rs::Result<::rs::Option<::std::int32_t>,
                                    ::rs::Option<::std::int32_t>>>>,
      ::rs::Result<::rs::Option<::rs::Result<::std::int32_t,
                                             ::rs::alloc::string::String>>,
                   ::rs::Result<::rs::Option<::std::int32_t>,
                                ::rs::Option<::std::int32_t>>>,
      U>
inline ::rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>>::
    Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<
          ::rs::Result<::rs::Option<::rs::Result<::std::int32_t,
                                                 ::rs::alloc::string::String>>,
                       ::rs::Result<::rs::Option<::std::int32_t>,
                                    ::rs::Option<::std::int32_t>>>>,
      ::rs::Result<::rs::Option<::rs::Result<::std::int32_t,
                                             ::rs::alloc::string::String>>,
                   ::rs::Result<::rs::Option<::std::int32_t>,
                                ::rs::Option<::std::int32_t>>>,
      U>
inline ::rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>,
                 ::rs::Option<::std::int32_t>>>>& ::
rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>>::
operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<
      ::rs::Result<::rs::Option<::rs::Result<::std::int32_t,
                                             ::rs::alloc::string::String>>,
                   ::rs::Result<::rs::Option<::std::int32_t>,
                                ::rs::Option<::std::int32_t>>>,
      Opt>
inline ::rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>>::
    Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<
      ::rs::Result<::rs::Option<::rs::Result<::std::int32_t,
                                             ::rs::alloc::string::String>>,
                   ::rs::Result<::rs::Option<::std::int32_t>,
                                ::rs::Option<::std::int32_t>>>,
      Opt>
inline ::rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>,
                 ::rs::Option<::std::int32_t>>>>& ::
rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>>::
operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>>::
    Option(::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<::rs::Option<::std::uint32_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::rs::Option<::std::uint32_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<::rs::Option<::std::uint32_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<::rs::Option<::std::uint32_t>>);
inline ::rs::Option<::std::uint32_t>::Option(::crubit::UnsafeRelocateTag,
                                             Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<::rs::Option<::std::uint32_t>>);
inline constexpr ::std::uint32_t(
    ::rs::Option<::std::uint32_t>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void(::rs::Option<::std::uint32_t>::set_tag)(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::std::uint32_t>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::std::uint32_t>& ::rs::Option<
    ::std::uint32_t>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::std::uint32_t>,
                                            ::std::uint32_t, U>
inline ::rs::Option<::std::uint32_t>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::std::uint32_t>,
                                            ::std::uint32_t, U>
inline ::rs::Option<::std::uint32_t>& ::rs::Option<::std::uint32_t>::operator=(
    U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::std::uint32_t, Opt>
inline ::rs::Option<::std::uint32_t>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::std::uint32_t, Opt>
inline ::rs::Option<::std::uint32_t>& ::rs::Option<::std::uint32_t>::operator=(
    Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::std::uint32_t>::Option(::std::in_place_t ip,
                                             Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<::rs::Option<::std::uint8_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::rs::Option<::std::uint8_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<::rs::Option<::std::uint8_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<::rs::Option<::std::uint8_t>>);
inline ::rs::Option<::std::uint8_t>::Option(::crubit::UnsafeRelocateTag,
                                            Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<::rs::Option<::std::uint8_t>>);
inline constexpr ::std::uint8_t(
    ::rs::Option<::std::uint8_t>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void(::rs::Option<::std::uint8_t>::set_tag)(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::std::uint8_t>::Option(::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::std::uint8_t>& ::rs::Option<
    ::std::uint8_t>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::std::uint8_t>,
                                            ::std::uint8_t, U>
inline ::rs::Option<::std::uint8_t>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::std::uint8_t>,
                                            ::std::uint8_t, U>
inline ::rs::Option<::std::uint8_t>& ::rs::Option<::std::uint8_t>::operator=(
    U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::std::uint8_t, Opt>
inline ::rs::Option<::std::uint8_t>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::std::uint8_t, Opt>
inline ::rs::Option<::std::uint8_t>& ::rs::Option<::std::uint8_t>::operator=(
    Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::std::uint8_t>::Option(::std::in_place_t ip,
                                            Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs::Option<crubit::type_identity_t<void(void*, void*)>*>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs::Option<crubit::type_identity_t<void(void*, void*)>*>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs::Option<crubit::type_identity_t<void(void*, void*)>*>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs::Option<crubit::type_identity_t<void(void*, void*)>*>>);
inline ::rs::Option<crubit::type_identity_t<void(void*, void*)>*>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              ::rs::Option<crubit::type_identity_t<void(void*, void*)>*>>);
inline constexpr ::std::uint32_t(
    ::rs::Option<crubit::type_identity_t<void(void*, void*)>*>::tag)()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void(
    ::rs::Option<crubit::type_identity_t<void(void*, void*)>*>::set_tag)(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<crubit::type_identity_t<void(void*, void*)>*>::
    Option(::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<crubit::type_identity_t<void(void*, void*)>*>& ::
rs::Option<crubit::type_identity_t<void(void*, void*)>*>::operator=(
    ::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<crubit::type_identity_t<void(void*, void*)>*>,
      crubit::type_identity_t<void(void*, void*)>*, U>
inline ::rs::Option<crubit::type_identity_t<void(void*, void*)>*>::Option(
    U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<crubit::type_identity_t<void(void*, void*)>*>,
      crubit::type_identity_t<void(void*, void*)>*, U>
inline ::rs::Option<crubit::type_identity_t<void(void*, void*)>*>& ::rs::
    Option<crubit::type_identity_t<void(void*, void*)>*>::operator=(
        U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<
      crubit::type_identity_t<void(void*, void*)>*, Opt>
inline ::rs::Option<crubit::type_identity_t<void(void*, void*)>*>::Option(
    Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<
      crubit::type_identity_t<void(void*, void*)>*, Opt>
inline ::rs::Option<crubit::type_identity_t<void(void*, void*)>*>& ::rs::
    Option<crubit::type_identity_t<void(void*, void*)>*>::operator=(
        Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<crubit::type_identity_t<void(void*, void*)>*>::Option(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<::rs::Option<::std::uint32_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::rs::Option<::std::uint32_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<::rs::Option<::std::uint32_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<::rs::Option<::std::uint32_t>>);
inline ::rs::Option<::std::uint32_t>::Option(::crubit::UnsafeRelocateTag,
                                             Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<::rs::Option<::std::uint32_t>>);
inline constexpr ::std::uint32_t(
    ::rs::Option<::std::uint32_t>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void(::rs::Option<::std::uint32_t>::set_tag)(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::std::uint32_t>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::std::uint32_t>& ::rs::Option<
    ::std::uint32_t>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::std::uint32_t>,
                                            ::std::uint32_t, U>
inline ::rs::Option<::std::uint32_t>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::std::uint32_t>,
                                            ::std::uint32_t, U>
inline ::rs::Option<::std::uint32_t>& ::rs::Option<::std::uint32_t>::operator=(
    U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::std::uint32_t, Opt>
inline ::rs::Option<::std::uint32_t>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::std::uint32_t, Opt>
inline ::rs::Option<::std::uint32_t>& ::rs::Option<::std::uint32_t>::operator=(
    Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::std::uint32_t>::Option(::std::in_place_t ip,
                                             Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e(
    ::rs::Result<::std::int32_t, ::rs::alloc::string::String> const&,
    ::rs::Result<::std::int32_t, ::rs::alloc::string::String>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e(
    ::rs::Result<::std::int32_t, ::rs::alloc::string::String>&,
    ::rs::Result<::std::int32_t, ::rs::alloc::string::String> const&);
}
inline ::rs::Result<::std::int32_t, ::rs::alloc::string::String>::Result(
    const Result& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e(
          other, this);
}
inline ::rs::Result<::std::int32_t, ::rs::alloc::string::String>& ::rs::Result<
    ::std::int32_t,
    ::rs::alloc::string::String>::operator=(const Result& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e(
            *this, other);
  }
  return *this;
}
inline ::rs::Result<::std::int32_t, ::rs::alloc::string::String>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline ::rs::Result<::std::int32_t,
                    ::rs::alloc::string::String>::~Result() noexcept {
  this->Reset();
}
inline constexpr ::std::uint32_t(
    ::rs::Result<::std::int32_t, ::rs::alloc::string::String>::tag)()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void(
    ::rs::Result<::std::int32_t, ::rs::alloc::string::String>::set_tag)(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::std::int32_t, ::rs::alloc::string::String>,
           ::std::int32_t, U>)
inline constexpr ::rs::Result<
    ::std::int32_t, ::rs::alloc::string::String>::Result(U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::std::int32_t, ::rs::alloc::string::String>,
           ::std::int32_t, U>)
inline constexpr ::rs::Result<::std::int32_t, ::rs::alloc::string::String>& ::
rs::Result<::std::int32_t, ::rs::alloc::string::String>::operator=(
    U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<::rs::alloc::string::String, F>)
inline constexpr ::rs::Result<::std::int32_t, ::rs::alloc::string::String>::
    Result(::rs::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<::rs::alloc::string::String, F>)
inline constexpr ::rs::Result<::std::int32_t, ::rs::alloc::string::String>& ::
rs::Result<::std::int32_t, ::rs::alloc::string::String>::operator=(
    ::rs::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr ::rs::Result<::std::int32_t, ::rs::alloc::string::String>::
    Result(::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr ::rs::Result<::std::int32_t, ::rs::alloc::string::String>::
    Result(::rs::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
inline ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>::
    Result(::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline ::rs::Result<::option::HasNoDefault,
                    ::rs::alloc::string::String>::~Result() noexcept {
  this->Reset();
}
inline constexpr ::std::uint32_t(
    ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>::tag)()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void(
    ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>::set_tag)(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>,
           ::option::HasNoDefault, U>)
inline constexpr ::rs::Result<
    ::option::HasNoDefault,
    ::rs::alloc::string::String>::Result(U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>,
           ::option::HasNoDefault, U>)
inline constexpr ::rs::Result<::option::HasNoDefault,
                              ::rs::alloc::string::String>& ::
rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>::operator=(
    U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<::rs::alloc::string::String, F>)
inline constexpr ::rs::Result<
    ::option::HasNoDefault,
    ::rs::alloc::string::String>::Result(::rs::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<::rs::alloc::string::String, F>)
inline constexpr ::rs::Result<::option::HasNoDefault,
                              ::rs::alloc::string::String>& ::
rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>::operator=(
    ::rs::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr ::rs::
    Result<::option::HasNoDefault, ::rs::alloc::string::String>::Result(
        ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr ::rs::
    Result<::option::HasNoDefault, ::rs::alloc::string::String>::Result(
        ::rs::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<::rs::Result<
                  ::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>);
static_assert(::std::is_trivially_copy_assignable_v<::rs::Result<
                  ::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>);
static_assert(::std::is_trivially_move_constructible_v<::rs::Result<
                  ::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>);
static_assert(::std::is_trivially_move_assignable_v<::rs::Result<
                  ::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>);
inline ::rs::
    Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>::Result(
        ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<::rs::Result<
                  ::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>);
inline constexpr ::std::uint32_t(
    ::rs::Result<::rs::Option<::std::int32_t>,
                 ::rs::Option<::std::int32_t>>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void(::rs::Result<::rs::Option<::std::int32_t>,
                                   ::rs::Option<::std::int32_t>>::set_tag)(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::rs::Option<::std::int32_t>,
                        ::rs::Option<::std::int32_t>>,
           ::rs::Option<::std::int32_t>, U>)
inline constexpr ::rs::Result<
    ::rs::Option<::std::int32_t>,
    ::rs::Option<::std::int32_t>>::Result(U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::rs::Option<::std::int32_t>,
                        ::rs::Option<::std::int32_t>>,
           ::rs::Option<::std::int32_t>, U>)
inline constexpr ::rs::Result<::rs::Option<::std::int32_t>,
                              ::rs::Option<::std::int32_t>>& ::
rs::Result<::rs::Option<::std::int32_t>,
           ::rs::Option<::std::int32_t>>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<::rs::Option<::std::int32_t>, F>)
inline constexpr ::rs::Result<
    ::rs::Option<::std::int32_t>,
    ::rs::Option<::std::int32_t>>::Result(::rs::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<::rs::Option<::std::int32_t>, F>)
inline constexpr ::rs::Result<::rs::Option<::std::int32_t>,
                              ::rs::Option<::std::int32_t>>& ::
rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>::
operator=(::rs::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr ::rs::
    Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>::Result(
        ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr ::rs::
    Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>::Result(
        ::rs::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e(
    ::rs::Result<
        ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        ::rs::Result<::rs::Option<::std::int32_t>,
                     ::rs::Option<::std::int32_t>>> const&,
    ::rs::Result<
        ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        ::rs::Result<::rs::Option<::std::int32_t>,
                     ::rs::Option<::std::int32_t>>>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e(
    ::rs::Result<
        ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        ::rs::Result<::rs::Option<::std::int32_t>,
                     ::rs::Option<::std::int32_t>>>&,
    ::rs::Result<
        ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        ::rs::Result<::rs::Option<::std::int32_t>,
                     ::rs::Option<::std::int32_t>>> const&);
}
inline ::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>::
    Result(const Result& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e(
          other, this);
}
inline ::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>,
                 ::rs::Option<::std::int32_t>>>& ::
rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>::
operator=(const Result& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e(
            *this, other);
  }
  return *this;
}
inline ::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>::
    Result(::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline ::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>,
                 ::rs::Option<::std::int32_t>>>::~Result() noexcept {
  this->Reset();
}
inline constexpr ::std::uint32_t(
    ::rs::Result<
        ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        ::rs::Result<::rs::Option<::std::int32_t>,
                     ::rs::Option<::std::int32_t>>>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void(
    ::rs::Result<
        ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        ::rs::Result<::rs::Option<::std::int32_t>,
                     ::rs::Option<::std::int32_t>>>::set_tag)(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::rs::Option<::rs::Result<::std::int32_t,
                                                  ::rs::alloc::string::String>>,
                        ::rs::Result<::rs::Option<::std::int32_t>,
                                     ::rs::Option<::std::int32_t>>>,
           ::rs::Option<
               ::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           U>)
inline constexpr ::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>,
                 ::rs::Option<::std::int32_t>>>::Result(U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::rs::Option<::rs::Result<::std::int32_t,
                                                  ::rs::alloc::string::String>>,
                        ::rs::Result<::rs::Option<::std::int32_t>,
                                     ::rs::Option<::std::int32_t>>>,
           ::rs::Option<
               ::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           U>)
inline constexpr ::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>,
                 ::rs::Option<::std::int32_t>>>& ::
rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>,
                 ::rs::Option<::std::int32_t>>>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<
           ::rs::Result<::rs::Option<::std::int32_t>,
                        ::rs::Option<::std::int32_t>>,
           F>)
inline constexpr ::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>::
    Result(::rs::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<
           ::rs::Result<::rs::Option<::std::int32_t>,
                        ::rs::Option<::std::int32_t>>,
           F>)
inline constexpr ::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>,
                 ::rs::Option<::std::int32_t>>>& ::
rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>::
operator=(::rs::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr ::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>::
    Result(::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr ::rs::Result<
    ::rs::Option<::rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    ::rs::Result<::rs::Option<::std::int32_t>, ::rs::Option<::std::int32_t>>>::
    Result(::rs::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#pragma clang diagnostic pop
#endif  // CC_BINDINGS_FROM_RS_TEST_ENUMS_OPTION_GOLDEN
