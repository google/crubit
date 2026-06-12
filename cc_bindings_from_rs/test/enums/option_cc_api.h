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
#include "support/internal/check.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/memswap.h"
#include "support/internal/move_assign.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/option.h"
#include "support/rs_std/result.h"
#include "support/rs_std/str_ref.h"

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
// cc_bindings_from_rs/test/enums/option.rs;l=228:
// Type bindings for option_golden::BridgedType suppressed due to being mapped
// to an existing C++ type (int)

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: CloneNoDefault") alignas(
    1) [[clang::trivial_abi]] CloneNoDefault final {
 public:
  // `option_golden::CloneNoDefault` doesn't implement the `Default` trait
  CloneNoDefault() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~CloneNoDefault() = default;
  CloneNoDefault(CloneNoDefault&&) = default;
  CloneNoDefault& operator=(CloneNoDefault&&) = default;

  // Clone::clone
  CloneNoDefault(const CloneNoDefault&);

  // Clone::clone_from
  ::option::CloneNoDefault& operator=(const CloneNoDefault&);

  CloneNoDefault(::crubit::UnsafeRelocateTag, CloneNoDefault&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::uint8_t val;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: CopyNoDefault") alignas(1)
    [[clang::trivial_abi]] CopyNoDefault final {
 public:
  // `option_golden::CopyNoDefault` doesn't implement the `Default` trait
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
  union {
    ::std::uint8_t val;
  };

 private:
  static void __crubit_field_offset_assertions();
};

using FreeFunc CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: FreeFunc") =
    crubit::type_identity_t<void(void*, void*)>*;
// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: HasDefault") alignas(8)
    [[clang::trivial_abi]] HasDefault final {
 public:
  // Default::default
  HasDefault();

  // Drop::drop
  ~HasDefault();

  HasDefault(HasDefault&&);
  ::option::HasDefault& operator=(HasDefault&&);

  // `option_golden::HasDefault` doesn't implement the `Clone` trait
  HasDefault(const HasDefault&) = delete;
  HasDefault& operator=(const HasDefault&) = delete;
  HasDefault(::crubit::UnsafeRelocateTag, HasDefault&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::option::HasDefault new_(rs::StrRef s);

  // CRUBIT_ANNOTATE: must_bind=
  rs::StrRef get_string_inside_option() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  union {
    ::rs::alloc::string::String foo;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: HasNoDefault") alignas(8)
    [[clang::trivial_abi]] HasNoDefault final {
 public:
  // `option_golden::HasNoDefault` doesn't implement the `Default` trait
  HasNoDefault() = delete;

  // Drop::drop
  ~HasNoDefault();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  HasNoDefault(HasNoDefault&&) = delete;
  ::option::HasNoDefault& operator=(HasNoDefault&&) = delete;
  // `option_golden::HasNoDefault` doesn't implement the `Clone` trait
  HasNoDefault(const HasNoDefault&) = delete;
  HasNoDefault& operator=(const HasNoDefault&) = delete;
  HasNoDefault(::crubit::UnsafeRelocateTag, HasNoDefault&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::option::HasNoDefault new_(rs::StrRef s);

  // CRUBIT_ANNOTATE: must_bind=
  rs::StrRef get_string_inside_option() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  union {
    ::rs::alloc::string::String foo;
  };
  union {
    ::std::uint32_t a;
  };

 private:
  unsigned char __padding1[4];

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
  LessThan20U8(::crubit::UnsafeRelocateTag, LessThan20U8&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static rs::Option<::option::LessThan20U8> new_(::std::uint8_t value);

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
  // Default::default
  OptZst();

  // No custom `Drop` impl and no custom "drop glue" required
  ~OptZst() = default;
  OptZst(OptZst&&) = default;
  OptZst& operator=(OptZst&&) = default;

  // `option_golden::OptZst` doesn't implement the `Clone` trait
  OptZst(const OptZst&) = delete;
  OptZst& operator=(const OptZst&) = delete;
  OptZst(::crubit::UnsafeRelocateTag, OptZst&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

 private:
  // Field type has been replaced with a blob of bytes: Failed to format type
  // for the definition of `option_golden::Unit`: Zero-sized types (ZSTs) are
  // not supported (b/258259459)
  ::std::array<unsigned char, 1> val;

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: option_golden :: OptionWithSizeTypes") alignas(8) [[clang::trivial_abi]]
OptionWithSizeTypes final {
 public:
  // `option_golden::OptionWithSizeTypes` doesn't implement the `Default` trait
  OptionWithSizeTypes() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~OptionWithSizeTypes() = default;
  OptionWithSizeTypes(OptionWithSizeTypes&&) = default;
  OptionWithSizeTypes& operator=(OptionWithSizeTypes&&) = default;

  // `option_golden::OptionWithSizeTypes` doesn't implement the `Clone` trait
  OptionWithSizeTypes(const OptionWithSizeTypes&) = delete;
  OptionWithSizeTypes& operator=(const OptionWithSizeTypes&) = delete;
  OptionWithSizeTypes(::crubit::UnsafeRelocateTag,
                      OptionWithSizeTypes&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

 private:
  // Field type has been replaced with a blob of bytes: b/491106325 - isize and
  // usize types are not yet supported as generic type arguments.
  ::std::array<unsigned char, 16> uval;
  // Field type has been replaced with a blob of bytes: b/491106325 - isize and
  // usize types are not yet supported as generic type arguments.
  ::std::array<unsigned char, 16> ival;

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
rs::Option<::std::int32_t const*> pass_option_ptr(
    rs::Option<::std::int32_t const*> x);

// CRUBIT_ANNOTATE: must_bind=
rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>
return_option_result();

rs::Option<rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>
return_option_result_unmovable();

// CRUBIT_ANNOTATE: must_bind=
rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>
stress_testing_nested_types();

// CRUBIT_ANNOTATE: must_bind=
rs::Option<::std::uint32_t> stringify_len(
    rs::Option<::option::HasDefault> const& x);

// Error generating bindings for function `option_golden::take_option_bridged`
// defined at
// cc_bindings_from_rs/test/enums/option.rs;l=230:
// Error handling parameter #0 of type
// `std::option::Option<option_golden::BridgedType>`: Generic types are not
// supported yet (b/259749095)

void take_option_result_unmovable(
    rs::Option<rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>
        _x);

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020_x0000003e
template <>
struct alignas(8)
    CRUBIT_INTERNAL_RUST_TYPE("std :: option :: Option < * const i32 >")
        rs::Option<::std::int32_t const*> {
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

  explicit Option(::std::optional<::std::int32_t const*>&& value) noexcept;
  Option& operator=(::std::optional<::std::int32_t const*>&& value) noexcept;

  template <typename... Args>
  Option(::std::in_place_t, Args&&... args) noexcept;
  ~Option() noexcept = default;
  operator ::std::optional<::std::int32_t const*>() && noexcept;
  bool has_value() const noexcept;
  ::std::int32_t const*& operator*() &;
  ::std::int32_t const* const& operator*() const&;
  ::std::int32_t const*&& operator*() &&;
  ::std::int32_t const** operator->();
  ::std::int32_t const* const* operator->() const;

 private:
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;
  void check_has_value() const;

 private:
  unsigned char storage_[16];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < i32 >") rs::Option<::std::int32_t> {
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
  bool has_value() const noexcept;
  ::std::int32_t& operator*() &;
  ::std::int32_t const& operator*() const&;
  ::std::int32_t&& operator*() &&;
  ::std::int32_t* operator->();
  ::std::int32_t const* operator->() const;

 private:
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;
  void check_has_value() const;

 private:
  unsigned char storage_[8];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: option_golden :: CloneNoDefault >")
    rs::Option<::option::CloneNoDefault> {
 public:
  // Clone::clone
  Option(const Option&);

  // Clone::clone_from
  rs::Option<::option::CloneNoDefault>& operator=(const Option&);

  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  constexpr Option();

  constexpr explicit Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;

  Option(::option::CloneNoDefault&& value) noexcept;
  Option& operator=(::option::CloneNoDefault&& value) noexcept;

  explicit Option(::std::optional<::option::CloneNoDefault>&& value) noexcept;
  Option& operator=(::std::optional<::option::CloneNoDefault>&& value) noexcept;

  template <typename... Args>
  Option(::std::in_place_t, Args&&... args) noexcept;
  ~Option() noexcept = default;
  operator ::std::optional<::option::CloneNoDefault>() && noexcept;
  bool has_value() const noexcept;
  ::option::CloneNoDefault& operator*() &;
  ::option::CloneNoDefault const& operator*() const&;
  ::option::CloneNoDefault&& operator*() &&;
  ::option::CloneNoDefault* operator->();
  ::option::CloneNoDefault const* operator->() const;

 private:
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;
  void check_has_value() const;

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
  // `option_golden::OptCloneNoDefault` doesn't implement the `Default` trait
  OptCloneNoDefault() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~OptCloneNoDefault() = default;
  OptCloneNoDefault(OptCloneNoDefault&&) = default;
  OptCloneNoDefault& operator=(OptCloneNoDefault&&) = default;

  // Clone::clone
  OptCloneNoDefault(const OptCloneNoDefault&);

  // Clone::clone_from
  ::option::OptCloneNoDefault& operator=(const OptCloneNoDefault&);

  OptCloneNoDefault(::crubit::UnsafeRelocateTag, OptCloneNoDefault&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::option::OptCloneNoDefault new_(::std::uint8_t x);

  union {
    rs::Option<::option::CloneNoDefault> val;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: option_golden :: CopyNoDefault >")
    rs::Option<::option::CopyNoDefault> {
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

  Option(::option::CopyNoDefault&& value) noexcept;
  Option& operator=(::option::CopyNoDefault&& value) noexcept;

  explicit Option(::std::optional<::option::CopyNoDefault>&& value) noexcept;
  Option& operator=(::std::optional<::option::CopyNoDefault>&& value) noexcept;

  template <typename... Args>
  Option(::std::in_place_t, Args&&... args) noexcept;
  ~Option() noexcept = default;
  operator ::std::optional<::option::CopyNoDefault>() && noexcept;
  bool has_value() const noexcept;
  ::option::CopyNoDefault& operator*() &;
  ::option::CopyNoDefault const& operator*() const&;
  ::option::CopyNoDefault&& operator*() &&;
  ::option::CopyNoDefault* operator->();
  ::option::CopyNoDefault const* operator->() const;

 private:
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;
  void check_has_value() const;

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
  // `option_golden::OptCopyNoDefault` doesn't implement the `Default` trait
  OptCopyNoDefault() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~OptCopyNoDefault() = default;
  OptCopyNoDefault(OptCopyNoDefault&&) = default;
  OptCopyNoDefault& operator=(OptCopyNoDefault&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  OptCopyNoDefault(const OptCopyNoDefault&) = default;
  OptCopyNoDefault& operator=(const OptCopyNoDefault&) = default;
  OptCopyNoDefault(::crubit::UnsafeRelocateTag, OptCopyNoDefault&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::option::OptCopyNoDefault new_(::std::uint8_t x);

  union {
    rs::Option<::option::CopyNoDefault> val;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: option_golden :: HasDefault >")
    rs::Option<::option::HasDefault> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&);
  rs::Option<::option::HasDefault>& operator=(Option&&);
  Option(::crubit::UnsafeRelocateTag, Option&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  constexpr Option();

  constexpr explicit Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;

  Option(::option::HasDefault&& value) noexcept;
  Option& operator=(::option::HasDefault&& value) noexcept;

  explicit Option(::std::optional<::option::HasDefault>&& value) noexcept;
  Option& operator=(::std::optional<::option::HasDefault>&& value) noexcept;

  template <typename... Args>
  Option(::std::in_place_t, Args&&... args) noexcept;
  constexpr ~Option() noexcept;
  operator ::std::optional<::option::HasDefault>() && noexcept;
  bool has_value() const noexcept;
  ::option::HasDefault& operator*() &;
  ::option::HasDefault const& operator*() const&;
  ::option::HasDefault&& operator*() &&;
  ::option::HasDefault* operator->();
  ::option::HasDefault const* operator->() const;

 private:
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;
  void check_has_value() const;

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
  // `option_golden::OptDefaultWithDrop` doesn't implement the `Default` trait
  OptDefaultWithDrop() = delete;

  // Drop::drop
  ~OptDefaultWithDrop();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  OptDefaultWithDrop(OptDefaultWithDrop&&) = delete;
  ::option::OptDefaultWithDrop& operator=(OptDefaultWithDrop&&) = delete;
  // `option_golden::OptDefaultWithDrop` doesn't implement the `Clone` trait
  OptDefaultWithDrop(const OptDefaultWithDrop&) = delete;
  OptDefaultWithDrop& operator=(const OptDefaultWithDrop&) = delete;
  OptDefaultWithDrop(::crubit::UnsafeRelocateTag, OptDefaultWithDrop&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::option::OptDefaultWithDrop new_(rs::StrRef s);

  union {
    rs::Option<::option::HasDefault> opt;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: option_golden :: HasNoDefault >")
    rs::Option<::option::HasNoDefault> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&);
  rs::Option<::option::HasNoDefault>& operator=(Option&&);
  Option(::crubit::UnsafeRelocateTag, Option&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  constexpr Option();

  constexpr explicit Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;

  explicit Option(::std::optional<::option::HasNoDefault>&& value) noexcept;
  Option& operator=(::std::optional<::option::HasNoDefault>&& value) noexcept;

  template <typename... Args>
  Option(::std::in_place_t, Args&&... args) noexcept;
  constexpr ~Option() noexcept;
  operator ::std::optional<::option::HasNoDefault>() && noexcept;
  bool has_value() const noexcept;
  ::option::HasNoDefault& operator*() &;
  ::option::HasNoDefault const& operator*() const&;
  ::option::HasNoDefault&& operator*() &&;
  ::option::HasNoDefault* operator->();
  ::option::HasNoDefault const* operator->() const;

 private:
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;
  void check_has_value() const;

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
  // `option_golden::OptNoDefaultWithDrop` doesn't implement the `Default` trait
  OptNoDefaultWithDrop() = delete;

  // Drop::drop
  ~OptNoDefaultWithDrop();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  OptNoDefaultWithDrop(OptNoDefaultWithDrop&&) = delete;
  ::option::OptNoDefaultWithDrop& operator=(OptNoDefaultWithDrop&&) = delete;
  // `option_golden::OptNoDefaultWithDrop` doesn't implement the `Clone` trait
  OptNoDefaultWithDrop(const OptNoDefaultWithDrop&) = delete;
  OptNoDefaultWithDrop& operator=(const OptNoDefaultWithDrop&) = delete;
  OptNoDefaultWithDrop(::crubit::UnsafeRelocateTag,
                       OptNoDefaultWithDrop&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::option::OptNoDefaultWithDrop new_(rs::StrRef s);

  // CRUBIT_ANNOTATE: must_bind=
  rs::StrRef get_string_inside_option() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  union {
    rs::Option<::option::HasNoDefault> val;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasOptions_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasOptions_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: option_golden :: HasOptions >")
    rs::Option<::option::HasOptions> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  constexpr Option();

  constexpr explicit Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;

  Option(::option::HasOptions&& value) noexcept;
  Option& operator=(::option::HasOptions&& value) noexcept;

  explicit Option(::std::optional<::option::HasOptions>&& value) noexcept;
  Option& operator=(::std::optional<::option::HasOptions>&& value) noexcept;

  template <typename... Args>
  Option(::std::in_place_t, Args&&... args) noexcept;
  ~Option() noexcept = default;
  operator ::std::optional<::option::HasOptions>() && noexcept;
  bool has_value() const noexcept;
  ::option::HasOptions& operator*() &;
  ::option::HasOptions const& operator*() const&;
  ::option::HasOptions&& operator*() &&;
  ::option::HasOptions* operator->();
  ::option::HasOptions const* operator->() const;

 private:
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;
  void check_has_value() const;

 private:
  unsigned char storage_[4];
};
#endif

namespace option {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: HasHasOptions") alignas(1)
    [[clang::trivial_abi]] HasHasOptions final {
 public:
  // `option_golden::HasHasOptions` doesn't implement the `Default` trait
  HasHasOptions() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~HasHasOptions() = default;
  HasHasOptions(HasHasOptions&&) = default;
  HasHasOptions& operator=(HasHasOptions&&) = default;

  // `option_golden::HasHasOptions` doesn't implement the `Clone` trait
  HasHasOptions(const HasHasOptions&) = delete;
  HasHasOptions& operator=(const HasHasOptions&) = delete;
  HasHasOptions(::crubit::UnsafeRelocateTag, HasHasOptions&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::option::HasHasOptions new_(::std::uint8_t value);

  union {
    rs::Option<::option::HasOptions> me;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: option_golden :: LessThan20U8 >")
    rs::Option<::option::LessThan20U8> {
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

  Option(::option::LessThan20U8&& value) noexcept;
  Option& operator=(::option::LessThan20U8&& value) noexcept;

  explicit Option(::std::optional<::option::LessThan20U8>&& value) noexcept;
  Option& operator=(::std::optional<::option::LessThan20U8>&& value) noexcept;

  template <typename... Args>
  Option(::std::in_place_t, Args&&... args) noexcept;
  ~Option() noexcept = default;
  operator ::std::optional<::option::LessThan20U8>() && noexcept;
  bool has_value() const noexcept;
  ::option::LessThan20U8& operator*() &;
  ::option::LessThan20U8 const& operator*() const&;
  ::option::LessThan20U8&& operator*() &&;
  ::option::LessThan20U8* operator->();
  ::option::LessThan20U8 const* operator->() const;

 private:
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;
  void check_has_value() const;

 private:
  unsigned char storage_[1];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: core :: option :: Option < :: option_golden "
    ":: LessThan20U8 > >") rs::Option<rs::Option<::option::LessThan20U8>> {
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

  Option(rs::Option<::option::LessThan20U8>&& value) noexcept;
  Option& operator=(rs::Option<::option::LessThan20U8>&& value) noexcept;

  explicit Option(
      ::std::optional<rs::Option<::option::LessThan20U8>>&& value) noexcept;
  Option& operator=(
      ::std::optional<rs::Option<::option::LessThan20U8>>&& value) noexcept;

  template <typename... Args>
  Option(::std::in_place_t, Args&&... args) noexcept;
  ~Option() noexcept = default;
  operator ::std::optional<rs::Option<::option::LessThan20U8>>() && noexcept;
  bool has_value() const noexcept;
  rs::Option<::option::LessThan20U8>& operator*() &;
  rs::Option<::option::LessThan20U8> const& operator*() const&;
  rs::Option<::option::LessThan20U8>&& operator*() &&;
  rs::Option<::option::LessThan20U8>* operator->();
  rs::Option<::option::LessThan20U8> const* operator->() const;

 private:
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;
  void check_has_value() const;

 private:
  unsigned char storage_[1];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < u32 >") rs::Option<::std::uint32_t> {
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

  Option(::std::uint32_t&& value) noexcept;
  Option& operator=(::std::uint32_t&& value) noexcept;

  explicit Option(::std::optional<::std::uint32_t>&& value) noexcept;
  Option& operator=(::std::optional<::std::uint32_t>&& value) noexcept;

  template <typename... Args>
  Option(::std::in_place_t, Args&&... args) noexcept;
  ~Option() noexcept = default;
  operator ::std::optional<::std::uint32_t>() && noexcept;
  bool has_value() const noexcept;
  ::std::uint32_t& operator*() &;
  ::std::uint32_t const& operator*() const&;
  ::std::uint32_t&& operator*() &&;
  ::std::uint32_t* operator->();
  ::std::uint32_t const* operator->() const;

 private:
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;
  void check_has_value() const;

 private:
  unsigned char storage_[8];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < u8 >") rs::Option<::std::uint8_t> {
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

  Option(::std::uint8_t&& value) noexcept;
  Option& operator=(::std::uint8_t&& value) noexcept;

  explicit Option(::std::optional<::std::uint8_t>&& value) noexcept;
  Option& operator=(::std::optional<::std::uint8_t>&& value) noexcept;

  template <typename... Args>
  Option(::std::in_place_t, Args&&... args) noexcept;
  ~Option() noexcept = default;
  operator ::std::optional<::std::uint8_t>() && noexcept;
  bool has_value() const noexcept;
  ::std::uint8_t& operator*() &;
  ::std::uint8_t const& operator*() const&;
  ::std::uint8_t&& operator*() &&;
  ::std::uint8_t* operator->();
  ::std::uint8_t const* operator->() const;

 private:
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;
  void check_has_value() const;

 private:
  unsigned char storage_[2];
};
#endif

namespace option {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: HasOptions") alignas(1)
    [[clang::trivial_abi]] HasOptions final {
 public:
  // `option_golden::HasOptions` doesn't implement the `Default` trait
  HasOptions() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~HasOptions() = default;
  HasOptions(HasOptions&&) = default;
  HasOptions& operator=(HasOptions&&) = default;

  // `option_golden::HasOptions` doesn't implement the `Clone` trait
  HasOptions(const HasOptions&) = delete;
  HasOptions& operator=(const HasOptions&) = delete;
  HasOptions(::crubit::UnsafeRelocateTag, HasOptions&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::option::HasOptions new_(::std::uint8_t value);

  // CRUBIT_ANNOTATE: must_bind=
  static ::option::HasOptions with_option(rs::Option<::std::uint8_t> value);

  // CRUBIT_ANNOTATE: must_bind=
  static ::option::HasOptions from_ref(rs::Option<::std::uint8_t> const& value);

  // CRUBIT_ANNOTATE: must_bind=
  static ::option::HasOptions with_none();

  union {
    rs::Option<::std::uint8_t> direct;
  };
  union {
    rs::Option<::option::LessThan20U8> niche;
  };
  union {
    rs::Option<rs::Option<::option::LessThan20U8>> nested;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < unsafe extern \"C\" fn (* mut :: core :: ffi :: "
    "c_void , * mut :: core :: ffi :: c_void) >")
    rs::Option<crubit::type_identity_t<void(void*, void*)>*> {
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

  explicit Option(
      ::std::optional<crubit::type_identity_t<void(void*, void*)>*>&&
          value) noexcept;
  Option& operator=(
      ::std::optional<crubit::type_identity_t<void(void*, void*)>*>&&
          value) noexcept;

  template <typename... Args>
  Option(::std::in_place_t, Args&&... args) noexcept;
  ~Option() noexcept = default;
  operator ::std::optional<
      crubit::type_identity_t<void(void*, void*)>*>() && noexcept;
  bool has_value() const noexcept;
  crubit::type_identity_t<void(void*, void*)>*& operator*() &;
  crubit::type_identity_t<void(void*, void*)>* const& operator*() const&;
  crubit::type_identity_t<void(void*, void*)>*&& operator*() &&;
  crubit::type_identity_t<void(void*, void*)>** operator->();
  crubit::type_identity_t<void(void*, void*)>* const* operator->() const;

 private:
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;
  void check_has_value() const;

 private:
  unsigned char storage_[8];
};
#endif

namespace option {

struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: ZStream") alignas(8)
    [[clang::trivial_abi]] ZStream final {
 public:
  // `option_golden::ZStream` doesn't implement the `Default` trait
  ZStream() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~ZStream() = default;
  ZStream(ZStream&&) = default;
  ZStream& operator=(ZStream&&) = default;

  // `option_golden::ZStream` doesn't implement the `Clone` trait
  ZStream(const ZStream&) = delete;
  ZStream& operator=(const ZStream&) = delete;
  ZStream(::crubit::UnsafeRelocateTag, ZStream&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    rs::Option<crubit::type_identity_t<void(void*, void*)>*> zfree;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < i32 , :: alloc :: string :: String >")
    rs::Result<::std::int32_t, ::rs::alloc::string::String> {
 public:
  // Clone::clone
  Result(const Result&);

  // Clone::clone_from
  rs::Result<::std::int32_t, ::rs::alloc::string::String>& operator=(
      const Result&);

  Result(::crubit::UnsafeRelocateTag, Result&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Result(::std::int32_t&& ok) noexcept;
  Result& operator=(::std::int32_t&& ok) noexcept;
  Result(rs::unexpected<::rs::alloc::string::String>&& err) noexcept;
  Result& operator=(rs::unexpected<::rs::alloc::string::String>&& err) noexcept;
  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  ::std::int32_t& value() &;
  ::std::int32_t&& value() &&;
  ::rs::alloc::string::String& err() &;
  ::rs::alloc::string::String&& err() &&;
  ::std::int32_t& operator*() &;
  ::std::int32_t const& operator*() const&;
  ::std::int32_t&& operator*() &&;
  ::std::int32_t* operator->();
  ::std::int32_t const* operator->() const;
  ~Result() noexcept;

 private:
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;
  void check_has_ok() const;
  void check_has_err() const;

 private:
  unsigned char __storage[24];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: core :: result :: Result < i32 , :: alloc :: "
    "string :: String > >")
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>> {
 public:
  // Clone::clone
  Option(const Option&);

  // Clone::clone_from
  rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&
  operator=(const Option&);

  Option(Option&&);
  rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&
  operator=(Option&&);
  Option(::crubit::UnsafeRelocateTag, Option&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  constexpr Option();

  constexpr explicit Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;

  Option(
      rs::Result<::std::int32_t, ::rs::alloc::string::String>&& value) noexcept;
  Option& operator=(
      rs::Result<::std::int32_t, ::rs::alloc::string::String>&& value) noexcept;

  explicit Option(
      ::std::optional<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&&
          value) noexcept;
  Option& operator=(
      ::std::optional<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&&
          value) noexcept;

  template <typename... Args>
  Option(::std::in_place_t, Args&&... args) noexcept;
  constexpr ~Option() noexcept;
  operator ::std::optional<
      rs::Result<::std::int32_t, ::rs::alloc::string::String>>() && noexcept;
  bool has_value() const noexcept;
  rs::Result<::std::int32_t, ::rs::alloc::string::String>& operator*() &;
  rs::Result<::std::int32_t, ::rs::alloc::string::String> const& operator*()
      const&;
  rs::Result<::std::int32_t, ::rs::alloc::string::String>&& operator*() &&;
  rs::Result<::std::int32_t, ::rs::alloc::string::String>* operator->();
  rs::Result<::std::int32_t, ::rs::alloc::string::String> const* operator->()
      const;

 private:
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;
  void check_has_value() const;

 private:
  unsigned char storage_[24];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: option_golden :: HasNoDefault , :: alloc :: "
    "string :: String >")
    rs::Result<::option::HasNoDefault, ::rs::alloc::string::String> {
 public:
  // `core::result::Result` doesn't implement the `Clone` trait
  Result(const Result&) = delete;
  Result& operator=(const Result&) = delete;
  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  Result(Result&&) = delete;
  rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>& operator=(
      Result&&) = delete;
  Result(::crubit::UnsafeRelocateTag, Result&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }  // Move constructor is not available for the Ok variant because
     // option_golden::HasNoDefault does not have a move constructor

  Result(rs::unexpected<::rs::alloc::string::String>&& err) noexcept;
  Result& operator=(rs::unexpected<::rs::alloc::string::String>&& err) noexcept;
  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  ::option::HasNoDefault& value() &;
  ::option::HasNoDefault&& value() &&;
  ::rs::alloc::string::String& err() &;
  ::rs::alloc::string::String&& err() &&;
  ::option::HasNoDefault& operator*() &;
  ::option::HasNoDefault const& operator*() const&;
  ::option::HasNoDefault&& operator*() &&;
  ::option::HasNoDefault* operator->();
  ::option::HasNoDefault const* operator->() const;
  ~Result() noexcept;

 private:
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;
  void check_has_ok() const;
  void check_has_err() const;

 private:
  unsigned char __storage[32];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: core :: result :: Result < :: option_golden "
    ":: HasNoDefault , :: alloc :: string :: String > >") rs::
    Option<rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&);
  rs::Option<rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>&
  operator=(Option&&);
  Option(::crubit::UnsafeRelocateTag, Option&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  constexpr Option();

  constexpr explicit Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;

  explicit Option(
      ::std::optional<
          rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>&&
          value) noexcept;
  Option& operator=(
      ::std::optional<
          rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>&&
          value) noexcept;

  template <typename... Args>
  Option(::std::in_place_t, Args&&... args) noexcept;
  constexpr ~Option() noexcept;
  operator ::std::optional<rs::Result<
      ::option::HasNoDefault, ::rs::alloc::string::String>>() && noexcept;
  bool has_value() const noexcept;
  rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>&
  operator*() &;
  rs::Result<::option::HasNoDefault, ::rs::alloc::string::String> const&
  operator*() const&;
  rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>&&
  operator*() &&;
  rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>* operator->();
  rs::Result<::option::HasNoDefault, ::rs::alloc::string::String> const*
  operator->() const;

 private:
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;
  void check_has_value() const;

 private:
  unsigned char storage_[32];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: core :: option :: Option < i32 > , :: core "
    ":: option :: Option < i32 > >")
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Result(const Result&) = default;
  Result& operator=(const Result&) = default;
  Result(Result&&) = default;
  Result& operator=(Result&&) = default;

  Result(::crubit::UnsafeRelocateTag, Result&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Result(rs::Option<::std::int32_t>&& ok) noexcept;
  Result& operator=(rs::Option<::std::int32_t>&& ok) noexcept;
  Result(rs::unexpected<rs::Option<::std::int32_t>>&& err) noexcept;
  Result& operator=(rs::unexpected<rs::Option<::std::int32_t>>&& err) noexcept;
  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  rs::Option<::std::int32_t>& value() &;
  rs::Option<::std::int32_t>&& value() &&;
  rs::Option<::std::int32_t>& err() &;
  rs::Option<::std::int32_t>&& err() &&;
  rs::Option<::std::int32_t>& operator*() &;
  rs::Option<::std::int32_t> const& operator*() const&;
  rs::Option<::std::int32_t>&& operator*() &&;
  rs::Option<::std::int32_t>* operator->();
  rs::Option<::std::int32_t> const* operator->() const;
  ~Result() noexcept = default;

 private:
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;
  void check_has_ok() const;
  void check_has_err() const;

 private:
  unsigned char __storage[12];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: core :: option :: Option < :: core :: result "
    ":: Result < i32 , :: alloc :: string :: String > > , :: core :: result :: "
    "Result < :: core :: option :: Option < i32 > , :: core :: option :: "
    "Option < i32 > > >") rs::
    Result<rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>> {
 public:
  // Clone::clone
  Result(const Result&);

  // Clone::clone_from
  rs::Result<
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>&
  operator=(const Result&);

  Result(::crubit::UnsafeRelocateTag, Result&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Result(rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&&
             ok) noexcept;
  Result& operator=(
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&&
          ok) noexcept;
  Result(rs::unexpected<rs::Result<rs::Option<::std::int32_t>,
                                   rs::Option<::std::int32_t>>>&& err) noexcept;
  Result& operator=(
      rs::unexpected<rs::Result<rs::Option<::std::int32_t>,
                                rs::Option<::std::int32_t>>>&& err) noexcept;
  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&
  value() &;
  rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&&
  value() &&;
  rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>& err() &;
  rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>&& err() &&;
  rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&
  operator*() &;
  rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>> const&
  operator*() const&;
  rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&&
  operator*() &&;
  rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>*
  operator->();
  rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>> const*
  operator->() const;
  ~Result() noexcept;

 private:
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;
  void check_has_ok() const;
  void check_has_err() const;

 private:
  unsigned char __storage[24];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: core :: result :: Result < :: core :: option "
    ":: Option < :: core :: result :: Result < i32 , :: alloc :: string :: "
    "String > > , :: core :: result :: Result < :: core :: option :: Option < "
    "i32 > , :: core :: option :: Option < i32 > > > >")
    rs::Option<rs::Result<
        rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>> {
 public:
  // Clone::clone
  Option(const Option&);

  // Clone::clone_from
  rs::Option<rs::Result<
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>&
  operator=(const Option&);

  Option(Option&&);
  rs::Option<rs::Result<
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>&
  operator=(Option&&);
  Option(::crubit::UnsafeRelocateTag, Option&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  constexpr Option();

  constexpr explicit Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;

  Option(rs::Result<
         rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
         rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>&&
             value) noexcept;
  Option& operator=(
      rs::Result<
          rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
          rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>&&
          value) noexcept;

  explicit Option(
      ::std::optional<rs::Result<
          rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
          rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>&&
          value) noexcept;
  Option& operator=(
      ::std::optional<rs::Result<
          rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
          rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>&&
          value) noexcept;

  template <typename... Args>
  Option(::std::in_place_t, Args&&... args) noexcept;
  constexpr ~Option() noexcept;
  operator ::std::optional<rs::Result<
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      rs::Result<rs::Option<::std::int32_t>,
                 rs::Option<::std::int32_t>>>>() && noexcept;
  bool has_value() const noexcept;
  rs::Result<
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>&
  operator*() &;
  rs::Result<
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>> const&
  operator*() const&;
  rs::Result<
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>&&
  operator*() &&;
  rs::Result<
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>*
  operator->();
  rs::Result<
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>> const*
  operator->() const;

 private:
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;
  void check_has_value() const;

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
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(::option::CloneNoDefault const&,
                                     ::option::CloneNoDefault* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(::option::CloneNoDefault&,
                                           ::option::CloneNoDefault const&);
}
inline ::option::CloneNoDefault::CloneNoDefault(const CloneNoDefault& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline ::option::CloneNoDefault& ::option::CloneNoDefault::operator=(
    const CloneNoDefault& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
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
extern "C" void __crubit_thunk_default(::option::HasDefault* __ret_ptr);
}
inline ::option::HasDefault::HasDefault() {
  __crubit_internal::__crubit_thunk_default(this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::option::HasDefault&);
}
inline HasDefault::~HasDefault() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
inline ::option::HasDefault::HasDefault(HasDefault&& other) : HasDefault() {
  *this = ::std::move(other);
}
inline ::option::HasDefault& ::option::HasDefault::operator=(
    HasDefault&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(rs::StrRef, ::option::HasDefault* __ret_ptr);
}
inline ::option::HasDefault HasDefault::new_(rs::StrRef s) {
  crubit::Slot<::option::HasDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(s, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" rs::StrRef __crubit_thunk_get_ustring_uinside_uoption(
    ::option::HasDefault const&);
}
inline rs::StrRef HasDefault::get_string_inside_option() const& $(__anon1)
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
extern "C" void __crubit_thunk_drop(::option::HasNoDefault&);
}
inline HasNoDefault::~HasNoDefault() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(rs::StrRef,
                                   ::option::HasNoDefault* __ret_ptr);
}
inline ::option::HasNoDefault HasNoDefault::new_(rs::StrRef s) {
  crubit::Slot<::option::HasNoDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(s, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" rs::StrRef __crubit_thunk_get_ustring_uinside_uoption(
    ::option::HasNoDefault const&);
}
inline rs::StrRef HasNoDefault::get_string_inside_option() const& $(__anon1)
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
extern "C" void __crubit_thunk_with_uoption(rs::Option<::std::uint8_t>*,
                                            ::option::HasOptions* __ret_ptr);
}
inline ::option::HasOptions HasOptions::with_option(
    rs::Option<::std::uint8_t> value) {
  crubit::Slot<::option::HasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_with_uoption(&value,
                                                 __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_from_uref(rs::Option<::std::uint8_t> const&,
                                         ::option::HasOptions* __ret_ptr);
}
inline ::option::HasOptions HasOptions::from_ref(
    rs::Option<::std::uint8_t> const& value) {
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
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(
    ::std::uint8_t, rs::Option<::option::LessThan20U8>* __ret_ptr);
}
inline rs::Option<::option::LessThan20U8> LessThan20U8::new_(
    ::std::uint8_t value) {
  crubit::Slot<rs::Option<::option::LessThan20U8>>
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
extern "C" void __crubit_thunk_clone(::option::OptCloneNoDefault const&,
                                     ::option::OptCloneNoDefault* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(::option::OptCloneNoDefault&,
                                           ::option::OptCloneNoDefault const&);
}
inline ::option::OptCloneNoDefault::OptCloneNoDefault(
    const OptCloneNoDefault& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline ::option::OptCloneNoDefault& ::option::OptCloneNoDefault::operator=(
    const OptCloneNoDefault& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
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
extern "C" void __crubit_thunk_drop(::option::OptDefaultWithDrop&);
}
inline OptDefaultWithDrop::~OptDefaultWithDrop() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(rs::StrRef,
                                   ::option::OptDefaultWithDrop* __ret_ptr);
}
inline ::option::OptDefaultWithDrop OptDefaultWithDrop::new_(rs::StrRef s) {
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
extern "C" void __crubit_thunk_drop(::option::OptNoDefaultWithDrop&);
}
inline OptNoDefaultWithDrop::~OptNoDefaultWithDrop() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(rs::StrRef,
                                   ::option::OptNoDefaultWithDrop* __ret_ptr);
}
inline ::option::OptNoDefaultWithDrop OptNoDefaultWithDrop::new_(rs::StrRef s) {
  crubit::Slot<::option::OptNoDefaultWithDrop> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(s, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" rs::StrRef __crubit_thunk_get_ustring_uinside_uoption(
    ::option::OptNoDefaultWithDrop const&);
}
inline rs::StrRef OptNoDefaultWithDrop::get_string_inside_option() const& $(
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
extern "C" void __crubit_thunk_default(::option::OptZst* __ret_ptr);
}
inline ::option::OptZst::OptZst() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(::std::is_trivially_destructible_v<OptZst>);
static_assert(::std::is_trivially_move_constructible_v<::option::OptZst>);
static_assert(::std::is_trivially_move_assignable_v<::option::OptZst>);
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
inline void OptionWithSizeTypes::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(OptionWithSizeTypes, uval));
  static_assert(16 == offsetof(OptionWithSizeTypes, ival));
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
    rs::Option<::std::int32_t const*>*,
    rs::Option<::std::int32_t const*>* __ret_ptr);
}
inline rs::Option<::std::int32_t const*> pass_option_ptr(
    rs::Option<::std::int32_t const*> x) {
  crubit::Slot<rs::Option<::std::int32_t const*>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_pass_uoption_uptr(&x,
                                                      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uoption_uresult(
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>*
        __ret_ptr);
}
inline rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>
return_option_result() {
  crubit::Slot<
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_return_uoption_uresult(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uoption_uresult_uunmovable(
    rs::Option<rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>*
        __ret_ptr);
}
inline rs::Option<
    rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>
return_option_result_unmovable() {
  crubit::Slot<rs::Option<
      rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_return_uoption_uresult_uunmovable(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_stress_utesting_unested_utypes(
    rs::Option<rs::Result<
        rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>*
        __ret_ptr);
}
inline rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>
stress_testing_nested_types() {
  crubit::Slot<rs::Option<rs::Result<
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_stress_utesting_unested_utypes(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_stringify_ulen(
    rs::Option<::option::HasDefault> const&,
    rs::Option<::std::uint32_t>* __ret_ptr);
}
inline rs::Option<::std::uint32_t> stringify_len(
    rs::Option<::option::HasDefault> const& x) {
  crubit::Slot<rs::Option<::std::uint32_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_stringify_ulen(x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_take_uoption_uresult_uunmovable(
    rs::Option<
        rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>*);
}
inline void take_option_result_unmovable(
    rs::Option<rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>
        _x) {
  crubit::Slot _x_slot((::std::move(_x)));
  return __crubit_internal::__crubit_thunk_take_uoption_uresult_uunmovable(
      _x_slot.Get());
}

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs::Option<::std::int32_t const*>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<rs::Option<::std::int32_t const*>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs::Option<::std::int32_t const*>>);
static_assert(
    ::std::is_trivially_move_assignable_v<rs::Option<::std::int32_t const*>>);
inline constexpr rs::Option<::std::int32_t const*>::Option() { set_tag(0); }
inline constexpr rs::Option<::std::int32_t const*>::Option(
    ::std::nullopt_t) noexcept {
  set_tag(0);
}
inline constexpr rs::Option<::std::int32_t const*>&
rs::Option<::std::int32_t const*>::operator=(::std::nullopt_t) noexcept {
  if (tag() != 0) {
    ::std::destroy_at(reinterpret_cast<::std::int32_t const**>(storage_ + 8));
  }
  set_tag(0);
  return *this;
}
inline rs::Option<::std::int32_t const*>::Option(
    ::std::optional<::std::int32_t const*>&& value) noexcept {
  if (value.has_value()) {
    set_tag(1);
    ::std::int32_t const** some =
        reinterpret_cast<::std::int32_t const**>(storage_ + 8);
    *some = value.value();
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(0);
  }
}
inline rs::Option<::std::int32_t const*>&
rs::Option<::std::int32_t const*>::operator=(
    ::std::optional<::std::int32_t const*>&& value) noexcept {
  if (tag() != 0) {
    ::std::destroy_at(reinterpret_cast<::std::int32_t const**>(storage_ + 8));
  }
  if (value.has_value()) {
    set_tag(1);
    ::std::int32_t const** some =
        reinterpret_cast<::std::int32_t const**>(storage_ + 8);
    *some = value.value();
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(0);
  }
  return *this;
}
template <typename... Args>
inline rs::Option<::std::int32_t const*>::Option(::std::in_place_t,
                                                 Args&&... args) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::std::int32_t const**>(storage_ + 8),
                      ::std::forward<Args>(args)...);
}
static_assert(
    ::std::is_trivially_destructible_v<rs::Option<::std::int32_t const*>>);
inline rs::Option<::std::int32_t const*>::operator ::std::optional<
    ::std::int32_t const*>() && noexcept {
  if (tag() == 0) {
    return ::std::nullopt;
  } else {
    ::std::int32_t const*& value =
        *reinterpret_cast<::std::int32_t const**>(storage_ + 8);
    ::std::optional<::std::int32_t const*> return_value(::std::move(value));
    ::std::destroy_at(&value);
    set_tag(0);
    return return_value;
  }
}
inline bool rs::Option<::std::int32_t const*>::has_value() const noexcept {
  return tag() != 0;
}
inline void rs::Option<::std::int32_t const*>::check_has_value() const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs::Option";
}
inline ::std::int32_t const*& rs::Option<::std::int32_t const*>::operator*() & {
  check_has_value();
  return *reinterpret_cast<::std::int32_t const**>(storage_ + 8);
}
inline ::std::int32_t const* const&
rs::Option<::std::int32_t const*>::operator*() const& {
  check_has_value();
  return *reinterpret_cast<::std::int32_t const* const*>(storage_ + 8);
}
inline ::std::int32_t const*&&
rs::Option<::std::int32_t const*>::operator*() && {
  check_has_value();
  return ::std::move(*reinterpret_cast<::std::int32_t const**>(storage_ + 8));
}
inline ::std::int32_t const** rs::Option<::std::int32_t const*>::operator->() {
  check_has_value();
  return reinterpret_cast<::std::int32_t const**>(storage_ + 8);
}
inline ::std::int32_t const* const*
rs::Option<::std::int32_t const*>::operator->() const {
  check_has_value();
  return reinterpret_cast<::std::int32_t const* const*>(storage_ + 8);
}
inline constexpr ::std::uint64_t rs::Option<::std::int32_t const*>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void rs::Option<::std::int32_t const*>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<rs::Option<::std::int32_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<rs::Option<::std::int32_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<rs::Option<::std::int32_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<rs::Option<::std::int32_t>>);
inline constexpr rs::Option<::std::int32_t>::Option() { set_tag(0); }
inline constexpr rs::Option<::std::int32_t>::Option(::std::nullopt_t) noexcept {
  set_tag(0);
}
inline constexpr rs::Option<::std::int32_t>&
rs::Option<::std::int32_t>::operator=(::std::nullopt_t) noexcept {
  if (tag() != 0) {
    ::std::destroy_at(reinterpret_cast<::std::int32_t*>(storage_ + 4));
  }
  set_tag(0);
  return *this;
}
inline rs::Option<::std::int32_t>::Option(::std::int32_t&& value) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::std::int32_t*>(storage_ + 4),
                      ::std::move(value));
}
inline rs::Option<::std::int32_t>& rs::Option<::std::int32_t>::operator=(
    ::std::int32_t&& value) noexcept {
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
inline rs::Option<::std::int32_t>::Option(
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
inline rs::Option<::std::int32_t>& rs::Option<::std::int32_t>::operator=(
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
inline rs::Option<::std::int32_t>::Option(::std::in_place_t,
                                          Args&&... args) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::std::int32_t*>(storage_ + 4),
                      ::std::forward<Args>(args)...);
}
static_assert(::std::is_trivially_destructible_v<rs::Option<::std::int32_t>>);
inline rs::Option<::std::int32_t>::operator ::std::optional<
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
inline bool rs::Option<::std::int32_t>::has_value() const noexcept {
  return tag() != 0;
}
inline void rs::Option<::std::int32_t>::check_has_value() const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs::Option";
}
inline ::std::int32_t& rs::Option<::std::int32_t>::operator*() & {
  check_has_value();
  return *reinterpret_cast<::std::int32_t*>(storage_ + 4);
}
inline ::std::int32_t const& rs::Option<::std::int32_t>::operator*() const& {
  check_has_value();
  return *reinterpret_cast<::std::int32_t const*>(storage_ + 4);
}
inline ::std::int32_t&& rs::Option<::std::int32_t>::operator*() && {
  check_has_value();
  return ::std::move(*reinterpret_cast<::std::int32_t*>(storage_ + 4));
}
inline ::std::int32_t* rs::Option<::std::int32_t>::operator->() {
  check_has_value();
  return reinterpret_cast<::std::int32_t*>(storage_ + 4);
}
inline ::std::int32_t const* rs::Option<::std::int32_t>::operator->() const {
  check_has_value();
  return reinterpret_cast<::std::int32_t const*>(storage_ + 4);
}
inline constexpr ::std::uint32_t rs::Option<::std::int32_t>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void rs::Option<::std::int32_t>::set_tag(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    rs::Option<::option::CloneNoDefault> const&,
    rs::Option<::option::CloneNoDefault>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(
    rs::Option<::option::CloneNoDefault>&,
    rs::Option<::option::CloneNoDefault> const&);
}
inline rs::Option<::option::CloneNoDefault>::Option(const Option& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline rs::Option<::option::CloneNoDefault>&
rs::Option<::option::CloneNoDefault>::operator=(const Option& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
static_assert(::std::is_trivially_move_constructible_v<
              rs::Option<::option::CloneNoDefault>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs::Option<::option::CloneNoDefault>>);
inline constexpr rs::Option<::option::CloneNoDefault>::Option() { set_tag(0); }
inline constexpr rs::Option<::option::CloneNoDefault>::Option(
    ::std::nullopt_t) noexcept {
  set_tag(0);
}
inline constexpr rs::Option<::option::CloneNoDefault>&
rs::Option<::option::CloneNoDefault>::operator=(::std::nullopt_t) noexcept {
  if (tag() != 0) {
    ::std::destroy_at(
        reinterpret_cast<::option::CloneNoDefault*>(storage_ + 1));
  }
  set_tag(0);
  return *this;
}
inline rs::Option<::option::CloneNoDefault>::Option(
    ::option::CloneNoDefault&& value) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::option::CloneNoDefault*>(storage_ + 1),
                      ::std::move(value));
}
inline rs::Option<::option::CloneNoDefault>&
rs::Option<::option::CloneNoDefault>::operator=(
    ::option::CloneNoDefault&& value) noexcept {
  if (tag() != 0) {
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::option::CloneNoDefault*>(storage_ + 1),
        ::std::move(value));
  } else {
    set_tag(1);
    ::std::construct_at(
        reinterpret_cast<::option::CloneNoDefault*>(storage_ + 1),
        ::std::move(value));
  }
  return *this;
}
inline rs::Option<::option::CloneNoDefault>::Option(
    ::std::optional<::option::CloneNoDefault>&& value) noexcept {
  if (value.has_value()) {
    set_tag(1);
    ::option::CloneNoDefault* some =
        reinterpret_cast<::option::CloneNoDefault*>(storage_ + 1);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(0);
  }
}
inline rs::Option<::option::CloneNoDefault>&
rs::Option<::option::CloneNoDefault>::operator=(
    ::std::optional<::option::CloneNoDefault>&& value) noexcept {
  if (tag() != 0) {
    ::std::destroy_at(
        reinterpret_cast<::option::CloneNoDefault*>(storage_ + 1));
  }
  if (value.has_value()) {
    set_tag(1);
    ::option::CloneNoDefault* some =
        reinterpret_cast<::option::CloneNoDefault*>(storage_ + 1);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(0);
  }
  return *this;
}
template <typename... Args>
inline rs::Option<::option::CloneNoDefault>::Option(::std::in_place_t,
                                                    Args&&... args) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::option::CloneNoDefault*>(storage_ + 1),
                      ::std::forward<Args>(args)...);
}
static_assert(
    ::std::is_trivially_destructible_v<rs::Option<::option::CloneNoDefault>>);
inline rs::Option<::option::CloneNoDefault>::operator ::std::optional<
    ::option::CloneNoDefault>() && noexcept {
  if (tag() == 0) {
    return ::std::nullopt;
  } else {
    struct DeferSetTagNone {
      rs::Option<::option::CloneNoDefault>* _value;
      DeferSetTagNone(rs::Option<::option::CloneNoDefault>* self)
          : _value(self) {}
      ~DeferSetTagNone() { set_tag(0); }

     private:
      void set_tag(::std::uint8_t tag) { _value->set_tag(tag); }
    } defer(this);
    return ::std::make_optional<::option::CloneNoDefault>(
        crubit::UnsafeRelocateTag{},
        ::std::move(
            *reinterpret_cast<::option::CloneNoDefault*>(storage_ + 1)));
  }
}
inline bool rs::Option<::option::CloneNoDefault>::has_value() const noexcept {
  return tag() != 0;
}
inline void rs::Option<::option::CloneNoDefault>::check_has_value() const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs::Option";
}
inline ::option::CloneNoDefault&
rs::Option<::option::CloneNoDefault>::operator*() & {
  check_has_value();
  return *reinterpret_cast<::option::CloneNoDefault*>(storage_ + 1);
}
inline ::option::CloneNoDefault const&
rs::Option<::option::CloneNoDefault>::operator*() const& {
  check_has_value();
  return *reinterpret_cast<::option::CloneNoDefault const*>(storage_ + 1);
}
inline ::option::CloneNoDefault&&
rs::Option<::option::CloneNoDefault>::operator*() && {
  check_has_value();
  return ::std::move(
      *reinterpret_cast<::option::CloneNoDefault*>(storage_ + 1));
}
inline ::option::CloneNoDefault*
rs::Option<::option::CloneNoDefault>::operator->() {
  check_has_value();
  return reinterpret_cast<::option::CloneNoDefault*>(storage_ + 1);
}
inline ::option::CloneNoDefault const*
rs::Option<::option::CloneNoDefault>::operator->() const {
  check_has_value();
  return reinterpret_cast<::option::CloneNoDefault const*>(storage_ + 1);
}
inline constexpr ::std::uint8_t rs::Option<::option::CloneNoDefault>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs::Option<::option::CloneNoDefault>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs::Option<::option::CopyNoDefault>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<rs::Option<::option::CopyNoDefault>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs::Option<::option::CopyNoDefault>>);
static_assert(
    ::std::is_trivially_move_assignable_v<rs::Option<::option::CopyNoDefault>>);
inline constexpr rs::Option<::option::CopyNoDefault>::Option() { set_tag(0); }
inline constexpr rs::Option<::option::CopyNoDefault>::Option(
    ::std::nullopt_t) noexcept {
  set_tag(0);
}
inline constexpr rs::Option<::option::CopyNoDefault>&
rs::Option<::option::CopyNoDefault>::operator=(::std::nullopt_t) noexcept {
  if (tag() != 0) {
    ::std::destroy_at(reinterpret_cast<::option::CopyNoDefault*>(storage_ + 1));
  }
  set_tag(0);
  return *this;
}
inline rs::Option<::option::CopyNoDefault>::Option(
    ::option::CopyNoDefault&& value) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::option::CopyNoDefault*>(storage_ + 1),
                      ::std::move(value));
}
inline rs::Option<::option::CopyNoDefault>&
rs::Option<::option::CopyNoDefault>::operator=(
    ::option::CopyNoDefault&& value) noexcept {
  if (tag() != 0) {
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::option::CopyNoDefault*>(storage_ + 1),
        ::std::move(value));
  } else {
    set_tag(1);
    ::std::construct_at(
        reinterpret_cast<::option::CopyNoDefault*>(storage_ + 1),
        ::std::move(value));
  }
  return *this;
}
inline rs::Option<::option::CopyNoDefault>::Option(
    ::std::optional<::option::CopyNoDefault>&& value) noexcept {
  if (value.has_value()) {
    set_tag(1);
    ::option::CopyNoDefault* some =
        reinterpret_cast<::option::CopyNoDefault*>(storage_ + 1);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(0);
  }
}
inline rs::Option<::option::CopyNoDefault>&
rs::Option<::option::CopyNoDefault>::operator=(
    ::std::optional<::option::CopyNoDefault>&& value) noexcept {
  if (tag() != 0) {
    ::std::destroy_at(reinterpret_cast<::option::CopyNoDefault*>(storage_ + 1));
  }
  if (value.has_value()) {
    set_tag(1);
    ::option::CopyNoDefault* some =
        reinterpret_cast<::option::CopyNoDefault*>(storage_ + 1);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(0);
  }
  return *this;
}
template <typename... Args>
inline rs::Option<::option::CopyNoDefault>::Option(::std::in_place_t,
                                                   Args&&... args) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::option::CopyNoDefault*>(storage_ + 1),
                      ::std::forward<Args>(args)...);
}
static_assert(
    ::std::is_trivially_destructible_v<rs::Option<::option::CopyNoDefault>>);
inline rs::Option<::option::CopyNoDefault>::operator ::std::optional<
    ::option::CopyNoDefault>() && noexcept {
  if (tag() == 0) {
    return ::std::nullopt;
  } else {
    struct DeferSetTagNone {
      rs::Option<::option::CopyNoDefault>* _value;
      DeferSetTagNone(rs::Option<::option::CopyNoDefault>* self)
          : _value(self) {}
      ~DeferSetTagNone() { set_tag(0); }

     private:
      void set_tag(::std::uint8_t tag) { _value->set_tag(tag); }
    } defer(this);
    return ::std::make_optional<::option::CopyNoDefault>(
        crubit::UnsafeRelocateTag{},
        ::std::move(*reinterpret_cast<::option::CopyNoDefault*>(storage_ + 1)));
  }
}
inline bool rs::Option<::option::CopyNoDefault>::has_value() const noexcept {
  return tag() != 0;
}
inline void rs::Option<::option::CopyNoDefault>::check_has_value() const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs::Option";
}
inline ::option::CopyNoDefault&
rs::Option<::option::CopyNoDefault>::operator*() & {
  check_has_value();
  return *reinterpret_cast<::option::CopyNoDefault*>(storage_ + 1);
}
inline ::option::CopyNoDefault const&
rs::Option<::option::CopyNoDefault>::operator*() const& {
  check_has_value();
  return *reinterpret_cast<::option::CopyNoDefault const*>(storage_ + 1);
}
inline ::option::CopyNoDefault&&
rs::Option<::option::CopyNoDefault>::operator*() && {
  check_has_value();
  return ::std::move(*reinterpret_cast<::option::CopyNoDefault*>(storage_ + 1));
}
inline ::option::CopyNoDefault*
rs::Option<::option::CopyNoDefault>::operator->() {
  check_has_value();
  return reinterpret_cast<::option::CopyNoDefault*>(storage_ + 1);
}
inline ::option::CopyNoDefault const*
rs::Option<::option::CopyNoDefault>::operator->() const {
  check_has_value();
  return reinterpret_cast<::option::CopyNoDefault const*>(storage_ + 1);
}
inline constexpr ::std::uint8_t rs::Option<::option::CopyNoDefault>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs::Option<::option::CopyNoDefault>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
inline rs::Option<::option::HasDefault>::Option(Option&& other) : Option() {
  *this = ::std::move(other);
}
inline rs::Option<::option::HasDefault>&
rs::Option<::option::HasDefault>::operator=(Option&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline constexpr rs::Option<::option::HasDefault>::Option() {
  set_tag(UINT64_C(18446744073709551615));
}
inline constexpr rs::Option<::option::HasDefault>::Option(
    ::std::nullopt_t) noexcept {
  set_tag(UINT64_C(18446744073709551615));
}
inline constexpr rs::Option<::option::HasDefault>&
rs::Option<::option::HasDefault>::operator=(::std::nullopt_t) noexcept {
  if (tag() != UINT64_C(18446744073709551615)) {
    ::std::destroy_at(reinterpret_cast<::option::HasDefault*>(storage_));
  }
  set_tag(UINT64_C(18446744073709551615));
  return *this;
}
inline rs::Option<::option::HasDefault>::Option(
    ::option::HasDefault&& value) noexcept {
  ::std::construct_at(reinterpret_cast<::option::HasDefault*>(storage_),
                      ::std::move(value));
}
inline rs::Option<::option::HasDefault>& rs::Option<
    ::option::HasDefault>::operator=(::option::HasDefault&& value) noexcept {
  if (tag() != UINT64_C(18446744073709551615)) {
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::option::HasDefault*>(storage_), ::std::move(value));
  } else {
    ::std::construct_at(reinterpret_cast<::option::HasDefault*>(storage_),
                        ::std::move(value));
  }
  return *this;
}
inline rs::Option<::option::HasDefault>::Option(
    ::std::optional<::option::HasDefault>&& value) noexcept {
  if (value.has_value()) {
    ::option::HasDefault* some =
        reinterpret_cast<::option::HasDefault*>(storage_);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(UINT64_C(18446744073709551615));
  }
}
inline rs::Option<::option::HasDefault>&
rs::Option<::option::HasDefault>::operator=(
    ::std::optional<::option::HasDefault>&& value) noexcept {
  if (tag() != UINT64_C(18446744073709551615)) {
    ::std::destroy_at(reinterpret_cast<::option::HasDefault*>(storage_));
  }
  if (value.has_value()) {
    ::option::HasDefault* some =
        reinterpret_cast<::option::HasDefault*>(storage_);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(UINT64_C(18446744073709551615));
  }
  return *this;
}
template <typename... Args>
inline rs::Option<::option::HasDefault>::Option(::std::in_place_t,
                                                Args&&... args) noexcept {
  ::std::construct_at(reinterpret_cast<::option::HasDefault*>(storage_),
                      ::std::forward<Args>(args)...);
}
inline constexpr rs::Option<::option::HasDefault>::~Option() noexcept {
  if (tag() != UINT64_C(18446744073709551615)) {
    ::std::destroy_at(reinterpret_cast<::option::HasDefault*>(storage_));
  }
}
inline rs::Option<::option::HasDefault>::operator ::std::optional<
    ::option::HasDefault>() && noexcept {
  if (tag() == UINT64_C(18446744073709551615)) {
    return ::std::nullopt;
  } else {
    struct DeferSetTagNone {
      rs::Option<::option::HasDefault>* _value;
      DeferSetTagNone(rs::Option<::option::HasDefault>* self) : _value(self) {}
      ~DeferSetTagNone() { set_tag(UINT64_C(18446744073709551615)); }

     private:
      void set_tag(::std::uint64_t tag) { _value->set_tag(tag); }
    } defer(this);
    return ::std::make_optional<::option::HasDefault>(
        crubit::UnsafeRelocateTag{},
        ::std::move(*reinterpret_cast<::option::HasDefault*>(storage_)));
  }
}
inline bool rs::Option<::option::HasDefault>::has_value() const noexcept {
  return tag() != UINT64_C(18446744073709551615);
}
inline void rs::Option<::option::HasDefault>::check_has_value() const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs::Option";
}
inline ::option::HasDefault& rs::Option<::option::HasDefault>::operator*() & {
  check_has_value();
  return *reinterpret_cast<::option::HasDefault*>(storage_);
}
inline ::option::HasDefault const& rs::Option<::option::HasDefault>::operator*()
    const& {
  check_has_value();
  return *reinterpret_cast<::option::HasDefault const*>(storage_);
}
inline ::option::HasDefault&& rs::Option<::option::HasDefault>::operator*() && {
  check_has_value();
  return ::std::move(*reinterpret_cast<::option::HasDefault*>(storage_));
}
inline ::option::HasDefault* rs::Option<::option::HasDefault>::operator->() {
  check_has_value();
  return reinterpret_cast<::option::HasDefault*>(storage_);
}
inline ::option::HasDefault const*
rs::Option<::option::HasDefault>::operator->() const {
  check_has_value();
  return reinterpret_cast<::option::HasDefault const*>(storage_);
}
inline constexpr ::std::uint64_t rs::Option<::option::HasDefault>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void rs::Option<::option::HasDefault>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
inline rs::Option<::option::HasNoDefault>::Option(Option&& other) : Option() {
  *this = ::std::move(other);
}
inline rs::Option<::option::HasNoDefault>&
rs::Option<::option::HasNoDefault>::operator=(Option&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline constexpr rs::Option<::option::HasNoDefault>::Option() {
  set_tag(UINT64_C(18446744073709551615));
}
inline constexpr rs::Option<::option::HasNoDefault>::Option(
    ::std::nullopt_t) noexcept {
  set_tag(UINT64_C(18446744073709551615));
}
inline constexpr rs::Option<::option::HasNoDefault>&
rs::Option<::option::HasNoDefault>::operator=(::std::nullopt_t) noexcept {
  if (tag() != UINT64_C(18446744073709551615)) {
    ::std::destroy_at(reinterpret_cast<::option::HasNoDefault*>(storage_));
  }
  set_tag(UINT64_C(18446744073709551615));
  return *this;
}
inline rs::Option<::option::HasNoDefault>::Option(
    ::std::optional<::option::HasNoDefault>&& value) noexcept {
  if (value.has_value()) {
    ::option::HasNoDefault* some =
        reinterpret_cast<::option::HasNoDefault*>(storage_);
    ::std::construct_at(some, crubit::UnsafeRelocateTag{}, ::std::move(*value));
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(UINT64_C(18446744073709551615));
  }
}
inline rs::Option<::option::HasNoDefault>&
rs::Option<::option::HasNoDefault>::operator=(
    ::std::optional<::option::HasNoDefault>&& value) noexcept {
  if (tag() != UINT64_C(18446744073709551615)) {
    ::std::destroy_at(reinterpret_cast<::option::HasNoDefault*>(storage_));
  }
  if (value.has_value()) {
    ::option::HasNoDefault* some =
        reinterpret_cast<::option::HasNoDefault*>(storage_);
    ::std::construct_at(some, crubit::UnsafeRelocateTag{}, ::std::move(*value));
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(UINT64_C(18446744073709551615));
  }
  return *this;
}
template <typename... Args>
inline rs::Option<::option::HasNoDefault>::Option(::std::in_place_t,
                                                  Args&&... args) noexcept {
  ::std::construct_at(reinterpret_cast<::option::HasNoDefault*>(storage_),
                      ::std::forward<Args>(args)...);
}
inline constexpr rs::Option<::option::HasNoDefault>::~Option() noexcept {
  if (tag() != UINT64_C(18446744073709551615)) {
    ::std::destroy_at(reinterpret_cast<::option::HasNoDefault*>(storage_));
  }
}
inline rs::Option<::option::HasNoDefault>::operator ::std::optional<
    ::option::HasNoDefault>() && noexcept {
  if (tag() == UINT64_C(18446744073709551615)) {
    return ::std::nullopt;
  } else {
    struct DeferSetTagNone {
      rs::Option<::option::HasNoDefault>* _value;
      DeferSetTagNone(rs::Option<::option::HasNoDefault>* self)
          : _value(self) {}
      ~DeferSetTagNone() { set_tag(UINT64_C(18446744073709551615)); }

     private:
      void set_tag(::std::uint64_t tag) { _value->set_tag(tag); }
    } defer(this);
    return ::std::make_optional<::option::HasNoDefault>(
        crubit::UnsafeRelocateTag{},
        ::std::move(*reinterpret_cast<::option::HasNoDefault*>(storage_)));
  }
}
inline bool rs::Option<::option::HasNoDefault>::has_value() const noexcept {
  return tag() != UINT64_C(18446744073709551615);
}
inline void rs::Option<::option::HasNoDefault>::check_has_value() const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs::Option";
}
inline ::option::HasNoDefault&
rs::Option<::option::HasNoDefault>::operator*() & {
  check_has_value();
  return *reinterpret_cast<::option::HasNoDefault*>(storage_);
}
inline ::option::HasNoDefault const&
rs::Option<::option::HasNoDefault>::operator*() const& {
  check_has_value();
  return *reinterpret_cast<::option::HasNoDefault const*>(storage_);
}
inline ::option::HasNoDefault&&
rs::Option<::option::HasNoDefault>::operator*() && {
  check_has_value();
  return ::std::move(*reinterpret_cast<::option::HasNoDefault*>(storage_));
}
inline ::option::HasNoDefault*
rs::Option<::option::HasNoDefault>::operator->() {
  check_has_value();
  return reinterpret_cast<::option::HasNoDefault*>(storage_);
}
inline ::option::HasNoDefault const*
rs::Option<::option::HasNoDefault>::operator->() const {
  check_has_value();
  return reinterpret_cast<::option::HasNoDefault const*>(storage_);
}
inline constexpr ::std::uint64_t rs::Option<::option::HasNoDefault>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void rs::Option<::option::HasNoDefault>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasOptions_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasOptions_x00000020_x0000003e
static_assert(
    ::std::is_trivially_move_constructible_v<rs::Option<::option::HasOptions>>);
static_assert(
    ::std::is_trivially_move_assignable_v<rs::Option<::option::HasOptions>>);
inline constexpr rs::Option<::option::HasOptions>::Option() { set_tag(2); }
inline constexpr rs::Option<::option::HasOptions>::Option(
    ::std::nullopt_t) noexcept {
  set_tag(2);
}
inline constexpr rs::Option<::option::HasOptions>&
rs::Option<::option::HasOptions>::operator=(::std::nullopt_t) noexcept {
  if (tag() != 2) {
    ::std::destroy_at(reinterpret_cast<::option::HasOptions*>(storage_));
  }
  set_tag(2);
  return *this;
}
inline rs::Option<::option::HasOptions>::Option(
    ::option::HasOptions&& value) noexcept {
  ::std::construct_at(reinterpret_cast<::option::HasOptions*>(storage_),
                      ::std::move(value));
}
inline rs::Option<::option::HasOptions>& rs::Option<
    ::option::HasOptions>::operator=(::option::HasOptions&& value) noexcept {
  if (tag() != 2) {
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::option::HasOptions*>(storage_), ::std::move(value));
  } else {
    ::std::construct_at(reinterpret_cast<::option::HasOptions*>(storage_),
                        ::std::move(value));
  }
  return *this;
}
inline rs::Option<::option::HasOptions>::Option(
    ::std::optional<::option::HasOptions>&& value) noexcept {
  if (value.has_value()) {
    ::option::HasOptions* some =
        reinterpret_cast<::option::HasOptions*>(storage_);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(2);
  }
}
inline rs::Option<::option::HasOptions>&
rs::Option<::option::HasOptions>::operator=(
    ::std::optional<::option::HasOptions>&& value) noexcept {
  if (tag() != 2) {
    ::std::destroy_at(reinterpret_cast<::option::HasOptions*>(storage_));
  }
  if (value.has_value()) {
    ::option::HasOptions* some =
        reinterpret_cast<::option::HasOptions*>(storage_);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(2);
  }
  return *this;
}
template <typename... Args>
inline rs::Option<::option::HasOptions>::Option(::std::in_place_t,
                                                Args&&... args) noexcept {
  ::std::construct_at(reinterpret_cast<::option::HasOptions*>(storage_),
                      ::std::forward<Args>(args)...);
}
static_assert(
    ::std::is_trivially_destructible_v<rs::Option<::option::HasOptions>>);
inline rs::Option<::option::HasOptions>::operator ::std::optional<
    ::option::HasOptions>() && noexcept {
  if (tag() == 2) {
    return ::std::nullopt;
  } else {
    struct DeferSetTagNone {
      rs::Option<::option::HasOptions>* _value;
      DeferSetTagNone(rs::Option<::option::HasOptions>* self) : _value(self) {}
      ~DeferSetTagNone() { set_tag(2); }

     private:
      void set_tag(::std::uint8_t tag) { _value->set_tag(tag); }
    } defer(this);
    return ::std::make_optional<::option::HasOptions>(
        crubit::UnsafeRelocateTag{},
        ::std::move(*reinterpret_cast<::option::HasOptions*>(storage_)));
  }
}
inline bool rs::Option<::option::HasOptions>::has_value() const noexcept {
  return tag() != 2;
}
inline void rs::Option<::option::HasOptions>::check_has_value() const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs::Option";
}
inline ::option::HasOptions& rs::Option<::option::HasOptions>::operator*() & {
  check_has_value();
  return *reinterpret_cast<::option::HasOptions*>(storage_);
}
inline ::option::HasOptions const& rs::Option<::option::HasOptions>::operator*()
    const& {
  check_has_value();
  return *reinterpret_cast<::option::HasOptions const*>(storage_);
}
inline ::option::HasOptions&& rs::Option<::option::HasOptions>::operator*() && {
  check_has_value();
  return ::std::move(*reinterpret_cast<::option::HasOptions*>(storage_));
}
inline ::option::HasOptions* rs::Option<::option::HasOptions>::operator->() {
  check_has_value();
  return reinterpret_cast<::option::HasOptions*>(storage_);
}
inline ::option::HasOptions const*
rs::Option<::option::HasOptions>::operator->() const {
  check_has_value();
  return reinterpret_cast<::option::HasOptions const*>(storage_);
}
inline constexpr ::std::uint8_t rs::Option<::option::HasOptions>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs::Option<::option::HasOptions>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs::Option<::option::LessThan20U8>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<rs::Option<::option::LessThan20U8>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs::Option<::option::LessThan20U8>>);
static_assert(
    ::std::is_trivially_move_assignable_v<rs::Option<::option::LessThan20U8>>);
inline constexpr rs::Option<::option::LessThan20U8>::Option() { set_tag(255); }
inline constexpr rs::Option<::option::LessThan20U8>::Option(
    ::std::nullopt_t) noexcept {
  set_tag(255);
}
inline constexpr rs::Option<::option::LessThan20U8>&
rs::Option<::option::LessThan20U8>::operator=(::std::nullopt_t) noexcept {
  if (tag() != 255) {
    ::std::destroy_at(reinterpret_cast<::option::LessThan20U8*>(storage_));
  }
  set_tag(255);
  return *this;
}
inline rs::Option<::option::LessThan20U8>::Option(
    ::option::LessThan20U8&& value) noexcept {
  ::std::construct_at(reinterpret_cast<::option::LessThan20U8*>(storage_),
                      ::std::move(value));
}
inline rs::Option<::option::LessThan20U8>&
rs::Option<::option::LessThan20U8>::operator=(
    ::option::LessThan20U8&& value) noexcept {
  if (tag() != 255) {
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::option::LessThan20U8*>(storage_),
        ::std::move(value));
  } else {
    ::std::construct_at(reinterpret_cast<::option::LessThan20U8*>(storage_),
                        ::std::move(value));
  }
  return *this;
}
inline rs::Option<::option::LessThan20U8>::Option(
    ::std::optional<::option::LessThan20U8>&& value) noexcept {
  if (value.has_value()) {
    ::option::LessThan20U8* some =
        reinterpret_cast<::option::LessThan20U8*>(storage_);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(255);
  }
}
inline rs::Option<::option::LessThan20U8>&
rs::Option<::option::LessThan20U8>::operator=(
    ::std::optional<::option::LessThan20U8>&& value) noexcept {
  if (tag() != 255) {
    ::std::destroy_at(reinterpret_cast<::option::LessThan20U8*>(storage_));
  }
  if (value.has_value()) {
    ::option::LessThan20U8* some =
        reinterpret_cast<::option::LessThan20U8*>(storage_);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(255);
  }
  return *this;
}
template <typename... Args>
inline rs::Option<::option::LessThan20U8>::Option(::std::in_place_t,
                                                  Args&&... args) noexcept {
  ::std::construct_at(reinterpret_cast<::option::LessThan20U8*>(storage_),
                      ::std::forward<Args>(args)...);
}
static_assert(
    ::std::is_trivially_destructible_v<rs::Option<::option::LessThan20U8>>);
inline rs::Option<::option::LessThan20U8>::operator ::std::optional<
    ::option::LessThan20U8>() && noexcept {
  if (tag() == 255) {
    return ::std::nullopt;
  } else {
    struct DeferSetTagNone {
      rs::Option<::option::LessThan20U8>* _value;
      DeferSetTagNone(rs::Option<::option::LessThan20U8>* self)
          : _value(self) {}
      ~DeferSetTagNone() { set_tag(255); }

     private:
      void set_tag(::std::uint8_t tag) { _value->set_tag(tag); }
    } defer(this);
    return ::std::make_optional<::option::LessThan20U8>(
        crubit::UnsafeRelocateTag{},
        ::std::move(*reinterpret_cast<::option::LessThan20U8*>(storage_)));
  }
}
inline bool rs::Option<::option::LessThan20U8>::has_value() const noexcept {
  return tag() != 255;
}
inline void rs::Option<::option::LessThan20U8>::check_has_value() const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs::Option";
}
inline ::option::LessThan20U8&
rs::Option<::option::LessThan20U8>::operator*() & {
  check_has_value();
  return *reinterpret_cast<::option::LessThan20U8*>(storage_);
}
inline ::option::LessThan20U8 const&
rs::Option<::option::LessThan20U8>::operator*() const& {
  check_has_value();
  return *reinterpret_cast<::option::LessThan20U8 const*>(storage_);
}
inline ::option::LessThan20U8&&
rs::Option<::option::LessThan20U8>::operator*() && {
  check_has_value();
  return ::std::move(*reinterpret_cast<::option::LessThan20U8*>(storage_));
}
inline ::option::LessThan20U8*
rs::Option<::option::LessThan20U8>::operator->() {
  check_has_value();
  return reinterpret_cast<::option::LessThan20U8*>(storage_);
}
inline ::option::LessThan20U8 const*
rs::Option<::option::LessThan20U8>::operator->() const {
  check_has_value();
  return reinterpret_cast<::option::LessThan20U8 const*>(storage_);
}
inline constexpr ::std::uint8_t rs::Option<::option::LessThan20U8>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs::Option<::option::LessThan20U8>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020LessThan20U8_x00000020_x0000003e_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs::Option<rs::Option<::option::LessThan20U8>>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs::Option<rs::Option<::option::LessThan20U8>>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs::Option<rs::Option<::option::LessThan20U8>>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs::Option<rs::Option<::option::LessThan20U8>>>);
inline constexpr rs::Option<rs::Option<::option::LessThan20U8>>::Option() {
  set_tag(254);
}
inline constexpr rs::Option<rs::Option<::option::LessThan20U8>>::Option(
    ::std::nullopt_t) noexcept {
  set_tag(254);
}
inline constexpr rs::Option<rs::Option<::option::LessThan20U8>>& rs::Option<
    rs::Option<::option::LessThan20U8>>::operator=(::std::nullopt_t) noexcept {
  if (tag() != 254) {
    ::std::destroy_at(
        reinterpret_cast<rs::Option<::option::LessThan20U8>*>(storage_));
  }
  set_tag(254);
  return *this;
}
inline rs::Option<rs::Option<::option::LessThan20U8>>::Option(
    rs::Option<::option::LessThan20U8>&& value) noexcept {
  ::std::construct_at(
      reinterpret_cast<rs::Option<::option::LessThan20U8>*>(storage_),
      ::std::move(value));
}
inline rs::Option<rs::Option<::option::LessThan20U8>>&
rs::Option<rs::Option<::option::LessThan20U8>>::operator=(
    rs::Option<::option::LessThan20U8>&& value) noexcept {
  if (tag() != 254) {
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<rs::Option<::option::LessThan20U8>*>(storage_),
        ::std::move(value));
  } else {
    ::std::construct_at(
        reinterpret_cast<rs::Option<::option::LessThan20U8>*>(storage_),
        ::std::move(value));
  }
  return *this;
}
inline rs::Option<rs::Option<::option::LessThan20U8>>::Option(
    ::std::optional<rs::Option<::option::LessThan20U8>>&& value) noexcept {
  if (value.has_value()) {
    rs::Option<::option::LessThan20U8>* some =
        reinterpret_cast<rs::Option<::option::LessThan20U8>*>(storage_);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(254);
  }
}
inline rs::Option<rs::Option<::option::LessThan20U8>>&
rs::Option<rs::Option<::option::LessThan20U8>>::operator=(
    ::std::optional<rs::Option<::option::LessThan20U8>>&& value) noexcept {
  if (tag() != 254) {
    ::std::destroy_at(
        reinterpret_cast<rs::Option<::option::LessThan20U8>*>(storage_));
  }
  if (value.has_value()) {
    rs::Option<::option::LessThan20U8>* some =
        reinterpret_cast<rs::Option<::option::LessThan20U8>*>(storage_);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(254);
  }
  return *this;
}
template <typename... Args>
inline rs::Option<rs::Option<::option::LessThan20U8>>::Option(
    ::std::in_place_t, Args&&... args) noexcept {
  ::std::construct_at(
      reinterpret_cast<rs::Option<::option::LessThan20U8>*>(storage_),
      ::std::forward<Args>(args)...);
}
static_assert(::std::is_trivially_destructible_v<
              rs::Option<rs::Option<::option::LessThan20U8>>>);
inline rs::Option<rs::Option<::option::LessThan20U8>>::operator ::std::optional<
    rs::Option<::option::LessThan20U8>>() && noexcept {
  if (tag() == 254) {
    return ::std::nullopt;
  } else {
    struct DeferSetTagNone {
      rs::Option<rs::Option<::option::LessThan20U8>>* _value;
      DeferSetTagNone(rs::Option<rs::Option<::option::LessThan20U8>>* self)
          : _value(self) {}
      ~DeferSetTagNone() { set_tag(254); }

     private:
      void set_tag(::std::uint8_t tag) { _value->set_tag(tag); }
    } defer(this);
    return ::std::make_optional<rs::Option<::option::LessThan20U8>>(
        crubit::UnsafeRelocateTag{},
        ::std::move(
            *reinterpret_cast<rs::Option<::option::LessThan20U8>*>(storage_)));
  }
}
inline bool rs::Option<rs::Option<::option::LessThan20U8>>::has_value()
    const noexcept {
  return tag() != 254;
}
inline void rs::Option<rs::Option<::option::LessThan20U8>>::check_has_value()
    const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs::Option";
}
inline rs::Option<::option::LessThan20U8>&
rs::Option<rs::Option<::option::LessThan20U8>>::operator*() & {
  check_has_value();
  return *reinterpret_cast<rs::Option<::option::LessThan20U8>*>(storage_);
}
inline rs::Option<::option::LessThan20U8> const&
rs::Option<rs::Option<::option::LessThan20U8>>::operator*() const& {
  check_has_value();
  return *reinterpret_cast<rs::Option<::option::LessThan20U8> const*>(storage_);
}
inline rs::Option<::option::LessThan20U8>&&
rs::Option<rs::Option<::option::LessThan20U8>>::operator*() && {
  check_has_value();
  return ::std::move(
      *reinterpret_cast<rs::Option<::option::LessThan20U8>*>(storage_));
}
inline rs::Option<::option::LessThan20U8>*
rs::Option<rs::Option<::option::LessThan20U8>>::operator->() {
  check_has_value();
  return reinterpret_cast<rs::Option<::option::LessThan20U8>*>(storage_);
}
inline rs::Option<::option::LessThan20U8> const*
rs::Option<rs::Option<::option::LessThan20U8>>::operator->() const {
  check_has_value();
  return reinterpret_cast<rs::Option<::option::LessThan20U8> const*>(storage_);
}
inline constexpr ::std::uint8_t
rs::Option<rs::Option<::option::LessThan20U8>>::tag() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs::Option<rs::Option<::option::LessThan20U8>>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>> const&,
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>*
        __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&,
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>> const&);
}
inline rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>::
    Option(const Option& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&
rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>::operator=(
    const Option& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
inline rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>::
    Option(Option&& other)
    : Option() {
  *this = ::std::move(other);
}
inline rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&
rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>::operator=(
    Option&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline constexpr rs::Option<
    rs::Result<::std::int32_t, ::rs::alloc::string::String>>::Option() {
  set_tag(UINT64_C(18446744073709551614));
}
inline constexpr rs::
    Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>::Option(
        ::std::nullopt_t) noexcept {
  set_tag(UINT64_C(18446744073709551614));
}
inline constexpr rs::Option<
    rs::Result<::std::int32_t, ::rs::alloc::string::String>>&
rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>::operator=(
    ::std::nullopt_t) noexcept {
  if (tag() != UINT64_C(18446744073709551614)) {
    ::std::destroy_at(reinterpret_cast<
                      rs::Result<::std::int32_t, ::rs::alloc::string::String>*>(
        storage_));
  }
  set_tag(UINT64_C(18446744073709551614));
  return *this;
}
inline rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>::
    Option(rs::Result<::std::int32_t, ::rs::alloc::string::String>&&
               value) noexcept {
  ::std::construct_at(
      reinterpret_cast<
          rs::Result<::std::int32_t, ::rs::alloc::string::String>*>(storage_),
      ::std::move(value));
}
inline rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&
rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>::operator=(
    rs::Result<::std::int32_t, ::rs::alloc::string::String>&& value) noexcept {
  if (tag() != UINT64_C(18446744073709551614)) {
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<
            rs::Result<::std::int32_t, ::rs::alloc::string::String>*>(storage_),
        ::std::move(value));
  } else {
    ::std::construct_at(
        reinterpret_cast<
            rs::Result<::std::int32_t, ::rs::alloc::string::String>*>(storage_),
        ::std::move(value));
  }
  return *this;
}
inline rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>::
    Option(::std::optional<rs::Result<
               ::std::int32_t, ::rs::alloc::string::String>>&& value) noexcept {
  if (value.has_value()) {
    rs::Result<::std::int32_t, ::rs::alloc::string::String>* some =
        reinterpret_cast<
            rs::Result<::std::int32_t, ::rs::alloc::string::String>*>(storage_);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(UINT64_C(18446744073709551614));
  }
}
inline rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&
rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>::operator=(
    ::std::optional<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&&
        value) noexcept {
  if (tag() != UINT64_C(18446744073709551614)) {
    ::std::destroy_at(reinterpret_cast<
                      rs::Result<::std::int32_t, ::rs::alloc::string::String>*>(
        storage_));
  }
  if (value.has_value()) {
    rs::Result<::std::int32_t, ::rs::alloc::string::String>* some =
        reinterpret_cast<
            rs::Result<::std::int32_t, ::rs::alloc::string::String>*>(storage_);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(UINT64_C(18446744073709551614));
  }
  return *this;
}
template <typename... Args>
inline rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>::
    Option(::std::in_place_t, Args&&... args) noexcept {
  ::std::construct_at(
      reinterpret_cast<
          rs::Result<::std::int32_t, ::rs::alloc::string::String>*>(storage_),
      ::std::forward<Args>(args)...);
}
inline constexpr rs::Option<rs::Result<
    ::std::int32_t, ::rs::alloc::string::String>>::~Option() noexcept {
  if (tag() != UINT64_C(18446744073709551614)) {
    ::std::destroy_at(reinterpret_cast<
                      rs::Result<::std::int32_t, ::rs::alloc::string::String>*>(
        storage_));
  }
}
inline rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>::
operator ::std::optional<
    rs::Result<::std::int32_t, ::rs::alloc::string::String>>() && noexcept {
  if (tag() == UINT64_C(18446744073709551614)) {
    return ::std::nullopt;
  } else {
    struct DeferSetTagNone {
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>*
          _value;
      DeferSetTagNone(
          rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>*
              self)
          : _value(self) {}
      ~DeferSetTagNone() { set_tag(UINT64_C(18446744073709551614)); }

     private:
      void set_tag(::std::uint64_t tag) { _value->set_tag(tag); }
    } defer(this);
    return ::std::make_optional<
        rs::Result<::std::int32_t, ::rs::alloc::string::String>>(
        crubit::UnsafeRelocateTag{},
        ::std::move(*reinterpret_cast<
                    rs::Result<::std::int32_t, ::rs::alloc::string::String>*>(
            storage_)));
  }
}
inline bool
rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>::has_value()
    const noexcept {
  return tag() != UINT64_C(18446744073709551614);
}
inline void rs::Option<
    rs::Result<::std::int32_t, ::rs::alloc::string::String>>::check_has_value()
    const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs::Option";
}
inline rs::Result<::std::int32_t, ::rs::alloc::string::String>& rs::Option<
    rs::Result<::std::int32_t, ::rs::alloc::string::String>>::operator*() & {
  check_has_value();
  return *reinterpret_cast<
      rs::Result<::std::int32_t, ::rs::alloc::string::String>*>(storage_);
}
inline rs::Result<::std::int32_t, ::rs::alloc::string::String> const&
rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>::operator*()
    const& {
  check_has_value();
  return *reinterpret_cast<
      rs::Result<::std::int32_t, ::rs::alloc::string::String> const*>(storage_);
}
inline rs::Result<::std::int32_t, ::rs::alloc::string::String>&& rs::Option<
    rs::Result<::std::int32_t, ::rs::alloc::string::String>>::operator*() && {
  check_has_value();
  return ::std::move(
      *reinterpret_cast<
          rs::Result<::std::int32_t, ::rs::alloc::string::String>*>(storage_));
}
inline rs::Result<::std::int32_t, ::rs::alloc::string::String>* rs::Option<
    rs::Result<::std::int32_t, ::rs::alloc::string::String>>::operator->() {
  check_has_value();
  return reinterpret_cast<
      rs::Result<::std::int32_t, ::rs::alloc::string::String>*>(storage_);
}
inline rs::Result<::std::int32_t, ::rs::alloc::string::String> const*
rs::Option<rs::Result<::std::int32_t,
                      ::rs::alloc::string::String>>::operator->() const {
  check_has_value();
  return reinterpret_cast<
      rs::Result<::std::int32_t, ::rs::alloc::string::String> const*>(storage_);
}
inline constexpr ::std::uint64_t
rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void
rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e
inline rs::Option<rs::Result<::option::HasNoDefault,
                             ::rs::alloc::string::String>>::Option(Option&&
                                                                       other)
    : Option() {
  *this = ::std::move(other);
}
inline rs::Option<
    rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>&
rs::Option<rs::Result<::option::HasNoDefault,
                      ::rs::alloc::string::String>>::operator=(Option&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline constexpr rs::Option<
    rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::Option() {
  set_tag(UINT64_C(18446744073709551614));
}
inline constexpr rs::Option<
    rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
    Option(::std::nullopt_t) noexcept {
  set_tag(UINT64_C(18446744073709551614));
}
inline constexpr rs::Option<
    rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>&
rs::Option<rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
operator=(::std::nullopt_t) noexcept {
  if (tag() != UINT64_C(18446744073709551614)) {
    ::std::destroy_at(
        reinterpret_cast<
            rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>*>(
            storage_));
  }
  set_tag(UINT64_C(18446744073709551614));
  return *this;
}
inline rs::Option<
    rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
    Option(::std::optional<
           rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>&&
               value) noexcept {
  if (value.has_value()) {
    rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>* some =
        reinterpret_cast<
            rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>*>(
            storage_);
    ::std::construct_at(some, crubit::UnsafeRelocateTag{}, ::std::move(*value));
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(UINT64_C(18446744073709551614));
  }
}
inline rs::Option<
    rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>&
rs::Option<rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
operator=(::std::optional<
          rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>&&
              value) noexcept {
  if (tag() != UINT64_C(18446744073709551614)) {
    ::std::destroy_at(
        reinterpret_cast<
            rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>*>(
            storage_));
  }
  if (value.has_value()) {
    rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>* some =
        reinterpret_cast<
            rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>*>(
            storage_);
    ::std::construct_at(some, crubit::UnsafeRelocateTag{}, ::std::move(*value));
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(UINT64_C(18446744073709551614));
  }
  return *this;
}
template <typename... Args>
inline rs::Option<
    rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
    Option(::std::in_place_t, Args&&... args) noexcept {
  ::std::construct_at(
      reinterpret_cast<
          rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>*>(
          storage_),
      ::std::forward<Args>(args)...);
}
inline constexpr rs::Option<rs::Result<
    ::option::HasNoDefault, ::rs::alloc::string::String>>::~Option() noexcept {
  if (tag() != UINT64_C(18446744073709551614)) {
    ::std::destroy_at(
        reinterpret_cast<
            rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>*>(
            storage_));
  }
}
inline rs::Option<
    rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
operator ::std::optional<rs::Result<
    ::option::HasNoDefault, ::rs::alloc::string::String>>() && noexcept {
  if (tag() == UINT64_C(18446744073709551614)) {
    return ::std::nullopt;
  } else {
    struct DeferSetTagNone {
      rs::Option<rs::Result<::option::HasNoDefault,
                            ::rs::alloc::string::String>>* _value;
      DeferSetTagNone(rs::Option<rs::Result<::option::HasNoDefault,
                                            ::rs::alloc::string::String>>* self)
          : _value(self) {}
      ~DeferSetTagNone() { set_tag(UINT64_C(18446744073709551614)); }

     private:
      void set_tag(::std::uint64_t tag) { _value->set_tag(tag); }
    } defer(this);
    return ::std::make_optional<
        rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>(
        crubit::UnsafeRelocateTag{},
        ::std::move(*reinterpret_cast<rs::Result<::option::HasNoDefault,
                                                 ::rs::alloc::string::String>*>(
            storage_)));
  }
}
inline bool rs::Option<rs::Result<::option::HasNoDefault,
                                  ::rs::alloc::string::String>>::has_value()
    const noexcept {
  return tag() != UINT64_C(18446744073709551614);
}
inline void
rs::Option<rs::Result<::option::HasNoDefault,
                      ::rs::alloc::string::String>>::check_has_value() const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs::Option";
}
inline rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>&
rs::Option<rs::Result<::option::HasNoDefault,
                      ::rs::alloc::string::String>>::operator*() & {
  check_has_value();
  return *reinterpret_cast<
      rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>*>(
      storage_);
}
inline rs::Result<::option::HasNoDefault, ::rs::alloc::string::String> const&
rs::Option<rs::Result<::option::HasNoDefault,
                      ::rs::alloc::string::String>>::operator*() const& {
  check_has_value();
  return *reinterpret_cast<
      rs::Result<::option::HasNoDefault, ::rs::alloc::string::String> const*>(
      storage_);
}
inline rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>&&
rs::Option<rs::Result<::option::HasNoDefault,
                      ::rs::alloc::string::String>>::operator*() && {
  check_has_value();
  return ::std::move(
      *reinterpret_cast<
          rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>*>(
          storage_));
}
inline rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>*
rs::Option<rs::Result<::option::HasNoDefault,
                      ::rs::alloc::string::String>>::operator->() {
  check_has_value();
  return reinterpret_cast<
      rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>*>(
      storage_);
}
inline rs::Result<::option::HasNoDefault, ::rs::alloc::string::String> const*
rs::Option<rs::Result<::option::HasNoDefault,
                      ::rs::alloc::string::String>>::operator->() const {
  check_has_value();
  return reinterpret_cast<
      rs::Result<::option::HasNoDefault, ::rs::alloc::string::String> const*>(
      storage_);
}
inline constexpr ::std::uint64_t rs::Option<
    rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void
rs::Option<rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>>::
    set_tag(::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    rs::Option<rs::Result<
        rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        rs::Result<rs::Option<::std::int32_t>,
                   rs::Option<::std::int32_t>>>> const&,
    rs::Option<rs::Result<
        rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>*
        __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(
    rs::Option<rs::Result<
        rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>&,
    rs::Option<rs::Result<
        rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        rs::Result<rs::Option<::std::int32_t>,
                   rs::Option<::std::int32_t>>>> const&);
}
inline rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
    Option(const Option& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>&
rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
operator=(const Option& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
inline rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
    Option(Option&& other)
    : Option() {
  *this = ::std::move(other);
}
inline rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>&
rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
operator=(Option&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline constexpr rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
    Option() {
  set_tag(UINT64_C(18446744073709551612));
}
inline constexpr rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
    Option(::std::nullopt_t) noexcept {
  set_tag(UINT64_C(18446744073709551612));
}
inline constexpr rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>&
rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
operator=(::std::nullopt_t) noexcept {
  if (tag() != UINT64_C(18446744073709551612)) {
    ::std::destroy_at(
        reinterpret_cast<rs::Result<
            rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
            rs::Result<rs::Option<::std::int32_t>,
                       rs::Option<::std::int32_t>>>*>(storage_));
  }
  set_tag(UINT64_C(18446744073709551612));
  return *this;
}
inline rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
    Option(rs::Result<
           rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>&&
               value) noexcept {
  ::std::construct_at(
      reinterpret_cast<rs::Result<
          rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
          rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>*>(
          storage_),
      ::std::move(value));
}
inline rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>&
rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
operator=(rs::Result<
          rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
          rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>&&
              value) noexcept {
  if (tag() != UINT64_C(18446744073709551612)) {
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<rs::Result<
            rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
            rs::Result<rs::Option<::std::int32_t>,
                       rs::Option<::std::int32_t>>>*>(storage_),
        ::std::move(value));
  } else {
    ::std::construct_at(
        reinterpret_cast<rs::Result<
            rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
            rs::Result<rs::Option<::std::int32_t>,
                       rs::Option<::std::int32_t>>>*>(storage_),
        ::std::move(value));
  }
  return *this;
}
inline rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
    Option(
        ::std::optional<rs::Result<
            rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
            rs::Result<rs::Option<::std::int32_t>,
                       rs::Option<::std::int32_t>>>>&& value) noexcept {
  if (value.has_value()) {
    rs::Result<
        rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>*
        some = reinterpret_cast<rs::Result<
            rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
            rs::Result<rs::Option<::std::int32_t>,
                       rs::Option<::std::int32_t>>>*>(storage_);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(UINT64_C(18446744073709551612));
  }
}
inline rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>&
rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
operator=(
    ::std::optional<rs::Result<
        rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>&&
        value) noexcept {
  if (tag() != UINT64_C(18446744073709551612)) {
    ::std::destroy_at(
        reinterpret_cast<rs::Result<
            rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
            rs::Result<rs::Option<::std::int32_t>,
                       rs::Option<::std::int32_t>>>*>(storage_));
  }
  if (value.has_value()) {
    rs::Result<
        rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>*
        some = reinterpret_cast<rs::Result<
            rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
            rs::Result<rs::Option<::std::int32_t>,
                       rs::Option<::std::int32_t>>>*>(storage_);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(UINT64_C(18446744073709551612));
  }
  return *this;
}
template <typename... Args>
inline rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
    Option(::std::in_place_t, Args&&... args) noexcept {
  ::std::construct_at(
      reinterpret_cast<rs::Result<
          rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
          rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>*>(
          storage_),
      ::std::forward<Args>(args)...);
}
inline constexpr rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
    ~Option() noexcept {
  if (tag() != UINT64_C(18446744073709551612)) {
    ::std::destroy_at(
        reinterpret_cast<rs::Result<
            rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
            rs::Result<rs::Option<::std::int32_t>,
                       rs::Option<::std::int32_t>>>*>(storage_));
  }
}
inline rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
operator ::std::optional<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>,
               rs::Option<::std::int32_t>>>>() && noexcept {
  if (tag() == UINT64_C(18446744073709551612)) {
    return ::std::nullopt;
  } else {
    struct DeferSetTagNone {
      rs::Option<rs::Result<
          rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
          rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>*
          _value;
      DeferSetTagNone(
          rs::Option<
              rs::Result<rs::Option<rs::Result<::std::int32_t,
                                               ::rs::alloc::string::String>>,
                         rs::Result<rs::Option<::std::int32_t>,
                                    rs::Option<::std::int32_t>>>>* self)
          : _value(self) {}
      ~DeferSetTagNone() { set_tag(UINT64_C(18446744073709551612)); }

     private:
      void set_tag(::std::uint64_t tag) { _value->set_tag(tag); }
    } defer(this);
    return ::std::make_optional<rs::Result<
        rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>(
        crubit::UnsafeRelocateTag{},
        ::std::move(
            *reinterpret_cast<rs::Result<
                rs::Option<
                    rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
                rs::Result<rs::Option<::std::int32_t>,
                           rs::Option<::std::int32_t>>>*>(storage_)));
  }
}
inline bool rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
    has_value() const noexcept {
  return tag() != UINT64_C(18446744073709551612);
}
inline void rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
    check_has_value() const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs::Option";
}
inline rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>&
rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
operator*() & {
  check_has_value();
  return *reinterpret_cast<rs::Result<
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>*>(
      storage_);
}
inline rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>> const&
rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
operator*() const& {
  check_has_value();
  return *reinterpret_cast<rs::Result<
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      rs::Result<rs::Option<::std::int32_t>,
                 rs::Option<::std::int32_t>>> const*>(storage_);
}
inline rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>&&
rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
operator*() && {
  check_has_value();
  return ::std::move(
      *reinterpret_cast<rs::Result<
          rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
          rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>*>(
          storage_));
}
inline rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>*
rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
operator->() {
  check_has_value();
  return reinterpret_cast<rs::Result<
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>*>(
      storage_);
}
inline rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>> const*
rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
operator->() const {
  check_has_value();
  return reinterpret_cast<rs::Result<
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
      rs::Result<rs::Option<::std::int32_t>,
                 rs::Option<::std::int32_t>>> const*>(storage_);
}
inline constexpr ::std::uint64_t rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void rs::Option<rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>>::
    set_tag(::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<rs::Option<::std::uint32_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<rs::Option<::std::uint32_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<rs::Option<::std::uint32_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<rs::Option<::std::uint32_t>>);
inline constexpr rs::Option<::std::uint32_t>::Option() { set_tag(0); }
inline constexpr rs::Option<::std::uint32_t>::Option(
    ::std::nullopt_t) noexcept {
  set_tag(0);
}
inline constexpr rs::Option<::std::uint32_t>&
rs::Option<::std::uint32_t>::operator=(::std::nullopt_t) noexcept {
  if (tag() != 0) {
    ::std::destroy_at(reinterpret_cast<::std::uint32_t*>(storage_ + 4));
  }
  set_tag(0);
  return *this;
}
inline rs::Option<::std::uint32_t>::Option(::std::uint32_t&& value) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::std::uint32_t*>(storage_ + 4),
                      ::std::move(value));
}
inline rs::Option<::std::uint32_t>& rs::Option<::std::uint32_t>::operator=(
    ::std::uint32_t&& value) noexcept {
  if (tag() != 0) {
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::uint32_t*>(storage_ + 4), ::std::move(value));
  } else {
    set_tag(1);
    ::std::construct_at(reinterpret_cast<::std::uint32_t*>(storage_ + 4),
                        ::std::move(value));
  }
  return *this;
}
inline rs::Option<::std::uint32_t>::Option(
    ::std::optional<::std::uint32_t>&& value) noexcept {
  if (value.has_value()) {
    set_tag(1);
    ::std::uint32_t* some = reinterpret_cast<::std::uint32_t*>(storage_ + 4);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(0);
  }
}
inline rs::Option<::std::uint32_t>& rs::Option<::std::uint32_t>::operator=(
    ::std::optional<::std::uint32_t>&& value) noexcept {
  if (tag() != 0) {
    ::std::destroy_at(reinterpret_cast<::std::uint32_t*>(storage_ + 4));
  }
  if (value.has_value()) {
    set_tag(1);
    ::std::uint32_t* some = reinterpret_cast<::std::uint32_t*>(storage_ + 4);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(0);
  }
  return *this;
}
template <typename... Args>
inline rs::Option<::std::uint32_t>::Option(::std::in_place_t,
                                           Args&&... args) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::std::uint32_t*>(storage_ + 4),
                      ::std::forward<Args>(args)...);
}
static_assert(::std::is_trivially_destructible_v<rs::Option<::std::uint32_t>>);
inline rs::Option<::std::uint32_t>::operator ::std::optional<
    ::std::uint32_t>() && noexcept {
  if (tag() == 0) {
    return ::std::nullopt;
  } else {
    ::std::uint32_t& value = *reinterpret_cast<::std::uint32_t*>(storage_ + 4);
    ::std::optional<::std::uint32_t> return_value(::std::move(value));
    ::std::destroy_at(&value);
    set_tag(0);
    return return_value;
  }
}
inline bool rs::Option<::std::uint32_t>::has_value() const noexcept {
  return tag() != 0;
}
inline void rs::Option<::std::uint32_t>::check_has_value() const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs::Option";
}
inline ::std::uint32_t& rs::Option<::std::uint32_t>::operator*() & {
  check_has_value();
  return *reinterpret_cast<::std::uint32_t*>(storage_ + 4);
}
inline ::std::uint32_t const& rs::Option<::std::uint32_t>::operator*() const& {
  check_has_value();
  return *reinterpret_cast<::std::uint32_t const*>(storage_ + 4);
}
inline ::std::uint32_t&& rs::Option<::std::uint32_t>::operator*() && {
  check_has_value();
  return ::std::move(*reinterpret_cast<::std::uint32_t*>(storage_ + 4));
}
inline ::std::uint32_t* rs::Option<::std::uint32_t>::operator->() {
  check_has_value();
  return reinterpret_cast<::std::uint32_t*>(storage_ + 4);
}
inline ::std::uint32_t const* rs::Option<::std::uint32_t>::operator->() const {
  check_has_value();
  return reinterpret_cast<::std::uint32_t const*>(storage_ + 4);
}
inline constexpr ::std::uint32_t rs::Option<::std::uint32_t>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void rs::Option<::std::uint32_t>::set_tag(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<rs::Option<::std::uint8_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<rs::Option<::std::uint8_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<rs::Option<::std::uint8_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<rs::Option<::std::uint8_t>>);
inline constexpr rs::Option<::std::uint8_t>::Option() { set_tag(0); }
inline constexpr rs::Option<::std::uint8_t>::Option(::std::nullopt_t) noexcept {
  set_tag(0);
}
inline constexpr rs::Option<::std::uint8_t>&
rs::Option<::std::uint8_t>::operator=(::std::nullopt_t) noexcept {
  if (tag() != 0) {
    ::std::destroy_at(reinterpret_cast<::std::uint8_t*>(storage_ + 1));
  }
  set_tag(0);
  return *this;
}
inline rs::Option<::std::uint8_t>::Option(::std::uint8_t&& value) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::std::uint8_t*>(storage_ + 1),
                      ::std::move(value));
}
inline rs::Option<::std::uint8_t>& rs::Option<::std::uint8_t>::operator=(
    ::std::uint8_t&& value) noexcept {
  if (tag() != 0) {
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::uint8_t*>(storage_ + 1), ::std::move(value));
  } else {
    set_tag(1);
    ::std::construct_at(reinterpret_cast<::std::uint8_t*>(storage_ + 1),
                        ::std::move(value));
  }
  return *this;
}
inline rs::Option<::std::uint8_t>::Option(
    ::std::optional<::std::uint8_t>&& value) noexcept {
  if (value.has_value()) {
    set_tag(1);
    ::std::uint8_t* some = reinterpret_cast<::std::uint8_t*>(storage_ + 1);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(0);
  }
}
inline rs::Option<::std::uint8_t>& rs::Option<::std::uint8_t>::operator=(
    ::std::optional<::std::uint8_t>&& value) noexcept {
  if (tag() != 0) {
    ::std::destroy_at(reinterpret_cast<::std::uint8_t*>(storage_ + 1));
  }
  if (value.has_value()) {
    set_tag(1);
    ::std::uint8_t* some = reinterpret_cast<::std::uint8_t*>(storage_ + 1);
    *some = ::std::move(value.value());
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(0);
  }
  return *this;
}
template <typename... Args>
inline rs::Option<::std::uint8_t>::Option(::std::in_place_t,
                                          Args&&... args) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::std::uint8_t*>(storage_ + 1),
                      ::std::forward<Args>(args)...);
}
static_assert(::std::is_trivially_destructible_v<rs::Option<::std::uint8_t>>);
inline rs::Option<::std::uint8_t>::operator ::std::optional<
    ::std::uint8_t>() && noexcept {
  if (tag() == 0) {
    return ::std::nullopt;
  } else {
    ::std::uint8_t& value = *reinterpret_cast<::std::uint8_t*>(storage_ + 1);
    ::std::optional<::std::uint8_t> return_value(::std::move(value));
    ::std::destroy_at(&value);
    set_tag(0);
    return return_value;
  }
}
inline bool rs::Option<::std::uint8_t>::has_value() const noexcept {
  return tag() != 0;
}
inline void rs::Option<::std::uint8_t>::check_has_value() const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs::Option";
}
inline ::std::uint8_t& rs::Option<::std::uint8_t>::operator*() & {
  check_has_value();
  return *reinterpret_cast<::std::uint8_t*>(storage_ + 1);
}
inline ::std::uint8_t const& rs::Option<::std::uint8_t>::operator*() const& {
  check_has_value();
  return *reinterpret_cast<::std::uint8_t const*>(storage_ + 1);
}
inline ::std::uint8_t&& rs::Option<::std::uint8_t>::operator*() && {
  check_has_value();
  return ::std::move(*reinterpret_cast<::std::uint8_t*>(storage_ + 1));
}
inline ::std::uint8_t* rs::Option<::std::uint8_t>::operator->() {
  check_has_value();
  return reinterpret_cast<::std::uint8_t*>(storage_ + 1);
}
inline ::std::uint8_t const* rs::Option<::std::uint8_t>::operator->() const {
  check_has_value();
  return reinterpret_cast<::std::uint8_t const*>(storage_ + 1);
}
inline constexpr ::std::uint8_t rs::Option<::std::uint8_t>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs::Option<::std::uint8_t>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs::Option<crubit::type_identity_t<void(void*, void*)>*>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs::Option<crubit::type_identity_t<void(void*, void*)>*>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs::Option<crubit::type_identity_t<void(void*, void*)>*>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs::Option<crubit::type_identity_t<void(void*, void*)>*>>);
inline constexpr rs::Option<
    crubit::type_identity_t<void(void*, void*)>*>::Option() {
  set_tag(0);
}
inline constexpr rs::Option<crubit::type_identity_t<void(void*, void*)>*>::
    Option(::std::nullopt_t) noexcept {
  set_tag(0);
}
inline constexpr rs::Option<crubit::type_identity_t<void(void*, void*)>*>&
rs::Option<crubit::type_identity_t<void(void*, void*)>*>::operator=(
    ::std::nullopt_t) noexcept {
  if (tag() != 0) {
    ::std::destroy_at(
        reinterpret_cast<crubit::type_identity_t<void(void*, void*)>**>(
            storage_));
  }
  set_tag(0);
  return *this;
}
inline rs::Option<crubit::type_identity_t<void(void*, void*)>*>::Option(
    ::std::optional<crubit::type_identity_t<void(void*, void*)>*>&&
        value) noexcept {
  if (value.has_value()) {
    crubit::type_identity_t<void(void*, void*)>** some =
        reinterpret_cast<crubit::type_identity_t<void(void*, void*)>**>(
            storage_);
    *some = value.value();
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(0);
  }
}
inline rs::Option<crubit::type_identity_t<void(void*, void*)>*>&
rs::Option<crubit::type_identity_t<void(void*, void*)>*>::operator=(
    ::std::optional<crubit::type_identity_t<void(void*, void*)>*>&&
        value) noexcept {
  if (tag() != 0) {
    ::std::destroy_at(
        reinterpret_cast<crubit::type_identity_t<void(void*, void*)>**>(
            storage_));
  }
  if (value.has_value()) {
    crubit::type_identity_t<void(void*, void*)>** some =
        reinterpret_cast<crubit::type_identity_t<void(void*, void*)>**>(
            storage_);
    *some = value.value();
    ::std::construct_at(&value, ::std::nullopt);
  } else {
    set_tag(0);
  }
  return *this;
}
template <typename... Args>
inline rs::Option<crubit::type_identity_t<void(void*, void*)>*>::Option(
    ::std::in_place_t, Args&&... args) noexcept {
  ::std::construct_at(
      reinterpret_cast<crubit::type_identity_t<void(void*, void*)>**>(storage_),
      ::std::forward<Args>(args)...);
}
static_assert(::std::is_trivially_destructible_v<
              rs::Option<crubit::type_identity_t<void(void*, void*)>*>>);
