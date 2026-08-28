// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// option_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_OPTION_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_OPTION_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
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

// Error generating bindings for struct `option_golden::BridgedType` defined at
// cc_bindings_from_rs/test/enums/option.rs;l=234:
// Type bindings for option_golden::BridgedType suppressed due to being mapped
// to an existing C++ type (int)

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: CloneNoDefault") alignas(
    1) [[clang::trivial_abi]] CloneNoDefault final {
 public:
  ::std::uint8_t val = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: CopyNoDefault") alignas(1)
    [[clang::trivial_abi]] CopyNoDefault final {
 public:
  ::std::uint8_t val = {};

 private:
  static void __crubit_field_offset_assertions();
};

using FreeFunc CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: FreeFunc") =
    crubit::type_identity_t<void(void*, void*)>*;
// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: HasDefault") alignas(8)
    [[clang::trivial_abi]] HasDefault final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::option::HasDefault new_(rs_std::StrRef s);

  // CRUBIT_ANNOTATE: must_bind=
  rs_std::StrRef get_string_inside_option() const& $(__anon1)
      CRUBIT_LIFETIME_BOUND;

  ::rs::alloc::string::String foo = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: HasNoDefault") alignas(8)
    [[clang::trivial_abi]] HasNoDefault final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::option::HasNoDefault new_(rs_std::StrRef s);

  // CRUBIT_ANNOTATE: must_bind=
  rs_std::StrRef get_string_inside_option() const& $(__anon1)
      CRUBIT_LIFETIME_BOUND;

  ::rs::alloc::string::String foo = {};
  ::std::uint32_t a = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: LessThan20U8") alignas(1)
    [[clang::trivial_abi]] LessThan20U8 final {
 public:
  // `option_golden::LessThan20U8` doesn't implement the `Default` trait
  LessThan20U8() = delete;

  static constexpr LessThan20U8 MakeN0();

  static constexpr LessThan20U8 MakeN1();

  static constexpr LessThan20U8 MakeN2();

  static constexpr LessThan20U8 MakeN3();

  static constexpr LessThan20U8 MakeN4();

  static constexpr LessThan20U8 MakeN5();

  static constexpr LessThan20U8 MakeN6();

  static constexpr LessThan20U8 MakeN7();

  static constexpr LessThan20U8 MakeN8();

  static constexpr LessThan20U8 MakeN9();

  static constexpr LessThan20U8 MakeN10();

  static constexpr LessThan20U8 MakeN11();

  static constexpr LessThan20U8 MakeN12();

  static constexpr LessThan20U8 MakeN13();

  static constexpr LessThan20U8 MakeN14();

  static constexpr LessThan20U8 MakeN15();

  static constexpr LessThan20U8 MakeN16();

  static constexpr LessThan20U8 MakeN17();

  static constexpr LessThan20U8 MakeN18();

  static constexpr LessThan20U8 MakeN19();

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
  static rs_std::Option<::option::LessThan20U8> new_(::std::uint8_t value);

  // CRUBIT_ANNOTATE: must_bind=
  ::std::uint8_t value() const;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  ::std::array<unsigned char, 1> __opaque_blob_of_bytes;

 private:
  struct PrivateBytesTag {};
  constexpr LessThan20U8(PrivateBytesTag, ::std::array<unsigned char, 1> bytes)
      : __opaque_blob_of_bytes(bytes) {}

 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for struct `option_golden::OptUninhabited` defined
// at cc_bindings_from_rs/test/enums/option.rs;l=201:
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

// Error generating bindings for enum `option_golden::UninhabitedEnum` defined
// at cc_bindings_from_rs/test/enums/option.rs;l=199:
// Zero-sized types (ZSTs) are not supported (b/258259459)

// Error generating bindings for struct `option_golden::Unit` defined at
// cc_bindings_from_rs/test/enums/option.rs;l=206:
// Zero-sized types (ZSTs) are not supported (b/258259459)

using Voidpf CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: Voidpf") = void*;

// CRUBIT_ANNOTATE: must_bind=
rs_std::Option<::std::int32_t const*> pass_option_ptr(
    rs_std::Option<::std::int32_t const*> x);

// CRUBIT_ANNOTATE: must_bind=
rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>
return_option_result();

rs_std::Option<
    rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>
return_option_result_unmovable();

// CRUBIT_ANNOTATE: must_bind=
rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>
stress_testing_nested_types();

// CRUBIT_ANNOTATE: must_bind=
rs_std::Option<::std::uint32_t> stringify_len(
    rs_std::Option<::option::HasDefault> const& x);

// Error generating bindings for function `option_golden::take_option_bridged`
// defined at
// cc_bindings_from_rs/test/enums/option.rs;l=236:
// Error handling parameter #0 of type
// `std::option::Option<option_golden::BridgedType>`: Generic types are not
// supported yet (b/259749095)

void take_option_result_unmovable(
    rs_std::Option<
        rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>
        _x);

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < () >") rs_std::Option<rs_std::unit_t>
    : public rs_std::OptionBase<rs_std::Option<rs_std::unit_t>,
                                rs_std::unit_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      rs_std::OptionBase<rs_std::Option<rs_std::unit_t>, rs_std::unit_t>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option, rs_std::unit_t, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option, rs_std::unit_t, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<rs_std::unit_t, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<rs_std::unit_t, Opt>)
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint8_t;
  static constexpr tag_type kNoneVal = 0;
  rs_std::unit_t* some_ptr() noexcept {
    return reinterpret_cast<rs_std::unit_t*>(storage_ + 1);
  }
  rs_std::unit_t const* some_const_ptr() const noexcept {
    return reinterpret_cast<rs_std::unit_t const*>(storage_ + 1);
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

  rs_std::Option<rs_std::unit_t> unit = {};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020const_x00000020_x0000002a_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020const_x00000020_x0000002a_x00000020_x0000003e
template <>
struct alignas(8)
    CRUBIT_INTERNAL_RUST_TYPE("std :: option :: Option < * const i32 >")
        rs_std::Option<::std::int32_t const*>
    : public rs_std::OptionBase<rs_std::Option<::std::int32_t const*>,
                                ::std::int32_t const*> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = rs_std::OptionBase<rs_std::Option<::std::int32_t const*>,
                                       ::std::int32_t const*>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(
        rs_std::OptionForwardConstructible<Option, ::std::int32_t const*, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(
        rs_std::OptionForwardConstructible<Option, ::std::int32_t const*, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::std::int32_t const*, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::std::int32_t const*, Opt>)
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint64_t;
  static constexpr tag_type kNoneVal = 0;
  ::std::int32_t const** some_ptr() noexcept {
    return reinterpret_cast<::std::int32_t const**>(storage_ + 8);
  }
  ::std::int32_t const* const* some_const_ptr() const noexcept {
    return reinterpret_cast<::std::int32_t const* const*>(storage_ + 8);
  }
  void set_some_tag() noexcept { set_tag(1); }
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char storage_[16];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e
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

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < isize >") rs_std::Option<::std::int64_t>
    : public rs_std::OptionBase<rs_std::Option<::std::int64_t>,
                                ::std::int64_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      rs_std::OptionBase<rs_std::Option<::std::int64_t>, ::std::int64_t>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option, ::std::int64_t, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option, ::std::int64_t, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::std::int64_t, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::std::int64_t, Opt>)
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint64_t;
  static constexpr tag_type kNoneVal = 0;
  ::std::int64_t* some_ptr() noexcept {
    return reinterpret_cast<::std::int64_t*>(storage_ + 8);
  }
  ::std::int64_t const* some_const_ptr() const noexcept {
    return reinterpret_cast<::std::int64_t const*>(storage_ + 8);
  }
  void set_some_tag() noexcept { set_tag(1); }
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char storage_[16];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: option_golden :: CloneNoDefault >")
    rs_std::Option<::option::CloneNoDefault>
    : public rs_std::OptionBase<rs_std::Option<::option::CloneNoDefault>,
                                ::option::CloneNoDefault> {
 public:
  // Clone::clone
  Option(const Option&);

  // Clone::clone_from
  rs_std::Option<::option::CloneNoDefault>& operator=(const Option&);

  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = rs_std::OptionBase<rs_std::Option<::option::CloneNoDefault>,
                                       ::option::CloneNoDefault>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(
        rs_std::OptionForwardConstructible<Option, ::option::CloneNoDefault, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(
        rs_std::OptionForwardConstructible<Option, ::option::CloneNoDefault, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::option::CloneNoDefault, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::option::CloneNoDefault, Opt>)
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

  rs_std::Option<::option::CloneNoDefault> val = {};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: option_golden :: CopyNoDefault >")
    rs_std::Option<::option::CopyNoDefault>
    : public rs_std::OptionBase<rs_std::Option<::option::CopyNoDefault>,
                                ::option::CopyNoDefault> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = rs_std::OptionBase<rs_std::Option<::option::CopyNoDefault>,
                                       ::option::CopyNoDefault>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(
        rs_std::OptionForwardConstructible<Option, ::option::CopyNoDefault, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(
        rs_std::OptionForwardConstructible<Option, ::option::CopyNoDefault, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::option::CopyNoDefault, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::option::CopyNoDefault, Opt>)
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

  rs_std::Option<::option::CopyNoDefault> val = {};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: option_golden :: HasDefault >")
    rs_std::Option<::option::HasDefault>
    : public rs_std::OptionBase<rs_std::Option<::option::HasDefault>,
                                ::option::HasDefault> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&);
  rs_std::Option<::option::HasDefault>& operator=(Option&&);
  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = rs_std::OptionBase<rs_std::Option<::option::HasDefault>,
                                       ::option::HasDefault>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(
        rs_std::OptionForwardConstructible<Option, ::option::HasDefault, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(
        rs_std::OptionForwardConstructible<Option, ::option::HasDefault, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::option::HasDefault, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::option::HasDefault, Opt>)
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  constexpr ~Option() noexcept;

 private:
  friend base_type;
  using tag_type = ::std::uint64_t;
  static constexpr tag_type kNoneVal = UINT64_C(18446744073709551615);
  ::option::HasDefault* some_ptr() noexcept {
    return reinterpret_cast<::option::HasDefault*>(storage_);
  }
  ::option::HasDefault const* some_const_ptr() const noexcept {
    return reinterpret_cast<::option::HasDefault const*>(storage_);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char storage_[24];
};
#endif

namespace option {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: option_golden :: OptDefaultWithDrop") alignas(8) [[clang::trivial_abi]]
OptDefaultWithDrop final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::option::OptDefaultWithDrop new_(rs_std::StrRef s);

  rs_std::Option<::option::HasDefault> opt = {};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: option_golden :: HasNoDefault >")
    rs_std::Option<::option::HasNoDefault>
    : public rs_std::OptionBase<rs_std::Option<::option::HasNoDefault>,
                                ::option::HasNoDefault> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&);
  rs_std::Option<::option::HasNoDefault>& operator=(Option&&);
  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = rs_std::OptionBase<rs_std::Option<::option::HasNoDefault>,
                                       ::option::HasNoDefault>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(
        rs_std::OptionForwardConstructible<Option, ::option::HasNoDefault, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(
        rs_std::OptionForwardConstructible<Option, ::option::HasNoDefault, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::option::HasNoDefault, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::option::HasNoDefault, Opt>)
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  constexpr ~Option() noexcept;

 private:
  friend base_type;
  using tag_type = ::std::uint64_t;
  static constexpr tag_type kNoneVal = UINT64_C(18446744073709551615);
  ::option::HasNoDefault* some_ptr() noexcept {
    return reinterpret_cast<::option::HasNoDefault*>(storage_);
  }
  ::option::HasNoDefault const* some_const_ptr() const noexcept {
    return reinterpret_cast<::option::HasNoDefault const*>(storage_);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char storage_[32];
};
#endif

namespace option {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: option_golden :: OptNoDefaultWithDrop") alignas(8)
    [[clang::trivial_abi]] OptNoDefaultWithDrop final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::option::OptNoDefaultWithDrop new_(rs_std::StrRef s);

  // CRUBIT_ANNOTATE: must_bind=
  rs_std::StrRef get_string_inside_option() const& $(__anon1)
      CRUBIT_LIFETIME_BOUND;

  rs_std::Option<::option::HasNoDefault> val = {};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasOptions_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasOptions_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: option_golden :: HasOptions >")
    rs_std::Option<::option::HasOptions>
    : public rs_std::OptionBase<rs_std::Option<::option::HasOptions>,
                                ::option::HasOptions> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = rs_std::OptionBase<rs_std::Option<::option::HasOptions>,
                                       ::option::HasOptions>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(
        rs_std::OptionForwardConstructible<Option, ::option::HasOptions, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(
        rs_std::OptionForwardConstructible<Option, ::option::HasOptions, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::option::HasOptions, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::option::HasOptions, Opt>)
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

  rs_std::Option<::option::HasOptions> me = {};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: option_golden :: LessThan20U8 >")
    rs_std::Option<::option::LessThan20U8>
    : public rs_std::OptionBase<rs_std::Option<::option::LessThan20U8>,
                                ::option::LessThan20U8> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = rs_std::OptionBase<rs_std::Option<::option::LessThan20U8>,
                                       ::option::LessThan20U8>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(
        rs_std::OptionForwardConstructible<Option, ::option::LessThan20U8, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(
        rs_std::OptionForwardConstructible<Option, ::option::LessThan20U8, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::option::LessThan20U8, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::option::LessThan20U8, Opt>)
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

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: core :: option :: Option < :: option_golden "
    ":: LessThan20U8 > >")
    rs_std::Option<rs_std::Option<::option::LessThan20U8>>
    : public rs_std::OptionBase<
          rs_std::Option<rs_std::Option<::option::LessThan20U8>>,
          rs_std::Option<::option::LessThan20U8>> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      rs_std::OptionBase<rs_std::Option<rs_std::Option<::option::LessThan20U8>>,
                         rs_std::Option<::option::LessThan20U8>>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<
             Option, rs_std::Option<::option::LessThan20U8>, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<
             Option, rs_std::Option<::option::LessThan20U8>, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<
             rs_std::Option<::option::LessThan20U8>, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<
             rs_std::Option<::option::LessThan20U8>, Opt>)
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint8_t;
  static constexpr tag_type kNoneVal = 254;
  rs_std::Option<::option::LessThan20U8>* some_ptr() noexcept {
    return reinterpret_cast<rs_std::Option<::option::LessThan20U8>*>(storage_);
  }
  rs_std::Option<::option::LessThan20U8> const* some_const_ptr()
      const noexcept {
    return reinterpret_cast<rs_std::Option<::option::LessThan20U8> const*>(
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

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < u32 >") rs_std::Option<::std::uint32_t>
    : public rs_std::OptionBase<rs_std::Option<::std::uint32_t>,
                                ::std::uint32_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      rs_std::OptionBase<rs_std::Option<::std::uint32_t>, ::std::uint32_t>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option, ::std::uint32_t, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option, ::std::uint32_t, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::std::uint32_t, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::std::uint32_t, Opt>)
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

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020usigned_x00000020char_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020usigned_x00000020char_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < u8 >") rs_std::Option<::std::uint8_t>
    : public rs_std::OptionBase<rs_std::Option<::std::uint8_t>,
                                ::std::uint8_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      rs_std::OptionBase<rs_std::Option<::std::uint8_t>, ::std::uint8_t>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option, ::std::uint8_t, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option, ::std::uint8_t, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::std::uint8_t, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::std::uint8_t, Opt>)
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
  static ::option::HasOptions with_option(rs_std::Option<::std::uint8_t> value);

  // CRUBIT_ANNOTATE: must_bind=
  static ::option::HasOptions from_ref(
      rs_std::Option<::std::uint8_t> const& value);

  // CRUBIT_ANNOTATE: must_bind=
  static ::option::HasOptions with_none();

  rs_std::Option<::std::uint8_t> direct = {};
  rs_std::Option<::option::LessThan20U8> niche = {};
  rs_std::Option<rs_std::Option<::option::LessThan20U8>> nested = {};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < unsafe extern \"C\" fn (* mut :: core :: ffi :: "
    "c_void , * mut :: core :: ffi :: c_void) >")
    rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>
    : public rs_std::OptionBase<
          rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>,
          crubit::type_identity_t<void(void*, void*)>*> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = rs_std::OptionBase<
      rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>,
      crubit::type_identity_t<void(void*, void*)>*>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<
             Option, crubit::type_identity_t<void(void*, void*)>*, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<
             Option, crubit::type_identity_t<void(void*, void*)>*, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<
             crubit::type_identity_t<void(void*, void*)>*, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<
             crubit::type_identity_t<void(void*, void*)>*, Opt>)
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint64_t;
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
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char storage_[8];
};
#endif

namespace option {

struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: ZStream") alignas(8)
    [[clang::trivial_abi]] ZStream final {
 public:
  rs_std::Option<crubit::type_identity_t<void(void*, void*)>*> zfree = {};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < usize >") rs_std::Option<::std::uint64_t>
    : public rs_std::OptionBase<rs_std::Option<::std::uint64_t>,
                                ::std::uint64_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      rs_std::OptionBase<rs_std::Option<::std::uint64_t>, ::std::uint64_t>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option, ::std::uint64_t, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option, ::std::uint64_t, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::std::uint64_t, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::std::uint64_t, Opt>)
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint64_t;
  static constexpr tag_type kNoneVal = 0;
  ::std::uint64_t* some_ptr() noexcept {
    return reinterpret_cast<::std::uint64_t*>(storage_ + 8);
  }
  ::std::uint64_t const* some_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint64_t const*>(storage_ + 8);
  }
  void set_some_tag() noexcept { set_tag(1); }
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char storage_[16];
};
#endif

namespace option {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: option_golden :: OptionWithSizeTypes") alignas(8) [[clang::trivial_abi]]
OptionWithSizeTypes final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::option::OptionWithSizeTypes new_(
      rs_std::Option<::std::uint64_t> uval,
      rs_std::Option<::std::int64_t> ival);

  rs_std::Option<::std::uint64_t> uval = {};
  rs_std::Option<::std::int64_t> ival = {};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020int_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020int_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
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

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020int_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020int_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: core :: result :: Result < i32 , :: alloc :: "
    "string :: String > >")
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>
    : public rs_std::OptionBase<
          rs_std::Option<
              rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
          rs_std::Result<::std::int32_t, ::rs::alloc::string::String>> {
 public:
  // Clone::clone
  Option(const Option&);

  // Clone::clone_from
  rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>&
  operator=(const Option&);

  Option(Option&&);
  rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>&
  operator=(Option&&);
  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = rs_std::OptionBase<
      rs_std::Option<
          rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
      rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<
             Option,
             rs_std::Result<::std::int32_t, ::rs::alloc::string::String>, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<
             Option,
             rs_std::Result<::std::int32_t, ::rs::alloc::string::String>, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<
             rs_std::Result<::std::int32_t, ::rs::alloc::string::String>, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<
             rs_std::Result<::std::int32_t, ::rs::alloc::string::String>, Opt>)
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  constexpr ~Option() noexcept;

 private:
  friend base_type;
  using tag_type = ::std::uint64_t;
  static constexpr tag_type kNoneVal = UINT64_C(18446744073709551614);
  rs_std::Result<::std::int32_t, ::rs::alloc::string::String>*
  some_ptr() noexcept {
    return reinterpret_cast<
        rs_std::Result<::std::int32_t, ::rs::alloc::string::String>*>(storage_);
  }
  rs_std::Result<::std::int32_t, ::rs::alloc::string::String> const*
  some_const_ptr() const noexcept {
    return reinterpret_cast<
        rs_std::Result<::std::int32_t, ::rs::alloc::string::String> const*>(
        storage_);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char storage_[24];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: option_golden :: HasNoDefault , :: alloc :: "
    "string :: String >")
    rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>
    : public rs_std::ResultBase<
          rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>,
          ::option::HasNoDefault, ::rs::alloc::string::String> {
 public:
  // `core::result::Result` doesn't implement the `Clone` trait
  Result(const Result&) = delete;
  Result& operator=(const Result&) = delete;
  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  Result(Result&&) = delete;
  rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>&
  operator=(Result&&) = delete;
  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type = rs_std::ResultBase<
      rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>,
      ::option::HasNoDefault, ::rs::alloc::string::String>;
  template <typename U>
    requires(
        rs_std::ResultForwardConstructible<Result, ::option::HasNoDefault, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(
        rs_std::ResultForwardConstructible<Result, ::option::HasNoDefault, U>)
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
    return tag() != UINT64_C(18446744073709551615);
  }
  ::option::HasNoDefault* ok_ptr() noexcept {
    return reinterpret_cast<::option::HasNoDefault*>(__storage);
  }
  ::option::HasNoDefault const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::option::HasNoDefault const*>(__storage);
  }
  ::rs::alloc::string::String* err_ptr() noexcept {
    return reinterpret_cast<::rs::alloc::string::String*>(__storage + 8);
  }
  ::rs::alloc::string::String const* err_const_ptr() const noexcept {
    return reinterpret_cast<::rs::alloc::string::String const*>(__storage + 8);
  }
  void set_ok_tag() noexcept {}
  void set_err_tag() noexcept { set_tag(UINT64_C(18446744073709551615)); }
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char __storage[32];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: core :: result :: Result < :: option_golden "
    ":: HasNoDefault , :: alloc :: string :: String > >") rs_std::
    Option<rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>
    : public rs_std::OptionBase<
          rs_std::Option<rs_std::Result<::option::HasNoDefault,
                                        ::rs::alloc::string::String>>,
          rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&);
  rs_std::Option<
      rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>&
  operator=(Option&&);
  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = rs_std::OptionBase<
      rs_std::Option<
          rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>,
      rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(
        rs_std::OptionForwardConstructible<
            Option,
            rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>,
            U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(
        rs_std::OptionForwardConstructible<
            Option,
            rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>,
            U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(
        rs_std::OptionFromStdOptional<
            rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>,
            Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(
        rs_std::OptionFromStdOptional<
            rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>,
            Opt>)
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  constexpr ~Option() noexcept;

 private:
  friend base_type;
  using tag_type = ::std::uint64_t;
  static constexpr tag_type kNoneVal = UINT64_C(18446744073709551614);
  rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>*
  some_ptr() noexcept {
    return reinterpret_cast<
        rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>*>(
        storage_);
  }
  rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String> const*
  some_const_ptr() const noexcept {
    return reinterpret_cast<rs_std::Result<::option::HasNoDefault,
                                           ::rs::alloc::string::String> const*>(
        storage_);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char storage_[32];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: core :: option :: Option < i32 > , :: core "
    ":: option :: Option < i32 > >")
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>
    : public rs_std::ResultBase<rs_std::Result<rs_std::Option<::std::int32_t>,
                                               rs_std::Option<::std::int32_t>>,
                                rs_std::Option<::std::int32_t>,
                                rs_std::Option<::std::int32_t>> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Result(const Result&) = default;
  Result& operator=(const Result&) = default;
  Result(Result&&) = default;
  Result& operator=(Result&&) = default;

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type =
      rs_std::ResultBase<rs_std::Result<rs_std::Option<::std::int32_t>,
                                        rs_std::Option<::std::int32_t>>,
                         rs_std::Option<::std::int32_t>,
                         rs_std::Option<::std::int32_t>>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<
             Result, rs_std::Option<::std::int32_t>, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<
             Result, rs_std::Option<::std::int32_t>, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<
             rs_std::Option<::std::int32_t>, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<
             rs_std::Option<::std::int32_t>, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  rs_std::Option<::std::int32_t>* ok_ptr() noexcept {
    return reinterpret_cast<rs_std::Option<::std::int32_t>*>(__storage + 4);
  }
  rs_std::Option<::std::int32_t> const* ok_const_ptr() const noexcept {
    return reinterpret_cast<rs_std::Option<::std::int32_t> const*>(__storage +
                                                                   4);
  }
  rs_std::Option<::std::int32_t>* err_ptr() noexcept {
    return reinterpret_cast<rs_std::Option<::std::int32_t>*>(__storage + 4);
  }
  rs_std::Option<::std::int32_t> const* err_const_ptr() const noexcept {
    return reinterpret_cast<rs_std::Option<::std::int32_t> const*>(__storage +
                                                                   4);
  }
  void set_ok_tag() noexcept { set_tag(0); }
  void set_err_tag() noexcept { set_tag(1); }
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;

 private:
  unsigned char __storage[12];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020int_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020int_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: core :: option :: Option < :: core :: result "
    ":: Result < i32 , :: alloc :: string :: String > > , :: core :: result :: "
    "Result < :: core :: option :: Option < i32 > , :: core :: option :: "
    "Option < i32 > > >")
    rs_std::Result<rs_std::Option<rs_std::Result<::std::int32_t,
                                                 ::rs::alloc::string::String>>,
                   rs_std::Result<rs_std::Option<::std::int32_t>,
                                  rs_std::Option<::std::int32_t>>>
    : public rs_std::ResultBase<
          rs_std::Result<rs_std::Option<rs_std::Result<
                             ::std::int32_t, ::rs::alloc::string::String>>,
                         rs_std::Result<rs_std::Option<::std::int32_t>,
                                        rs_std::Option<::std::int32_t>>>,
          rs_std::Option<
              rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
          rs_std::Result<rs_std::Option<::std::int32_t>,
                         rs_std::Option<::std::int32_t>>> {
 public:
  // Clone::clone
  Result(const Result&);

  // Clone::clone_from
  rs_std::Result<rs_std::Option<rs_std::Result<::std::int32_t,
                                               ::rs::alloc::string::String>>,
                 rs_std::Result<rs_std::Option<::std::int32_t>,
                                rs_std::Option<::std::int32_t>>>&
  operator=(const Result&);

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type = rs_std::ResultBase<
      rs_std::Result<rs_std::Option<rs_std::Result<
                         ::std::int32_t, ::rs::alloc::string::String>>,
                     rs_std::Result<rs_std::Option<::std::int32_t>,
                                    rs_std::Option<::std::int32_t>>>,
      rs_std::Option<
          rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
      rs_std::Result<rs_std::Option<::std::int32_t>,
                     rs_std::Option<::std::int32_t>>>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<
             Result,
             rs_std::Option<
                 rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
             U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<
             Result,
             rs_std::Option<
                 rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
             U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<
             rs_std::Result<rs_std::Option<::std::int32_t>,
                            rs_std::Option<::std::int32_t>>,
             F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<
             rs_std::Result<rs_std::Option<::std::int32_t>,
                            rs_std::Option<::std::int32_t>>,
             F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept;

 private:
  friend base_type;
  bool has_value_impl() const noexcept {
    return tag() != UINT64_C(18446744073709551613);
  }
  rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>*
  ok_ptr() noexcept {
    return reinterpret_cast<rs_std::Option<
        rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>*>(
        __storage);
  }
  rs_std::Option<
      rs_std::Result<::std::int32_t, ::rs::alloc::string::String>> const*
  ok_const_ptr() const noexcept {
    return reinterpret_cast<rs_std::Option<
        rs_std::Result<::std::int32_t, ::rs::alloc::string::String>> const*>(
        __storage);
  }
  rs_std::Result<rs_std::Option<::std::int32_t>,
                 rs_std::Option<::std::int32_t>>*
  err_ptr() noexcept {
    return reinterpret_cast<rs_std::Result<rs_std::Option<::std::int32_t>,
                                           rs_std::Option<::std::int32_t>>*>(
        __storage + 8);
  }
  rs_std::Result<rs_std::Option<::std::int32_t>,
                 rs_std::Option<::std::int32_t>> const*
  err_const_ptr() const noexcept {
    return reinterpret_cast<rs_std::Result<
        rs_std::Option<::std::int32_t>, rs_std::Option<::std::int32_t>> const*>(
        __storage + 8);
  }
  void set_ok_tag() noexcept {}
  void set_err_tag() noexcept { set_tag(UINT64_C(18446744073709551613)); }
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char __storage[24];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020int_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020int_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: core :: result :: Result < :: core :: option "
    ":: Option < :: core :: result :: Result < i32 , :: alloc :: string :: "
    "String > > , :: core :: result :: Result < :: core :: option :: Option < "
    "i32 > , :: core :: option :: Option < i32 > > > >") rs_std::
    Option<rs_std::Result<rs_std::Option<rs_std::Result<
                              ::std::int32_t, ::rs::alloc::string::String>>,
                          rs_std::Result<rs_std::Option<::std::int32_t>,
                                         rs_std::Option<::std::int32_t>>>>
    : public rs_std::OptionBase<
          rs_std::Option<
              rs_std::Result<rs_std::Option<rs_std::Result<
                                 ::std::int32_t, ::rs::alloc::string::String>>,
                             rs_std::Result<rs_std::Option<::std::int32_t>,
                                            rs_std::Option<::std::int32_t>>>>,
          rs_std::Result<rs_std::Option<rs_std::Result<
                             ::std::int32_t, ::rs::alloc::string::String>>,
                         rs_std::Result<rs_std::Option<::std::int32_t>,
                                        rs_std::Option<::std::int32_t>>>> {
 public:
  // Clone::clone
  Option(const Option&);

  // Clone::clone_from
  rs_std::Option<
      rs_std::Result<rs_std::Option<rs_std::Result<
                         ::std::int32_t, ::rs::alloc::string::String>>,
                     rs_std::Result<rs_std::Option<::std::int32_t>,
                                    rs_std::Option<::std::int32_t>>>>&
  operator=(const Option&);

  Option(Option&&);
  rs_std::Option<
      rs_std::Result<rs_std::Option<rs_std::Result<
                         ::std::int32_t, ::rs::alloc::string::String>>,
                     rs_std::Result<rs_std::Option<::std::int32_t>,
                                    rs_std::Option<::std::int32_t>>>>&
  operator=(Option&&);
  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = rs_std::OptionBase<
      rs_std::Option<
          rs_std::Result<rs_std::Option<rs_std::Result<
                             ::std::int32_t, ::rs::alloc::string::String>>,
                         rs_std::Result<rs_std::Option<::std::int32_t>,
                                        rs_std::Option<::std::int32_t>>>>,
      rs_std::Result<rs_std::Option<rs_std::Result<
                         ::std::int32_t, ::rs::alloc::string::String>>,
                     rs_std::Result<rs_std::Option<::std::int32_t>,
                                    rs_std::Option<::std::int32_t>>>>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<
             Option,
             rs_std::Result<rs_std::Option<rs_std::Result<
                                ::std::int32_t, ::rs::alloc::string::String>>,
                            rs_std::Result<rs_std::Option<::std::int32_t>,
                                           rs_std::Option<::std::int32_t>>>,
             U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<
             Option,
             rs_std::Result<rs_std::Option<rs_std::Result<
                                ::std::int32_t, ::rs::alloc::string::String>>,
                            rs_std::Result<rs_std::Option<::std::int32_t>,
                                           rs_std::Option<::std::int32_t>>>,
             U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<
             rs_std::Result<rs_std::Option<rs_std::Result<
                                ::std::int32_t, ::rs::alloc::string::String>>,
                            rs_std::Result<rs_std::Option<::std::int32_t>,
                                           rs_std::Option<::std::int32_t>>>,
             Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<
             rs_std::Result<rs_std::Option<rs_std::Result<
                                ::std::int32_t, ::rs::alloc::string::String>>,
                            rs_std::Result<rs_std::Option<::std::int32_t>,
                                           rs_std::Option<::std::int32_t>>>,
             Opt>)
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  constexpr ~Option() noexcept;

 private:
  friend base_type;
  using tag_type = ::std::uint64_t;
  static constexpr tag_type kNoneVal = UINT64_C(18446744073709551612);
  rs_std::Result<rs_std::Option<rs_std::Result<::std::int32_t,
                                               ::rs::alloc::string::String>>,
                 rs_std::Result<rs_std::Option<::std::int32_t>,
                                rs_std::Option<::std::int32_t>>>*
  some_ptr() noexcept {
    return reinterpret_cast<rs_std::Result<
        rs_std::Option<
            rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
        rs_std::Result<rs_std::Option<::std::int32_t>,
                       rs_std::Option<::std::int32_t>>>*>(storage_);
  }
  rs_std::Result<rs_std::Option<rs_std::Result<::std::int32_t,
                                               ::rs::alloc::string::String>>,
                 rs_std::Result<rs_std::Option<::std::int32_t>,
                                rs_std::Option<::std::int32_t>>> const*
  some_const_ptr() const noexcept {
    return reinterpret_cast<rs_std::Result<
        rs_std::Option<
            rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
        rs_std::Result<rs_std::Option<::std::int32_t>,
                       rs_std::Option<::std::int32_t>>> const*>(storage_);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char storage_[24];
};
#endif

namespace option {

static_assert(
    sizeof(CloneNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CloneNoDefault>);
static_assert(
    ::std::is_trivially_move_constructible_v<::option::CloneNoDefault>);
static_assert(::std::is_trivially_move_assignable_v<::option::CloneNoDefault>);
inline void CloneNoDefault::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CloneNoDefault, val));
}
static_assert(
    sizeof(CopyNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CopyNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CopyNoDefault>);
static_assert(
    ::std::is_trivially_move_constructible_v<::option::CopyNoDefault>);
static_assert(::std::is_trivially_move_assignable_v<::option::CopyNoDefault>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::option::CopyNoDefault>);
static_assert(::std::is_trivially_copy_assignable_v<::option::CopyNoDefault>);
inline void CopyNoDefault::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CopyNoDefault, val));
}
static_assert(
    sizeof(HasDefault) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(HasDefault) == 8,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(rs_std::StrRef,
                                   ::option::HasDefault* __ret_ptr);
}
inline ::option::HasDefault HasDefault::new_(rs_std::StrRef s) {
  crubit::Slot<::option::HasDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(s, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" rs_std::StrRef __crubit_thunk_get_ustring_uinside_uoption(
    ::option::HasDefault const&);
}
inline rs_std::StrRef HasDefault::get_string_inside_option() const& $(__anon1)
    CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_ustring_uinside_uoption(self);
}
inline void HasDefault::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(HasDefault, foo));
}
static_assert(
    sizeof(HasHasOptions) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(HasHasOptions) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<HasHasOptions>);
