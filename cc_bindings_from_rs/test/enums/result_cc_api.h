// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// result_golden
// Features: callables, supported, types

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_RESULT_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_RESULT_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/check.h"
#include "support/internal/memswap.h"
#include "support/internal/move_assign.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/result.h"
#include "support/rs_std/str_ref.h"

#include <array>
#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_alloc.h"

namespace result {

// Generated from:
// cc_bindings_from_rs/test/enums/result.rs;l=41
struct CRUBIT_INTERNAL_RUST_TYPE(":: result_golden :: CloneNoDefault") alignas(
    1) [[clang::trivial_abi]] CloneNoDefault final {
 public:
  // `result_golden::CloneNoDefault` doesn't implement the `Default` trait
  CloneNoDefault() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~CloneNoDefault() = default;
  CloneNoDefault(CloneNoDefault&&) = default;
  CloneNoDefault& operator=(CloneNoDefault&&) = default;

  // Clone::clone
  CloneNoDefault(const CloneNoDefault&);

  // Clone::clone_from
  ::result::CloneNoDefault& operator=(const CloneNoDefault&);

  CloneNoDefault(::crubit::UnsafeRelocateTag, CloneNoDefault&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/result.rs;l=42
    ::std::uint8_t val;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/enums/result.rs;l=26
struct CRUBIT_INTERNAL_RUST_TYPE(":: result_golden :: CopyNoDefault") alignas(1)
    [[clang::trivial_abi]] CopyNoDefault final {
 public:
  // `result_golden::CopyNoDefault` doesn't implement the `Default` trait
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
    // Generated from:
    // cc_bindings_from_rs/test/enums/result.rs;l=27
    ::std::uint8_t val;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/enums/result.rs;l=58
struct CRUBIT_INTERNAL_RUST_TYPE(":: result_golden :: HasDefault") alignas(8)
    [[clang::trivial_abi]] HasDefault final {
 public:
  // Default::default
  HasDefault();

  // Drop::drop
  ~HasDefault();

  HasDefault(HasDefault&&);
  ::result::HasDefault& operator=(HasDefault&&);

  // `result_golden::HasDefault` doesn't implement the `Clone` trait
  HasDefault(const HasDefault&) = delete;
  HasDefault& operator=(const HasDefault&) = delete;
  HasDefault(::crubit::UnsafeRelocateTag, HasDefault&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/result.rs;l=62
  static ::result::HasDefault new_(rs_std::StrRef val);

  // Generated from:
  // cc_bindings_from_rs/test/enums/result.rs;l=66
  rs_std::StrRef val() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/result.rs;l=59
    ::rs::alloc::string::String val_;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/enums/result.rs;l=83
struct CRUBIT_INTERNAL_RUST_TYPE(":: result_golden :: HasNoDefault") alignas(8)
    [[clang::trivial_abi]] HasNoDefault final {
 public:
  // `result_golden::HasNoDefault` doesn't implement the `Default` trait
  HasNoDefault() = delete;

  // Drop::drop
  ~HasNoDefault();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  HasNoDefault(HasNoDefault&&) = delete;
  ::result::HasNoDefault& operator=(HasNoDefault&&) = delete;
  // `result_golden::HasNoDefault` doesn't implement the `Clone` trait
  HasNoDefault(const HasNoDefault&) = delete;
  HasNoDefault& operator=(const HasNoDefault&) = delete;
  HasNoDefault(::crubit::UnsafeRelocateTag, HasNoDefault&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/result.rs;l=87
  rs_std::StrRef val() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/result.rs;l=84
    ::rs::alloc::string::String val_;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/enums/result.rs;l=135
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: result_golden :: ResultWithSizeTypes") alignas(8) [[clang::trivial_abi]]
ResultWithSizeTypes final {
 public:
  // `result_golden::ResultWithSizeTypes` doesn't implement the `Default` trait
  ResultWithSizeTypes() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~ResultWithSizeTypes() = default;
  ResultWithSizeTypes(ResultWithSizeTypes&&) = default;
  ResultWithSizeTypes& operator=(ResultWithSizeTypes&&) = default;

  // `result_golden::ResultWithSizeTypes` doesn't implement the `Clone` trait
  ResultWithSizeTypes(const ResultWithSizeTypes&) = delete;
  ResultWithSizeTypes& operator=(const ResultWithSizeTypes&) = delete;
  ResultWithSizeTypes(::crubit::UnsafeRelocateTag,
                      ResultWithSizeTypes&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

 private:
  // Field type has been replaced with a blob of bytes: b/491106325 - isize and
  // usize types are not yet supported as generic type arguments.
  ::std::array<unsigned char, 16> uval_in_ok;
  // Field type has been replaced with a blob of bytes: b/491106325 - isize and
  // usize types are not yet supported as generic type arguments.
  ::std::array<unsigned char, 16> uval_in_err;
  // Field type has been replaced with a blob of bytes: b/491106325 - isize and
  // usize types are not yet supported as generic type arguments.
  ::std::array<unsigned char, 16> ival_in_ok;
  // Field type has been replaced with a blob of bytes: b/491106325 - isize and
  // usize types are not yet supported as generic type arguments.
  ::std::array<unsigned char, 16> ival_in_err;

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace result

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: result_golden :: CloneNoDefault , u8 >")
    rs_std::Result<::result::CloneNoDefault, ::std::uint8_t> {
 public:
  // Clone::clone
  Result(const Result&);

  // Clone::clone_from
  rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>& operator=(
      const Result&);

  Result(Result&&) = default;
  Result& operator=(Result&&) = default;

  Result(::crubit::UnsafeRelocateTag, Result&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Result(::result::CloneNoDefault&& ok) noexcept;
  Result& operator=(::result::CloneNoDefault&& ok) noexcept;
  Result(rs_std::unexpected<::std::uint8_t>&& err) noexcept;
  Result& operator=(rs_std::unexpected<::std::uint8_t>&& err) noexcept;
  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs_std::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  ::result::CloneNoDefault& value() &;
  ::result::CloneNoDefault&& value() &&;
  ::std::uint8_t& err() &;
  ::std::uint8_t&& err() &&;
  ~Result() noexcept = default;

 private:
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;
  void check_has_ok();
  void check_has_err();

 private:
  unsigned char __storage[2];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: result_golden :: CopyNoDefault , u8 >")
    rs_std::Result<::result::CopyNoDefault, ::std::uint8_t> {
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
  Result(::result::CopyNoDefault&& ok) noexcept;
  Result& operator=(::result::CopyNoDefault&& ok) noexcept;
  Result(rs_std::unexpected<::std::uint8_t>&& err) noexcept;
  Result& operator=(rs_std::unexpected<::std::uint8_t>&& err) noexcept;
  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs_std::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  ::result::CopyNoDefault& value() &;
  ::result::CopyNoDefault&& value() &&;
  ::std::uint8_t& err() &;
  ::std::uint8_t&& err() &&;
  ~Result() noexcept = default;

 private:
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;
  void check_has_ok();
  void check_has_err();

 private:
  unsigned char __storage[2];
};
#endif

namespace result {

// Generated from:
// cc_bindings_from_rs/test/enums/result.rs;l=103
::std::uint8_t take_result_copy_no_default_ok(
    rs_std::Result<::result::CopyNoDefault, ::std::uint8_t> const& r);

}  // namespace result

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: result_golden :: HasDefault , u8 >")
    rs_std::Result<::result::HasDefault, ::std::uint8_t> {
 public:
  // `core::result::Result` doesn't implement the `Clone` trait
  Result(const Result&) = delete;
  Result& operator=(const Result&) = delete;
  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  Result(Result&&) = delete;
  rs_std::Result<::result::HasDefault, ::std::uint8_t>& operator=(Result&&) =
      delete;
  Result(::crubit::UnsafeRelocateTag, Result&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Result(::result::HasDefault&& ok) noexcept;
  Result& operator=(::result::HasDefault&& ok) noexcept;
  Result(rs_std::unexpected<::std::uint8_t>&& err) noexcept;
  Result& operator=(rs_std::unexpected<::std::uint8_t>&& err) noexcept;
  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs_std::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  ::result::HasDefault& value() &;
  ::result::HasDefault&& value() &&;
  ::std::uint8_t& err() &;
  ::std::uint8_t&& err() &&;
  ~Result() noexcept;

 private:
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;
  void check_has_ok();
  void check_has_err();

 private:
  unsigned char __storage[24];
};
#endif

namespace result {

// Generated from:
// cc_bindings_from_rs/test/enums/result.rs;l=117
rs_std::StrRef take_result_has_default(
    rs_std::Result<::result::HasDefault, ::std::uint8_t> const* $(__anon1)
        crubit_nonnull r CRUBIT_LIFETIME_BOUND);

}  // namespace result

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: result_golden :: HasNoDefault , u8 >")
    rs_std::Result<::result::HasNoDefault, ::std::uint8_t> {
 public:
  // `core::result::Result` doesn't implement the `Clone` trait
  Result(const Result&) = delete;
  Result& operator=(const Result&) = delete;
  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  Result(Result&&) = delete;
  rs_std::Result<::result::HasNoDefault, ::std::uint8_t>& operator=(Result&&) =
      delete;
  Result(::crubit::UnsafeRelocateTag, Result&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }  // Move constructor is not available for the Ok variant because
     // result_golden::HasNoDefault does not have a move constructor

  Result(rs_std::unexpected<::std::uint8_t>&& err) noexcept;
  Result& operator=(rs_std::unexpected<::std::uint8_t>&& err) noexcept;
  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs_std::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  ::result::HasNoDefault& value() &;
  ::result::HasNoDefault&& value() &&;
  ::std::uint8_t& err() &;
  ::std::uint8_t&& err() &&;
  ~Result() noexcept;

 private:
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;
  void check_has_ok();
  void check_has_err();

 private:
  unsigned char __storage[24];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
template <>
struct alignas(4)
    CRUBIT_INTERNAL_RUST_TYPE("std :: result :: Result < u32 , u32 >")
        rs_std::Result<::std::uint32_t, ::std::uint32_t> {
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
  Result(::std::uint32_t&& ok) noexcept;
  Result& operator=(::std::uint32_t&& ok) noexcept;
  Result(rs_std::unexpected<::std::uint32_t>&& err) noexcept;
  Result& operator=(rs_std::unexpected<::std::uint32_t>&& err) noexcept;
  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs_std::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  ::std::uint32_t& value() &;
  ::std::uint32_t&& value() &&;
  ::std::uint32_t& err() &;
  ::std::uint32_t&& err() &&;
  ~Result() noexcept = default;

 private:
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;
  void check_has_ok();
  void check_has_err();

 private:
  unsigned char __storage[8];
};
#endif

namespace result {

// Generated from:
// cc_bindings_from_rs/test/enums/result.rs;l=5
struct CRUBIT_INTERNAL_RUST_TYPE(":: result_golden :: GetsResult") alignas(4)
    [[clang::trivial_abi]] GetsResult final {
 public:
  // `result_golden::GetsResult` doesn't implement the `Default` trait
  GetsResult() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~GetsResult() = default;
  GetsResult(GetsResult&&) = default;
  GetsResult& operator=(GetsResult&&) = default;

  // `result_golden::GetsResult` doesn't implement the `Clone` trait
  GetsResult(const GetsResult&) = delete;
  GetsResult& operator=(const GetsResult&) = delete;
  GetsResult(::crubit::UnsafeRelocateTag, GetsResult&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/result.rs;l=9
  static ::result::GetsResult new_(::std::uint32_t val);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/result.rs;l=6
    rs_std::Result<::std::uint32_t, ::std::uint32_t> value;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace result

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: core :: result :: Result < u32 , u32 > , u32 "
    ">") rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
                        ::std::uint32_t> {
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
  Result(rs_std::Result<::std::uint32_t, ::std::uint32_t>&& ok) noexcept;
  Result& operator=(
      rs_std::Result<::std::uint32_t, ::std::uint32_t>&& ok) noexcept;
  Result(rs_std::unexpected<::std::uint32_t>&& err) noexcept;
  Result& operator=(rs_std::unexpected<::std::uint32_t>&& err) noexcept;
  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs_std::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  rs_std::Result<::std::uint32_t, ::std::uint32_t>& value() &;
  rs_std::Result<::std::uint32_t, ::std::uint32_t>&& value() &&;
  ::std::uint32_t& err() &;
  ::std::uint32_t&& err() &&;
  ~Result() noexcept = default;

 private:
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;
  void check_has_ok();
  void check_has_err();

 private:
  unsigned char __storage[8];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < u32 , :: core :: result :: Result < u32 , u32 > "
    ">") rs_std::Result<::std::uint32_t,
                        rs_std::Result<::std::uint32_t, ::std::uint32_t>> {
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
  Result(::std::uint32_t&& ok) noexcept;
  Result& operator=(::std::uint32_t&& ok) noexcept;
  Result(rs_std::unexpected<rs_std::Result<::std::uint32_t, ::std::uint32_t>>&&
             err) noexcept;
  Result& operator=(
      rs_std::unexpected<rs_std::Result<::std::uint32_t, ::std::uint32_t>>&&
          err) noexcept;
  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs_std::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  ::std::uint32_t& value() &;
  ::std::uint32_t&& value() &&;
  rs_std::Result<::std::uint32_t, ::std::uint32_t>& err() &;
  rs_std::Result<::std::uint32_t, ::std::uint32_t>&& err() &&;
  ~Result() noexcept = default;

 private:
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;
  void check_has_ok();
  void check_has_err();

 private:
  unsigned char __storage[8];
};
#endif

namespace result {

// Generated from:
// cc_bindings_from_rs/test/enums/result.rs;l=14
struct CRUBIT_INTERNAL_RUST_TYPE(":: result_golden :: NestedResult") alignas(4)
    [[clang::trivial_abi]] NestedResult final {
 public:
  // `result_golden::NestedResult` doesn't implement the `Default` trait
  NestedResult() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~NestedResult() = default;
  NestedResult(NestedResult&&) = default;
  NestedResult& operator=(NestedResult&&) = default;

  // `result_golden::NestedResult` doesn't implement the `Clone` trait
  NestedResult(const NestedResult&) = delete;
  NestedResult& operator=(const NestedResult&) = delete;
  NestedResult(::crubit::UnsafeRelocateTag, NestedResult&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/result.rs;l=20
  static ::result::NestedResult new_(::std::uint32_t val);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/result.rs;l=15
    rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
                   ::std::uint32_t>
        in_ok;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/result.rs;l=16
    rs_std::Result<::std::uint32_t,
                   rs_std::Result<::std::uint32_t, ::std::uint32_t>>
        in_err;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace result

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < u8 , :: result_golden :: CloneNoDefault >")
    rs_std::Result<::std::uint8_t, ::result::CloneNoDefault> {
 public:
  // Clone::clone
  Result(const Result&);

  // Clone::clone_from
  rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>& operator=(
      const Result&);

  Result(Result&&) = default;
  Result& operator=(Result&&) = default;

  Result(::crubit::UnsafeRelocateTag, Result&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Result(::std::uint8_t&& ok) noexcept;
  Result& operator=(::std::uint8_t&& ok) noexcept;
  Result(rs_std::unexpected<::result::CloneNoDefault>&& err) noexcept;
  Result& operator=(
      rs_std::unexpected<::result::CloneNoDefault>&& err) noexcept;
  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs_std::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  ::std::uint8_t& value() &;
  ::std::uint8_t&& value() &&;
  ::result::CloneNoDefault& err() &;
  ::result::CloneNoDefault&& err() &&;
  ~Result() noexcept = default;

 private:
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;
  void check_has_ok();
  void check_has_err();

 private:
  unsigned char __storage[2];
};
#endif

namespace result {

// Generated from:
// cc_bindings_from_rs/test/enums/result.rs;l=44
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: result_golden :: CloneNoDefaultResult") alignas(1)
    [[clang::trivial_abi]] CloneNoDefaultResult final {
 public:
  // `result_golden::CloneNoDefaultResult` doesn't implement the `Default` trait
  CloneNoDefaultResult() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~CloneNoDefaultResult() = default;
  CloneNoDefaultResult(CloneNoDefaultResult&&) = default;
  CloneNoDefaultResult& operator=(CloneNoDefaultResult&&) = default;

  // `result_golden::CloneNoDefaultResult` doesn't implement the `Clone` trait
  CloneNoDefaultResult(const CloneNoDefaultResult&) = delete;
  CloneNoDefaultResult& operator=(const CloneNoDefaultResult&) = delete;
  CloneNoDefaultResult(::crubit::UnsafeRelocateTag,
                       CloneNoDefaultResult&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/result.rs;l=49
  static ::result::CloneNoDefaultResult new_(::std::uint8_t val);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/result.rs;l=45
    rs_std::Result<::result::CloneNoDefault, ::std::uint8_t> in_ok;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/result.rs;l=46
    rs_std::Result<::std::uint8_t, ::result::CloneNoDefault> in_err;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/enums/result.rs;l=110
::std::uint8_t take_result_clone_no_default_err(
    rs_std::Result<::std::uint8_t, ::result::CloneNoDefault> const& r);

}  // namespace result

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < u8 , :: result_golden :: CopyNoDefault >")
    rs_std::Result<::std::uint8_t, ::result::CopyNoDefault> {
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
  Result(::std::uint8_t&& ok) noexcept;
  Result& operator=(::std::uint8_t&& ok) noexcept;
  Result(rs_std::unexpected<::result::CopyNoDefault>&& err) noexcept;
  Result& operator=(rs_std::unexpected<::result::CopyNoDefault>&& err) noexcept;
  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs_std::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  ::std::uint8_t& value() &;
  ::std::uint8_t&& value() &&;
  ::result::CopyNoDefault& err() &;
  ::result::CopyNoDefault&& err() &&;
  ~Result() noexcept = default;

 private:
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;
  void check_has_ok();
  void check_has_err();

 private:
  unsigned char __storage[2];
};
#endif

namespace result {

// Generated from:
// cc_bindings_from_rs/test/enums/result.rs;l=30
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: result_golden :: CopyNoDefaultResult") alignas(1) [[clang::trivial_abi]]
CopyNoDefaultResult final {
 public:
  // `result_golden::CopyNoDefaultResult` doesn't implement the `Default` trait
  CopyNoDefaultResult() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~CopyNoDefaultResult() = default;
  CopyNoDefaultResult(CopyNoDefaultResult&&) = default;
  CopyNoDefaultResult& operator=(CopyNoDefaultResult&&) = default;

  // `result_golden::CopyNoDefaultResult` doesn't implement the `Clone` trait
  CopyNoDefaultResult(const CopyNoDefaultResult&) = delete;
  CopyNoDefaultResult& operator=(const CopyNoDefaultResult&) = delete;
  CopyNoDefaultResult(::crubit::UnsafeRelocateTag,
                      CopyNoDefaultResult&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/result.rs;l=35
  static ::result::CopyNoDefaultResult new_(::std::uint8_t val);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/result.rs;l=31
    rs_std::Result<::result::CopyNoDefault, ::std::uint8_t> in_ok;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/result.rs;l=32
    rs_std::Result<::std::uint8_t, ::result::CopyNoDefault> in_err;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace result

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < u8 , :: result_golden :: HasDefault >")
    rs_std::Result<::std::uint8_t, ::result::HasDefault> {
 public:
  // `core::result::Result` doesn't implement the `Clone` trait
  Result(const Result&) = delete;
  Result& operator=(const Result&) = delete;
  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  Result(Result&&) = delete;
  rs_std::Result<::std::uint8_t, ::result::HasDefault>& operator=(Result&&) =
      delete;
  Result(::crubit::UnsafeRelocateTag, Result&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Result(::std::uint8_t&& ok) noexcept;
  Result& operator=(::std::uint8_t&& ok) noexcept;
  Result(rs_std::unexpected<::result::HasDefault>&& err) noexcept;
  Result& operator=(rs_std::unexpected<::result::HasDefault>&& err) noexcept;
  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs_std::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  ::std::uint8_t& value() &;
  ::std::uint8_t&& value() &&;
  ::result::HasDefault& err() &;
  ::result::HasDefault&& err() &&;
  ~Result() noexcept;

 private:
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;
  void check_has_ok();
  void check_has_err();

 private:
  unsigned char __storage[24];
};
#endif

namespace result {

// Generated from:
// cc_bindings_from_rs/test/enums/result.rs;l=70
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: result_golden :: HasDefaultResult") alignas(8) [[clang::trivial_abi]]
HasDefaultResult final {
 public:
  // `result_golden::HasDefaultResult` doesn't implement the `Default` trait
  HasDefaultResult() = delete;

  // Drop::drop
  ~HasDefaultResult();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  HasDefaultResult(HasDefaultResult&&) = delete;
  ::result::HasDefaultResult& operator=(HasDefaultResult&&) = delete;
  // `result_golden::HasDefaultResult` doesn't implement the `Clone` trait
  HasDefaultResult(const HasDefaultResult&) = delete;
  HasDefaultResult& operator=(const HasDefaultResult&) = delete;
  HasDefaultResult(::crubit::UnsafeRelocateTag, HasDefaultResult&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/result.rs;l=75
  static ::result::HasDefaultResult new_(rs_std::StrRef val);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/result.rs;l=71
    rs_std::Result<::result::HasDefault, ::std::uint8_t> in_ok;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/result.rs;l=72
    rs_std::Result<::std::uint8_t, ::result::HasDefault> in_err;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace result

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < u8 , :: result_golden :: HasNoDefault >")
    rs_std::Result<::std::uint8_t, ::result::HasNoDefault> {
 public:
  // `core::result::Result` doesn't implement the `Clone` trait
  Result(const Result&) = delete;
  Result& operator=(const Result&) = delete;
  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  Result(Result&&) = delete;
  rs_std::Result<::std::uint8_t, ::result::HasNoDefault>& operator=(Result&&) =
      delete;
  Result(::crubit::UnsafeRelocateTag, Result&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Result(::std::uint8_t&& ok) noexcept;
  Result& operator=(::std::uint8_t&& ok) noexcept;
  // Move constructor not available for Err variant because
  // result_golden::HasNoDefault has not move constructor

  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs_std::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  ::std::uint8_t& value() &;
  ::std::uint8_t&& value() &&;
  ::result::HasNoDefault& err() &;
  ::result::HasNoDefault&& err() &&;
  ~Result() noexcept;

 private:
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;
  void check_has_ok();
  void check_has_err();

 private:
  unsigned char __storage[24];
};
#endif

namespace result {

// Generated from:
// cc_bindings_from_rs/test/enums/result.rs;l=91
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: result_golden :: HasNoDefaultResult") alignas(8) [[clang::trivial_abi]]
HasNoDefaultResult final {
 public:
  // `result_golden::HasNoDefaultResult` doesn't implement the `Default` trait
  HasNoDefaultResult() = delete;

  // Drop::drop
  ~HasNoDefaultResult();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  HasNoDefaultResult(HasNoDefaultResult&&) = delete;
  ::result::HasNoDefaultResult& operator=(HasNoDefaultResult&&) = delete;
  // `result_golden::HasNoDefaultResult` doesn't implement the `Clone` trait
  HasNoDefaultResult(const HasNoDefaultResult&) = delete;
  HasNoDefaultResult& operator=(const HasNoDefaultResult&) = delete;
  HasNoDefaultResult(::crubit::UnsafeRelocateTag, HasNoDefaultResult&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/result.rs;l=96
  static ::result::HasNoDefaultResult new_(rs_std::StrRef val);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/result.rs;l=92
    rs_std::Result<::result::HasNoDefault, ::std::uint8_t> in_ok;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/result.rs;l=93
    rs_std::Result<::std::uint8_t, ::result::HasNoDefault> in_err;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace result

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(1)
    CRUBIT_INTERNAL_RUST_TYPE("std :: result :: Result < u8 , u8 >")
        rs_std::Result<::std::uint8_t, ::std::uint8_t> {
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
  Result(::std::uint8_t&& ok) noexcept;
  Result& operator=(::std::uint8_t&& ok) noexcept;
  Result(rs_std::unexpected<::std::uint8_t>&& err) noexcept;
  Result& operator=(rs_std::unexpected<::std::uint8_t>&& err) noexcept;
  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs_std::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  ::std::uint8_t& value() &;
  ::std::uint8_t&& value() &&;
  ::std::uint8_t& err() &;
  ::std::uint8_t&& err() &&;
  ~Result() noexcept = default;

 private:
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;
  void check_has_ok();
  void check_has_err();

 private:
  unsigned char __storage[2];
};
#endif

namespace result {

// Generated from:
// cc_bindings_from_rs/test/enums/result.rs;l=131
rs_std::Result<::std::uint8_t, ::std::uint8_t> return_result_by_value();

// Generated from:
// cc_bindings_from_rs/test/enums/result.rs;l=124
::std::uint8_t take_result_by_value(
    rs_std::Result<::std::uint8_t, ::std::uint8_t> r);

static_assert(
    sizeof(CloneNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CloneNoDefault>);
static_assert(
    ::std::is_trivially_move_constructible_v<::result::CloneNoDefault>);
static_assert(::std::is_trivially_move_assignable_v<::result::CloneNoDefault>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(::result::CloneNoDefault const&,
                                     ::result::CloneNoDefault* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(::result::CloneNoDefault&,
                                           ::result::CloneNoDefault const&);
}
inline ::result::CloneNoDefault::CloneNoDefault(const CloneNoDefault& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline ::result::CloneNoDefault& ::result::CloneNoDefault::operator=(
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
    sizeof(CloneNoDefaultResult) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneNoDefaultResult) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CloneNoDefaultResult>);
static_assert(
    ::std::is_trivially_move_constructible_v<::result::CloneNoDefaultResult>);
static_assert(
    ::std::is_trivially_move_assignable_v<::result::CloneNoDefaultResult>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uint8_t,
                                   ::result::CloneNoDefaultResult* __ret_ptr);
}
inline ::result::CloneNoDefaultResult CloneNoDefaultResult::new_(
    ::std::uint8_t val) {
  crubit::Slot<::result::CloneNoDefaultResult> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void CloneNoDefaultResult::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CloneNoDefaultResult, in_ok));
  static_assert(2 == offsetof(CloneNoDefaultResult, in_err));
}
static_assert(
    sizeof(CopyNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CopyNoDefault) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CopyNoDefault>);
static_assert(
    ::std::is_trivially_move_constructible_v<::result::CopyNoDefault>);
static_assert(::std::is_trivially_move_assignable_v<::result::CopyNoDefault>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::result::CopyNoDefault>);
static_assert(::std::is_trivially_copy_assignable_v<::result::CopyNoDefault>);
inline void CopyNoDefault::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CopyNoDefault, val));
}
static_assert(
    sizeof(CopyNoDefaultResult) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CopyNoDefaultResult) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CopyNoDefaultResult>);
static_assert(
    ::std::is_trivially_move_constructible_v<::result::CopyNoDefaultResult>);
static_assert(
    ::std::is_trivially_move_assignable_v<::result::CopyNoDefaultResult>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uint8_t,
                                   ::result::CopyNoDefaultResult* __ret_ptr);
}
inline ::result::CopyNoDefaultResult CopyNoDefaultResult::new_(
    ::std::uint8_t val) {
  crubit::Slot<::result::CopyNoDefaultResult> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void CopyNoDefaultResult::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CopyNoDefaultResult, in_ok));
  static_assert(2 == offsetof(CopyNoDefaultResult, in_err));
}
static_assert(
    sizeof(GetsResult) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(GetsResult) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<GetsResult>);
static_assert(::std::is_trivially_move_constructible_v<::result::GetsResult>);
static_assert(::std::is_trivially_move_assignable_v<::result::GetsResult>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uint32_t,
                                   ::result::GetsResult* __ret_ptr);
}
inline ::result::GetsResult GetsResult::new_(::std::uint32_t val) {
  crubit::Slot<::result::GetsResult> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void GetsResult::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(GetsResult, value));
}
static_assert(
    sizeof(HasDefault) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(HasDefault) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::result::HasDefault* __ret_ptr);
}
inline ::result::HasDefault::HasDefault() {
  __crubit_internal::__crubit_thunk_default(this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::result::HasDefault&);
}
inline HasDefault::~HasDefault() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
inline ::result::HasDefault::HasDefault(HasDefault&& other) : HasDefault() {
  *this = ::std::move(other);
}
inline ::result::HasDefault& ::result::HasDefault::operator=(
    HasDefault&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(rs_std::StrRef,
                                   ::result::HasDefault* __ret_ptr);
}
inline ::result::HasDefault HasDefault::new_(rs_std::StrRef val) {
  crubit::Slot<::result::HasDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" rs_std::StrRef __crubit_thunk_val(::result::HasDefault const&);
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
    sizeof(HasDefaultResult) == 48,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(HasDefaultResult) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::result::HasDefaultResult&);
}
inline HasDefaultResult::~HasDefaultResult() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(rs_std::StrRef,
                                   ::result::HasDefaultResult* __ret_ptr);
}
inline ::result::HasDefaultResult HasDefaultResult::new_(rs_std::StrRef val) {
  crubit::Slot<::result::HasDefaultResult> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void HasDefaultResult::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(HasDefaultResult, in_ok));
  static_assert(24 == offsetof(HasDefaultResult, in_err));
}
static_assert(
    sizeof(HasNoDefault) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(HasNoDefault) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::result::HasNoDefault&);
}
inline HasNoDefault::~HasNoDefault() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
namespace __crubit_internal {
extern "C" rs_std::StrRef __crubit_thunk_val(::result::HasNoDefault const&);
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
    sizeof(HasNoDefaultResult) == 48,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(HasNoDefaultResult) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::result::HasNoDefaultResult&);
}
inline HasNoDefaultResult::~HasNoDefaultResult() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(rs_std::StrRef,
                                   ::result::HasNoDefaultResult* __ret_ptr);
}
inline ::result::HasNoDefaultResult HasNoDefaultResult::new_(
    rs_std::StrRef val) {
  crubit::Slot<::result::HasNoDefaultResult> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void HasNoDefaultResult::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(HasNoDefaultResult, in_ok));
  static_assert(24 == offsetof(HasNoDefaultResult, in_err));
}
static_assert(
    sizeof(NestedResult) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NestedResult) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NestedResult>);
static_assert(::std::is_trivially_move_constructible_v<::result::NestedResult>);
static_assert(::std::is_trivially_move_assignable_v<::result::NestedResult>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::uint32_t,
                                   ::result::NestedResult* __ret_ptr);
}
inline ::result::NestedResult NestedResult::new_(::std::uint32_t val) {
  crubit::Slot<::result::NestedResult> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void NestedResult::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NestedResult, in_ok));
  static_assert(8 == offsetof(NestedResult, in_err));
}
static_assert(
    sizeof(ResultWithSizeTypes) == 64,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(ResultWithSizeTypes) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<ResultWithSizeTypes>);
static_assert(
    ::std::is_trivially_move_constructible_v<::result::ResultWithSizeTypes>);
static_assert(
    ::std::is_trivially_move_assignable_v<::result::ResultWithSizeTypes>);
inline void ResultWithSizeTypes::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ResultWithSizeTypes, uval_in_ok));
  static_assert(16 == offsetof(ResultWithSizeTypes, uval_in_err));
  static_assert(32 == offsetof(ResultWithSizeTypes, ival_in_ok));
  static_assert(48 == offsetof(ResultWithSizeTypes, ival_in_err));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uresult_uby_uvalue(
    rs_std::Result<::std::uint8_t, ::std::uint8_t>* __ret_ptr);
}
inline rs_std::Result<::std::uint8_t, ::std::uint8_t> return_result_by_value() {
  crubit::Slot<rs_std::Result<::std::uint8_t, ::std::uint8_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_return_uresult_uby_uvalue(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::uint8_t __crubit_thunk_take_uresult_uby_uvalue(
    rs_std::Result<::std::uint8_t, ::std::uint8_t>*);
}
inline ::std::uint8_t take_result_by_value(
    rs_std::Result<::std::uint8_t, ::std::uint8_t> r) {
  return __crubit_internal::__crubit_thunk_take_uresult_uby_uvalue(&r);
}

namespace __crubit_internal {
extern "C" ::std::uint8_t __crubit_thunk_take_uresult_uclone_uno_udefault_uerr(
    rs_std::Result<::std::uint8_t, ::result::CloneNoDefault> const&);
}
inline ::std::uint8_t take_result_clone_no_default_err(
    rs_std::Result<::std::uint8_t, ::result::CloneNoDefault> const& r) {
  return __crubit_internal::
      __crubit_thunk_take_uresult_uclone_uno_udefault_uerr(r);
}

namespace __crubit_internal {
extern "C" ::std::uint8_t __crubit_thunk_take_uresult_ucopy_uno_udefault_uok(
    rs_std::Result<::result::CopyNoDefault, ::std::uint8_t> const&);
}
inline ::std::uint8_t take_result_copy_no_default_ok(
    rs_std::Result<::result::CopyNoDefault, ::std::uint8_t> const& r) {
  return __crubit_internal::__crubit_thunk_take_uresult_ucopy_uno_udefault_uok(
      r);
}

namespace __crubit_internal {
extern "C" rs_std::StrRef __crubit_thunk_take_uresult_uhas_udefault(
    rs_std::Result<::result::HasDefault, ::std::uint8_t> const* $(__anon1)
        crubit_nonnull);
}
inline rs_std::StrRef take_result_has_default(
    rs_std::Result<::result::HasDefault, ::std::uint8_t> const* $(__anon1)
        crubit_nonnull r CRUBIT_LIFETIME_BOUND) {
  return __crubit_internal::__crubit_thunk_take_uresult_uhas_udefault(r);
}

}  // namespace result

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    rs_std::Result<::result::CloneNoDefault, ::std::uint8_t> const&,
    rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(
    rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>&,
    rs_std::Result<::result::CloneNoDefault, ::std::uint8_t> const&);
}
inline rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::Result(
    const Result& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>& rs_std::Result<
    ::result::CloneNoDefault, ::std::uint8_t>::operator=(const Result& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>>);
inline rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::Result(
    ::result::CloneNoDefault&& ok) noexcept {
  set_tag(0);
  ::std::construct_at(
      reinterpret_cast<::result::CloneNoDefault*>(__storage + 1),
      ::std::move(ok));
}
inline rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>&
rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::operator=(
    ::result::CloneNoDefault&& ok) noexcept {
  if (!has_value()) {
    ::std::destroy_at(reinterpret_cast<::std::uint8_t*>(__storage + 1));
    set_tag(0);
    ::std::construct_at(
        reinterpret_cast<::result::CloneNoDefault*>(__storage + 1),
        ::std::move(ok));
  } else {
    set_tag(0);
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::result::CloneNoDefault*>(__storage + 1),
        ::std::move(ok));
  }
  return *this;
}

inline rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::Result(
    rs_std::unexpected<::std::uint8_t>&& err) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 1),
                      ::std::move(err.error()));
}
inline rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>&
rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::operator=(
    rs_std::unexpected<::std::uint8_t>&& err) noexcept {
  if (has_value()) {
    ::std::destroy_at(__storage + 1);
    set_tag(1);
    ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 1),
                        ::std::move(err.error()));
  } else {
    set_tag(1);
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::uint8_t*>(__storage + 1),
        ::std::move(err.error()));
  }
  return *this;
}

template <typename... Args>
inline rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::Result(
    std::in_place_t, Args&&... args) {
  set_tag(0);
  std::construct_at(__storage + 1, std::forward<Args>(args)...);
}
template <typename... Args>
inline rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::Result(
    rs_std::unexpect_t, Args&&... args) {
  set_tag(1);
  std::construct_at(__storage + 1, std::forward<Args>(args)...);
}
inline constexpr rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::
operator bool() const noexcept {
  return has_value();
}
inline constexpr bool rs_std::Result<
    ::result::CloneNoDefault, ::std::uint8_t>::has_value() const noexcept {
  return tag() == 0;
}
inline ::result::CloneNoDefault&
rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::value() & {
  check_has_ok();
  return *reinterpret_cast<::result::CloneNoDefault*>(__storage + 1);
}
inline ::result::CloneNoDefault&&
rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::value() && {
  check_has_ok();
  return ::std::move(
      *reinterpret_cast<::result::CloneNoDefault*>(__storage + 1));
}
inline ::std::uint8_t&
rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::err() & {
  check_has_err();
  return *reinterpret_cast<::std::uint8_t*>(__storage + 1);
}
inline ::std::uint8_t&&
rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::err() && {
  check_has_err();
  return ::std::move(*reinterpret_cast<::std::uint8_t*>(__storage + 1));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>>);
inline constexpr ::std::uint8_t rs_std::Result<
    ::result::CloneNoDefault, ::std::uint8_t>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void
rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

inline void
rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::check_has_ok() {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs_std::Result";
}
inline void
rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::check_has_err() {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs_std::Result";
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>>);
inline rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>::Result(
    ::result::CopyNoDefault&& ok) noexcept {
  set_tag(0);
  ::std::construct_at(reinterpret_cast<::result::CopyNoDefault*>(__storage + 1),
                      ::std::move(ok));
}
inline rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>&
rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>::operator=(
    ::result::CopyNoDefault&& ok) noexcept {
  if (!has_value()) {
    ::std::destroy_at(reinterpret_cast<::std::uint8_t*>(__storage + 1));
    set_tag(0);
    ::std::construct_at(
        reinterpret_cast<::result::CopyNoDefault*>(__storage + 1),
        ::std::move(ok));
  } else {
    set_tag(0);
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::result::CopyNoDefault*>(__storage + 1),
        ::std::move(ok));
  }
  return *this;
}

inline rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>::Result(
    rs_std::unexpected<::std::uint8_t>&& err) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 1),
                      ::std::move(err.error()));
}
inline rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>&
rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>::operator=(
    rs_std::unexpected<::std::uint8_t>&& err) noexcept {
  if (has_value()) {
    ::std::destroy_at(__storage + 1);
    set_tag(1);
    ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 1),
                        ::std::move(err.error()));
  } else {
    set_tag(1);
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::uint8_t*>(__storage + 1),
        ::std::move(err.error()));
  }
  return *this;
}

