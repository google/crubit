// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// option_golden
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector,
// supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_OPTION_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_OPTION_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#include "support/annotations_internal.h"
#include "support/bridge.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/option.h"
#include "support/rs_std/str_ref.h"

#include <cstddef>
#include <cstdint>
#include <optional>
#include <type_traits>
#include <utility>

namespace option {
struct HasOptions;
// Generated from:
// cc_bindings_from_rs/test/enums/option.rs;l=117
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: CloneNoDefault") alignas(
    1) [[clang::trivial_abi]] CloneNoDefault final {
 public:
  // `option_golden::CloneNoDefault` doesn't implement the `Default` trait
  CloneNoDefault() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~CloneNoDefault() = default;
  CloneNoDefault(CloneNoDefault&&) = default;
  ::option::CloneNoDefault& operator=(CloneNoDefault&&) = default;

  // Clone::clone
  CloneNoDefault(const CloneNoDefault&);

  // Clone::clone_from
  ::option::CloneNoDefault& operator=(const CloneNoDefault&);

  CloneNoDefault(::crubit::UnsafeRelocateTag, CloneNoDefault&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/option.rs;l=118
    std::uint8_t val;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/enums/option.rs;l=132
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: CopyNoDefault") alignas(1)
    [[clang::trivial_abi]] CopyNoDefault final {
 public:
  // `option_golden::CopyNoDefault` doesn't implement the `Default` trait
  CopyNoDefault() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~CopyNoDefault() = default;
  CopyNoDefault(CopyNoDefault&&) = default;
  ::option::CopyNoDefault& operator=(CopyNoDefault&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  CopyNoDefault(const CopyNoDefault&) = default;
  ::option::CopyNoDefault& operator=(const CopyNoDefault&) = default;
  CopyNoDefault(::crubit::UnsafeRelocateTag, CopyNoDefault&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/option.rs;l=133
    std::uint8_t val;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/enums/option.rs;l=66
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
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=71
  static ::option::HasDefault new_(rs_std::StrRef s);

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=75
  rs_std::StrRef get_string_inside_option() const& $(__anon1)
      CRUBIT_LIFETIME_BOUND;

 private:
  // Field type has been replaced with a blob of bytes: Type
  // `std::string::String` comes from the `alloc` crate, but no `--crate-header`
  // was specified for this crate
  unsigned char foo[24];

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/enums/option.rs;l=89
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
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=94
  static ::option::HasNoDefault new_(rs_std::StrRef s);

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=97
  rs_std::StrRef get_string_inside_option() const& $(__anon1)
      CRUBIT_LIFETIME_BOUND;

 private:
  // Field type has been replaced with a blob of bytes: Type
  // `std::string::String` comes from the `alloc` crate, but no `--crate-header`
  // was specified for this crate
  unsigned char foo[24];

 public:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/option.rs;l=91
    std::uint32_t a;
  };

 private:
  unsigned char __padding1[4];

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/enums/option.rs;l=8
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: NonMaxU8") alignas(1)
    [[clang::trivial_abi]] NonMaxU8 final {
 public:
  // `option_golden::NonMaxU8` doesn't implement the `Default` trait
  NonMaxU8() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~NonMaxU8() = default;
  NonMaxU8(NonMaxU8&&) = default;
  ::option::NonMaxU8& operator=(NonMaxU8&&) = default;

  // `option_golden::NonMaxU8` doesn't implement the `Clone` trait
  NonMaxU8(const NonMaxU8&) = delete;
  NonMaxU8& operator=(const NonMaxU8&) = delete;
  NonMaxU8(::crubit::UnsafeRelocateTag, NonMaxU8&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=11
  std::uint8_t value() const;

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/option.rs;l=8
    std::uint8_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for `option_golden::OptUninhabited` defined at
// cc_bindings_from_rs/test/enums/option.rs;l=150:
// Zero-sized types (ZSTs) are not supported (b/258259459)

// Generated from:
// cc_bindings_from_rs/test/enums/option.rs;l=158
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: OptZst") alignas(1)
    [[clang::trivial_abi]] OptZst final {
 public:
  // Default::default
  OptZst();

  // No custom `Drop` impl and no custom "drop glue" required
  ~OptZst() = default;
  OptZst(OptZst&&) = default;
  ::option::OptZst& operator=(OptZst&&) = default;

  // `option_golden::OptZst` doesn't implement the `Clone` trait
  OptZst(const OptZst&) = delete;
  OptZst& operator=(const OptZst&) = delete;
  OptZst(::crubit::UnsafeRelocateTag, OptZst&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  // Field type has been replaced with a blob of bytes: Failed to format type
  // for the definition of `option_golden::Unit`: Zero-sized types (ZSTs) are
  // not supported (b/258259459)
  unsigned char val[1];

 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for `option_golden::UninhabitedEnum` defined at
// cc_bindings_from_rs/test/enums/option.rs;l=148:
// Zero-sized types (ZSTs) are not supported (b/258259459)

// Error generating bindings for `option_golden::Unit` defined at
// cc_bindings_from_rs/test/enums/option.rs;l=155:
// Zero-sized types (ZSTs) are not supported (b/258259459)

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e
#define _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e
template <>
struct rs_std::Option<::option::CloneNoDefault> {
 public:
  // Clone::clone
  Option(const Option&);

  // Clone::clone_from
  rs_std::Option<::option::CloneNoDefault>& operator=(const Option&);

  Option(Option&&) = default;
  rs_std::Option<::option::CloneNoDefault>& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value) {
    memcpy(this, &value, sizeof(value));
  }
  Option();

  explicit Option(std::nullopt_t) noexcept;
  Option& operator=(std::nullopt_t) noexcept;

  Option(::option::CloneNoDefault&& value) noexcept;
  Option& operator=(::option::CloneNoDefault&& value) noexcept;

  explicit Option(std::optional<::option::CloneNoDefault>&& value) noexcept;
  Option& operator=(std::optional<::option::CloneNoDefault>&& value) noexcept;

  template <typename... Args>
  Option(std::in_place_t, Args&&... args) noexcept;
  ~Option() noexcept = default;
  operator std::optional<::option::CloneNoDefault>() && noexcept;
  bool has_value() noexcept;

 private:
  std::uint8_t* tag() noexcept;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  unsigned char __opaque_blob_of_bytes[2];

 private:
  static void __crubit_field_offset_assertions();
};
#endif

namespace option {

// Generated from:
// cc_bindings_from_rs/test/enums/option.rs;l=122
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: option_golden :: OptCloneNoDefault") alignas(1) [[clang::trivial_abi]]
OptCloneNoDefault final {
 public:
  // `option_golden::OptCloneNoDefault` doesn't implement the `Default` trait
  OptCloneNoDefault() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~OptCloneNoDefault() = default;
  OptCloneNoDefault(OptCloneNoDefault&&) = default;
  ::option::OptCloneNoDefault& operator=(OptCloneNoDefault&&) = default;

  // Clone::clone
  OptCloneNoDefault(const OptCloneNoDefault&);

  // Clone::clone_from
  ::option::OptCloneNoDefault& operator=(const OptCloneNoDefault&);

  OptCloneNoDefault(::crubit::UnsafeRelocateTag, OptCloneNoDefault&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=126
  static ::option::OptCloneNoDefault new_(std::uint8_t x);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/option.rs;l=123
    rs_std::Option<::option::CloneNoDefault> val;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aCopyNoDefault_x0000003e
#define _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aCopyNoDefault_x0000003e
template <>
struct rs_std::Option<::option::CopyNoDefault> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  rs_std::Option<::option::CopyNoDefault>& operator=(const Option&) = default;
  Option(Option&&) = default;
  rs_std::Option<::option::CopyNoDefault>& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value) {
    memcpy(this, &value, sizeof(value));
  }
  Option();

  explicit Option(std::nullopt_t) noexcept;
  Option& operator=(std::nullopt_t) noexcept;

  Option(::option::CopyNoDefault&& value) noexcept;
  Option& operator=(::option::CopyNoDefault&& value) noexcept;

  explicit Option(std::optional<::option::CopyNoDefault>&& value) noexcept;
  Option& operator=(std::optional<::option::CopyNoDefault>&& value) noexcept;

  template <typename... Args>
  Option(std::in_place_t, Args&&... args) noexcept;
  ~Option() noexcept = default;
  operator std::optional<::option::CopyNoDefault>() && noexcept;
  bool has_value() noexcept;

 private:
  std::uint8_t* tag() noexcept;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  unsigned char __opaque_blob_of_bytes[2];

 private:
  static void __crubit_field_offset_assertions();
};
#endif

namespace option {

// Generated from:
// cc_bindings_from_rs/test/enums/option.rs;l=137
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: option_golden :: OptCopyNoDefault") alignas(1) [[clang::trivial_abi]]
OptCopyNoDefault final {
 public:
  // `option_golden::OptCopyNoDefault` doesn't implement the `Default` trait
  OptCopyNoDefault() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~OptCopyNoDefault() = default;
  OptCopyNoDefault(OptCopyNoDefault&&) = default;
  ::option::OptCopyNoDefault& operator=(OptCopyNoDefault&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  OptCopyNoDefault(const OptCopyNoDefault&) = default;
  ::option::OptCopyNoDefault& operator=(const OptCopyNoDefault&) = default;
  OptCopyNoDefault(::crubit::UnsafeRelocateTag, OptCopyNoDefault&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=142
  static ::option::OptCopyNoDefault new_(std::uint8_t x);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/option.rs;l=138
    rs_std::Option<::option::CopyNoDefault> val;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aHasDefault_x0000003e
#define _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aHasDefault_x0000003e
template <>
struct rs_std::Option<::option::HasDefault> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&);
  rs_std::Option<::option::HasDefault>& operator=(Option&&);
  Option(::crubit::UnsafeRelocateTag, Option&& value) {
    memcpy(this, &value, sizeof(value));
  }
  Option();

  explicit Option(std::nullopt_t) noexcept;
  Option& operator=(std::nullopt_t) noexcept;

  Option(::option::HasDefault&& value) noexcept;
  Option& operator=(::option::HasDefault&& value) noexcept;

  explicit Option(std::optional<::option::HasDefault>&& value) noexcept;
  Option& operator=(std::optional<::option::HasDefault>&& value) noexcept;

  template <typename... Args>
  Option(std::in_place_t, Args&&... args) noexcept;
  ~Option() noexcept;
  operator std::optional<::option::HasDefault>() && noexcept;
  bool has_value() noexcept;

 private:
  std::uint64_t* tag() noexcept;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  unsigned char __opaque_blob_of_bytes[24];

 private:
  static void __crubit_field_offset_assertions();
};
#endif

namespace option {

// Generated from:
// cc_bindings_from_rs/test/enums/option.rs;l=80
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
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=84
  static ::option::OptDefaultWithDrop new_(rs_std::StrRef s);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/option.rs;l=81
    rs_std::Option<::option::HasDefault> opt;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/enums/option.rs;l=162
std::optional<std::uintptr_t> stringify_len(
    rs_std::Option<::option::HasDefault> const& x);

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aHasNoDefault_x0000003e
#define _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aHasNoDefault_x0000003e
template <>
struct rs_std::Option<::option::HasNoDefault> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&);
  rs_std::Option<::option::HasNoDefault>& operator=(Option&&);
  Option(::crubit::UnsafeRelocateTag, Option&& value) {
    memcpy(this, &value, sizeof(value));
  }
  Option();

  explicit Option(std::nullopt_t) noexcept;
  Option& operator=(std::nullopt_t) noexcept;

  explicit Option(std::optional<::option::HasNoDefault>&& value) noexcept;
  Option& operator=(std::optional<::option::HasNoDefault>&& value) noexcept;

  template <typename... Args>
  Option(std::in_place_t, Args&&... args) noexcept;
  ~Option() noexcept;
  operator std::optional<::option::HasNoDefault>() && noexcept;
  bool has_value() noexcept;

 private:
  std::uint64_t* tag() noexcept;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  unsigned char __opaque_blob_of_bytes[32];

 private:
  static void __crubit_field_offset_assertions();
};
#endif

namespace option {

// Generated from:
// cc_bindings_from_rs/test/enums/option.rs;l=102
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
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=107
  static ::option::OptNoDefaultWithDrop new_(rs_std::StrRef s);

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=111
  rs_std::StrRef get_string_inside_option() const& $(__anon1)
      CRUBIT_LIFETIME_BOUND;

  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/option.rs;l=103
    rs_std::Option<::option::HasNoDefault> val;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aHasOptions_x0000003e
#define _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aHasOptions_x0000003e
template <>
struct rs_std::Option<::option::HasOptions> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&) = default;
  rs_std::Option<::option::HasOptions>& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value) {
    memcpy(this, &value, sizeof(value));
  }
  Option();

  explicit Option(std::nullopt_t) noexcept;
  Option& operator=(std::nullopt_t) noexcept;

  Option(::option::HasOptions&& value) noexcept;
  Option& operator=(::option::HasOptions&& value) noexcept;

  explicit Option(std::optional<::option::HasOptions>&& value) noexcept;
  Option& operator=(std::optional<::option::HasOptions>&& value) noexcept;

  template <typename... Args>
  Option(std::in_place_t, Args&&... args) noexcept;
  ~Option() noexcept = default;
  operator std::optional<::option::HasOptions>() && noexcept;
  bool has_value() noexcept;

 private:
  std::uint8_t* tag() noexcept;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  unsigned char __opaque_blob_of_bytes[4];

 private:
  static void __crubit_field_offset_assertions();
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aNonMaxU8_x0000003e
#define _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aNonMaxU8_x0000003e
template <>
struct rs_std::Option<::option::NonMaxU8> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&) = default;
  rs_std::Option<::option::NonMaxU8>& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value) {
    memcpy(this, &value, sizeof(value));
  }
  Option();

  explicit Option(std::nullopt_t) noexcept;
  Option& operator=(std::nullopt_t) noexcept;

  Option(::option::NonMaxU8&& value) noexcept;
  Option& operator=(::option::NonMaxU8&& value) noexcept;

  explicit Option(std::optional<::option::NonMaxU8>&& value) noexcept;
  Option& operator=(std::optional<::option::NonMaxU8>&& value) noexcept;

  template <typename... Args>
  Option(std::in_place_t, Args&&... args) noexcept;
  ~Option() noexcept = default;
  operator std::optional<::option::NonMaxU8>() && noexcept;
  bool has_value() noexcept;

 private:
  std::uint8_t* tag() noexcept;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  unsigned char __opaque_blob_of_bytes[1];

 private:
  static void __crubit_field_offset_assertions();
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aNonMaxU8_x0000003e_x0000003e
#define _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aNonMaxU8_x0000003e_x0000003e
template <>
struct rs_std::Option<rs_std::Option<::option::NonMaxU8>> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&) = default;
  rs_std::Option<rs_std::Option<::option::NonMaxU8>>& operator=(Option&&) =
      default;

  Option(::crubit::UnsafeRelocateTag, Option&& value) {
    memcpy(this, &value, sizeof(value));
  }
  Option();

  explicit Option(std::nullopt_t) noexcept;
  Option& operator=(std::nullopt_t) noexcept;

  Option(rs_std::Option<::option::NonMaxU8>&& value) noexcept;
  Option& operator=(rs_std::Option<::option::NonMaxU8>&& value) noexcept;

  explicit Option(
      std::optional<rs_std::Option<::option::NonMaxU8>>&& value) noexcept;
  Option& operator=(
      std::optional<rs_std::Option<::option::NonMaxU8>>&& value) noexcept;

  template <typename... Args>
  Option(std::in_place_t, Args&&... args) noexcept;
  ~Option() noexcept = default;
  operator std::optional<rs_std::Option<::option::NonMaxU8>>() && noexcept;
  bool has_value() noexcept;

 private:
  std::uint8_t* tag() noexcept;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  unsigned char __opaque_blob_of_bytes[1];

 private:
  static void __crubit_field_offset_assertions();
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cu8_x0000003e
#define _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cu8_x0000003e
template <>
struct rs_std::Option<std::uint8_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  rs_std::Option<std::uint8_t>& operator=(const Option&) = default;
  Option(Option&&) = default;
  rs_std::Option<std::uint8_t>& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value) {
    memcpy(this, &value, sizeof(value));
  }
  Option();

  explicit Option(std::nullopt_t) noexcept;
  Option& operator=(std::nullopt_t) noexcept;

  explicit Option(std::optional<std::uint8_t>&& value) noexcept;
  Option& operator=(std::optional<std::uint8_t>&& value) noexcept;

  template <typename... Args>
  Option(std::in_place_t, Args&&... args) noexcept;
  ~Option() noexcept = default;
  operator std::optional<std::uint8_t>() && noexcept;
  bool has_value() noexcept;

 private:
  std::uint8_t* tag() noexcept;

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  unsigned char __opaque_blob_of_bytes[2];

 private:
  static void __crubit_field_offset_assertions();
};
#endif

namespace option {

// Generated from:
// cc_bindings_from_rs/test/enums/option.rs;l=16
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: HasOptions") alignas(1)
    [[clang::trivial_abi]] HasOptions final {
 public:
  // `option_golden::HasOptions` doesn't implement the `Default` trait
  HasOptions() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~HasOptions() = default;
  HasOptions(HasOptions&&) = default;
  ::option::HasOptions& operator=(HasOptions&&) = default;

  // `option_golden::HasOptions` doesn't implement the `Clone` trait
  HasOptions(const HasOptions&) = delete;
  HasOptions& operator=(const HasOptions&) = delete;
  HasOptions(::crubit::UnsafeRelocateTag, HasOptions&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=23
  static ::option::HasOptions new_(std::uint8_t value);

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=33
  static ::option::HasOptions with_option(std::optional<std::uint8_t> value);

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=43
  static ::option::HasOptions from_ref(
      rs_std::Option<std::uint8_t> const& value);

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=50
  static ::option::HasOptions with_none();

  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/option.rs;l=19
    rs_std::Option<std::uint8_t> direct;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/option.rs;l=17
    rs_std::Option<::option::NonMaxU8> niche;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/option.rs;l=18
    rs_std::Option<rs_std::Option<::option::NonMaxU8>> nested;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/enums/option.rs;l=55
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: HasHasOptions") alignas(1)
    [[clang::trivial_abi]] HasHasOptions final {
 public:
  // `option_golden::HasHasOptions` doesn't implement the `Default` trait
  HasHasOptions() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~HasHasOptions() = default;
  HasHasOptions(HasHasOptions&&) = default;
  ::option::HasHasOptions& operator=(HasHasOptions&&) = default;

  // `option_golden::HasHasOptions` doesn't implement the `Clone` trait
  HasHasOptions(const HasHasOptions&) = delete;
  HasHasOptions& operator=(const HasHasOptions&) = delete;
  HasHasOptions(::crubit::UnsafeRelocateTag, HasHasOptions&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=60
  static ::option::HasHasOptions new_(std::uint8_t value);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/option.rs;l=56
    rs_std::Option<::option::HasOptions> me;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(CloneNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<CloneNoDefault>);
static_assert(std::is_trivially_move_constructible_v<::option::CloneNoDefault>);
static_assert(std::is_trivially_move_assignable_v<::option::CloneNoDefault>);
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
static_assert(std::is_trivially_destructible_v<CopyNoDefault>);
static_assert(std::is_trivially_move_constructible_v<::option::CopyNoDefault>);
static_assert(std::is_trivially_move_assignable_v<::option::CopyNoDefault>);
static_assert(std::is_trivially_copy_constructible_v<::option::CopyNoDefault>);
static_assert(std::is_trivially_copy_assignable_v<::option::CopyNoDefault>);
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
  *this = std::move(other);
}
inline ::option::HasDefault& ::option::HasDefault::operator=(
    HasDefault&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(rs_std::StrRef,
                                   ::option::HasDefault* __ret_ptr);
}
inline ::option::HasDefault HasDefault::new_(rs_std::StrRef s) {
  crubit::Slot<::option::HasDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(s, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
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
static_assert(std::is_trivially_destructible_v<HasHasOptions>);
static_assert(std::is_trivially_move_constructible_v<::option::HasHasOptions>);
static_assert(std::is_trivially_move_assignable_v<::option::HasHasOptions>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(std::uint8_t,
                                   ::option::HasHasOptions* __ret_ptr);
}
inline ::option::HasHasOptions HasHasOptions::new_(std::uint8_t value) {
  crubit::Slot<::option::HasHasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(value, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
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
extern "C" void __crubit_thunk_new(rs_std::StrRef,
                                   ::option::HasNoDefault* __ret_ptr);
}
inline ::option::HasNoDefault HasNoDefault::new_(rs_std::StrRef s) {
  crubit::Slot<::option::HasNoDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(s, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
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
static_assert(std::is_trivially_destructible_v<HasOptions>);
static_assert(std::is_trivially_move_constructible_v<::option::HasOptions>);
static_assert(std::is_trivially_move_assignable_v<::option::HasOptions>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(std::uint8_t,
                                   ::option::HasOptions* __ret_ptr);
}
inline ::option::HasOptions HasOptions::new_(std::uint8_t value) {
  crubit::Slot<::option::HasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(value, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_with_uoption(unsigned char*,
                                            ::option::HasOptions* __ret_ptr);
}
inline ::option::HasOptions HasOptions::with_option(
    std::optional<std::uint8_t> value) {
  unsigned char value_buffer
      [::crubit::OptionAbi<::crubit::TransmuteAbi<std::uint8_t>>::kSize];
  ::crubit::internal::Encode<
      ::crubit::OptionAbi<::crubit::TransmuteAbi<std::uint8_t>>>(
      ::crubit::OptionAbi<::crubit::TransmuteAbi<std::uint8_t>>(
          ::crubit::TransmuteAbi<std::uint8_t>()),
      value_buffer, value);
  crubit::Slot<::option::HasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_with_uoption(value_buffer,
                                                 __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_from_uref(rs_std::Option<std::uint8_t> const&,
                                         ::option::HasOptions* __ret_ptr);
}
inline ::option::HasOptions HasOptions::from_ref(
    rs_std::Option<std::uint8_t> const& value) {
  crubit::Slot<::option::HasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_from_uref(value, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_with_unone(::option::HasOptions* __ret_ptr);
}
inline ::option::HasOptions HasOptions::with_none() {
  crubit::Slot<::option::HasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_with_unone(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void HasOptions::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(HasOptions, direct));
  static_assert(2 == offsetof(HasOptions, niche));
  static_assert(3 == offsetof(HasOptions, nested));
}
static_assert(
    sizeof(NonMaxU8) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NonMaxU8) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<NonMaxU8>);
static_assert(std::is_trivially_move_constructible_v<::option::NonMaxU8>);
static_assert(std::is_trivially_move_assignable_v<::option::NonMaxU8>);
namespace __crubit_internal {
extern "C" std::uint8_t __crubit_thunk_value(::option::NonMaxU8 const&);
}
inline std::uint8_t NonMaxU8::value() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_value(self);
}
inline void NonMaxU8::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NonMaxU8, __field0));
}
static_assert(
    sizeof(OptCloneNoDefault) == 2,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(OptCloneNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<OptCloneNoDefault>);
static_assert(
    std::is_trivially_move_constructible_v<::option::OptCloneNoDefault>);
static_assert(std::is_trivially_move_assignable_v<::option::OptCloneNoDefault>);
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
extern "C" void __crubit_thunk_new(std::uint8_t,
                                   ::option::OptCloneNoDefault* __ret_ptr);
}
inline ::option::OptCloneNoDefault OptCloneNoDefault::new_(std::uint8_t x) {
  crubit::Slot<::option::OptCloneNoDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
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
static_assert(std::is_trivially_destructible_v<OptCopyNoDefault>);
static_assert(
    std::is_trivially_move_constructible_v<::option::OptCopyNoDefault>);
static_assert(std::is_trivially_move_assignable_v<::option::OptCopyNoDefault>);
static_assert(
    std::is_trivially_copy_constructible_v<::option::OptCopyNoDefault>);
static_assert(std::is_trivially_copy_assignable_v<::option::OptCopyNoDefault>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(std::uint8_t,
                                   ::option::OptCopyNoDefault* __ret_ptr);
}
inline ::option::OptCopyNoDefault OptCopyNoDefault::new_(std::uint8_t x) {
  crubit::Slot<::option::OptCopyNoDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
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
extern "C" void __crubit_thunk_new(rs_std::StrRef,
                                   ::option::OptDefaultWithDrop* __ret_ptr);
}
inline ::option::OptDefaultWithDrop OptDefaultWithDrop::new_(rs_std::StrRef s) {
  crubit::Slot<::option::OptDefaultWithDrop> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(s, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
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
extern "C" void __crubit_thunk_new(rs_std::StrRef,
                                   ::option::OptNoDefaultWithDrop* __ret_ptr);
}
inline ::option::OptNoDefaultWithDrop OptNoDefaultWithDrop::new_(
    rs_std::StrRef s) {
  crubit::Slot<::option::OptNoDefaultWithDrop> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(s, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
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
extern "C" void __crubit_thunk_default(::option::OptZst* __ret_ptr);
}
inline ::option::OptZst::OptZst() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<OptZst>);
static_assert(std::is_trivially_move_constructible_v<::option::OptZst>);
static_assert(std::is_trivially_move_assignable_v<::option::OptZst>);
inline void OptZst::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(OptZst, val));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_stringify_ulen(
    rs_std::Option<::option::HasDefault> const&, unsigned char* __ret_ptr);
}
inline std::optional<std::uintptr_t> stringify_len(
    rs_std::Option<::option::HasDefault> const& x) {
  unsigned char __return_value_storage
      [::crubit::OptionAbi<::crubit::TransmuteAbi<std::uintptr_t>>::kSize];
  __crubit_internal::__crubit_thunk_stringify_ulen(x, __return_value_storage);
  return ::crubit::internal::Decode<
      ::crubit::OptionAbi<::crubit::TransmuteAbi<std::uintptr_t>>>(
      ::crubit::OptionAbi<::crubit::TransmuteAbi<std::uintptr_t>>(
          ::crubit::TransmuteAbi<std::uintptr_t>()),
      __return_value_storage);
}

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    rs_std::Option<::option::CloneNoDefault> const&,
    rs_std::Option<::option::CloneNoDefault>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(
    rs_std::Option<::option::CloneNoDefault>&,
    rs_std::Option<::option::CloneNoDefault> const&);
}
inline rs_std::Option<::option::CloneNoDefault>::Option(const Option& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline rs_std::Option<::option::CloneNoDefault>&
rs_std::Option<::option::CloneNoDefault>::operator=(const Option& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
static_assert(std::is_trivially_move_constructible_v<
              rs_std::Option<::option::CloneNoDefault>>);
static_assert(std::is_trivially_move_assignable_v<
              rs_std::Option<::option::CloneNoDefault>>);
inline rs_std::Option<::option::CloneNoDefault>::Option() { *this->tag() = 0; }
inline rs_std::Option<::option::CloneNoDefault>::Option(
    std::nullopt_t) noexcept {
  *this->tag() = 0;
}
inline rs_std::Option<::option::CloneNoDefault>&
rs_std::Option<::option::CloneNoDefault>::operator=(std::nullopt_t) noexcept {
  if (*this->tag() != 0) {
    std::destroy_at(reinterpret_cast<::option::CloneNoDefault*>(
        reinterpret_cast<char*>(this) + 1));
  }
  *this->tag() = 0;
  return *this;
}
inline rs_std::Option<::option::CloneNoDefault>::Option(
    ::option::CloneNoDefault&& value) noexcept {
  *this->tag() = 1;
  std::construct_at(reinterpret_cast<::option::CloneNoDefault*>(
                        reinterpret_cast<char*>(this) + 1),
                    std::move(value));
}
inline rs_std::Option<::option::CloneNoDefault>&
rs_std::Option<::option::CloneNoDefault>::operator=(
    ::option::CloneNoDefault&& value) noexcept {
  if (*this->tag() != 0) {
    *reinterpret_cast<::option::CloneNoDefault*>(reinterpret_cast<char*>(this) +
                                                 1) = std::move(value);
  } else {
    *this->tag() = 1;
    std::construct_at(reinterpret_cast<::option::CloneNoDefault*>(
                          reinterpret_cast<char*>(this) + 1),
                      std::move(value));
  }
  return *this;
}
inline rs_std::Option<::option::CloneNoDefault>::Option(
    std::optional<::option::CloneNoDefault>&& value) noexcept {
  if (value.has_value()) {
    *this->tag() = 1;
    ::option::CloneNoDefault* some =
        reinterpret_cast<::option::CloneNoDefault*>(
            reinterpret_cast<char*>(this) + 1);
    *some = std::move(value.value());
    std::construct_at(&value, std::nullopt);
  } else {
    *this->tag() = 0;
  }
}
inline rs_std::Option<::option::CloneNoDefault>&
rs_std::Option<::option::CloneNoDefault>::operator=(
    std::optional<::option::CloneNoDefault>&& value) noexcept {
  if (*this->tag() != 0) {
    std::destroy_at(reinterpret_cast<::option::CloneNoDefault*>(
        reinterpret_cast<char*>(this) + 1));
  }
  if (value.has_value()) {
    *this->tag() = 1;
    ::option::CloneNoDefault* some =
        reinterpret_cast<::option::CloneNoDefault*>(
            reinterpret_cast<char*>(this) + 1);
    *some = std::move(value.value());
    std::construct_at(&value, std::nullopt);
  } else {
    *this->tag() = 0;
  }
  return *this;
}
template <typename... Args>
inline rs_std::Option<::option::CloneNoDefault>::Option(
    std::in_place_t, Args&&... args) noexcept {
  *this->tag() = 1;
  std::construct_at(reinterpret_cast<::option::CloneNoDefault*>(
                        reinterpret_cast<char*>(this) + 1),
                    std::forward<Args>(args)...);
}
static_assert(
    std::is_trivially_destructible_v<rs_std::Option<::option::CloneNoDefault>>);
inline rs_std::Option<::option::CloneNoDefault>::operator std::optional<
    ::option::CloneNoDefault>() && noexcept {
  if (*this->tag() == 0) {
    return std::nullopt;
  } else {
    struct DeferSetTagNone {
      std::uint8_t* _value;
      DeferSetTagNone(std::uint8_t* tag) : _value(tag) {}
      ~DeferSetTagNone() { *this->tag() = 0; }
      std::uint8_t* tag() noexcept { return _value; }
    } defer(this->tag());
    return std::make_optional<::option::CloneNoDefault>(
        crubit::UnsafeRelocateTag{},
        std::move(*reinterpret_cast<::option::CloneNoDefault*>(
            reinterpret_cast<char*>(this) + 1)));
  }
}
inline bool rs_std::Option<::option::CloneNoDefault>::has_value() noexcept {
  return *this->tag() != 0;
}
std::uint8_t* rs_std::Option<::option::CloneNoDefault>::tag() noexcept {
  return reinterpret_cast<std::uint8_t*>(reinterpret_cast<char*>(this) + 0);
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aCopyNoDefault_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aCopyNoDefault_x0000003e
    static_assert(std::is_trivially_copy_constructible_v<
                  rs_std::Option<::option::CopyNoDefault>>);
static_assert(std::is_trivially_copy_assignable_v<
              rs_std::Option<::option::CopyNoDefault>>);
static_assert(std::is_trivially_move_constructible_v<
              rs_std::Option<::option::CopyNoDefault>>);
static_assert(std::is_trivially_move_assignable_v<
              rs_std::Option<::option::CopyNoDefault>>);
inline rs_std::Option<::option::CopyNoDefault>::Option() { *this->tag() = 0; }
inline rs_std::Option<::option::CopyNoDefault>::Option(
    std::nullopt_t) noexcept {
  *this->tag() = 0;
}
inline rs_std::Option<::option::CopyNoDefault>&
rs_std::Option<::option::CopyNoDefault>::operator=(std::nullopt_t) noexcept {
  if (*this->tag() != 0) {
    std::destroy_at(reinterpret_cast<::option::CopyNoDefault*>(
        reinterpret_cast<char*>(this) + 1));
  }
  *this->tag() = 0;
  return *this;
}
inline rs_std::Option<::option::CopyNoDefault>::Option(
    ::option::CopyNoDefault&& value) noexcept {
  *this->tag() = 1;
  std::construct_at(reinterpret_cast<::option::CopyNoDefault*>(
                        reinterpret_cast<char*>(this) + 1),
                    std::move(value));
}
inline rs_std::Option<::option::CopyNoDefault>&
rs_std::Option<::option::CopyNoDefault>::operator=(
    ::option::CopyNoDefault&& value) noexcept {
  if (*this->tag() != 0) {
    *reinterpret_cast<::option::CopyNoDefault*>(reinterpret_cast<char*>(this) +
                                                1) = std::move(value);
  } else {
    *this->tag() = 1;
    std::construct_at(reinterpret_cast<::option::CopyNoDefault*>(
                          reinterpret_cast<char*>(this) + 1),
                      std::move(value));
  }
  return *this;
}
inline rs_std::Option<::option::CopyNoDefault>::Option(
    std::optional<::option::CopyNoDefault>&& value) noexcept {
  if (value.has_value()) {
    *this->tag() = 1;
    ::option::CopyNoDefault* some = reinterpret_cast<::option::CopyNoDefault*>(
        reinterpret_cast<char*>(this) + 1);
    *some = std::move(value.value());
    std::construct_at(&value, std::nullopt);
  } else {
    *this->tag() = 0;
  }
}
inline rs_std::Option<::option::CopyNoDefault>&
rs_std::Option<::option::CopyNoDefault>::operator=(
    std::optional<::option::CopyNoDefault>&& value) noexcept {
  if (*this->tag() != 0) {
    std::destroy_at(reinterpret_cast<::option::CopyNoDefault*>(
        reinterpret_cast<char*>(this) + 1));
  }
  if (value.has_value()) {
    *this->tag() = 1;
    ::option::CopyNoDefault* some = reinterpret_cast<::option::CopyNoDefault*>(
        reinterpret_cast<char*>(this) + 1);
    *some = std::move(value.value());
    std::construct_at(&value, std::nullopt);
  } else {
    *this->tag() = 0;
  }
  return *this;
}
template <typename... Args>
inline rs_std::Option<::option::CopyNoDefault>::Option(
    std::in_place_t, Args&&... args) noexcept {
  *this->tag() = 1;
  std::construct_at(reinterpret_cast<::option::CopyNoDefault*>(
                        reinterpret_cast<char*>(this) + 1),
                    std::forward<Args>(args)...);
}
static_assert(
    std::is_trivially_destructible_v<rs_std::Option<::option::CopyNoDefault>>);
inline rs_std::Option<::option::CopyNoDefault>::operator std::optional<
    ::option::CopyNoDefault>() && noexcept {
  if (*this->tag() == 0) {
    return std::nullopt;
  } else {
    struct DeferSetTagNone {
      std::uint8_t* _value;
      DeferSetTagNone(std::uint8_t* tag) : _value(tag) {}
      ~DeferSetTagNone() { *this->tag() = 0; }
      std::uint8_t* tag() noexcept { return _value; }
    } defer(this->tag());
    return std::make_optional<::option::CopyNoDefault>(
        crubit::UnsafeRelocateTag{},
        std::move(*reinterpret_cast<::option::CopyNoDefault*>(
            reinterpret_cast<char*>(this) + 1)));
  }
}
inline bool rs_std::Option<::option::CopyNoDefault>::has_value() noexcept {
  return *this->tag() != 0;
}
std::uint8_t* rs_std::Option<::option::CopyNoDefault>::tag() noexcept {
  return reinterpret_cast<std::uint8_t*>(reinterpret_cast<char*>(this) + 0);
}
#endif
#ifndef _CRUBIT_BINDINGS_FOR_IMPL_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aHasDefault_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aHasDefault_x0000003e
    inline rs_std::Option < ::option::HasDefault>
    ::Option(Option&& other) : Option() {
  *this = std::move(other);
}
inline rs_std::Option<::option::HasDefault>&
rs_std::Option<::option::HasDefault>::operator=(Option&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline rs_std::Option<::option::HasDefault>::Option() {
  *this->tag() = UINT64_C(9223372036854775808);
}
inline rs_std::Option<::option::HasDefault>::Option(std::nullopt_t) noexcept {
  *this->tag() = UINT64_C(9223372036854775808);
}
inline rs_std::Option<::option::HasDefault>&
rs_std::Option<::option::HasDefault>::operator=(std::nullopt_t) noexcept {
  if (*this->tag() != UINT64_C(9223372036854775808)) {
    std::destroy_at(reinterpret_cast<::option::HasDefault*>(this));
  }
  *this->tag() = UINT64_C(9223372036854775808);
  return *this;
}
inline rs_std::Option<::option::HasDefault>::Option(
    ::option::HasDefault&& value) noexcept {
  std::construct_at(reinterpret_cast<::option::HasDefault*>(this),
                    std::move(value));
}
inline rs_std::Option<::option::HasDefault>& rs_std::Option<
    ::option::HasDefault>::operator=(::option::HasDefault&& value) noexcept {
  if (*this->tag() != UINT64_C(9223372036854775808)) {
    *reinterpret_cast<::option::HasDefault*>(this) = std::move(value);
  } else {
    std::construct_at(reinterpret_cast<::option::HasDefault*>(this),
                      std::move(value));
  }
  return *this;
}
inline rs_std::Option<::option::HasDefault>::Option(
    std::optional<::option::HasDefault>&& value) noexcept {
  if (value.has_value()) {
    ::option::HasDefault* some = reinterpret_cast<::option::HasDefault*>(this);
    *some = std::move(value.value());
    std::construct_at(&value, std::nullopt);
  } else {
    *this->tag() = UINT64_C(9223372036854775808);
  }
}
inline rs_std::Option<::option::HasDefault>&
rs_std::Option<::option::HasDefault>::operator=(
    std::optional<::option::HasDefault>&& value) noexcept {
  if (*this->tag() != UINT64_C(9223372036854775808)) {
    std::destroy_at(reinterpret_cast<::option::HasDefault*>(this));
  }
  if (value.has_value()) {
    ::option::HasDefault* some = reinterpret_cast<::option::HasDefault*>(this);
    *some = std::move(value.value());
    std::construct_at(&value, std::nullopt);
  } else {
    *this->tag() = UINT64_C(9223372036854775808);
  }
  return *this;
}
template <typename... Args>
inline rs_std::Option<::option::HasDefault>::Option(std::in_place_t,
                                                    Args&&... args) noexcept {
  std::construct_at(reinterpret_cast<::option::HasDefault*>(this),
                    std::forward<Args>(args)...);
}
inline rs_std::Option<::option::HasDefault>::~Option() noexcept {
  if (*this->tag() != UINT64_C(9223372036854775808)) {
    std::destroy_at(reinterpret_cast<::option::HasDefault*>(this));
  }
}
inline rs_std::Option<::option::HasDefault>::operator std::optional<
    ::option::HasDefault>() && noexcept {
  if (*this->tag() == UINT64_C(9223372036854775808)) {
    return std::nullopt;
  } else {
    struct DeferSetTagNone {
      std::uint64_t* _value;
      DeferSetTagNone(std::uint64_t* tag) : _value(tag) {}
      ~DeferSetTagNone() { *this->tag() = UINT64_C(9223372036854775808); }
      std::uint64_t* tag() noexcept { return _value; }
    } defer(this->tag());
    return std::make_optional<::option::HasDefault>(
        crubit::UnsafeRelocateTag{},
        std::move(*reinterpret_cast<::option::HasDefault*>(this)));
  }
}
inline bool rs_std::Option<::option::HasDefault>::has_value() noexcept {
  return *this->tag() != UINT64_C(9223372036854775808);
}
std::uint64_t* rs_std::Option<::option::HasDefault>::tag() noexcept {
  return reinterpret_cast<std::uint64_t*>(reinterpret_cast<char*>(this) + 0);
}
#endif
#ifndef _CRUBIT_BINDINGS_FOR_IMPL_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aHasNoDefault_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aHasNoDefault_x0000003e
    inline rs_std::Option < ::option::HasNoDefault>
    ::Option(Option&& other) : Option() {
  *this = std::move(other);
}
inline rs_std::Option<::option::HasNoDefault>&
rs_std::Option<::option::HasNoDefault>::operator=(Option&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline rs_std::Option<::option::HasNoDefault>::Option() {
  *this->tag() = UINT64_C(9223372036854775808);
}
inline rs_std::Option<::option::HasNoDefault>::Option(std::nullopt_t) noexcept {
  *this->tag() = UINT64_C(9223372036854775808);
}
inline rs_std::Option<::option::HasNoDefault>&
rs_std::Option<::option::HasNoDefault>::operator=(std::nullopt_t) noexcept {
  if (*this->tag() != UINT64_C(9223372036854775808)) {
    std::destroy_at(reinterpret_cast<::option::HasNoDefault*>(this));
  }
  *this->tag() = UINT64_C(9223372036854775808);
  return *this;
}
inline rs_std::Option<::option::HasNoDefault>::Option(
    std::optional<::option::HasNoDefault>&& value) noexcept {
  if (value.has_value()) {
    ::option::HasNoDefault* some =
        reinterpret_cast<::option::HasNoDefault*>(this);
    std::construct_at(some, crubit::UnsafeRelocateTag{}, std::move(*value));
    std::construct_at(&value, std::nullopt);
  } else {
    *this->tag() = UINT64_C(9223372036854775808);
  }
}
inline rs_std::Option<::option::HasNoDefault>&
rs_std::Option<::option::HasNoDefault>::operator=(
    std::optional<::option::HasNoDefault>&& value) noexcept {
  if (*this->tag() != UINT64_C(9223372036854775808)) {
    std::destroy_at(reinterpret_cast<::option::HasNoDefault*>(this));
  }
  if (value.has_value()) {
    ::option::HasNoDefault* some =
        reinterpret_cast<::option::HasNoDefault*>(this);
    std::construct_at(some, crubit::UnsafeRelocateTag{}, std::move(*value));
    std::construct_at(&value, std::nullopt);
  } else {
    *this->tag() = UINT64_C(9223372036854775808);
  }
  return *this;
}
template <typename... Args>
inline rs_std::Option<::option::HasNoDefault>::Option(std::in_place_t,
                                                      Args&&... args) noexcept {
  std::construct_at(reinterpret_cast<::option::HasNoDefault*>(this),
                    std::forward<Args>(args)...);
}
inline rs_std::Option<::option::HasNoDefault>::~Option() noexcept {
  if (*this->tag() != UINT64_C(9223372036854775808)) {
    std::destroy_at(reinterpret_cast<::option::HasNoDefault*>(this));
  }
}
inline rs_std::Option<::option::HasNoDefault>::operator std::optional<
    ::option::HasNoDefault>() && noexcept {
  if (*this->tag() == UINT64_C(9223372036854775808)) {
    return std::nullopt;
  } else {
    struct DeferSetTagNone {
      std::uint64_t* _value;
      DeferSetTagNone(std::uint64_t* tag) : _value(tag) {}
      ~DeferSetTagNone() { *this->tag() = UINT64_C(9223372036854775808); }
      std::uint64_t* tag() noexcept { return _value; }
    } defer(this->tag());
    return std::make_optional<::option::HasNoDefault>(
        crubit::UnsafeRelocateTag{},
        std::move(*reinterpret_cast<::option::HasNoDefault*>(this)));
  }
}
inline bool rs_std::Option<::option::HasNoDefault>::has_value() noexcept {
  return *this->tag() != UINT64_C(9223372036854775808);
}
std::uint64_t* rs_std::Option<::option::HasNoDefault>::tag() noexcept {
  return reinterpret_cast<std::uint64_t*>(reinterpret_cast<char*>(this) + 0);
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aHasOptions_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aHasOptions_x0000003e
    static_assert(std::is_trivially_move_constructible_v<
                  rs_std::Option<::option::HasOptions>>);
static_assert(
    std::is_trivially_move_assignable_v<rs_std::Option<::option::HasOptions>>);
inline rs_std::Option<::option::HasOptions>::Option() { *this->tag() = 2; }
inline rs_std::Option<::option::HasOptions>::Option(std::nullopt_t) noexcept {
  *this->tag() = 2;
}
inline rs_std::Option<::option::HasOptions>&
rs_std::Option<::option::HasOptions>::operator=(std::nullopt_t) noexcept {
  if (*this->tag() != 2) {
    std::destroy_at(reinterpret_cast<::option::HasOptions*>(this));
  }
  *this->tag() = 2;
  return *this;
}
inline rs_std::Option<::option::HasOptions>::Option(
    ::option::HasOptions&& value) noexcept {
  std::construct_at(reinterpret_cast<::option::HasOptions*>(this),
                    std::move(value));
}
inline rs_std::Option<::option::HasOptions>& rs_std::Option<
    ::option::HasOptions>::operator=(::option::HasOptions&& value) noexcept {
  if (*this->tag() != 2) {
    *reinterpret_cast<::option::HasOptions*>(this) = std::move(value);
  } else {
    std::construct_at(reinterpret_cast<::option::HasOptions*>(this),
                      std::move(value));
  }
  return *this;
}
inline rs_std::Option<::option::HasOptions>::Option(
    std::optional<::option::HasOptions>&& value) noexcept {
  if (value.has_value()) {
    ::option::HasOptions* some = reinterpret_cast<::option::HasOptions*>(this);
    *some = std::move(value.value());
    std::construct_at(&value, std::nullopt);
  } else {
    *this->tag() = 2;
  }
}
inline rs_std::Option<::option::HasOptions>&
rs_std::Option<::option::HasOptions>::operator=(
    std::optional<::option::HasOptions>&& value) noexcept {
  if (*this->tag() != 2) {
    std::destroy_at(reinterpret_cast<::option::HasOptions*>(this));
  }
  if (value.has_value()) {
    ::option::HasOptions* some = reinterpret_cast<::option::HasOptions*>(this);
    *some = std::move(value.value());
    std::construct_at(&value, std::nullopt);
  } else {
    *this->tag() = 2;
  }
  return *this;
}
template <typename... Args>
inline rs_std::Option<::option::HasOptions>::Option(std::in_place_t,
                                                    Args&&... args) noexcept {
  std::construct_at(reinterpret_cast<::option::HasOptions*>(this),
                    std::forward<Args>(args)...);
}
static_assert(
    std::is_trivially_destructible_v<rs_std::Option<::option::HasOptions>>);
inline rs_std::Option<::option::HasOptions>::operator std::optional<
    ::option::HasOptions>() && noexcept {
  if (*this->tag() == 2) {
    return std::nullopt;
  } else {
    struct DeferSetTagNone {
      std::uint8_t* _value;
      DeferSetTagNone(std::uint8_t* tag) : _value(tag) {}
      ~DeferSetTagNone() { *this->tag() = 2; }
      std::uint8_t* tag() noexcept { return _value; }
    } defer(this->tag());
    return std::make_optional<::option::HasOptions>(
        crubit::UnsafeRelocateTag{},
        std::move(*reinterpret_cast<::option::HasOptions*>(this)));
  }
}
inline bool rs_std::Option<::option::HasOptions>::has_value() noexcept {
  return *this->tag() != 2;
}
std::uint8_t* rs_std::Option<::option::HasOptions>::tag() noexcept {
  return reinterpret_cast<std::uint8_t*>(reinterpret_cast<char*>(this) + 0);
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aNonMaxU8_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aNonMaxU8_x0000003e
    static_assert(std::is_trivially_move_constructible_v<
                  rs_std::Option<::option::NonMaxU8>>);
static_assert(
    std::is_trivially_move_assignable_v<rs_std::Option<::option::NonMaxU8>>);
inline rs_std::Option<::option::NonMaxU8>::Option() { *this->tag() = 251; }
inline rs_std::Option<::option::NonMaxU8>::Option(std::nullopt_t) noexcept {
  *this->tag() = 251;
}
inline rs_std::Option<::option::NonMaxU8>&
rs_std::Option<::option::NonMaxU8>::operator=(std::nullopt_t) noexcept {
  if (*this->tag() != 251) {
    std::destroy_at(reinterpret_cast<::option::NonMaxU8*>(this));
  }
  *this->tag() = 251;
  return *this;
}
inline rs_std::Option<::option::NonMaxU8>::Option(
    ::option::NonMaxU8&& value) noexcept {
  std::construct_at(reinterpret_cast<::option::NonMaxU8*>(this),
                    std::move(value));
}
inline rs_std::Option<::option::NonMaxU8>& rs_std::Option<
    ::option::NonMaxU8>::operator=(::option::NonMaxU8&& value) noexcept {
  if (*this->tag() != 251) {
    *reinterpret_cast<::option::NonMaxU8*>(this) = std::move(value);
  } else {
    std::construct_at(reinterpret_cast<::option::NonMaxU8*>(this),
                      std::move(value));
  }
  return *this;
}
inline rs_std::Option<::option::NonMaxU8>::Option(
    std::optional<::option::NonMaxU8>&& value) noexcept {
  if (value.has_value()) {
    ::option::NonMaxU8* some = reinterpret_cast<::option::NonMaxU8*>(this);
    *some = std::move(value.value());
    std::construct_at(&value, std::nullopt);
  } else {
    *this->tag() = 251;
  }
}
inline rs_std::Option<::option::NonMaxU8>&
rs_std::Option<::option::NonMaxU8>::operator=(
    std::optional<::option::NonMaxU8>&& value) noexcept {
  if (*this->tag() != 251) {
    std::destroy_at(reinterpret_cast<::option::NonMaxU8*>(this));
  }
  if (value.has_value()) {
    ::option::NonMaxU8* some = reinterpret_cast<::option::NonMaxU8*>(this);
    *some = std::move(value.value());
    std::construct_at(&value, std::nullopt);
  } else {
    *this->tag() = 251;
  }
  return *this;
}
template <typename... Args>
inline rs_std::Option<::option::NonMaxU8>::Option(std::in_place_t,
                                                  Args&&... args) noexcept {
  std::construct_at(reinterpret_cast<::option::NonMaxU8*>(this),
                    std::forward<Args>(args)...);
}
static_assert(
    std::is_trivially_destructible_v<rs_std::Option<::option::NonMaxU8>>);
inline rs_std::Option<::option::NonMaxU8>::operator std::optional<
    ::option::NonMaxU8>() && noexcept {
  if (*this->tag() == 251) {
    return std::nullopt;
  } else {
    struct DeferSetTagNone {
      std::uint8_t* _value;
      DeferSetTagNone(std::uint8_t* tag) : _value(tag) {}
      ~DeferSetTagNone() { *this->tag() = 251; }
      std::uint8_t* tag() noexcept { return _value; }
    } defer(this->tag());
    return std::make_optional<::option::NonMaxU8>(
        crubit::UnsafeRelocateTag{},
        std::move(*reinterpret_cast<::option::NonMaxU8*>(this)));
  }
}
inline bool rs_std::Option<::option::NonMaxU8>::has_value() noexcept {
  return *this->tag() != 251;
}
std::uint8_t* rs_std::Option<::option::NonMaxU8>::tag() noexcept {
  return reinterpret_cast<std::uint8_t*>(reinterpret_cast<char*>(this) + 0);
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aNonMaxU8_x0000003e_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aNonMaxU8_x0000003e_x0000003e
    static_assert(std::is_trivially_move_constructible_v<
                  rs_std::Option<rs_std::Option<::option::NonMaxU8>>>);
static_assert(std::is_trivially_move_assignable_v<
              rs_std::Option<rs_std::Option<::option::NonMaxU8>>>);
inline rs_std::Option<rs_std::Option<::option::NonMaxU8>>::Option() {
  *this->tag() = 252;
}
inline rs_std::Option<rs_std::Option<::option::NonMaxU8>>::Option(
    std::nullopt_t) noexcept {
  *this->tag() = 252;
}
inline rs_std::Option<rs_std::Option<::option::NonMaxU8>>& rs_std::Option<
    rs_std::Option<::option::NonMaxU8>>::operator=(std::nullopt_t) noexcept {
  if (*this->tag() != 252) {
    std::destroy_at(
        reinterpret_cast<rs_std::Option<::option::NonMaxU8>*>(this));
  }
  *this->tag() = 252;
  return *this;
}
inline rs_std::Option<rs_std::Option<::option::NonMaxU8>>::Option(
    rs_std::Option<::option::NonMaxU8>&& value) noexcept {
  std::construct_at(reinterpret_cast<rs_std::Option<::option::NonMaxU8>*>(this),
                    std::move(value));
}
inline rs_std::Option<rs_std::Option<::option::NonMaxU8>>&
rs_std::Option<rs_std::Option<::option::NonMaxU8>>::operator=(
    rs_std::Option<::option::NonMaxU8>&& value) noexcept {
  if (*this->tag() != 252) {
    *reinterpret_cast<rs_std::Option<::option::NonMaxU8>*>(this) =
        std::move(value);
  } else {
    std::construct_at(
        reinterpret_cast<rs_std::Option<::option::NonMaxU8>*>(this),
        std::move(value));
  }
  return *this;
}
inline rs_std::Option<rs_std::Option<::option::NonMaxU8>>::Option(
    std::optional<rs_std::Option<::option::NonMaxU8>>&& value) noexcept {
  if (value.has_value()) {
    rs_std::Option<::option::NonMaxU8>* some =
        reinterpret_cast<rs_std::Option<::option::NonMaxU8>*>(this);
    *some = std::move(value.value());
    std::construct_at(&value, std::nullopt);
  } else {
    *this->tag() = 252;
  }
}
inline rs_std::Option<rs_std::Option<::option::NonMaxU8>>&
rs_std::Option<rs_std::Option<::option::NonMaxU8>>::operator=(
    std::optional<rs_std::Option<::option::NonMaxU8>>&& value) noexcept {
  if (*this->tag() != 252) {
    std::destroy_at(
        reinterpret_cast<rs_std::Option<::option::NonMaxU8>*>(this));
  }
  if (value.has_value()) {
    rs_std::Option<::option::NonMaxU8>* some =
        reinterpret_cast<rs_std::Option<::option::NonMaxU8>*>(this);
    *some = std::move(value.value());
    std::construct_at(&value, std::nullopt);
  } else {
    *this->tag() = 252;
  }
  return *this;
}
template <typename... Args>
inline rs_std::Option<rs_std::Option<::option::NonMaxU8>>::Option(
    std::in_place_t, Args&&... args) noexcept {
  std::construct_at(reinterpret_cast<rs_std::Option<::option::NonMaxU8>*>(this),
                    std::forward<Args>(args)...);
}
static_assert(std::is_trivially_destructible_v<
              rs_std::Option<rs_std::Option<::option::NonMaxU8>>>);
inline rs_std::Option<rs_std::Option<::option::NonMaxU8>>::operator std::
    optional<rs_std::Option<::option::NonMaxU8>>() && noexcept {
  if (*this->tag() == 252) {
    return std::nullopt;
  } else {
    struct DeferSetTagNone {
      std::uint8_t* _value;
      DeferSetTagNone(std::uint8_t* tag) : _value(tag) {}
      ~DeferSetTagNone() { *this->tag() = 252; }
      std::uint8_t* tag() noexcept { return _value; }
    } defer(this->tag());
    return std::make_optional<rs_std::Option<::option::NonMaxU8>>(
        crubit::UnsafeRelocateTag{},
        std::move(
            *reinterpret_cast<rs_std::Option<::option::NonMaxU8>*>(this)));
  }
}
inline bool
rs_std::Option<rs_std::Option<::option::NonMaxU8>>::has_value() noexcept {
  return *this->tag() != 252;
}
std::uint8_t*
rs_std::Option<rs_std::Option<::option::NonMaxU8>>::tag() noexcept {
  return reinterpret_cast<std::uint8_t*>(reinterpret_cast<char*>(this) + 0);
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cu8_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cu8_x0000003e
    static_assert(
        std::is_trivially_copy_constructible_v<rs_std::Option<std::uint8_t>>);
static_assert(
    std::is_trivially_copy_assignable_v<rs_std::Option<std::uint8_t>>);
static_assert(
    std::is_trivially_move_constructible_v<rs_std::Option<std::uint8_t>>);
static_assert(
    std::is_trivially_move_assignable_v<rs_std::Option<std::uint8_t>>);
inline rs_std::Option<std::uint8_t>::Option() { *this->tag() = 0; }
inline rs_std::Option<std::uint8_t>::Option(std::nullopt_t) noexcept {
  *this->tag() = 0;
}
inline rs_std::Option<std::uint8_t>& rs_std::Option<std::uint8_t>::operator=(
    std::nullopt_t) noexcept {
  if (*this->tag() != 0) {
    std::destroy_at(
        reinterpret_cast<std::uint8_t*>(reinterpret_cast<char*>(this) + 1));
  }
  *this->tag() = 0;
  return *this;
}
inline rs_std::Option<std::uint8_t>::Option(
    std::optional<std::uint8_t>&& value) noexcept {
  if (value.has_value()) {
    *this->tag() = 1;
    std::uint8_t* some =
        reinterpret_cast<std::uint8_t*>(reinterpret_cast<char*>(this) + 1);
    *some = value.value();
    std::construct_at(&value, std::nullopt);
  } else {
    *this->tag() = 0;
  }
}
inline rs_std::Option<std::uint8_t>& rs_std::Option<std::uint8_t>::operator=(
    std::optional<std::uint8_t>&& value) noexcept {
  if (*this->tag() != 0) {
    std::destroy_at(
        reinterpret_cast<std::uint8_t*>(reinterpret_cast<char*>(this) + 1));
  }
  if (value.has_value()) {
    *this->tag() = 1;
    std::uint8_t* some =
        reinterpret_cast<std::uint8_t*>(reinterpret_cast<char*>(this) + 1);
    *some = value.value();
    std::construct_at(&value, std::nullopt);
  } else {
    *this->tag() = 0;
  }
  return *this;
}
template <typename... Args>
inline rs_std::Option<std::uint8_t>::Option(std::in_place_t,
                                            Args&&... args) noexcept {
  *this->tag() = 1;
  std::construct_at(
      reinterpret_cast<std::uint8_t*>(reinterpret_cast<char*>(this) + 1),
      std::forward<Args>(args)...);
}
static_assert(std::is_trivially_destructible_v<rs_std::Option<std::uint8_t>>);
inline rs_std::Option<std::uint8_t>::operator std::optional<
    std::uint8_t>() && noexcept {
  if (*this->tag() == 0) {
    return std::nullopt;
  } else {
    std::uint8_t& value =
        *reinterpret_cast<std::uint8_t*>(reinterpret_cast<char*>(this) + 1);
    std::optional<std::uint8_t> return_value(std::move(value));
    std::destroy_at(&value);
    *this->tag() = 0;
    return return_value;
  }
}
inline bool rs_std::Option<std::uint8_t>::has_value() noexcept {
  return *this->tag() != 0;
}
std::uint8_t* rs_std::Option<std::uint8_t>::tag() noexcept {
  return reinterpret_cast<std::uint8_t*>(reinterpret_cast<char*>(this) + 0);
}
#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_OPTION_GOLDEN