static_assert(
    ::std::is_trivially_move_constructible_v<::option::HasHasOptions>);
static_assert(::std::is_trivially_move_assignable_v<::option::HasHasOptions>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uint8_t,
                                   ::option::HasHasOptions* __ret_ptr);
}
inline ::option::HasHasOptions HasHasOptions::new_(::std::uint8_t value) {
  crubit::Slot<::option::HasHasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void HasHasOptions::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(HasHasOptions, me));
}
static_assert(
    sizeof(HasNoDefault) == 32,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(HasNoDefault) == 8,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(rs_std::StrRef,
                                   ::option::HasNoDefault* __ret_ptr);
}
inline ::option::HasNoDefault HasNoDefault::new_(rs_std::StrRef s) {
  crubit::Slot<::option::HasNoDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(s, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" rs_std::StrRef __crubit_thunk_get_ustring_uinside_uoption(
    ::option::HasNoDefault const&);
}
inline rs_std::StrRef HasNoDefault::get_string_inside_option() const& $(__anon1)
    CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_ustring_uinside_uoption(self);
}
inline void HasNoDefault::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(HasNoDefault, foo));
  static_assert(24 == offsetof(HasNoDefault, a));
}
static_assert(
    sizeof(HasOptions) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(HasOptions) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<HasOptions>);