template <typename... Args>
inline rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>::Result(
    std::in_place_t, Args&&... args) {
  set_tag(0);
  std::construct_at(__storage + 1, std::forward<Args>(args)...);
}
template <typename... Args>
inline rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>::Result(
    rs_std::unexpect_t, Args&&... args) {
  set_tag(1);
  std::construct_at(__storage + 1, std::forward<Args>(args)...);
}
inline constexpr rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>::
operator bool() const noexcept {
  return has_value();
}
inline constexpr bool rs_std::Result<
    ::result::CopyNoDefault, ::std::uint8_t>::has_value() const noexcept {
  return tag() == 0;
}
inline ::result::CopyNoDefault&
rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>::value() & {
  check_has_ok();
  return *reinterpret_cast<::result::CopyNoDefault*>(__storage + 1);
}
inline ::result::CopyNoDefault&&
rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>::value() && {
  check_has_ok();
  return ::std::move(
      *reinterpret_cast<::result::CopyNoDefault*>(__storage + 1));
}
inline ::std::uint8_t&
rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>::err() & {
  check_has_err();
  return *reinterpret_cast<::std::uint8_t*>(__storage + 1);
}
inline ::std::uint8_t&&
rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>::err() && {
  check_has_err();
  return ::std::move(*reinterpret_cast<::std::uint8_t*>(__storage + 1));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>>);