inline rs::Option<crubit::type_identity_t<void(void*, void*)>*>::operator ::
    std::optional<crubit::type_identity_t<void(void*, void*)>*>() && noexcept {
  if (tag() == 0) {
    return ::std::nullopt;
  } else {
    crubit::type_identity_t<void(void*, void*)>*& value =
        *reinterpret_cast<crubit::type_identity_t<void(void*, void*)>**>(
            storage_);
    ::std::optional<crubit::type_identity_t<void(void*, void*)>*> return_value(
        ::std::move(value));
    ::std::destroy_at(&value);
    set_tag(0);
    return return_value;
  }
}
inline bool rs::Option<
    crubit::type_identity_t<void(void*, void*)>*>::has_value() const noexcept {
  return tag() != 0;
}
inline void rs::Option<
    crubit::type_identity_t<void(void*, void*)>*>::check_has_value() const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs::Option";
}
inline crubit::type_identity_t<void(void*, void*)>*&
rs::Option<crubit::type_identity_t<void(void*, void*)>*>::operator*() & {
  check_has_value();
  return *reinterpret_cast<crubit::type_identity_t<void(void*, void*)>**>(
      storage_);
}
inline crubit::type_identity_t<void(void*, void*)>* const&
rs::Option<crubit::type_identity_t<void(void*, void*)>*>::operator*() const& {
  check_has_value();
  return *reinterpret_cast<crubit::type_identity_t<void(void*, void*)>* const*>(
      storage_);
}
inline crubit::type_identity_t<void(void*, void*)>*&&
rs::Option<crubit::type_identity_t<void(void*, void*)>*>::operator*() && {
  check_has_value();
  return ::std::move(
      *reinterpret_cast<crubit::type_identity_t<void(void*, void*)>**>(
          storage_));
}
inline crubit::type_identity_t<void(void*, void*)>**
rs::Option<crubit::type_identity_t<void(void*, void*)>*>::operator->() {
  check_has_value();
  return reinterpret_cast<crubit::type_identity_t<void(void*, void*)>**>(
      storage_);
}
inline crubit::type_identity_t<void(void*, void*)>* const*
rs::Option<crubit::type_identity_t<void(void*, void*)>*>::operator->() const {
  check_has_value();
  return reinterpret_cast<crubit::type_identity_t<void(void*, void*)>* const*>(
      storage_);
}
inline constexpr ::std::uint64_t rs::Option<
    crubit::type_identity_t<void(void*, void*)>*>::tag() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void