static_assert(::std::is_trivially_move_constructible_v<::option::HasOptions>);
static_assert(::std::is_trivially_move_assignable_v<::option::HasOptions>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uint8_t,
                                   ::option::HasOptions* __ret_ptr);
}
inline ::option::HasOptions HasOptions::new_(::std::uint8_t value) {
  crubit::Slot<::option::HasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_with_uoption(rs_std::Option<::std::uint8_t>*,
                                            ::option::HasOptions* __ret_ptr);
}
inline ::option::HasOptions HasOptions::with_option(
    rs_std::Option<::std::uint8_t> value) {
  crubit::Slot<::option::HasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_with_uoption(&value,
                                                 __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_from_uref(rs_std::Option<::std::uint8_t> const&,
                                         ::option::HasOptions* __ret_ptr);
}
inline ::option::HasOptions HasOptions::from_ref(
    rs_std::Option<::std::uint8_t> const& value) {
  crubit::Slot<::option::HasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_from_uref(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_with_unone(::option::HasOptions* __ret_ptr);
}
inline ::option::HasOptions HasOptions::with_none() {
  crubit::Slot<::option::HasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_with_unone(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void HasOptions::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(HasOptions, direct));
  static_assert(2 == offsetof(HasOptions, niche));
  static_assert(3 == offsetof(HasOptions, nested));
}
static_assert(
    sizeof(LessThan20U8) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(LessThan20U8) == 1,
    "Verify that ADT layout didn't change since this header got generated");

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN0() {
  return LessThan20U8(PrivateBytesTag{}, {0});
}

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN1() {
  return LessThan20U8(PrivateBytesTag{}, {1});
}

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN2() {
  return LessThan20U8(PrivateBytesTag{}, {2});
}

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN3() {
  return LessThan20U8(PrivateBytesTag{}, {3});
}

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN4() {
  return LessThan20U8(PrivateBytesTag{}, {4});
}

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN5() {
  return LessThan20U8(PrivateBytesTag{}, {5});
}

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN6() {
  return LessThan20U8(PrivateBytesTag{}, {6});
}

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN7() {
  return LessThan20U8(PrivateBytesTag{}, {7});
}

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN8() {
  return LessThan20U8(PrivateBytesTag{}, {8});
}

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN9() {
  return LessThan20U8(PrivateBytesTag{}, {9});
}

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN10() {
  return LessThan20U8(PrivateBytesTag{}, {10});
}

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN11() {
  return LessThan20U8(PrivateBytesTag{}, {11});
}

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN12() {
  return LessThan20U8(PrivateBytesTag{}, {12});
}

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN13() {
  return LessThan20U8(PrivateBytesTag{}, {13});
}

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN14() {
  return LessThan20U8(PrivateBytesTag{}, {14});
}

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN15() {
  return LessThan20U8(PrivateBytesTag{}, {15});
}

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN16() {
  return LessThan20U8(PrivateBytesTag{}, {16});
}

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN17() {
  return LessThan20U8(PrivateBytesTag{}, {17});
}

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN18() {
  return LessThan20U8(PrivateBytesTag{}, {18});
}

// `static` constructor
inline constexpr LessThan20U8 LessThan20U8::MakeN19() {
  return LessThan20U8(PrivateBytesTag{}, {19});
}
static_assert(::std::is_trivially_destructible_v<LessThan20U8>);
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
    ::std::uint8_t, rs_std::Option<::option::LessThan20U8>* __ret_ptr);
}
inline rs_std::Option<::option::LessThan20U8> LessThan20U8::new_(
    ::std::uint8_t value) {
  crubit::Slot<rs_std::Option<::option::LessThan20U8>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::uint8_t __crubit_thunk_value(::option::LessThan20U8*);
}
inline ::std::uint8_t LessThan20U8::value() const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_value(&self);
}
inline void LessThan20U8::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(LessThan20U8, __opaque_blob_of_bytes));
}
static_assert(
    sizeof(OptCloneNoDefault) == 2,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(OptCloneNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<OptCloneNoDefault>);
static_assert(
    ::std::is_trivially_move_constructible_v<::option::OptCloneNoDefault>);
static_assert(
    ::std::is_trivially_move_assignable_v<::option::OptCloneNoDefault>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uint8_t,
                                   ::option::OptCloneNoDefault* __ret_ptr);
}
inline ::option::OptCloneNoDefault OptCloneNoDefault::new_(::std::uint8_t x) {
  crubit::Slot<::option::OptCloneNoDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void OptCloneNoDefault::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(OptCloneNoDefault, val));
}
static_assert(
    sizeof(OptCopyNoDefault) == 2,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(OptCopyNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<OptCopyNoDefault>);
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
inline ::option::OptCopyNoDefault OptCopyNoDefault::new_(::std::uint8_t x) {
  crubit::Slot<::option::OptCopyNoDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void OptCopyNoDefault::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(OptCopyNoDefault, val));
}
static_assert(
    sizeof(OptDefaultWithDrop) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(OptDefaultWithDrop) == 8,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(rs_std::StrRef,
                                   ::option::OptDefaultWithDrop* __ret_ptr);
}
inline ::option::OptDefaultWithDrop OptDefaultWithDrop::new_(rs_std::StrRef s) {
  crubit::Slot<::option::OptDefaultWithDrop> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(s, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void OptDefaultWithDrop::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(OptDefaultWithDrop, opt));
}
static_assert(
    sizeof(OptNoDefaultWithDrop) == 32,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(OptNoDefaultWithDrop) == 8,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(rs_std::StrRef,
                                   ::option::OptNoDefaultWithDrop* __ret_ptr);
}
inline ::option::OptNoDefaultWithDrop OptNoDefaultWithDrop::new_(
    rs_std::StrRef s) {
  crubit::Slot<::option::OptNoDefaultWithDrop> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(s, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" rs_std::StrRef __crubit_thunk_get_ustring_uinside_uoption(
    ::option::OptNoDefaultWithDrop const&);
}
inline rs_std::StrRef OptNoDefaultWithDrop::get_string_inside_option() const& $(
    __anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_ustring_uinside_uoption(self);
}
inline void OptNoDefaultWithDrop::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(OptNoDefaultWithDrop, val));
}
static_assert(
    sizeof(OptZst) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(OptZst) == 1,
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
static_assert(::std::is_trivially_destructible_v<OptZst>);
static_assert(::std::is_trivially_move_constructible_v<::option::OptZst>);
static_assert(::std::is_trivially_move_assignable_v<::option::OptZst>);
inline ::option::OptZst::OptZst(::crubit::UnsafeRelocateTag, OptZst&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void OptZst::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(OptZst, val));
}
static_assert(
    sizeof(OptionWithSizeTypes) == 32,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(OptionWithSizeTypes) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<OptionWithSizeTypes>);