inline constexpr ::std::uint8_t
rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void
rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

inline void
rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>::check_has_ok() {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs_std::Result";
}
inline void
rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>::check_has_err() {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs_std::Result";
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
inline rs_std::Result<::result::HasDefault, ::std::uint8_t>::Result(
    ::result::HasDefault&& ok) noexcept {
  ::std::construct_at(reinterpret_cast<::result::HasDefault*>(__storage),
                      ::std::move(ok));
}
inline rs_std::Result<::result::HasDefault, ::std::uint8_t>&
rs_std::Result<::result::HasDefault, ::std::uint8_t>::operator=(
    ::result::HasDefault&& ok) noexcept {
  if (!has_value()) {
    ::std::destroy_at(reinterpret_cast<::std::uint8_t*>(__storage + 8));
    ::std::construct_at(reinterpret_cast<::result::HasDefault*>(__storage),
                        ::std::move(ok));
  } else {
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::result::HasDefault*>(__storage), ::std::move(ok));
  }
  return *this;
}

inline rs_std::Result<::result::HasDefault, ::std::uint8_t>::Result(
    rs_std::unexpected<::std::uint8_t>&& err) noexcept {
  set_tag(UINT64_C(18446744073709551615));
  ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 8),
                      ::std::move(err.error()));
}
inline rs_std::Result<::result::HasDefault, ::std::uint8_t>&
rs_std::Result<::result::HasDefault, ::std::uint8_t>::operator=(
    rs_std::unexpected<::std::uint8_t>&& err) noexcept {
  if (has_value()) {
    ::std::destroy_at(__storage);
    set_tag(UINT64_C(18446744073709551615));
    ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 8),
                        ::std::move(err.error()));
  } else {
    set_tag(UINT64_C(18446744073709551615));
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::uint8_t*>(__storage + 8),
        ::std::move(err.error()));
  }
  return *this;
}