rs::Option<crubit::type_identity_t<void(void*, void*)>*>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    rs::Result<::std::int32_t, ::rs::alloc::string::String> const&,
    rs::Result<::std::int32_t, ::rs::alloc::string::String>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(
    rs::Result<::std::int32_t, ::rs::alloc::string::String>&,
    rs::Result<::std::int32_t, ::rs::alloc::string::String> const&);
}
inline rs::Result<::std::int32_t, ::rs::alloc::string::String>::Result(
    const Result& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline rs::Result<::std::int32_t, ::rs::alloc::string::String>&
rs::Result<::std::int32_t, ::rs::alloc::string::String>::operator=(
    const Result& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
inline rs::Result<::std::int32_t, ::rs::alloc::string::String>::Result(
    ::std::int32_t&& ok) noexcept {
  set_tag(UINT64_C(18446744073709551615));
  ::std::construct_at(reinterpret_cast<::std::int32_t*>(__storage + 8),
                      ::std::move(ok));
}
inline rs::Result<::std::int32_t, ::rs::alloc::string::String>&
rs::Result<::std::int32_t, ::rs::alloc::string::String>::operator=(
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

inline rs::Result<::std::int32_t, ::rs::alloc::string::String>::Result(
    rs::unexpected<::rs::alloc::string::String>&& err) noexcept {
  ::std::construct_at(reinterpret_cast<::rs::alloc::string::String*>(__storage),
                      ::std::move(err.error()));
}
inline rs::Result<::std::int32_t, ::rs::alloc::string::String>&
rs::Result<::std::int32_t, ::rs::alloc::string::String>::operator=(
    rs::unexpected<::rs::alloc::string::String>&& err) noexcept {
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
inline rs::Result<::std::int32_t, ::rs::alloc::string::String>::Result(
    ::std::in_place_t, Args&&... args) {
  set_tag(UINT64_C(18446744073709551615));
  ::std::construct_at(__storage + 8, ::std::forward<Args>(args)...);
}
template <typename... Args>
inline rs::Result<::std::int32_t, ::rs::alloc::string::String>::Result(
    rs::unexpect_t, Args&&... args) {
  ::std::construct_at(__storage, ::std::forward<Args>(args)...);
}
inline constexpr rs::Result<::std::int32_t, ::rs::alloc::string::String>::
operator bool() const noexcept {
  return has_value();
}
inline constexpr bool rs::Result<
    ::std::int32_t, ::rs::alloc::string::String>::has_value() const noexcept {
  return tag() == UINT64_C(18446744073709551615);
}
inline ::std::int32_t&
rs::Result<::std::int32_t, ::rs::alloc::string::String>::value() & {
  check_has_ok();
  return *reinterpret_cast<::std::int32_t*>(__storage + 8);
}
inline ::std::int32_t&&
rs::Result<::std::int32_t, ::rs::alloc::string::String>::value() && {
  check_has_ok();
  return ::std::move(*reinterpret_cast<::std::int32_t*>(__storage + 8));
}
inline ::rs::alloc::string::String&
rs::Result<::std::int32_t, ::rs::alloc::string::String>::err() & {
  check_has_err();
  return *reinterpret_cast<::rs::alloc::string::String*>(__storage);
}
inline ::rs::alloc::string::String&&
rs::Result<::std::int32_t, ::rs::alloc::string::String>::err() && {
  check_has_err();
  return ::std::move(
      *reinterpret_cast<::rs::alloc::string::String*>(__storage));
}
inline ::std::int32_t&
rs::Result<::std::int32_t, ::rs::alloc::string::String>::operator*() & {
  check_has_ok();
  return *reinterpret_cast<::std::int32_t*>(__storage + 8);
}
inline ::std::int32_t const&
rs::Result<::std::int32_t, ::rs::alloc::string::String>::operator*() const& {
  check_has_ok();
  return *reinterpret_cast<::std::int32_t const*>(__storage + 8);
}
inline ::std::int32_t&&
rs::Result<::std::int32_t, ::rs::alloc::string::String>::operator*() && {
  check_has_ok();
  return ::std::move(*reinterpret_cast<::std::int32_t*>(__storage + 8));
}
inline ::std::int32_t*
rs::Result<::std::int32_t, ::rs::alloc::string::String>::operator->() {
  check_has_ok();
  return reinterpret_cast<::std::int32_t*>(__storage + 8);
}
inline ::std::int32_t const*
rs::Result<::std::int32_t, ::rs::alloc::string::String>::operator->() const {
  check_has_ok();
  return reinterpret_cast<::std::int32_t const*>(__storage + 8);
}
inline rs::Result<::std::int32_t,
                  ::rs::alloc::string::String>::~Result() noexcept {
  if (has_value()) {
    ::std::destroy_at(reinterpret_cast<::std::int32_t*>(__storage + 8));
  } else {
    ::std::destroy_at(
        reinterpret_cast<::rs::alloc::string::String*>(__storage));
  }
}
inline constexpr ::std::uint64_t
rs::Result<::std::int32_t, ::rs::alloc::string::String>::tag() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void
rs::Result<::std::int32_t, ::rs::alloc::string::String>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

inline void
rs::Result<::std::int32_t, ::rs::alloc::string::String>::check_has_ok() const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs::Result";
}
inline void
rs::Result<::std::int32_t, ::rs::alloc::string::String>::check_has_err() const {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs::Result";
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e

inline rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>::Result(
    rs::unexpected<::rs::alloc::string::String>&& err) noexcept {
  set_tag(UINT64_C(18446744073709551615));
  ::std::construct_at(
      reinterpret_cast<::rs::alloc::string::String*>(__storage + 8),
      ::std::move(err.error()));
}
inline rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>&
rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>::operator=(
    rs::unexpected<::rs::alloc::string::String>&& err) noexcept {
  if (has_value()) {
    ::std::destroy_at(__storage);
    set_tag(UINT64_C(18446744073709551615));
    ::std::construct_at(
        reinterpret_cast<::rs::alloc::string::String*>(__storage + 8),
        ::std::move(err.error()));
  } else {
    set_tag(UINT64_C(18446744073709551615));
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::rs::alloc::string::String*>(__storage + 8),
        ::std::move(err.error()));
  }
  return *this;
}

template <typename... Args>
inline rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>::Result(
    ::std::in_place_t, Args&&... args) {
  ::std::construct_at(__storage, ::std::forward<Args>(args)...);
}
template <typename... Args>
inline rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>::Result(
    rs::unexpect_t, Args&&... args) {
  set_tag(UINT64_C(18446744073709551615));
  ::std::construct_at(__storage + 8, ::std::forward<Args>(args)...);
}
inline constexpr rs::Result<::option::HasNoDefault,
                            ::rs::alloc::string::String>::
operator bool() const noexcept {
  return has_value();
}
inline constexpr bool
rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>::has_value()
    const noexcept {
  return tag() != UINT64_C(18446744073709551615);
}
inline ::option::HasNoDefault&
rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>::value() & {
  check_has_ok();
  return *reinterpret_cast<::option::HasNoDefault*>(__storage);
}
inline ::option::HasNoDefault&&
rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>::value() && {
  check_has_ok();
  return ::std::move(*reinterpret_cast<::option::HasNoDefault*>(__storage));
}
inline ::rs::alloc::string::String&
rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>::err() & {
  check_has_err();
  return *reinterpret_cast<::rs::alloc::string::String*>(__storage + 8);
}
inline ::rs::alloc::string::String&&
rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>::err() && {
  check_has_err();
  return ::std::move(
      *reinterpret_cast<::rs::alloc::string::String*>(__storage + 8));
}
inline ::option::HasNoDefault&
rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>::operator*() & {
  check_has_ok();
  return *reinterpret_cast<::option::HasNoDefault*>(__storage);
}
inline ::option::HasNoDefault const& rs::Result<
    ::option::HasNoDefault, ::rs::alloc::string::String>::operator*() const& {
  check_has_ok();
  return *reinterpret_cast<::option::HasNoDefault const*>(__storage);
}
inline ::option::HasNoDefault&& rs::Result<
    ::option::HasNoDefault, ::rs::alloc::string::String>::operator*() && {
  check_has_ok();
  return ::std::move(*reinterpret_cast<::option::HasNoDefault*>(__storage));
}
inline ::option::HasNoDefault*
rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>::operator->() {
  check_has_ok();
  return reinterpret_cast<::option::HasNoDefault*>(__storage);
}
inline ::option::HasNoDefault const* rs::Result<
    ::option::HasNoDefault, ::rs::alloc::string::String>::operator->() const {
  check_has_ok();
  return reinterpret_cast<::option::HasNoDefault const*>(__storage);
}
inline rs::Result<::option::HasNoDefault,
                  ::rs::alloc::string::String>::~Result() noexcept {
  if (has_value()) {
    ::std::destroy_at(reinterpret_cast<::option::HasNoDefault*>(__storage));
  } else {
    ::std::destroy_at(
        reinterpret_cast<::rs::alloc::string::String*>(__storage + 8));
  }
}
inline constexpr ::std::uint64_t
rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void
rs::Result<::option::HasNoDefault, ::rs::alloc::string::String>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

inline void rs::Result<::option::HasNoDefault,
                       ::rs::alloc::string::String>::check_has_ok() const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs::Result";
}
inline void rs::Result<::option::HasNoDefault,
                       ::rs::alloc::string::String>::check_has_err() const {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs::Result";
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<
        rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<
        rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>);
static_assert(
    ::std::is_trivially_move_constructible_v<
        rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>);
static_assert(
    ::std::is_trivially_move_assignable_v<
        rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>);
inline rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>::
    Result(rs::Option<::std::int32_t>&& ok) noexcept {
  set_tag(0);
  ::std::construct_at(
      reinterpret_cast<rs::Option<::std::int32_t>*>(__storage + 4),
      ::std::move(ok));
}
inline rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>&
rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>::operator=(
    rs::Option<::std::int32_t>&& ok) noexcept {
  if (!has_value()) {
    ::std::destroy_at(
        reinterpret_cast<rs::Option<::std::int32_t>*>(__storage + 4));
    set_tag(0);
    ::std::construct_at(
        reinterpret_cast<rs::Option<::std::int32_t>*>(__storage + 4),
        ::std::move(ok));
  } else {
    set_tag(0);
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<rs::Option<::std::int32_t>*>(__storage + 4),
        ::std::move(ok));
  }
  return *this;
}

inline rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>::
    Result(rs::unexpected<rs::Option<::std::int32_t>>&& err) noexcept {
  set_tag(1);
  ::std::construct_at(
      reinterpret_cast<rs::Option<::std::int32_t>*>(__storage + 4),
      ::std::move(err.error()));
}
inline rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>&
rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>::operator=(
    rs::unexpected<rs::Option<::std::int32_t>>&& err) noexcept {
  if (has_value()) {
    ::std::destroy_at(__storage + 4);
    set_tag(1);
    ::std::construct_at(
        reinterpret_cast<rs::Option<::std::int32_t>*>(__storage + 4),
        ::std::move(err.error()));
  } else {
    set_tag(1);
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<rs::Option<::std::int32_t>*>(__storage + 4),
        ::std::move(err.error()));
  }
  return *this;
}