static_assert(
    ::std::is_trivially_move_constructible_v<::option::OptionWithSizeTypes>);
static_assert(
    ::std::is_trivially_move_assignable_v<::option::OptionWithSizeTypes>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(rs_std::Option<::std::uint64_t>*,
                                   rs_std::Option<::std::int64_t>*,
                                   ::option::OptionWithSizeTypes* __ret_ptr);
}
inline ::option::OptionWithSizeTypes OptionWithSizeTypes::new_(
    rs_std::Option<::std::uint64_t> uval, rs_std::Option<::std::int64_t> ival) {
  crubit::Slot<::option::OptionWithSizeTypes> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(&uval, &ival, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void OptionWithSizeTypes::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(OptionWithSizeTypes, uval));
  static_assert(16 == offsetof(OptionWithSizeTypes, ival));
}
static_assert(
    sizeof(UnitOptionField) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(UnitOptionField) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<UnitOptionField>);
static_assert(
    ::std::is_trivially_move_constructible_v<::option::UnitOptionField>);
static_assert(::std::is_trivially_move_assignable_v<::option::UnitOptionField>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new_uwith_usome(
    ::option::UnitOptionField* __ret_ptr);
}
inline ::option::UnitOptionField UnitOptionField::new_with_some() {
  crubit::Slot<::option::UnitOptionField> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new_uwith_usome(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void UnitOptionField::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(UnitOptionField, unit));
}
static_assert(
    sizeof(ZStream) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(ZStream) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<ZStream>);