template <typename... Args>
inline rs_std::Result<::result::HasDefault, ::std::uint8_t>::Result(
    std::in_place_t, Args&&... args) {
  std::construct_at(__storage, std::forward<Args>(args)...);
}
template <typename... Args>
inline rs_std::Result<::result::HasDefault, ::std::uint8_t>::Result(
    rs_std::unexpect_t, Args&&... args) {
  set_tag(UINT64_C(18446744073709551615));
  std::construct_at(__storage + 8, std::forward<Args>(args)...);
}
inline constexpr rs_std::Result<::result::HasDefault, ::std::uint8_t>::
operator bool() const noexcept {
  return has_value();
}
inline constexpr bool rs_std::Result<
    ::result::HasDefault, ::std::uint8_t>::has_value() const noexcept {
  return tag() != UINT64_C(18446744073709551615);
}
inline ::result::HasDefault&
rs_std::Result<::result::HasDefault, ::std::uint8_t>::value() & {
  check_has_ok();
  return *reinterpret_cast<::result::HasDefault*>(__storage);
}
inline ::result::HasDefault&&
rs_std::Result<::result::HasDefault, ::std::uint8_t>::value() && {
  check_has_ok();
  return ::std::move(*reinterpret_cast<::result::HasDefault*>(__storage));
}
inline ::std::uint8_t&
rs_std::Result<::result::HasDefault, ::std::uint8_t>::err() & {
  check_has_err();
  return *reinterpret_cast<::std::uint8_t*>(__storage + 8);
}
inline ::std::uint8_t&&
rs_std::Result<::result::HasDefault, ::std::uint8_t>::err() && {
  check_has_err();
  return ::std::move(*reinterpret_cast<::std::uint8_t*>(__storage + 8));
}
inline rs_std::Result<::result::HasDefault,
                      ::std::uint8_t>::~Result() noexcept {
  if (has_value()) {
    ::std::destroy_at(reinterpret_cast<::result::HasDefault*>(__storage));
  } else {
    ::std::destroy_at(reinterpret_cast<::std::uint8_t*>(__storage + 8));
  }
}
inline constexpr ::std::uint64_t
rs_std::Result<::result::HasDefault, ::std::uint8_t>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void
rs_std::Result<::result::HasDefault, ::std::uint8_t>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint64_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