template <typename... Args>
inline rs::Result<rs::Option<::std::int32_t>,
                  rs::Option<::std::int32_t>>::Result(::std::in_place_t,
                                                      Args&&... args) {
  set_tag(0);
  ::std::construct_at(__storage + 4, ::std::forward<Args>(args)...);
}
template <typename... Args>
inline rs::Result<rs::Option<::std::int32_t>,
                  rs::Option<::std::int32_t>>::Result(rs::unexpect_t,
                                                      Args&&... args) {
  set_tag(1);
  ::std::construct_at(__storage + 4, ::std::forward<Args>(args)...);
}
inline constexpr rs::Result<rs::Option<::std::int32_t>,
                            rs::Option<::std::int32_t>>::
operator bool() const noexcept {
  return has_value();
}
inline constexpr bool
rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>::has_value()
    const noexcept {
  return tag() == 0;
}
inline rs::Option<::std::int32_t>&
rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>::value() & {
  check_has_ok();
  return *reinterpret_cast<rs::Option<::std::int32_t>*>(__storage + 4);
}
inline rs::Option<::std::int32_t>&&
rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>::value() && {
  check_has_ok();
  return ::std::move(
      *reinterpret_cast<rs::Option<::std::int32_t>*>(__storage + 4));
}
inline rs::Option<::std::int32_t>&
rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>::err() & {
  check_has_err();
  return *reinterpret_cast<rs::Option<::std::int32_t>*>(__storage + 4);
}
inline rs::Option<::std::int32_t>&&
rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>::err() && {
  check_has_err();
  return ::std::move(
      *reinterpret_cast<rs::Option<::std::int32_t>*>(__storage + 4));
}
inline rs::Option<::std::int32_t>& rs::Result<
    rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>::operator*() & {
  check_has_ok();
  return *reinterpret_cast<rs::Option<::std::int32_t>*>(__storage + 4);
}
inline rs::Option<::std::int32_t> const&
rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>::operator*()
    const& {
  check_has_ok();
  return *reinterpret_cast<rs::Option<::std::int32_t> const*>(__storage + 4);
}
inline rs::Option<::std::int32_t>&& rs::Result<
    rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>::operator*() && {
  check_has_ok();
  return ::std::move(
      *reinterpret_cast<rs::Option<::std::int32_t>*>(__storage + 4));
}
inline rs::Option<::std::int32_t>* rs::Result<
    rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>::operator->() {
  check_has_ok();
  return reinterpret_cast<rs::Option<::std::int32_t>*>(__storage + 4);
}
inline rs::Option<::std::int32_t> const*
rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>::operator->()
    const {
  check_has_ok();
  return reinterpret_cast<rs::Option<::std::int32_t> const*>(__storage + 4);
}
static_assert(
    ::std::is_trivially_destructible_v<
        rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>);