static_assert(::std::is_trivially_move_constructible_v<::option::ZStream>);
static_assert(::std::is_trivially_move_assignable_v<::option::ZStream>);
inline void ZStream::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ZStream, zfree));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_pass_uoption_uptr(
    rs_std::Option<::std::int32_t const*>*,
    rs_std::Option<::std::int32_t const*>* __ret_ptr);
}
inline rs_std::Option<::std::int32_t const*> pass_option_ptr(
    rs_std::Option<::std::int32_t const*> x) {
  crubit::Slot<rs_std::Option<::std::int32_t const*>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_pass_uoption_uptr(&x,
                                                      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uoption_uresult(
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>*
        __ret_ptr);
}
inline rs_std::Option<
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>
return_option_result() {
  crubit::Slot<rs_std::Option<
      rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_return_uoption_uresult(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uoption_uresult_uunmovable(
    rs_std::Option<rs_std::Result<::option::HasNoDefault,
                                  ::rs::alloc::string::String>>* __ret_ptr);
}
inline rs_std::Option<
    rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>
return_option_result_unmovable() {
  crubit::Slot<rs_std::Option<
      rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_return_uoption_uresult_uunmovable(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_stress_utesting_unested_utypes(
    rs_std::Option<rs_std::Result<
        rs_std::Option<
            rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
        rs_std::Result<rs_std::Option<::std::int32_t>,
                       rs_std::Option<::std::int32_t>>>>* __ret_ptr);
}
inline rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>
stress_testing_nested_types() {
  crubit::Slot<rs_std::Option<
      rs_std::Result<rs_std::Option<rs_std::Result<
                         ::std::int32_t, ::rs::alloc::string::String>>,
                     rs_std::Result<rs_std::Option<::std::int32_t>,
                                    rs_std::Option<::std::int32_t>>>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_stress_utesting_unested_utypes(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_stringify_ulen(
    rs_std::Option<::option::HasDefault> const&,
    rs_std::Option<::std::uint32_t>* __ret_ptr);
}
inline rs_std::Option<::std::uint32_t> stringify_len(
    rs_std::Option<::option::HasDefault> const& x) {
  crubit::Slot<rs_std::Option<::std::uint32_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_stringify_ulen(x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_take_uoption_uresult_uunmovable(
    rs_std::Option<
        rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>*);
}
inline void take_option_result_unmovable(
    rs_std::Option<
        rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>
        _x) {
  crubit::Slot _x_slot((::std::move(_x)));
  return __crubit_internal::__crubit_thunk_take_uoption_uresult_uunmovable(
      _x_slot.Get());
}

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<rs_std::Option<rs_std::unit_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<rs_std::Option<rs_std::unit_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<rs_std::Option<rs_std::unit_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<rs_std::Option<rs_std::unit_t>>);
inline rs_std::Option<rs_std::unit_t>::Option(::crubit::UnsafeRelocateTag,
                                              Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<rs_std::Option<rs_std::unit_t>>);
inline constexpr ::std::uint8_t rs_std::Option<rs_std::unit_t>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs_std::Option<rs_std::unit_t>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<rs_std::unit_t>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<rs_std::unit_t>&
rs_std::Option<rs_std::unit_t>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<rs_std::Option<rs_std::unit_t>,
                                              rs_std::unit_t, U>)
inline rs_std::Option<rs_std::unit_t>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<rs_std::Option<rs_std::unit_t>,
                                              rs_std::unit_t, U>)
inline rs_std::Option<rs_std::unit_t>&
rs_std::Option<rs_std::unit_t>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<rs_std::unit_t, Opt>)
inline rs_std::Option<rs_std::unit_t>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<rs_std::unit_t, Opt>)
inline rs_std::Option<rs_std::unit_t>&
rs_std::Option<rs_std::unit_t>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<rs_std::unit_t>::Option(::std::in_place_t ip,
                                              Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020const_x00000020_x0000002a_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020const_x00000020_x0000002a_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Option<::std::int32_t const*>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Option<::std::int32_t const*>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Option<::std::int32_t const*>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Option<::std::int32_t const*>>);