inline void
rs_std::Result<::result::HasDefault, ::std::uint8_t>::check_has_ok() {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs_std::Result";
}
inline void
rs_std::Result<::result::HasDefault, ::std::uint8_t>::check_has_err() {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs_std::Result";
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e

inline rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::Result(
    rs_std::unexpected<::std::uint8_t>&& err) noexcept {
  set_tag(UINT64_C(18446744073709551615));
  ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 8),
                      ::std::move(err.error()));
}
inline rs_std::Result<::result::HasNoDefault, ::std::uint8_t>&
rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::operator=(
    rs_std::unexpected<::std::uint8_t>&& err) noexcept {
  if (has_value()) {
    ::std::destroy_at(__storage);
    set_tag(UINT64_C(18446744073709551615));
    ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 8),
                        ::std::move(err.error()));
  } else {
    set_tag(UINT64_C(18446744073709551615));
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::uint8_t*>(__storage + 8),
        ::std::move(err.error()));
  }
  return *this;
}

template <typename... Args>
inline rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::Result(
    std::in_place_t, Args&&... args) {
  std::construct_at(__storage, std::forward<Args>(args)...);
}
template <typename... Args>
inline rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::Result(
    rs_std::unexpect_t, Args&&... args) {
  set_tag(UINT64_C(18446744073709551615));
  std::construct_at(__storage + 8, std::forward<Args>(args)...);
}
inline constexpr rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::
operator bool() const noexcept {
  return has_value();
}
inline constexpr bool rs_std::Result<
    ::result::HasNoDefault, ::std::uint8_t>::has_value() const noexcept {
  return tag() != UINT64_C(18446744073709551615);
}
inline ::result::HasNoDefault&
rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::value() & {
  check_has_ok();
  return *reinterpret_cast<::result::HasNoDefault*>(__storage);
}
inline ::result::HasNoDefault&&
rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::value() && {
  check_has_ok();
  return ::std::move(*reinterpret_cast<::result::HasNoDefault*>(__storage));
}
inline ::std::uint8_t&
rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::err() & {
  check_has_err();
  return *reinterpret_cast<::std::uint8_t*>(__storage + 8);
}
inline ::std::uint8_t&&
rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::err() && {
  check_has_err();
  return ::std::move(*reinterpret_cast<::std::uint8_t*>(__storage + 8));
}
inline rs_std::Result<::result::HasNoDefault,
                      ::std::uint8_t>::~Result() noexcept {
  if (has_value()) {
    ::std::destroy_at(reinterpret_cast<::result::HasNoDefault*>(__storage));
  } else {
    ::std::destroy_at(reinterpret_cast<::std::uint8_t*>(__storage + 8));
  }
}
inline constexpr ::std::uint64_t
rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void
rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint64_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

inline void
rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::check_has_ok() {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs_std::Result";
}
inline void
rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::check_has_err() {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs_std::Result";
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<rs_std::Result<
        rs_std::Result<::std::uint32_t, ::std::uint32_t>, ::std::uint32_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<rs_std::Result<
        rs_std::Result<::std::uint32_t, ::std::uint32_t>, ::std::uint32_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<rs_std::Result<
        rs_std::Result<::std::uint32_t, ::std::uint32_t>, ::std::uint32_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<rs_std::Result<
        rs_std::Result<::std::uint32_t, ::std::uint32_t>, ::std::uint32_t>>);
inline rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
                      ::std::uint32_t>::
    Result(rs_std::Result<::std::uint32_t, ::std::uint32_t>&& ok) noexcept {
  ::std::construct_at(
      reinterpret_cast<rs_std::Result<::std::uint32_t, ::std::uint32_t>*>(
          __storage),
      ::std::move(ok));
}
inline rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
                      ::std::uint32_t>&
rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
               ::std::uint32_t>::
operator=(rs_std::Result<::std::uint32_t, ::std::uint32_t>&& ok) noexcept {
  if (!has_value()) {
    ::std::destroy_at(reinterpret_cast<::std::uint32_t*>(__storage + 4));
    ::std::construct_at(
        reinterpret_cast<rs_std::Result<::std::uint32_t, ::std::uint32_t>*>(
            __storage),
        ::std::move(ok));
  } else {
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<rs_std::Result<::std::uint32_t, ::std::uint32_t>*>(
            __storage),
        ::std::move(ok));
  }
  return *this;
}

inline rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
                      ::std::uint32_t>::
    Result(rs_std::unexpected<::std::uint32_t>&& err) noexcept {
  set_tag(UINT32_C(4294967295));
  ::std::construct_at(reinterpret_cast<::std::uint32_t*>(__storage + 4),
                      ::std::move(err.error()));
}
inline rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
                      ::std::uint32_t>&
rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
               ::std::uint32_t>::operator=(rs_std::unexpected<::std::uint32_t>&&
                                               err) noexcept {
  if (has_value()) {
    ::std::destroy_at(__storage);
    set_tag(UINT32_C(4294967295));
    ::std::construct_at(reinterpret_cast<::std::uint32_t*>(__storage + 4),
                        ::std::move(err.error()));
  } else {
    set_tag(UINT32_C(4294967295));
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::uint32_t*>(__storage + 4),
        ::std::move(err.error()));
  }
  return *this;
}