inline constexpr ::std::uint32_t
rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void
rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>::set_tag(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

inline void rs::Result<rs::Option<::std::int32_t>,
                       rs::Option<::std::int32_t>>::check_has_ok() const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs::Result";
}
inline void rs::Result<rs::Option<::std::int32_t>,
                       rs::Option<::std::int32_t>>::check_has_err() const {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs::Result";
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    rs::Result<
        rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        rs::Result<rs::Option<::std::int32_t>,
                   rs::Option<::std::int32_t>>> const&,
    rs::Result<
        rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>*
        __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(
    rs::Result<
        rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>&,
    rs::Result<
        rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
        rs::Result<rs::Option<::std::int32_t>,
                   rs::Option<::std::int32_t>>> const&);
}
inline rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>,
               rs::Option<::std::int32_t>>>::Result(const Result& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>&
rs::Result<rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>::
operator=(const Result& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
inline rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>::
    Result(rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&&
               ok) noexcept {
  ::std::construct_at(
      reinterpret_cast<
          rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>*>(
          __storage),
      ::std::move(ok));
}
inline rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>&
rs::Result<rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>::
operator=(rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&&
              ok) noexcept {
  if (!has_value()) {
    ::std::destroy_at(reinterpret_cast<rs::Result<rs::Option<::std::int32_t>,
                                                  rs::Option<::std::int32_t>>*>(
        __storage + 8));
    ::std::construct_at(
        reinterpret_cast<rs::Option<
            rs::Result<::std::int32_t, ::rs::alloc::string::String>>*>(
            __storage),
        ::std::move(ok));
  } else {
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<rs::Option<
            rs::Result<::std::int32_t, ::rs::alloc::string::String>>*>(
            __storage),
        ::std::move(ok));
  }
  return *this;
}

inline rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>::
    Result(rs::unexpected<
           rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>&&
               err) noexcept {
  set_tag(UINT64_C(18446744073709551613));
  ::std::construct_at(
      reinterpret_cast<
          rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>*>(
          __storage + 8),
      ::std::move(err.error()));
}
inline rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>&
rs::Result<rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>::
operator=(rs::unexpected<
          rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>&&
              err) noexcept {
  if (has_value()) {
    ::std::destroy_at(__storage);
    set_tag(UINT64_C(18446744073709551613));
    ::std::construct_at(
        reinterpret_cast<rs::Result<rs::Option<::std::int32_t>,
                                    rs::Option<::std::int32_t>>*>(__storage +
                                                                  8),
        ::std::move(err.error()));
  } else {
    set_tag(UINT64_C(18446744073709551613));
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<rs::Result<rs::Option<::std::int32_t>,
                                    rs::Option<::std::int32_t>>*>(__storage +
                                                                  8),
        ::std::move(err.error()));
  }
  return *this;
}