inline rs_std::Option<::std::int32_t const*>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<rs_std::Option<::std::int32_t const*>>);
inline constexpr ::std::uint64_t rs_std::Option<::std::int32_t const*>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void rs_std::Option<::std::int32_t const*>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<::std::int32_t const*>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<::std::int32_t const*>&
rs_std::Option<::std::int32_t const*>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::std::int32_t const*>, ::std::int32_t const*, U>)
inline rs_std::Option<::std::int32_t const*>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::std::int32_t const*>, ::std::int32_t const*, U>)
inline rs_std::Option<::std::int32_t const*>&
rs_std::Option<::std::int32_t const*>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::std::int32_t const*, Opt>)
inline rs_std::Option<::std::int32_t const*>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::std::int32_t const*, Opt>)
inline rs_std::Option<::std::int32_t const*>&
rs_std::Option<::std::int32_t const*>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<::std::int32_t const*>::Option(::std::in_place_t ip,
                                                     Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e
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

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<rs_std::Option<::std::int64_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<rs_std::Option<::std::int64_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<rs_std::Option<::std::int64_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<rs_std::Option<::std::int64_t>>);
inline rs_std::Option<::std::int64_t>::Option(::crubit::UnsafeRelocateTag,
                                              Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<rs_std::Option<::std::int64_t>>);
inline constexpr ::std::uint64_t rs_std::Option<::std::int64_t>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void rs_std::Option<::std::int64_t>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<::std::int64_t>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<::std::int64_t>&
rs_std::Option<::std::int64_t>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<rs_std::Option<::std::int64_t>,
                                              ::std::int64_t, U>)
inline rs_std::Option<::std::int64_t>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<rs_std::Option<::std::int64_t>,
                                              ::std::int64_t, U>)
inline rs_std::Option<::std::int64_t>&
rs_std::Option<::std::int64_t>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::std::int64_t, Opt>)
inline rs_std::Option<::std::int64_t>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::std::int64_t, Opt>)
inline rs_std::Option<::std::int64_t>&
rs_std::Option<::std::int64_t>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<::std::int64_t>::Option(::std::in_place_t ip,
                                              Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e(
    rs_std::Option<::option::CloneNoDefault> const&,
    rs_std::Option<::option::CloneNoDefault>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e(
    rs_std::Option<::option::CloneNoDefault>&,
    rs_std::Option<::option::CloneNoDefault> const&);
}
inline rs_std::Option<::option::CloneNoDefault>::Option(const Option& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e(
          other, this);
}
inline rs_std::Option<::option::CloneNoDefault>&
rs_std::Option<::option::CloneNoDefault>::operator=(const Option& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e(
            *this, other);
  }
  return *this;
}
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Option<::option::CloneNoDefault>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Option<::option::CloneNoDefault>>);
inline rs_std::Option<::option::CloneNoDefault>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Option<::option::CloneNoDefault>>);
inline constexpr ::std::uint8_t rs_std::Option<::option::CloneNoDefault>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs_std::Option<::option::CloneNoDefault>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<::option::CloneNoDefault>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<::option::CloneNoDefault>&
rs_std::Option<::option::CloneNoDefault>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::option::CloneNoDefault>, ::option::CloneNoDefault,
           U>)
inline rs_std::Option<::option::CloneNoDefault>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::option::CloneNoDefault>, ::option::CloneNoDefault,
           U>)
inline rs_std::Option<::option::CloneNoDefault>&
rs_std::Option<::option::CloneNoDefault>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::option::CloneNoDefault, Opt>)
inline rs_std::Option<::option::CloneNoDefault>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::option::CloneNoDefault, Opt>)
inline rs_std::Option<::option::CloneNoDefault>&
rs_std::Option<::option::CloneNoDefault>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<::option::CloneNoDefault>::Option(::std::in_place_t ip,
                                                        Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Option<::option::CopyNoDefault>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Option<::option::CopyNoDefault>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Option<::option::CopyNoDefault>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Option<::option::CopyNoDefault>>);
inline rs_std::Option<::option::CopyNoDefault>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Option<::option::CopyNoDefault>>);
inline constexpr ::std::uint8_t rs_std::Option<::option::CopyNoDefault>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs_std::Option<::option::CopyNoDefault>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<::option::CopyNoDefault>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<::option::CopyNoDefault>&
rs_std::Option<::option::CopyNoDefault>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::option::CopyNoDefault>, ::option::CopyNoDefault, U>)
inline rs_std::Option<::option::CopyNoDefault>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::option::CopyNoDefault>, ::option::CopyNoDefault, U>)
inline rs_std::Option<::option::CopyNoDefault>&
rs_std::Option<::option::CopyNoDefault>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::option::CopyNoDefault, Opt>)
inline rs_std::Option<::option::CopyNoDefault>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::option::CopyNoDefault, Opt>)
inline rs_std::Option<::option::CopyNoDefault>&
rs_std::Option<::option::CopyNoDefault>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<::option::CopyNoDefault>::Option(::std::in_place_t ip,
                                                       Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
inline rs_std::Option<::option::HasDefault>::Option(Option&& other) : Option() {
  *this = ::std::move(other);
}
inline rs_std::Option<::option::HasDefault>&
rs_std::Option<::option::HasDefault>::operator=(Option&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline rs_std::Option<::option::HasDefault>::Option(::crubit::UnsafeRelocateTag,
                                                    Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline constexpr rs_std::Option<::option::HasDefault>::~Option() noexcept {
  this->reset();
}
inline constexpr ::std::uint64_t rs_std::Option<::option::HasDefault>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void rs_std::Option<::option::HasDefault>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<::option::HasDefault>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<::option::HasDefault>&
rs_std::Option<::option::HasDefault>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::option::HasDefault>, ::option::HasDefault, U>)
inline rs_std::Option<::option::HasDefault>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::option::HasDefault>, ::option::HasDefault, U>)
inline rs_std::Option<::option::HasDefault>&
rs_std::Option<::option::HasDefault>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::option::HasDefault, Opt>)
inline rs_std::Option<::option::HasDefault>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::option::HasDefault, Opt>)
inline rs_std::Option<::option::HasDefault>&
rs_std::Option<::option::HasDefault>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<::option::HasDefault>::Option(::std::in_place_t ip,
                                                    Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
inline rs_std::Option<::option::HasNoDefault>::Option(Option&& other)
    : Option() {
  *this = ::std::move(other);
}
inline rs_std::Option<::option::HasNoDefault>&
rs_std::Option<::option::HasNoDefault>::operator=(Option&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline rs_std::Option<::option::HasNoDefault>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline constexpr rs_std::Option<::option::HasNoDefault>::~Option() noexcept {
  this->reset();
}
inline constexpr ::std::uint64_t rs_std::Option<::option::HasNoDefault>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void rs_std::Option<::option::HasNoDefault>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<::option::HasNoDefault>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<::option::HasNoDefault>&
rs_std::Option<::option::HasNoDefault>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::option::HasNoDefault>, ::option::HasNoDefault, U>)
inline rs_std::Option<::option::HasNoDefault>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::option::HasNoDefault>, ::option::HasNoDefault, U>)
inline rs_std::Option<::option::HasNoDefault>&
rs_std::Option<::option::HasNoDefault>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::option::HasNoDefault, Opt>)
inline rs_std::Option<::option::HasNoDefault>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::option::HasNoDefault, Opt>)
inline rs_std::Option<::option::HasNoDefault>&
rs_std::Option<::option::HasNoDefault>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<::option::HasNoDefault>::Option(::std::in_place_t ip,
                                                      Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasOptions_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasOptions_x00000020_x0000003e
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Option<::option::HasOptions>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Option<::option::HasOptions>>);
inline rs_std::Option<::option::HasOptions>::Option(::crubit::UnsafeRelocateTag,
                                                    Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<rs_std::Option<::option::HasOptions>>);
inline constexpr ::std::uint8_t rs_std::Option<::option::HasOptions>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs_std::Option<::option::HasOptions>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<::option::HasOptions>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<::option::HasOptions>&
rs_std::Option<::option::HasOptions>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::option::HasOptions>, ::option::HasOptions, U>)
inline rs_std::Option<::option::HasOptions>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::option::HasOptions>, ::option::HasOptions, U>)
inline rs_std::Option<::option::HasOptions>&
rs_std::Option<::option::HasOptions>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::option::HasOptions, Opt>)
inline rs_std::Option<::option::HasOptions>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::option::HasOptions, Opt>)
inline rs_std::Option<::option::HasOptions>&
rs_std::Option<::option::HasOptions>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<::option::HasOptions>::Option(::std::in_place_t ip,
                                                    Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Option<::option::LessThan20U8>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Option<::option::LessThan20U8>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Option<::option::LessThan20U8>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Option<::option::LessThan20U8>>);
inline rs_std::Option<::option::LessThan20U8>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<rs_std::Option<::option::LessThan20U8>>);
inline constexpr ::std::uint8_t rs_std::Option<::option::LessThan20U8>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs_std::Option<::option::LessThan20U8>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<::option::LessThan20U8>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<::option::LessThan20U8>&
rs_std::Option<::option::LessThan20U8>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::option::LessThan20U8>, ::option::LessThan20U8, U>)
inline rs_std::Option<::option::LessThan20U8>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::option::LessThan20U8>, ::option::LessThan20U8, U>)
inline rs_std::Option<::option::LessThan20U8>&
rs_std::Option<::option::LessThan20U8>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::option::LessThan20U8, Opt>)
inline rs_std::Option<::option::LessThan20U8>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::option::LessThan20U8, Opt>)
inline rs_std::Option<::option::LessThan20U8>&
rs_std::Option<::option::LessThan20U8>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<::option::LessThan20U8>::Option(::std::in_place_t ip,
                                                      Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Option<rs_std::Option<::option::LessThan20U8>>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Option<rs_std::Option<::option::LessThan20U8>>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Option<rs_std::Option<::option::LessThan20U8>>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Option<rs_std::Option<::option::LessThan20U8>>>);
inline rs_std::Option<rs_std::Option<::option::LessThan20U8>>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Option<rs_std::Option<::option::LessThan20U8>>>);
inline constexpr ::std::uint8_t
rs_std::Option<rs_std::Option<::option::LessThan20U8>>::tag() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void
rs_std::Option<rs_std::Option<::option::LessThan20U8>>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<rs_std::Option<::option::LessThan20U8>>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<rs_std::Option<::option::LessThan20U8>>&
rs_std::Option<rs_std::Option<::option::LessThan20U8>>::operator=(
    ::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<rs_std::Option<::option::LessThan20U8>>,
           rs_std::Option<::option::LessThan20U8>, U>)