template <typename... Args>
inline rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
                      ::std::uint32_t>::Result(std::in_place_t,
                                               Args&&... args) {
  std::construct_at(__storage, std::forward<Args>(args)...);
}
template <typename... Args>
inline rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
                      ::std::uint32_t>::Result(rs_std::unexpect_t,
                                               Args&&... args) {
  set_tag(UINT32_C(4294967295));
  std::construct_at(__storage + 4, std::forward<Args>(args)...);
}
inline constexpr rs_std::Result<
    rs_std::Result<::std::uint32_t, ::std::uint32_t>, ::std::uint32_t>::
operator bool() const noexcept {
  return has_value();
}
inline constexpr bool
rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
               ::std::uint32_t>::has_value() const noexcept {
  return tag() != UINT32_C(4294967295);
}
inline rs_std::Result<::std::uint32_t, ::std::uint32_t>&
rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
               ::std::uint32_t>::value() & {
  check_has_ok();
  return *reinterpret_cast<rs_std::Result<::std::uint32_t, ::std::uint32_t>*>(
      __storage);
}
inline rs_std::Result<::std::uint32_t, ::std::uint32_t>&&
rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
               ::std::uint32_t>::value() && {
  check_has_ok();
  return ::std::move(
      *reinterpret_cast<rs_std::Result<::std::uint32_t, ::std::uint32_t>*>(
          __storage));
}
inline ::std::uint32_t&
rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
               ::std::uint32_t>::err() & {
  check_has_err();
  return *reinterpret_cast<::std::uint32_t*>(__storage + 4);
}
inline ::std::uint32_t&&
rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
               ::std::uint32_t>::err() && {
  check_has_err();
  return ::std::move(*reinterpret_cast<::std::uint32_t*>(__storage + 4));
}
static_assert(
    ::std::is_trivially_destructible_v<rs_std::Result<
        rs_std::Result<::std::uint32_t, ::std::uint32_t>, ::std::uint32_t>>);
inline constexpr ::std::uint32_t
rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
               ::std::uint32_t>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void
rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
               ::std::uint32_t>::set_tag(::std::uint32_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint32_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

inline void rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
                           ::std::uint32_t>::check_has_ok() {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs_std::Result";
}
inline void rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
                           ::std::uint32_t>::check_has_err() {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs_std::Result";
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<rs_std::Result<
        ::std::uint32_t, rs_std::Result<::std::uint32_t, ::std::uint32_t>>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<rs_std::Result<
        ::std::uint32_t, rs_std::Result<::std::uint32_t, ::std::uint32_t>>>);
static_assert(
    ::std::is_trivially_move_constructible_v<rs_std::Result<
        ::std::uint32_t, rs_std::Result<::std::uint32_t, ::std::uint32_t>>>);
static_assert(
    ::std::is_trivially_move_assignable_v<rs_std::Result<
        ::std::uint32_t, rs_std::Result<::std::uint32_t, ::std::uint32_t>>>);
inline rs_std::Result<::std::uint32_t,
                      rs_std::Result<::std::uint32_t, ::std::uint32_t>>::
    Result(::std::uint32_t&& ok) noexcept {
  set_tag(UINT32_C(4294967295));
  ::std::construct_at(reinterpret_cast<::std::uint32_t*>(__storage + 4),
                      ::std::move(ok));
}
inline rs_std::Result<::std::uint32_t,
                      rs_std::Result<::std::uint32_t, ::std::uint32_t>>&
rs_std::Result<::std::uint32_t,
               rs_std::Result<::std::uint32_t, ::std::uint32_t>>::
operator=(::std::uint32_t&& ok) noexcept {
  if (!has_value()) {
    ::std::destroy_at(
        reinterpret_cast<rs_std::Result<::std::uint32_t, ::std::uint32_t>*>(
            __storage));
    set_tag(UINT32_C(4294967295));
    ::std::construct_at(reinterpret_cast<::std::uint32_t*>(__storage + 4),
                        ::std::move(ok));
  } else {
    set_tag(UINT32_C(4294967295));
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::uint32_t*>(__storage + 4), ::std::move(ok));
  }
  return *this;
}

inline rs_std::Result<::std::uint32_t,
                      rs_std::Result<::std::uint32_t, ::std::uint32_t>>::
    Result(
        rs_std::unexpected<rs_std::Result<::std::uint32_t, ::std::uint32_t>>&&
            err) noexcept {
  ::std::construct_at(
      reinterpret_cast<rs_std::Result<::std::uint32_t, ::std::uint32_t>*>(
          __storage),
      ::std::move(err.error()));
}
inline rs_std::Result<::std::uint32_t,
                      rs_std::Result<::std::uint32_t, ::std::uint32_t>>&
rs_std::Result<::std::uint32_t,
               rs_std::Result<::std::uint32_t, ::std::uint32_t>>::
operator=(rs_std::unexpected<rs_std::Result<::std::uint32_t, ::std::uint32_t>>&&
              err) noexcept {
  if (has_value()) {
    ::std::destroy_at(__storage + 4);
    ::std::construct_at(
        reinterpret_cast<rs_std::Result<::std::uint32_t, ::std::uint32_t>*>(
            __storage),
        ::std::move(err.error()));
  } else {
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<rs_std::Result<::std::uint32_t, ::std::uint32_t>*>(
            __storage),
        ::std::move(err.error()));
  }
  return *this;
}

template <typename... Args>
inline rs_std::Result<
    ::std::uint32_t,
    rs_std::Result<::std::uint32_t, ::std::uint32_t>>::Result(std::in_place_t,
                                                              Args&&... args) {
  set_tag(UINT32_C(4294967295));
  std::construct_at(__storage + 4, std::forward<Args>(args)...);
}
template <typename... Args>
inline rs_std::Result<::std::uint32_t,
                      rs_std::Result<::std::uint32_t, ::std::uint32_t>>::
    Result(rs_std::unexpect_t, Args&&... args) {
  std::construct_at(__storage, std::forward<Args>(args)...);
}
inline constexpr rs_std::Result<
    ::std::uint32_t, rs_std::Result<::std::uint32_t, ::std::uint32_t>>::
operator bool() const noexcept {
  return has_value();
}
inline constexpr bool
rs_std::Result<::std::uint32_t,
               rs_std::Result<::std::uint32_t, ::std::uint32_t>>::has_value()
    const noexcept {
  return tag() == UINT32_C(4294967295);
}
inline ::std::uint32_t&
rs_std::Result<::std::uint32_t,
               rs_std::Result<::std::uint32_t, ::std::uint32_t>>::value() & {
  check_has_ok();
  return *reinterpret_cast<::std::uint32_t*>(__storage + 4);
}
inline ::std::uint32_t&&
rs_std::Result<::std::uint32_t,
               rs_std::Result<::std::uint32_t, ::std::uint32_t>>::value() && {
  check_has_ok();
  return ::std::move(*reinterpret_cast<::std::uint32_t*>(__storage + 4));
}
inline rs_std::Result<::std::uint32_t, ::std::uint32_t>&
rs_std::Result<::std::uint32_t,
               rs_std::Result<::std::uint32_t, ::std::uint32_t>>::err() & {
  check_has_err();
  return *reinterpret_cast<rs_std::Result<::std::uint32_t, ::std::uint32_t>*>(
      __storage);
}
inline rs_std::Result<::std::uint32_t, ::std::uint32_t>&&
rs_std::Result<::std::uint32_t,
               rs_std::Result<::std::uint32_t, ::std::uint32_t>>::err() && {
  check_has_err();
  return ::std::move(
      *reinterpret_cast<rs_std::Result<::std::uint32_t, ::std::uint32_t>*>(
          __storage));
}
static_assert(
    ::std::is_trivially_destructible_v<rs_std::Result<
        ::std::uint32_t, rs_std::Result<::std::uint32_t, ::std::uint32_t>>>);
inline constexpr ::std::uint32_t rs_std::Result<
    ::std::uint32_t, rs_std::Result<::std::uint32_t, ::std::uint32_t>>::tag()
    const& noexcept {
  std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void rs_std::Result<
    ::std::uint32_t, rs_std::Result<::std::uint32_t, ::std::uint32_t>>::
    set_tag(::std::uint32_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint32_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

inline void rs_std::Result<
    ::std::uint32_t,
    rs_std::Result<::std::uint32_t, ::std::uint32_t>>::check_has_ok() {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs_std::Result";
}
inline void rs_std::Result<
    ::std::uint32_t,
    rs_std::Result<::std::uint32_t, ::std::uint32_t>>::check_has_err() {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs_std::Result";
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Result<::std::uint32_t, ::std::uint32_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Result<::std::uint32_t, ::std::uint32_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<::std::uint32_t, ::std::uint32_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<::std::uint32_t, ::std::uint32_t>>);
inline rs_std::Result<::std::uint32_t, ::std::uint32_t>::Result(
    ::std::uint32_t&& ok) noexcept {
  set_tag(0);
  ::std::construct_at(reinterpret_cast<::std::uint32_t*>(__storage + 4),
                      ::std::move(ok));
}
inline rs_std::Result<::std::uint32_t, ::std::uint32_t>&
rs_std::Result<::std::uint32_t, ::std::uint32_t>::operator=(
    ::std::uint32_t&& ok) noexcept {
  if (!has_value()) {
    ::std::destroy_at(reinterpret_cast<::std::uint32_t*>(__storage + 4));
    set_tag(0);
    ::std::construct_at(reinterpret_cast<::std::uint32_t*>(__storage + 4),
                        ::std::move(ok));
  } else {
    set_tag(0);
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::uint32_t*>(__storage + 4), ::std::move(ok));
  }
  return *this;
}

inline rs_std::Result<::std::uint32_t, ::std::uint32_t>::Result(
    rs_std::unexpected<::std::uint32_t>&& err) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::std::uint32_t*>(__storage + 4),
                      ::std::move(err.error()));
}
inline rs_std::Result<::std::uint32_t, ::std::uint32_t>&
rs_std::Result<::std::uint32_t, ::std::uint32_t>::operator=(
    rs_std::unexpected<::std::uint32_t>&& err) noexcept {
  if (has_value()) {
    ::std::destroy_at(__storage + 4);
    set_tag(1);
    ::std::construct_at(reinterpret_cast<::std::uint32_t*>(__storage + 4),
                        ::std::move(err.error()));
  } else {
    set_tag(1);
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::uint32_t*>(__storage + 4),
        ::std::move(err.error()));
  }
  return *this;
}