template <typename... Args>
inline rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>::
    Result(::std::in_place_t, Args&&... args) {
  ::std::construct_at(__storage, ::std::forward<Args>(args)...);
}
template <typename... Args>
inline rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>::
    Result(rs::unexpect_t, Args&&... args) {
  set_tag(UINT64_C(18446744073709551613));
  ::std::construct_at(__storage + 8, ::std::forward<Args>(args)...);
}
inline constexpr rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>::
operator bool() const noexcept {
  return has_value();
}
inline constexpr bool
rs::Result<rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs::Result<rs::Option<::std::int32_t>,
                      rs::Option<::std::int32_t>>>::has_value() const noexcept {
  return tag() != UINT64_C(18446744073709551613);
}
inline rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&
rs::Result<rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs::Result<rs::Option<::std::int32_t>,
                      rs::Option<::std::int32_t>>>::value() & {
  check_has_ok();
  return *reinterpret_cast<
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>*>(
      __storage);
}
inline rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&&
rs::Result<rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs::Result<rs::Option<::std::int32_t>,
                      rs::Option<::std::int32_t>>>::value() && {
  check_has_ok();
  return ::std::move(
      *reinterpret_cast<
          rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>*>(
          __storage));
}
inline rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>&
rs::Result<rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs::Result<rs::Option<::std::int32_t>,
                      rs::Option<::std::int32_t>>>::err() & {
  check_has_err();
  return *reinterpret_cast<
      rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>*>(
      __storage + 8);
}
inline rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>&&
rs::Result<rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs::Result<rs::Option<::std::int32_t>,
                      rs::Option<::std::int32_t>>>::err() && {
  check_has_err();
  return ::std::move(
      *reinterpret_cast<
          rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>*>(
          __storage + 8));
}
inline rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&
rs::Result<rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs::Result<rs::Option<::std::int32_t>,
                      rs::Option<::std::int32_t>>>::operator*() & {
  check_has_ok();
  return *reinterpret_cast<
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>*>(
      __storage);
}
inline rs::Option<
    rs::Result<::std::int32_t, ::rs::alloc::string::String>> const&