inline rs_std::Option<rs_std::Option<::option::LessThan20U8>>::Option(
    U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<rs_std::Option<::option::LessThan20U8>>,
           rs_std::Option<::option::LessThan20U8>, U>)
inline rs_std::Option<rs_std::Option<::option::LessThan20U8>>&
rs_std::Option<rs_std::Option<::option::LessThan20U8>>::operator=(
    U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<rs_std::Option<::option::LessThan20U8>,
                                         Opt>)
inline rs_std::Option<rs_std::Option<::option::LessThan20U8>>::Option(
    Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<rs_std::Option<::option::LessThan20U8>,
                                         Opt>)
inline rs_std::Option<rs_std::Option<::option::LessThan20U8>>&
rs_std::Option<rs_std::Option<::option::LessThan20U8>>::operator=(
    Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<rs_std::Option<::option::LessThan20U8>>::Option(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020int_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020int_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e(
    rs_std::Option<
        rs_std::Result<::std::int32_t, ::rs::alloc::string::String>> const&,
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>*
        __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e(
    rs_std::Option<
        rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>&,
    rs_std::Option<
        rs_std::Result<::std::int32_t, ::rs::alloc::string::String>> const&);
}
inline rs_std::Option<rs_std::Result<
    ::std::int32_t, ::rs::alloc::string::String>>::Option(const Option& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e(
          other, this);
}
inline rs_std::Option<
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>&
rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>::
operator=(const Option& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e(
            *this, other);
  }
  return *this;
}
inline rs_std::Option<rs_std::Result<
    ::std::int32_t, ::rs::alloc::string::String>>::Option(Option&& other)
    : Option() {
  *this = ::std::move(other);
}
inline rs_std::Option<
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>&
rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>::
operator=(Option&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline rs_std::
    Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>::Option(
        ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline constexpr rs_std::Option<rs_std::Result<
    ::std::int32_t, ::rs::alloc::string::String>>::~Option() noexcept {
  this->reset();
}
inline constexpr ::std::uint64_t rs_std::Option<
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void
rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>::
    set_tag(::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::
    Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>::Option(
        ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>&
rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>::
operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<
               rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs_std::Result<::std::int32_t, ::rs::alloc::string::String>, U>)
inline rs_std::Option<rs_std::Result<
    ::std::int32_t, ::rs::alloc::string::String>>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<
               rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs_std::Result<::std::int32_t, ::rs::alloc::string::String>, U>)
inline rs_std::Option<
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>&
rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>::
operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<
           rs_std::Result<::std::int32_t, ::rs::alloc::string::String>, Opt>)
inline rs_std::Option<rs_std::Result<
    ::std::int32_t, ::rs::alloc::string::String>>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<
           rs_std::Result<::std::int32_t, ::rs::alloc::string::String>, Opt>)
inline rs_std::Option<
    rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>&
rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>::
operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::
    Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>::Option(
        ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
inline rs_std::Option<rs_std::Result<
    ::option::HasNoDefault, ::rs::alloc::string::String>>::Option(Option&&
                                                                      other)
    : Option() {
  *this = ::std::move(other);
}
inline rs_std::Option<
    rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>&
rs_std::Option<
    rs_std::Result<::option::HasNoDefault,
                   ::rs::alloc::string::String>>::operator=(Option&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline rs_std::Option<
    rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
    Option(::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline constexpr rs_std::Option<rs_std::Result<
    ::option::HasNoDefault, ::rs::alloc::string::String>>::~Option() noexcept {
  this->reset();
}
inline constexpr ::std::uint64_t rs_std::Option<
    rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void rs_std::Option<
    rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
    set_tag(::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<
    rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
    Option(::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<
    rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>&
rs_std::Option<
    rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<rs_std::Result<::option::HasNoDefault,
                                         ::rs::alloc::string::String>>,
           rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>,
           U>)
inline rs_std::Option<
    rs_std::Result<::option::HasNoDefault,
                   ::rs::alloc::string::String>>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<rs_std::Result<::option::HasNoDefault,
                                         ::rs::alloc::string::String>>,
           rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>,
           U>)
inline rs_std::Option<
    rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>&
rs_std::Option<
    rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<
           rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>,
           Opt>)
inline rs_std::Option<
    rs_std::Result<::option::HasNoDefault,
                   ::rs::alloc::string::String>>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<
           rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>,
           Opt>)
inline rs_std::Option<
    rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>&
rs_std::Option<
    rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<
    rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
    Option(::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020int_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020int_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e_x0000003e(
    rs_std::Option<
        rs_std::Result<rs_std::Option<rs_std::Result<
                           ::std::int32_t, ::rs::alloc::string::String>>,
                       rs_std::Result<rs_std::Option<::std::int32_t>,
                                      rs_std::Option<::std::int32_t>>>> const&,
    rs_std::Option<rs_std::Result<
        rs_std::Option<
            rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
        rs_std::Result<rs_std::Option<::std::int32_t>,
                       rs_std::Option<::std::int32_t>>>>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e_x0000003e(
    rs_std::Option<
        rs_std::Result<rs_std::Option<rs_std::Result<
                           ::std::int32_t, ::rs::alloc::string::String>>,
                       rs_std::Result<rs_std::Option<::std::int32_t>,
                                      rs_std::Option<::std::int32_t>>>>&,
    rs_std::Option<
        rs_std::Result<rs_std::Option<rs_std::Result<
                           ::std::int32_t, ::rs::alloc::string::String>>,
                       rs_std::Result<rs_std::Option<::std::int32_t>,
                                      rs_std::Option<::std::int32_t>>>> const&);
}
inline rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>::Option(const Option&
                                                                 other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e_x0000003e(
          other, this);
}
inline rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>&
rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>::operator=(const Option&
                                                                    other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e_x0000003e(
            *this, other);
  }
  return *this;
}
inline rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>::Option(Option&& other)
    : Option() {
  *this = ::std::move(other);
}
inline rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>&
rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>::operator=(Option&&
                                                                    other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>::
    Option(::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline constexpr rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>::~Option() noexcept {
  this->reset();
}
inline constexpr ::std::uint64_t rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>::tag() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>::
    set_tag(::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>::
    Option(::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>&
rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>::
operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<
               rs_std::Result<rs_std::Option<rs_std::Result<
                                  ::std::int32_t, ::rs::alloc::string::String>>,
                              rs_std::Result<rs_std::Option<::std::int32_t>,
                                             rs_std::Option<::std::int32_t>>>>,
           rs_std::Result<rs_std::Option<rs_std::Result<
                              ::std::int32_t, ::rs::alloc::string::String>>,
                          rs_std::Result<rs_std::Option<::std::int32_t>,
                                         rs_std::Option<::std::int32_t>>>,
           U>)
inline rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<
               rs_std::Result<rs_std::Option<rs_std::Result<
                                  ::std::int32_t, ::rs::alloc::string::String>>,
                              rs_std::Result<rs_std::Option<::std::int32_t>,
                                             rs_std::Option<::std::int32_t>>>>,
           rs_std::Result<rs_std::Option<rs_std::Result<
                              ::std::int32_t, ::rs::alloc::string::String>>,
                          rs_std::Result<rs_std::Option<::std::int32_t>,
                                         rs_std::Option<::std::int32_t>>>,
           U>)
inline rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>&
rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>::
operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<
           rs_std::Result<rs_std::Option<rs_std::Result<
                              ::std::int32_t, ::rs::alloc::string::String>>,
                          rs_std::Result<rs_std::Option<::std::int32_t>,
                                         rs_std::Option<::std::int32_t>>>,
           Opt>)
inline rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>::Option(Opt&&
                                                                 value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<
           rs_std::Result<rs_std::Option<rs_std::Result<
                              ::std::int32_t, ::rs::alloc::string::String>>,
                          rs_std::Result<rs_std::Option<::std::int32_t>,
                                         rs_std::Option<::std::int32_t>>>,
           Opt>)
inline rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>&
rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>::
operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>>::
    Option(::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<rs_std::Option<::std::uint32_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<rs_std::Option<::std::uint32_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<rs_std::Option<::std::uint32_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<rs_std::Option<::std::uint32_t>>);
inline rs_std::Option<::std::uint32_t>::Option(::crubit::UnsafeRelocateTag,
                                               Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<rs_std::Option<::std::uint32_t>>);
inline constexpr ::std::uint32_t rs_std::Option<::std::uint32_t>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void rs_std::Option<::std::uint32_t>::set_tag(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<::std::uint32_t>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<::std::uint32_t>&
rs_std::Option<::std::uint32_t>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<rs_std::Option<::std::uint32_t>,
                                              ::std::uint32_t, U>)
inline rs_std::Option<::std::uint32_t>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<rs_std::Option<::std::uint32_t>,
                                              ::std::uint32_t, U>)
inline rs_std::Option<::std::uint32_t>&
rs_std::Option<::std::uint32_t>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::std::uint32_t, Opt>)
inline rs_std::Option<::std::uint32_t>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::std::uint32_t, Opt>)
inline rs_std::Option<::std::uint32_t>&
rs_std::Option<::std::uint32_t>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<::std::uint32_t>::Option(::std::in_place_t ip,
                                               Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020usigned_x00000020char_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020usigned_x00000020char_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<rs_std::Option<::std::uint8_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<rs_std::Option<::std::uint8_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<rs_std::Option<::std::uint8_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<rs_std::Option<::std::uint8_t>>);
inline rs_std::Option<::std::uint8_t>::Option(::crubit::UnsafeRelocateTag,
                                              Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<rs_std::Option<::std::uint8_t>>);