template <typename... Args>
inline rs_std::Result<::std::uint32_t, ::std::uint32_t>::Result(
    std::in_place_t, Args&&... args) {
  set_tag(0);
  std::construct_at(__storage + 4, std::forward<Args>(args)...);
}
template <typename... Args>
inline rs_std::Result<::std::uint32_t, ::std::uint32_t>::Result(
    rs_std::unexpect_t, Args&&... args) {
  set_tag(1);
  std::construct_at(__storage + 4, std::forward<Args>(args)...);
}
inline constexpr rs_std::Result<::std::uint32_t, ::std::uint32_t>::
operator bool() const noexcept {
  return has_value();
}
inline constexpr bool
rs_std::Result<::std::uint32_t, ::std::uint32_t>::has_value() const noexcept {
  return tag() == 0;
}
inline ::std::uint32_t&
rs_std::Result<::std::uint32_t, ::std::uint32_t>::value() & {
  check_has_ok();
  return *reinterpret_cast<::std::uint32_t*>(__storage + 4);
}
inline ::std::uint32_t&&
rs_std::Result<::std::uint32_t, ::std::uint32_t>::value() && {
  check_has_ok();
  return ::std::move(*reinterpret_cast<::std::uint32_t*>(__storage + 4));
}
inline ::std::uint32_t&
rs_std::Result<::std::uint32_t, ::std::uint32_t>::err() & {
  check_has_err();
  return *reinterpret_cast<::std::uint32_t*>(__storage + 4);
}
inline ::std::uint32_t&&
rs_std::Result<::std::uint32_t, ::std::uint32_t>::err() && {
  check_has_err();
  return ::std::move(*reinterpret_cast<::std::uint32_t*>(__storage + 4));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Result<::std::uint32_t, ::std::uint32_t>>);
inline constexpr ::std::uint32_t
rs_std::Result<::std::uint32_t, ::std::uint32_t>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void rs_std::Result<::std::uint32_t, ::std::uint32_t>::set_tag(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint32_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

inline void rs_std::Result<::std::uint32_t, ::std::uint32_t>::check_has_ok() {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs_std::Result";
}
inline void rs_std::Result<::std::uint32_t, ::std::uint32_t>::check_has_err() {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs_std::Result";
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    rs_std::Result<::std::uint8_t, ::result::CloneNoDefault> const&,
    rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(
    rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>&,
    rs_std::Result<::std::uint8_t, ::result::CloneNoDefault> const&);
}
inline rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::Result(
    const Result& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>& rs_std::Result<
    ::std::uint8_t, ::result::CloneNoDefault>::operator=(const Result& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>>);
inline rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::Result(
    ::std::uint8_t&& ok) noexcept {
  set_tag(0);
  ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 1),
                      ::std::move(ok));
}
inline rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>&
rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::operator=(
    ::std::uint8_t&& ok) noexcept {
  if (!has_value()) {
    ::std::destroy_at(
        reinterpret_cast<::result::CloneNoDefault*>(__storage + 1));
    set_tag(0);
    ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 1),
                        ::std::move(ok));
  } else {
    set_tag(0);
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::uint8_t*>(__storage + 1), ::std::move(ok));
  }
  return *this;
}

inline rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::Result(
    rs_std::unexpected<::result::CloneNoDefault>&& err) noexcept {
  set_tag(1);
  ::std::construct_at(
      reinterpret_cast<::result::CloneNoDefault*>(__storage + 1),
      ::std::move(err.error()));
}
inline rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>&
rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::operator=(
    rs_std::unexpected<::result::CloneNoDefault>&& err) noexcept {
  if (has_value()) {
    ::std::destroy_at(__storage + 1);
    set_tag(1);
    ::std::construct_at(
        reinterpret_cast<::result::CloneNoDefault*>(__storage + 1),
        ::std::move(err.error()));
  } else {
    set_tag(1);
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::result::CloneNoDefault*>(__storage + 1),
        ::std::move(err.error()));
  }
  return *this;
}

template <typename... Args>
inline rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::Result(
    std::in_place_t, Args&&... args) {
  set_tag(0);
  std::construct_at(__storage + 1, std::forward<Args>(args)...);
}
template <typename... Args>
inline rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::Result(
    rs_std::unexpect_t, Args&&... args) {
  set_tag(1);
  std::construct_at(__storage + 1, std::forward<Args>(args)...);
}
inline constexpr rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::
operator bool() const noexcept {
  return has_value();
}
inline constexpr bool rs_std::Result<
    ::std::uint8_t, ::result::CloneNoDefault>::has_value() const noexcept {
  return tag() == 0;
}
inline ::std::uint8_t&
rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::value() & {
  check_has_ok();
  return *reinterpret_cast<::std::uint8_t*>(__storage + 1);
}
inline ::std::uint8_t&&
rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::value() && {
  check_has_ok();
  return ::std::move(*reinterpret_cast<::std::uint8_t*>(__storage + 1));
}
inline ::result::CloneNoDefault&
rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::err() & {
  check_has_err();
  return *reinterpret_cast<::result::CloneNoDefault*>(__storage + 1);
}
inline ::result::CloneNoDefault&&
rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::err() && {
  check_has_err();
  return ::std::move(
      *reinterpret_cast<::result::CloneNoDefault*>(__storage + 1));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>>);
inline constexpr ::std::uint8_t rs_std::Result<
    ::std::uint8_t, ::result::CloneNoDefault>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void
rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

inline void
rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::check_has_ok() {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs_std::Result";
}
inline void
rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::check_has_err() {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs_std::Result";
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>>);
inline rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>::Result(
    ::std::uint8_t&& ok) noexcept {
  set_tag(0);
  ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 1),
                      ::std::move(ok));
}
inline rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>&
rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>::operator=(
    ::std::uint8_t&& ok) noexcept {
  if (!has_value()) {
    ::std::destroy_at(
        reinterpret_cast<::result::CopyNoDefault*>(__storage + 1));
    set_tag(0);
    ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 1),
                        ::std::move(ok));
  } else {
    set_tag(0);
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::uint8_t*>(__storage + 1), ::std::move(ok));
  }
  return *this;
}

inline rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>::Result(
    rs_std::unexpected<::result::CopyNoDefault>&& err) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::result::CopyNoDefault*>(__storage + 1),
                      ::std::move(err.error()));
}
inline rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>&
rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>::operator=(
    rs_std::unexpected<::result::CopyNoDefault>&& err) noexcept {
  if (has_value()) {
    ::std::destroy_at(__storage + 1);
    set_tag(1);
    ::std::construct_at(
        reinterpret_cast<::result::CopyNoDefault*>(__storage + 1),
        ::std::move(err.error()));
  } else {
    set_tag(1);
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::result::CopyNoDefault*>(__storage + 1),
        ::std::move(err.error()));
  }
  return *this;
}

template <typename... Args>
inline rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>::Result(
    std::in_place_t, Args&&... args) {
  set_tag(0);
  std::construct_at(__storage + 1, std::forward<Args>(args)...);
}
template <typename... Args>
inline rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>::Result(
    rs_std::unexpect_t, Args&&... args) {
  set_tag(1);
  std::construct_at(__storage + 1, std::forward<Args>(args)...);
}
inline constexpr rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>::
operator bool() const noexcept {
  return has_value();
}
inline constexpr bool rs_std::Result<
    ::std::uint8_t, ::result::CopyNoDefault>::has_value() const noexcept {
  return tag() == 0;
}
inline ::std::uint8_t&
rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>::value() & {
  check_has_ok();
  return *reinterpret_cast<::std::uint8_t*>(__storage + 1);
}
inline ::std::uint8_t&&
rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>::value() && {
  check_has_ok();
  return ::std::move(*reinterpret_cast<::std::uint8_t*>(__storage + 1));
}
inline ::result::CopyNoDefault&
rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>::err() & {
  check_has_err();
  return *reinterpret_cast<::result::CopyNoDefault*>(__storage + 1);
}
inline ::result::CopyNoDefault&&
rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>::err() && {
  check_has_err();
  return ::std::move(
      *reinterpret_cast<::result::CopyNoDefault*>(__storage + 1));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>>);
inline constexpr ::std::uint8_t
rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void
rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

inline void
rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>::check_has_ok() {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs_std::Result";
}
inline void
rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>::check_has_err() {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs_std::Result";
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
inline rs_std::Result<::std::uint8_t, ::result::HasDefault>::Result(
    ::std::uint8_t&& ok) noexcept {
  set_tag(UINT64_C(18446744073709551615));
  ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 8),
                      ::std::move(ok));
}
inline rs_std::Result<::std::uint8_t, ::result::HasDefault>&
rs_std::Result<::std::uint8_t, ::result::HasDefault>::operator=(
    ::std::uint8_t&& ok) noexcept {
  if (!has_value()) {
    ::std::destroy_at(reinterpret_cast<::result::HasDefault*>(__storage));
    set_tag(UINT64_C(18446744073709551615));
    ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 8),
                        ::std::move(ok));
  } else {
    set_tag(UINT64_C(18446744073709551615));
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::uint8_t*>(__storage + 8), ::std::move(ok));
  }
  return *this;
}