rs::Result<rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs::Result<rs::Option<::std::int32_t>,
                      rs::Option<::std::int32_t>>>::operator*() const& {
  check_has_ok();
  return *reinterpret_cast<rs::Option<
      rs::Result<::std::int32_t, ::rs::alloc::string::String>> const*>(
      __storage);
}
inline rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>&&
rs::Result<rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs::Result<rs::Option<::std::int32_t>,
                      rs::Option<::std::int32_t>>>::operator*() && {
  check_has_ok();
  return ::std::move(
      *reinterpret_cast<
          rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>*>(
          __storage));
}
inline rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>*
rs::Result<rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs::Result<rs::Option<::std::int32_t>,
                      rs::Option<::std::int32_t>>>::operator->() {
  check_has_ok();
  return reinterpret_cast<
      rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>*>(
      __storage);
}
inline rs::Option<
    rs::Result<::std::int32_t, ::rs::alloc::string::String>> const*
rs::Result<rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs::Result<rs::Option<::std::int32_t>,
                      rs::Option<::std::int32_t>>>::operator->() const {
  check_has_ok();
  return reinterpret_cast<rs::Option<
      rs::Result<::std::int32_t, ::rs::alloc::string::String>> const*>(
      __storage);
}
inline rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>,
               rs::Option<::std::int32_t>>>::~Result() noexcept {
  if (has_value()) {
    ::std::destroy_at(
        reinterpret_cast<rs::Option<
            rs::Result<::std::int32_t, ::rs::alloc::string::String>>*>(
            __storage));
  } else {
    ::std::destroy_at(reinterpret_cast<rs::Result<rs::Option<::std::int32_t>,
                                                  rs::Option<::std::int32_t>>*>(
        __storage + 8));
  }
}
inline constexpr ::std::uint64_t rs::Result<
    rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
    rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void
rs::Result<rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs::Result<rs::Option<::std::int32_t>, rs::Option<::std::int32_t>>>::
    set_tag(::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

inline void
rs::Result<rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs::Result<rs::Option<::std::int32_t>,
                      rs::Option<::std::int32_t>>>::check_has_ok() const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs::Result";
}
inline void
rs::Result<rs::Option<rs::Result<::std::int32_t, ::rs::alloc::string::String>>,
           rs::Result<rs::Option<::std::int32_t>,
                      rs::Option<::std::int32_t>>>::check_has_err() const {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs::Result";
}

#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_OPTION_GOLDEN