inline constexpr ::std::uint8_t rs_std::Option<::std::uint8_t>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs_std::Option<::std::uint8_t>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<::std::uint8_t>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<::std::uint8_t>&
rs_std::Option<::std::uint8_t>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<rs_std::Option<::std::uint8_t>,
                                              ::std::uint8_t, U>)
inline rs_std::Option<::std::uint8_t>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<rs_std::Option<::std::uint8_t>,
                                              ::std::uint8_t, U>)
inline rs_std::Option<::std::uint8_t>&
rs_std::Option<::std::uint8_t>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::std::uint8_t, Opt>)
inline rs_std::Option<::std::uint8_t>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::std::uint8_t, Opt>)
inline rs_std::Option<::std::uint8_t>&
rs_std::Option<::std::uint8_t>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<::std::uint8_t>::Option(::std::in_place_t ip,
                                              Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>>);
inline rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>>);
inline constexpr ::std::uint64_t rs_std::Option<
    crubit::type_identity_t<void(void*, void*)>*>::tag() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void
rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>::
    Option(::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>&
rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>::operator=(
    ::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>,
           crubit::type_identity_t<void(void*, void*)>*, U>)
inline rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>::Option(
    U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>,
           crubit::type_identity_t<void(void*, void*)>*, U>)
inline rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>&
rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>::operator=(
    U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<
           crubit::type_identity_t<void(void*, void*)>*, Opt>)
inline rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>::Option(
    Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<
           crubit::type_identity_t<void(void*, void*)>*, Opt>)
inline rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>&
rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>::operator=(
    Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<crubit::type_identity_t<void(void*, void*)>*>::Option(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<rs_std::Option<::std::uint64_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<rs_std::Option<::std::uint64_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<rs_std::Option<::std::uint64_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<rs_std::Option<::std::uint64_t>>);
inline rs_std::Option<::std::uint64_t>::Option(::crubit::UnsafeRelocateTag,
                                               Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<rs_std::Option<::std::uint64_t>>);
inline constexpr ::std::uint64_t rs_std::Option<::std::uint64_t>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void rs_std::Option<::std::uint64_t>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<::std::uint64_t>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<::std::uint64_t>&
rs_std::Option<::std::uint64_t>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<rs_std::Option<::std::uint64_t>,
                                              ::std::uint64_t, U>)
inline rs_std::Option<::std::uint64_t>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<rs_std::Option<::std::uint64_t>,
                                              ::std::uint64_t, U>)
inline rs_std::Option<::std::uint64_t>&
rs_std::Option<::std::uint64_t>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::std::uint64_t, Opt>)
inline rs_std::Option<::std::uint64_t>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::std::uint64_t, Opt>)
inline rs_std::Option<::std::uint64_t>&
rs_std::Option<::std::uint64_t>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<::std::uint64_t>::Option(::std::in_place_t ip,
                                               Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020int_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020int_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
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

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
inline rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>::
    Result(::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Result<::option::HasNoDefault,
                      ::rs::alloc::string::String>::~Result() noexcept {
  this->Reset();
}
inline constexpr ::std::uint64_t
rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>::tag()
    const& noexcept {
  std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void
rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint64_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>,
           ::option::HasNoDefault, U>)
inline constexpr rs_std::Result<
    ::option::HasNoDefault,
    ::rs::alloc::string::String>::Result(U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>,
           ::option::HasNoDefault, U>)
inline constexpr rs_std::Result<::option::HasNoDefault,
                                ::rs::alloc::string::String>&
rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>::operator=(
    U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(
      rs_std::ResultUnexpectedConstructible<::rs::alloc::string::String, F>)
inline constexpr rs_std::Result<
    ::option::HasNoDefault,
    ::rs::alloc::string::String>::Result(rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(
      rs_std::ResultUnexpectedConstructible<::rs::alloc::string::String, F>)
inline constexpr rs_std::Result<::option::HasNoDefault,
                                ::rs::alloc::string::String>&
rs_std::Result<::option::HasNoDefault, ::rs::alloc::string::String>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::
    Result<::option::HasNoDefault, ::rs::alloc::string::String>::Result(
        ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::
    Result<::option::HasNoDefault, ::rs::alloc::string::String>::Result(
        rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<rs_std::Result<
        rs_std::Option<::std::int32_t>, rs_std::Option<::std::int32_t>>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<rs_std::Result<
        rs_std::Option<::std::int32_t>, rs_std::Option<::std::int32_t>>>);
static_assert(
    ::std::is_trivially_move_constructible_v<rs_std::Result<
        rs_std::Option<::std::int32_t>, rs_std::Option<::std::int32_t>>>);
static_assert(
    ::std::is_trivially_move_assignable_v<rs_std::Result<
        rs_std::Option<::std::int32_t>, rs_std::Option<::std::int32_t>>>);
inline rs_std::Result<
    rs_std::Option<::std::int32_t>,
    rs_std::Option<::std::int32_t>>::Result(::crubit::UnsafeRelocateTag,
                                            Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<rs_std::Result<
        rs_std::Option<::std::int32_t>, rs_std::Option<::std::int32_t>>>);
inline constexpr ::std::uint32_t
rs_std::Result<rs_std::Option<::std::int32_t>,
               rs_std::Option<::std::int32_t>>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void rs_std::Result<
    rs_std::Option<::std::int32_t>,
    rs_std::Option<::std::int32_t>>::set_tag(::std::uint32_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint32_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<rs_std::Option<::std::int32_t>,
                          rs_std::Option<::std::int32_t>>,
           rs_std::Option<::std::int32_t>, U>)
inline constexpr rs_std::Result<
    rs_std::Option<::std::int32_t>,
    rs_std::Option<::std::int32_t>>::Result(U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<rs_std::Option<::std::int32_t>,
                          rs_std::Option<::std::int32_t>>,
           rs_std::Option<::std::int32_t>, U>)
inline constexpr rs_std::Result<rs_std::Option<::std::int32_t>,
                                rs_std::Option<::std::int32_t>>&
rs_std::Result<rs_std::Option<::std::int32_t>,
               rs_std::Option<::std::int32_t>>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(
      rs_std::ResultUnexpectedConstructible<rs_std::Option<::std::int32_t>, F>)
inline constexpr rs_std::Result<rs_std::Option<::std::int32_t>,
                                rs_std::Option<::std::int32_t>>::
    Result(rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(
      rs_std::ResultUnexpectedConstructible<rs_std::Option<::std::int32_t>, F>)
inline constexpr rs_std::Result<rs_std::Option<::std::int32_t>,
                                rs_std::Option<::std::int32_t>>&
rs_std::Result<rs_std::Option<::std::int32_t>, rs_std::Option<::std::int32_t>>::
operator=(rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<
    rs_std::Option<::std::int32_t>,
    rs_std::Option<::std::int32_t>>::Result(::std::in_place_t ip,
                                            Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<
    rs_std::Option<::std::int32_t>,
    rs_std::Option<::std::int32_t>>::Result(rs_std::unexpect_t u,
                                            Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020int_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020int_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020int_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e(
    rs_std::Result<rs_std::Option<rs_std::Result<::std::int32_t,
                                                 ::rs::alloc::string::String>>,
                   rs_std::Result<rs_std::Option<::std::int32_t>,
                                  rs_std::Option<::std::int32_t>>> const&,
    rs_std::Result<rs_std::Option<rs_std::Result<::std::int32_t,
                                                 ::rs::alloc::string::String>>,
                   rs_std::Result<rs_std::Option<::std::int32_t>,
                                  rs_std::Option<::std::int32_t>>>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e(
    rs_std::Result<rs_std::Option<rs_std::Result<::std::int32_t,
                                                 ::rs::alloc::string::String>>,
                   rs_std::Result<rs_std::Option<::std::int32_t>,
                                  rs_std::Option<::std::int32_t>>>&,
    rs_std::Result<rs_std::Option<rs_std::Result<::std::int32_t,
                                                 ::rs::alloc::string::String>>,
                   rs_std::Result<rs_std::Option<::std::int32_t>,
                                  rs_std::Option<::std::int32_t>>> const&);
}
inline rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>::Result(const Result&
                                                                other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e(
          other, this);
}
inline rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>&
rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>::operator=(const Result&
                                                                   other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e(
            *this, other);
  }
  return *this;
}
inline rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>::
    Result(::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>::~Result() noexcept {
  this->Reset();
}
inline constexpr ::std::uint64_t rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>::set_tag(::std::uint64_t
                                                                 tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint64_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<rs_std::Option<rs_std::Result<
                              ::std::int32_t, ::rs::alloc::string::String>>,
                          rs_std::Result<rs_std::Option<::std::int32_t>,
                                         rs_std::Option<::std::int32_t>>>,
           rs_std::Option<
               rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
           U>)
inline constexpr rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>::Result(U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<rs_std::Option<rs_std::Result<
                              ::std::int32_t, ::rs::alloc::string::String>>,
                          rs_std::Result<rs_std::Option<::std::int32_t>,
                                         rs_std::Option<::std::int32_t>>>,
           rs_std::Option<
               rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
           U>)
inline constexpr rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>&
rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>::
operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<
           rs_std::Result<rs_std::Option<::std::int32_t>,
                          rs_std::Option<::std::int32_t>>,
           F>)
inline constexpr rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>::
    Result(rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<
           rs_std::Result<rs_std::Option<::std::int32_t>,
                          rs_std::Option<::std::int32_t>>,
           F>)
inline constexpr rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>&
rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>::
operator=(rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>::
    Result(::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<
    rs_std::Option<rs_std::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs_std::Result<rs_std::Option<::std::int32_t>,
                   rs_std::Option<::std::int32_t>>>::
    Result(rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_OPTION_GOLDEN