inline rs_std::Result<::std::uint8_t, ::result::HasDefault>::Result(
    rs_std::unexpected<::result::HasDefault>&& err) noexcept {
  ::std::construct_at(reinterpret_cast<::result::HasDefault*>(__storage),
                      ::std::move(err.error()));
}
inline rs_std::Result<::std::uint8_t, ::result::HasDefault>&
rs_std::Result<::std::uint8_t, ::result::HasDefault>::operator=(
    rs_std::unexpected<::result::HasDefault>&& err) noexcept {
  if (has_value()) {
    ::std::destroy_at(__storage + 8);
    ::std::construct_at(reinterpret_cast<::result::HasDefault*>(__storage),
                        ::std::move(err.error()));
  } else {
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::result::HasDefault*>(__storage),
        ::std::move(err.error()));
  }
  return *this;
}

template <typename... Args>
inline rs_std::Result<::std::uint8_t, ::result::HasDefault>::Result(
    std::in_place_t, Args&&... args) {
  set_tag(UINT64_C(18446744073709551615));
  std::construct_at(__storage + 8, std::forward<Args>(args)...);
}
template <typename... Args>
inline rs_std::Result<::std::uint8_t, ::result::HasDefault>::Result(
    rs_std::unexpect_t, Args&&... args) {
  std::construct_at(__storage, std::forward<Args>(args)...);
}
inline constexpr rs_std::Result<::std::uint8_t, ::result::HasDefault>::
operator bool() const noexcept {
  return has_value();
}
inline constexpr bool rs_std::Result<
    ::std::uint8_t, ::result::HasDefault>::has_value() const noexcept {
  return tag() == UINT64_C(18446744073709551615);
}
inline ::std::uint8_t&
rs_std::Result<::std::uint8_t, ::result::HasDefault>::value() & {
  check_has_ok();
  return *reinterpret_cast<::std::uint8_t*>(__storage + 8);
}
inline ::std::uint8_t&&
rs_std::Result<::std::uint8_t, ::result::HasDefault>::value() && {
  check_has_ok();
  return ::std::move(*reinterpret_cast<::std::uint8_t*>(__storage + 8));
}
inline ::result::HasDefault&
rs_std::Result<::std::uint8_t, ::result::HasDefault>::err() & {
  check_has_err();
  return *reinterpret_cast<::result::HasDefault*>(__storage);
}
inline ::result::HasDefault&&
rs_std::Result<::std::uint8_t, ::result::HasDefault>::err() && {
  check_has_err();
  return ::std::move(*reinterpret_cast<::result::HasDefault*>(__storage));
}
inline rs_std::Result<::std::uint8_t,
                      ::result::HasDefault>::~Result() noexcept {
  if (has_value()) {
    ::std::destroy_at(reinterpret_cast<::std::uint8_t*>(__storage + 8));
  } else {
    ::std::destroy_at(reinterpret_cast<::result::HasDefault*>(__storage));
  }
}
inline constexpr ::std::uint64_t
rs_std::Result<::std::uint8_t, ::result::HasDefault>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void
rs_std::Result<::std::uint8_t, ::result::HasDefault>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint64_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

inline void
rs_std::Result<::std::uint8_t, ::result::HasDefault>::check_has_ok() {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs_std::Result";
}
inline void
rs_std::Result<::std::uint8_t, ::result::HasDefault>::check_has_err() {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs_std::Result";
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
inline rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::Result(
    ::std::uint8_t&& ok) noexcept {
  set_tag(UINT64_C(18446744073709551615));
  ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 8),
                      ::std::move(ok));
}
inline rs_std::Result<::std::uint8_t, ::result::HasNoDefault>&
rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::operator=(
    ::std::uint8_t&& ok) noexcept {
  if (!has_value()) {
    ::std::destroy_at(reinterpret_cast<::result::HasNoDefault*>(__storage));
    set_tag(UINT64_C(18446744073709551615));
    ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 8),
                        ::std::move(ok));
  } else {
    set_tag(UINT64_C(18446744073709551615));
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::uint8_t*>(__storage + 8), ::std::move(ok));
  }
  return *this;
}

template <typename... Args>
inline rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::Result(
    std::in_place_t, Args&&... args) {
  set_tag(UINT64_C(18446744073709551615));
  std::construct_at(__storage + 8, std::forward<Args>(args)...);
}
template <typename... Args>
inline rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::Result(
    rs_std::unexpect_t, Args&&... args) {
  std::construct_at(__storage, std::forward<Args>(args)...);
}
inline constexpr rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::
operator bool() const noexcept {
  return has_value();
}
inline constexpr bool rs_std::Result<
    ::std::uint8_t, ::result::HasNoDefault>::has_value() const noexcept {
  return tag() == UINT64_C(18446744073709551615);
}
inline ::std::uint8_t&
rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::value() & {
  check_has_ok();
  return *reinterpret_cast<::std::uint8_t*>(__storage + 8);
}
inline ::std::uint8_t&&
rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::value() && {
  check_has_ok();
  return ::std::move(*reinterpret_cast<::std::uint8_t*>(__storage + 8));
}
inline ::result::HasNoDefault&
rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::err() & {
  check_has_err();
  return *reinterpret_cast<::result::HasNoDefault*>(__storage);
}
inline ::result::HasNoDefault&&
rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::err() && {
  check_has_err();
  return ::std::move(*reinterpret_cast<::result::HasNoDefault*>(__storage));
}
inline rs_std::Result<::std::uint8_t,
                      ::result::HasNoDefault>::~Result() noexcept {
  if (has_value()) {
    ::std::destroy_at(reinterpret_cast<::std::uint8_t*>(__storage + 8));
  } else {
    ::std::destroy_at(reinterpret_cast<::result::HasNoDefault*>(__storage));
  }
}
inline constexpr ::std::uint64_t
rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void
rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint64_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

inline void
rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::check_has_ok() {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs_std::Result";
}
inline void
rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::check_has_err() {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs_std::Result";
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Result<::std::uint8_t, ::std::uint8_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Result<::std::uint8_t, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<::std::uint8_t, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<::std::uint8_t, ::std::uint8_t>>);
inline rs_std::Result<::std::uint8_t, ::std::uint8_t>::Result(
    ::std::uint8_t&& ok) noexcept {
  set_tag(0);
  ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 1),
                      ::std::move(ok));
}
inline rs_std::Result<::std::uint8_t, ::std::uint8_t>& rs_std::Result<
    ::std::uint8_t, ::std::uint8_t>::operator=(::std::uint8_t&& ok) noexcept {
  if (!has_value()) {
    ::std::destroy_at(reinterpret_cast<::std::uint8_t*>(__storage + 1));
    set_tag(0);
    ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 1),
                        ::std::move(ok));
  } else {
    set_tag(0);
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::uint8_t*>(__storage + 1), ::std::move(ok));
  }
  return *this;
}

inline rs_std::Result<::std::uint8_t, ::std::uint8_t>::Result(
    rs_std::unexpected<::std::uint8_t>&& err) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 1),
                      ::std::move(err.error()));
}
inline rs_std::Result<::std::uint8_t, ::std::uint8_t>&
rs_std::Result<::std::uint8_t, ::std::uint8_t>::operator=(
    rs_std::unexpected<::std::uint8_t>&& err) noexcept {
  if (has_value()) {
    ::std::destroy_at(__storage + 1);
    set_tag(1);
    ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 1),
                        ::std::move(err.error()));
  } else {
    set_tag(1);
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::uint8_t*>(__storage + 1),
        ::std::move(err.error()));
  }
  return *this;
}

template <typename... Args>
inline rs_std::Result<::std::uint8_t, ::std::uint8_t>::Result(std::in_place_t,
                                                              Args&&... args) {
  set_tag(0);
  std::construct_at(__storage + 1, std::forward<Args>(args)...);
}
template <typename... Args>
inline rs_std::Result<::std::uint8_t, ::std::uint8_t>::Result(
    rs_std::unexpect_t, Args&&... args) {
  set_tag(1);
  std::construct_at(__storage + 1, std::forward<Args>(args)...);
}
inline constexpr rs_std::Result<::std::uint8_t, ::std::uint8_t>::operator bool()
    const noexcept {
  return has_value();
}
inline constexpr bool
rs_std::Result<::std::uint8_t, ::std::uint8_t>::has_value() const noexcept {
  return tag() == 0;
}
inline ::std::uint8_t&
rs_std::Result<::std::uint8_t, ::std::uint8_t>::value() & {
  check_has_ok();
  return *reinterpret_cast<::std::uint8_t*>(__storage + 1);
}
inline ::std::uint8_t&&
rs_std::Result<::std::uint8_t, ::std::uint8_t>::value() && {
  check_has_ok();
  return ::std::move(*reinterpret_cast<::std::uint8_t*>(__storage + 1));
}
inline ::std::uint8_t& rs_std::Result<::std::uint8_t, ::std::uint8_t>::err() & {
  check_has_err();
  return *reinterpret_cast<::std::uint8_t*>(__storage + 1);
}
inline ::std::uint8_t&&
rs_std::Result<::std::uint8_t, ::std::uint8_t>::err() && {
  check_has_err();
  return ::std::move(*reinterpret_cast<::std::uint8_t*>(__storage + 1));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Result<::std::uint8_t, ::std::uint8_t>>);
inline constexpr ::std::uint8_t
rs_std::Result<::std::uint8_t, ::std::uint8_t>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs_std::Result<::std::uint8_t, ::std::uint8_t>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

inline void rs_std::Result<::std::uint8_t, ::std::uint8_t>::check_has_ok() {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs_std::Result";
}
inline void rs_std::Result<::std::uint8_t, ::std::uint8_t>::check_has_err() {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs_std::Result";
}
#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_RESULT_GOLDEN
