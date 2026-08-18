// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// result_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_RESULT_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_RESULT_GOLDEN

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
#include "support/rs_std/result.h"
#include "support/rs_std/str_ref.h"
#include "support/rs_std/unit.h"

#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_alloc.h"

namespace result {

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

  CloneNoDefault(::crubit::UnsafeRelocateTag, CloneNoDefault&& value);

  union {
    ::std::uint8_t val;
  };

 private:
  static void __crubit_field_offset_assertions();
};

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
  CopyNoDefault(::crubit::UnsafeRelocateTag, CopyNoDefault&& value);

  union {
    ::std::uint8_t val;
  };

 private:
  static void __crubit_field_offset_assertions();
};

using FreeFunc CRUBIT_INTERNAL_RUST_TYPE(":: result_golden :: FreeFunc") =
    crubit::type_identity_t<void(void*, void*)>*;
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
  HasDefault(::crubit::UnsafeRelocateTag, HasDefault&& value);

  static ::result::HasDefault new_(rs_std::StrRef val);

  rs_std::StrRef val() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  union {
    ::rs::alloc::string::String val_;
  };

 private:
  static void __crubit_field_offset_assertions();
};

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
  HasNoDefault(::crubit::UnsafeRelocateTag, HasNoDefault&& value);

  rs_std::StrRef val() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  union {
    ::rs::alloc::string::String val_;
  };

 private:
  static void __crubit_field_offset_assertions();
};

using Voidpf CRUBIT_INTERNAL_RUST_TYPE(":: result_golden :: Voidpf") = void*;

rs_std::Result<::std::uint8_t, ::std::uint8_t> return_result_by_value();

rs_std::Result<::std::uint8_t, rs_std::unit_t> return_result_unit_err(
    bool has_err);

rs_std::Result<rs_std::unit_t, ::std::uint8_t> return_result_unit_ok(
    bool has_val);

::std::uint8_t take_result_by_value(
    rs_std::Result<::std::uint8_t, ::std::uint8_t> r);

::std::uint8_t take_result_clone_no_default_err(
    rs_std::Result<::std::uint8_t, ::result::CloneNoDefault> const& r);

::std::uint8_t take_result_copy_no_default_ok(
    rs_std::Result<::result::CopyNoDefault, ::std::uint8_t> const& r);

rs_std::StrRef take_result_has_default(
    rs_std::Result<::result::HasDefault, ::std::uint8_t> const* $(__anon1)
        crubit_nonnull r CRUBIT_LIFETIME_BOUND);

bool take_result_unit_err(rs_std::Result<::std::uint8_t, rs_std::unit_t> val);

bool take_result_unit_ok(rs_std::Result<rs_std::unit_t, ::std::uint8_t> val);

}  // namespace result

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(1)
    CRUBIT_INTERNAL_RUST_TYPE("std :: result :: Result < () , u8 >")
        rs_std::Result<rs_std::unit_t, ::std::uint8_t>
    : public rs_std::ResultBase<rs_std::Result<rs_std::unit_t, ::std::uint8_t>,
                                rs_std::unit_t, ::std::uint8_t> {
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
      rs_std::ResultBase<rs_std::Result<rs_std::unit_t, ::std::uint8_t>,
                         rs_std::unit_t, ::std::uint8_t>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, rs_std::unit_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, rs_std::unit_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  rs_std::unit_t* ok_ptr() noexcept {
    return reinterpret_cast<rs_std::unit_t*>(__storage + 1);
  }
  rs_std::unit_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<rs_std::unit_t const*>(__storage + 1);
  }
  ::std::uint8_t* err_ptr() noexcept {
    return reinterpret_cast<::std::uint8_t*>(__storage + 1);
  }
  ::std::uint8_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint8_t const*>(__storage + 1);
  }
  void set_ok_tag() noexcept { set_tag(0); }
  void set_err_tag() noexcept { set_tag(1); }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char __storage[2];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
template <>
struct alignas(8)
    CRUBIT_INTERNAL_RUST_TYPE("std :: result :: Result < i8 , isize >")
        rs_std::Result<::std::int8_t, ::std::int64_t>
    : public rs_std::ResultBase<rs_std::Result<::std::int8_t, ::std::int64_t>,
                                ::std::int8_t, ::std::int64_t> {
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
      rs_std::ResultBase<rs_std::Result<::std::int8_t, ::std::int64_t>,
                         ::std::int8_t, ::std::int64_t>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::int8_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::int8_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::int64_t, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::int64_t, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  ::std::int8_t* ok_ptr() noexcept {
    return reinterpret_cast<::std::int8_t*>(__storage + 1);
  }
  ::std::int8_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::std::int8_t const*>(__storage + 1);
  }
  ::std::int64_t* err_ptr() noexcept {
    return reinterpret_cast<::std::int64_t*>(__storage + 8);
  }
  ::std::int64_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::std::int64_t const*>(__storage + 8);
  }
  void set_ok_tag() noexcept { set_tag(0); }
  void set_err_tag() noexcept { set_tag(1); }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char __storage[16];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000003e
template <>
struct alignas(8)
    CRUBIT_INTERNAL_RUST_TYPE("std :: result :: Result < isize , i8 >")
        rs_std::Result<::std::int64_t, ::std::int8_t>
    : public rs_std::ResultBase<rs_std::Result<::std::int64_t, ::std::int8_t>,
                                ::std::int64_t, ::std::int8_t> {
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
      rs_std::ResultBase<rs_std::Result<::std::int64_t, ::std::int8_t>,
                         ::std::int64_t, ::std::int8_t>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::int64_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::int64_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::int8_t, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::int8_t, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  ::std::int64_t* ok_ptr() noexcept {
    return reinterpret_cast<::std::int64_t*>(__storage + 8);
  }
  ::std::int64_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::std::int64_t const*>(__storage + 8);
  }
  ::std::int8_t* err_ptr() noexcept {
    return reinterpret_cast<::std::int8_t*>(__storage + 1);
  }
  ::std::int8_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::std::int8_t const*>(__storage + 1);
  }
  void set_ok_tag() noexcept { set_tag(0); }
  void set_err_tag() noexcept { set_tag(1); }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char __storage[16];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: result_golden :: CloneNoDefault , u8 >")
    rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>
    : public rs_std::ResultBase<
          rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>,
          ::result::CloneNoDefault, ::std::uint8_t> {
 public:
  // Clone::clone
  Result(const Result&);

  // Clone::clone_from
  rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>& operator=(
      const Result&);

  Result(Result&&) = default;
  Result& operator=(Result&&) = default;

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type = rs_std::ResultBase<
      rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>,
      ::result::CloneNoDefault, ::std::uint8_t>;
  template <typename U>
    requires(
        rs_std::ResultForwardConstructible<Result, ::result::CloneNoDefault, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(
        rs_std::ResultForwardConstructible<Result, ::result::CloneNoDefault, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  ::result::CloneNoDefault* ok_ptr() noexcept {
    return reinterpret_cast<::result::CloneNoDefault*>(__storage + 1);
  }
  ::result::CloneNoDefault const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::result::CloneNoDefault const*>(__storage + 1);
  }
  ::std::uint8_t* err_ptr() noexcept {
    return reinterpret_cast<::std::uint8_t*>(__storage + 1);
  }
  ::std::uint8_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint8_t const*>(__storage + 1);
  }
  void set_ok_tag() noexcept { set_tag(0); }
  void set_err_tag() noexcept { set_tag(1); }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char __storage[2];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: result_golden :: CopyNoDefault , u8 >")
    rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>
    : public rs_std::ResultBase<
          rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>,
          ::result::CopyNoDefault, ::std::uint8_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Result(const Result&) = default;
  Result& operator=(const Result&) = default;
  Result(Result&&) = default;
  Result& operator=(Result&&) = default;

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type = rs_std::ResultBase<
      rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>,
      ::result::CopyNoDefault, ::std::uint8_t>;
  template <typename U>
    requires(
        rs_std::ResultForwardConstructible<Result, ::result::CopyNoDefault, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(
        rs_std::ResultForwardConstructible<Result, ::result::CopyNoDefault, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  ::result::CopyNoDefault* ok_ptr() noexcept {
    return reinterpret_cast<::result::CopyNoDefault*>(__storage + 1);
  }
  ::result::CopyNoDefault const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::result::CopyNoDefault const*>(__storage + 1);
  }
  ::std::uint8_t* err_ptr() noexcept {
    return reinterpret_cast<::std::uint8_t*>(__storage + 1);
  }
  ::std::uint8_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint8_t const*>(__storage + 1);
  }
  void set_ok_tag() noexcept { set_tag(0); }
  void set_err_tag() noexcept { set_tag(1); }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char __storage[2];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: result_golden :: HasDefault , u8 >")
    rs_std::Result<::result::HasDefault, ::std::uint8_t>
    : public rs_std::ResultBase<
          rs_std::Result<::result::HasDefault, ::std::uint8_t>,
          ::result::HasDefault, ::std::uint8_t> {
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
  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type =
      rs_std::ResultBase<rs_std::Result<::result::HasDefault, ::std::uint8_t>,
                         ::result::HasDefault, ::std::uint8_t>;
  template <typename U>
    requires(
        rs_std::ResultForwardConstructible<Result, ::result::HasDefault, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(
        rs_std::ResultForwardConstructible<Result, ::result::HasDefault, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
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
  ::result::HasDefault* ok_ptr() noexcept {
    return reinterpret_cast<::result::HasDefault*>(__storage);
  }
  ::result::HasDefault const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::result::HasDefault const*>(__storage);
  }
  ::std::uint8_t* err_ptr() noexcept {
    return reinterpret_cast<::std::uint8_t*>(__storage + 8);
  }
  ::std::uint8_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint8_t const*>(__storage + 8);
  }
  void set_ok_tag() noexcept {}
  void set_err_tag() noexcept { set_tag(UINT64_C(18446744073709551615)); }
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char __storage[24];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: result_golden :: HasNoDefault , u8 >")
    rs_std::Result<::result::HasNoDefault, ::std::uint8_t>
    : public rs_std::ResultBase<
          rs_std::Result<::result::HasNoDefault, ::std::uint8_t>,
          ::result::HasNoDefault, ::std::uint8_t> {
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
  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type =
      rs_std::ResultBase<rs_std::Result<::result::HasNoDefault, ::std::uint8_t>,
                         ::result::HasNoDefault, ::std::uint8_t>;
  template <typename U>
    requires(
        rs_std::ResultForwardConstructible<Result, ::result::HasNoDefault, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(
        rs_std::ResultForwardConstructible<Result, ::result::HasNoDefault, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
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
  ::result::HasNoDefault* ok_ptr() noexcept {
    return reinterpret_cast<::result::HasNoDefault*>(__storage);
  }
  ::result::HasNoDefault const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::result::HasNoDefault const*>(__storage);
  }
  ::std::uint8_t* err_ptr() noexcept {
    return reinterpret_cast<::std::uint8_t*>(__storage + 8);
  }
  ::std::uint8_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint8_t const*>(__storage + 8);
  }
  void set_ok_tag() noexcept {}
  void set_err_tag() noexcept { set_tag(UINT64_C(18446744073709551615)); }
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char __storage[24];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000003e
template <>
struct alignas(4)
    CRUBIT_INTERNAL_RUST_TYPE("std :: result :: Result < u32 , u32 >")
        rs_std::Result<::std::uint32_t, ::std::uint32_t>
    : public rs_std::ResultBase<
          rs_std::Result<::std::uint32_t, ::std::uint32_t>, ::std::uint32_t,
          ::std::uint32_t> {
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
      rs_std::ResultBase<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
                         ::std::uint32_t, ::std::uint32_t>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint32_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint32_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint32_t, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint32_t, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  ::std::uint32_t* ok_ptr() noexcept {
    return reinterpret_cast<::std::uint32_t*>(__storage + 4);
  }
  ::std::uint32_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint32_t const*>(__storage + 4);
  }
  ::std::uint32_t* err_ptr() noexcept {
    return reinterpret_cast<::std::uint32_t*>(__storage + 4);
  }
  ::std::uint32_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint32_t const*>(__storage + 4);
  }
  void set_ok_tag() noexcept { set_tag(0); }
  void set_err_tag() noexcept { set_tag(1); }
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;

 private:
  unsigned char __storage[8];
};
#endif

namespace result {

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
  GetsResult(::crubit::UnsafeRelocateTag, GetsResult&& value);

  static ::result::GetsResult new_(::std::uint32_t val);

  union {
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
                        ::std::uint32_t>
    : public rs_std::ResultBase<
          rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
                         ::std::uint32_t>,
          rs_std::Result<::std::uint32_t, ::std::uint32_t>, ::std::uint32_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Result(const Result&) = default;
  Result& operator=(const Result&) = default;
  Result(Result&&) = default;
  Result& operator=(Result&&) = default;

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type = rs_std::ResultBase<
      rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
                     ::std::uint32_t>,
      rs_std::Result<::std::uint32_t, ::std::uint32_t>, ::std::uint32_t>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<
             Result, rs_std::Result<::std::uint32_t, ::std::uint32_t>, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<
             Result, rs_std::Result<::std::uint32_t, ::std::uint32_t>, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint32_t, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint32_t, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() != 2; }
  rs_std::Result<::std::uint32_t, ::std::uint32_t>* ok_ptr() noexcept {
    return reinterpret_cast<rs_std::Result<::std::uint32_t, ::std::uint32_t>*>(
        __storage);
  }
  rs_std::Result<::std::uint32_t, ::std::uint32_t> const* ok_const_ptr()
      const noexcept {
    return reinterpret_cast<
        rs_std::Result<::std::uint32_t, ::std::uint32_t> const*>(__storage);
  }
  ::std::uint32_t* err_ptr() noexcept {
    return reinterpret_cast<::std::uint32_t*>(__storage + 4);
  }
  ::std::uint32_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint32_t const*>(__storage + 4);
  }
  void set_ok_tag() noexcept {}
  void set_err_tag() noexcept { set_tag(2); }
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;

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
                        rs_std::Result<::std::uint32_t, ::std::uint32_t>>
    : public rs_std::ResultBase<
          rs_std::Result<::std::uint32_t,
                         rs_std::Result<::std::uint32_t, ::std::uint32_t>>,
          ::std::uint32_t, rs_std::Result<::std::uint32_t, ::std::uint32_t>> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Result(const Result&) = default;
  Result& operator=(const Result&) = default;
  Result(Result&&) = default;
  Result& operator=(Result&&) = default;

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type = rs_std::ResultBase<
      rs_std::Result<::std::uint32_t,
                     rs_std::Result<::std::uint32_t, ::std::uint32_t>>,
      ::std::uint32_t, rs_std::Result<::std::uint32_t, ::std::uint32_t>>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint32_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint32_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<
             rs_std::Result<::std::uint32_t, ::std::uint32_t>, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<
             rs_std::Result<::std::uint32_t, ::std::uint32_t>, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 2; }
  ::std::uint32_t* ok_ptr() noexcept {
    return reinterpret_cast<::std::uint32_t*>(__storage + 4);
  }
  ::std::uint32_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint32_t const*>(__storage + 4);
  }
  rs_std::Result<::std::uint32_t, ::std::uint32_t>* err_ptr() noexcept {
    return reinterpret_cast<rs_std::Result<::std::uint32_t, ::std::uint32_t>*>(
        __storage);
  }
  rs_std::Result<::std::uint32_t, ::std::uint32_t> const* err_const_ptr()
      const noexcept {
    return reinterpret_cast<
        rs_std::Result<::std::uint32_t, ::std::uint32_t> const*>(__storage);
  }
  void set_ok_tag() noexcept { set_tag(2); }
  void set_err_tag() noexcept {}
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;

 private:
  unsigned char __storage[8];
};
#endif

namespace result {

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
  NestedResult(::crubit::UnsafeRelocateTag, NestedResult&& value);

  static ::result::NestedResult new_(::std::uint32_t val);

  union {
    rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
                   ::std::uint32_t>
        in_ok;
  };
  union {
    rs_std::Result<::std::uint32_t,
                   rs_std::Result<::std::uint32_t, ::std::uint32_t>>
        in_err;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace result

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000003e
template <>
struct alignas(1)
    CRUBIT_INTERNAL_RUST_TYPE("std :: result :: Result < u8 , () >")
        rs_std::Result<::std::uint8_t, rs_std::unit_t>
    : public rs_std::ResultBase<rs_std::Result<::std::uint8_t, rs_std::unit_t>,
                                ::std::uint8_t, rs_std::unit_t> {
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
      rs_std::ResultBase<rs_std::Result<::std::uint8_t, rs_std::unit_t>,
                         ::std::uint8_t, rs_std::unit_t>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint8_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint8_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<rs_std::unit_t, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<rs_std::unit_t, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  ::std::uint8_t* ok_ptr() noexcept {
    return reinterpret_cast<::std::uint8_t*>(__storage + 1);
  }
  ::std::uint8_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint8_t const*>(__storage + 1);
  }
  rs_std::unit_t* err_ptr() noexcept {
    return reinterpret_cast<rs_std::unit_t*>(__storage + 1);
  }
  rs_std::unit_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<rs_std::unit_t const*>(__storage + 1);
  }
  void set_ok_tag() noexcept { set_tag(0); }
  void set_err_tag() noexcept { set_tag(1); }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char __storage[2];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < u8 , :: result_golden :: CloneNoDefault >")
    rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>
    : public rs_std::ResultBase<
          rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>,
          ::std::uint8_t, ::result::CloneNoDefault> {
 public:
  // Clone::clone
  Result(const Result&);

  // Clone::clone_from
  rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>& operator=(
      const Result&);

  Result(Result&&) = default;
  Result& operator=(Result&&) = default;

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type = rs_std::ResultBase<
      rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>, ::std::uint8_t,
      ::result::CloneNoDefault>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint8_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint8_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::result::CloneNoDefault, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::result::CloneNoDefault, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  ::std::uint8_t* ok_ptr() noexcept {
    return reinterpret_cast<::std::uint8_t*>(__storage + 1);
  }
  ::std::uint8_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint8_t const*>(__storage + 1);
  }
  ::result::CloneNoDefault* err_ptr() noexcept {
    return reinterpret_cast<::result::CloneNoDefault*>(__storage + 1);
  }
  ::result::CloneNoDefault const* err_const_ptr() const noexcept {
    return reinterpret_cast<::result::CloneNoDefault const*>(__storage + 1);
  }
  void set_ok_tag() noexcept { set_tag(0); }
  void set_err_tag() noexcept { set_tag(1); }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char __storage[2];
};
#endif

namespace result {

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
                       CloneNoDefaultResult&& value);

  static ::result::CloneNoDefaultResult new_(::std::uint8_t val);

  union {
    rs_std::Result<::result::CloneNoDefault, ::std::uint8_t> in_ok;
  };
  union {
    rs_std::Result<::std::uint8_t, ::result::CloneNoDefault> in_err;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace result

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CopyNoDefault_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < u8 , :: result_golden :: CopyNoDefault >")
    rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>
    : public rs_std::ResultBase<
          rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>,
          ::std::uint8_t, ::result::CopyNoDefault> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Result(const Result&) = default;
  Result& operator=(const Result&) = default;
  Result(Result&&) = default;
  Result& operator=(Result&&) = default;

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type = rs_std::ResultBase<
      rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>, ::std::uint8_t,
      ::result::CopyNoDefault>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint8_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint8_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::result::CopyNoDefault, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::result::CopyNoDefault, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  ::std::uint8_t* ok_ptr() noexcept {
    return reinterpret_cast<::std::uint8_t*>(__storage + 1);
  }
  ::std::uint8_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint8_t const*>(__storage + 1);
  }
  ::result::CopyNoDefault* err_ptr() noexcept {
    return reinterpret_cast<::result::CopyNoDefault*>(__storage + 1);
  }
  ::result::CopyNoDefault const* err_const_ptr() const noexcept {
    return reinterpret_cast<::result::CopyNoDefault const*>(__storage + 1);
  }
  void set_ok_tag() noexcept { set_tag(0); }
  void set_err_tag() noexcept { set_tag(1); }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char __storage[2];
};
#endif

namespace result {

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
  CopyNoDefaultResult(::crubit::UnsafeRelocateTag, CopyNoDefaultResult&& value);

  static ::result::CopyNoDefaultResult new_(::std::uint8_t val);

  union {
    rs_std::Result<::result::CopyNoDefault, ::std::uint8_t> in_ok;
  };
  union {
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
    rs_std::Result<::std::uint8_t, ::result::HasDefault>
    : public rs_std::ResultBase<
          rs_std::Result<::std::uint8_t, ::result::HasDefault>, ::std::uint8_t,
          ::result::HasDefault> {
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
  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type =
      rs_std::ResultBase<rs_std::Result<::std::uint8_t, ::result::HasDefault>,
                         ::std::uint8_t, ::result::HasDefault>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint8_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint8_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::result::HasDefault, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::result::HasDefault, F>)
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
  ::std::uint8_t* ok_ptr() noexcept {
    return reinterpret_cast<::std::uint8_t*>(__storage + 8);
  }
  ::std::uint8_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint8_t const*>(__storage + 8);
  }
  ::result::HasDefault* err_ptr() noexcept {
    return reinterpret_cast<::result::HasDefault*>(__storage);
  }
  ::result::HasDefault const* err_const_ptr() const noexcept {
    return reinterpret_cast<::result::HasDefault const*>(__storage);
  }
  void set_ok_tag() noexcept { set_tag(UINT64_C(18446744073709551615)); }
  void set_err_tag() noexcept {}
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char __storage[24];
};
#endif

namespace result {

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
  HasDefaultResult(::crubit::UnsafeRelocateTag, HasDefaultResult&& value);

  static ::result::HasDefaultResult new_(rs_std::StrRef val);

  union {
    rs_std::Result<::result::HasDefault, ::std::uint8_t> in_ok;
  };
  union {
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
    rs_std::Result<::std::uint8_t, ::result::HasNoDefault>
    : public rs_std::ResultBase<
          rs_std::Result<::std::uint8_t, ::result::HasNoDefault>,
          ::std::uint8_t, ::result::HasNoDefault> {
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
  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type =
      rs_std::ResultBase<rs_std::Result<::std::uint8_t, ::result::HasNoDefault>,
                         ::std::uint8_t, ::result::HasNoDefault>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint8_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint8_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::result::HasNoDefault, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::result::HasNoDefault, F>)
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
  ::std::uint8_t* ok_ptr() noexcept {
    return reinterpret_cast<::std::uint8_t*>(__storage + 8);
  }
  ::std::uint8_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint8_t const*>(__storage + 8);
  }
  ::result::HasNoDefault* err_ptr() noexcept {
    return reinterpret_cast<::result::HasNoDefault*>(__storage);
  }
  ::result::HasNoDefault const* err_const_ptr() const noexcept {
    return reinterpret_cast<::result::HasNoDefault const*>(__storage);
  }
  void set_ok_tag() noexcept { set_tag(UINT64_C(18446744073709551615)); }
  void set_err_tag() noexcept {}
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char __storage[24];
};
#endif

namespace result {

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
  HasNoDefaultResult(::crubit::UnsafeRelocateTag, HasNoDefaultResult&& value);

  static ::result::HasNoDefaultResult new_(rs_std::StrRef val);

  union {
    rs_std::Result<::result::HasNoDefault, ::std::uint8_t> in_ok;
  };
  union {
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
        rs_std::Result<::std::uint8_t, ::std::uint8_t>
    : public rs_std::ResultBase<rs_std::Result<::std::uint8_t, ::std::uint8_t>,
                                ::std::uint8_t, ::std::uint8_t> {
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
      rs_std::ResultBase<rs_std::Result<::std::uint8_t, ::std::uint8_t>,
                         ::std::uint8_t, ::std::uint8_t>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint8_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint8_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  ::std::uint8_t* ok_ptr() noexcept {
    return reinterpret_cast<::std::uint8_t*>(__storage + 1);
  }
  ::std::uint8_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint8_t const*>(__storage + 1);
  }
  ::std::uint8_t* err_ptr() noexcept {
    return reinterpret_cast<::std::uint8_t*>(__storage + 1);
  }
  ::std::uint8_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint8_t const*>(__storage + 1);
  }
  void set_ok_tag() noexcept { set_tag(0); }
  void set_err_tag() noexcept { set_tag(1); }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char __storage[2];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000003e
template <>
struct alignas(8)
    CRUBIT_INTERNAL_RUST_TYPE("std :: result :: Result < u8 , usize >")
        rs_std::Result<::std::uint8_t, ::std::uint64_t>
    : public rs_std::ResultBase<rs_std::Result<::std::uint8_t, ::std::uint64_t>,
                                ::std::uint8_t, ::std::uint64_t> {
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
      rs_std::ResultBase<rs_std::Result<::std::uint8_t, ::std::uint64_t>,
                         ::std::uint8_t, ::std::uint64_t>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint8_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint8_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint64_t, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint64_t, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  ::std::uint8_t* ok_ptr() noexcept {
    return reinterpret_cast<::std::uint8_t*>(__storage + 1);
  }
  ::std::uint8_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint8_t const*>(__storage + 1);
  }
  ::std::uint64_t* err_ptr() noexcept {
    return reinterpret_cast<::std::uint64_t*>(__storage + 8);
  }
  ::std::uint64_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint64_t const*>(__storage + 8);
  }
  void set_ok_tag() noexcept { set_tag(0); }
  void set_err_tag() noexcept { set_tag(1); }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char __storage[16];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000002c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000002c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < unsafe extern \"C\" fn (* mut :: core :: ffi :: "
    "c_void , * mut :: core :: ffi :: c_void) , unsafe extern \"C\" fn (* mut "
    ":: core :: ffi :: c_void , * mut :: core :: ffi :: c_void) >")
    rs_std::Result<crubit::type_identity_t<void(void*, void*)>*,
                   crubit::type_identity_t<void(void*, void*)>*>
    : public rs_std::ResultBase<
          rs_std::Result<crubit::type_identity_t<void(void*, void*)>*,
                         crubit::type_identity_t<void(void*, void*)>*>,
          crubit::type_identity_t<void(void*, void*)>*,
          crubit::type_identity_t<void(void*, void*)>*> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Result(const Result&) = default;
  Result& operator=(const Result&) = default;
  Result(Result&&) = default;
  Result& operator=(Result&&) = default;

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type = rs_std::ResultBase<
      rs_std::Result<crubit::type_identity_t<void(void*, void*)>*,
                     crubit::type_identity_t<void(void*, void*)>*>,
      crubit::type_identity_t<void(void*, void*)>*,
      crubit::type_identity_t<void(void*, void*)>*>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<
             Result, crubit::type_identity_t<void(void*, void*)>*, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<
             Result, crubit::type_identity_t<void(void*, void*)>*, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<
             crubit::type_identity_t<void(void*, void*)>*, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<
             crubit::type_identity_t<void(void*, void*)>*, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  crubit::type_identity_t<void(void*, void*)>** ok_ptr() noexcept {
    return reinterpret_cast<crubit::type_identity_t<void(void*, void*)>**>(
        __storage + 8);
  }
  crubit::type_identity_t<void(void*, void*)>* const* ok_const_ptr()
      const noexcept {
    return reinterpret_cast<
        crubit::type_identity_t<void(void*, void*)>* const*>(__storage + 8);
  }
  crubit::type_identity_t<void(void*, void*)>** err_ptr() noexcept {
    return reinterpret_cast<crubit::type_identity_t<void(void*, void*)>**>(
        __storage + 8);
  }
  crubit::type_identity_t<void(void*, void*)>* const* err_const_ptr()
      const noexcept {
    return reinterpret_cast<
        crubit::type_identity_t<void(void*, void*)>* const*>(__storage + 8);
  }
  void set_ok_tag() noexcept { set_tag(0); }
  void set_err_tag() noexcept { set_tag(1); }
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char __storage[16];
};
#endif

namespace result {

struct CRUBIT_INTERNAL_RUST_TYPE(":: result_golden :: ZStream") alignas(8)
    [[clang::trivial_abi]] ZStream final {
 public:
  // `result_golden::ZStream` doesn't implement the `Default` trait
  ZStream() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~ZStream() = default;
  ZStream(ZStream&&) = default;
  ZStream& operator=(ZStream&&) = default;

  // `result_golden::ZStream` doesn't implement the `Clone` trait
  ZStream(const ZStream&) = delete;
  ZStream& operator=(const ZStream&) = delete;
  ZStream(::crubit::UnsafeRelocateTag, ZStream&& value);

  union {
    rs_std::Result<crubit::type_identity_t<void(void*, void*)>*,
                   crubit::type_identity_t<void(void*, void*)>*>
        zfree;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace result

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(8)
    CRUBIT_INTERNAL_RUST_TYPE("std :: result :: Result < usize , u8 >")
        rs_std::Result<::std::uint64_t, ::std::uint8_t>
    : public rs_std::ResultBase<rs_std::Result<::std::uint64_t, ::std::uint8_t>,
                                ::std::uint64_t, ::std::uint8_t> {
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
      rs_std::ResultBase<rs_std::Result<::std::uint64_t, ::std::uint8_t>,
                         ::std::uint64_t, ::std::uint8_t>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint64_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result, ::std::uint64_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  ::std::uint64_t* ok_ptr() noexcept {
    return reinterpret_cast<::std::uint64_t*>(__storage + 8);
  }
  ::std::uint64_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint64_t const*>(__storage + 8);
  }
  ::std::uint8_t* err_ptr() noexcept {
    return reinterpret_cast<::std::uint8_t*>(__storage + 1);
  }
  ::std::uint8_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint8_t const*>(__storage + 1);
  }
  void set_ok_tag() noexcept { set_tag(0); }
  void set_err_tag() noexcept { set_tag(1); }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char __storage[16];
};
#endif

namespace result {

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
  ResultWithSizeTypes(::crubit::UnsafeRelocateTag, ResultWithSizeTypes&& value);

  union {
    rs_std::Result<::std::uint64_t, ::std::uint8_t> uval_in_ok;
  };
  union {
    rs_std::Result<::std::uint8_t, ::std::uint64_t> uval_in_err;
  };
  union {
    rs_std::Result<::std::int64_t, ::std::int8_t> ival_in_ok;
  };
  union {
    rs_std::Result<::std::int8_t, ::std::int64_t> ival_in_err;
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
static_assert(::std::is_trivially_destructible_v<CloneNoDefault>);
static_assert(
    ::std::is_trivially_move_constructible_v<::result::CloneNoDefault>);
static_assert(::std::is_trivially_move_assignable_v<::result::CloneNoDefault>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_uresult_ugolden_x0000003a_x0000003aCloneNoDefault(
    ::result::CloneNoDefault const&, ::result::CloneNoDefault* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_uresult_ugolden_x0000003a_x0000003aCloneNoDefault(
    ::result::CloneNoDefault&, ::result::CloneNoDefault const&);
}
inline ::result::CloneNoDefault::CloneNoDefault(const CloneNoDefault& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_uresult_ugolden_x0000003a_x0000003aCloneNoDefault(
          other, this);
}
inline ::result::CloneNoDefault& ::result::CloneNoDefault::operator=(
    const CloneNoDefault& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_uresult_ugolden_x0000003a_x0000003aCloneNoDefault(
            *this, other);
  }
  return *this;
}
inline ::result::CloneNoDefault::CloneNoDefault(::crubit::UnsafeRelocateTag,
                                                CloneNoDefault&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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
inline ::result::CloneNoDefaultResult::CloneNoDefaultResult(
    ::crubit::UnsafeRelocateTag, CloneNoDefaultResult&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

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
inline ::result::CopyNoDefault::CopyNoDefault(::crubit::UnsafeRelocateTag,
                                              CopyNoDefault&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
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
inline ::result::CopyNoDefaultResult::CopyNoDefaultResult(
    ::crubit::UnsafeRelocateTag, CopyNoDefaultResult&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

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
inline ::result::GetsResult::GetsResult(::crubit::UnsafeRelocateTag,
                                        GetsResult&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

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
extern "C" void
__crubit_thunk_Default_udefault_uresult_ugolden_x0000003a_x0000003aHasDefault(
    ::result::HasDefault* __ret_ptr);
}
inline ::result::HasDefault::HasDefault() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_uresult_ugolden_x0000003a_x0000003aHasDefault(
          this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_uresult_ugolden_x0000003a_x0000003aHasDefault(
    ::result::HasDefault&);
}
inline HasDefault::~HasDefault() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_uresult_ugolden_x0000003a_x0000003aHasDefault(
          *this);
}
inline ::result::HasDefault::HasDefault(HasDefault&& other) : HasDefault() {
  *this = ::std::move(other);
}
inline ::result::HasDefault& ::result::HasDefault::operator=(
    HasDefault&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::result::HasDefault::HasDefault(::crubit::UnsafeRelocateTag,
                                        HasDefault&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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
extern "C" void
__crubit_thunk_Drop_udrop_uresult_ugolden_x0000003a_x0000003aHasDefaultResult(
    ::result::HasDefaultResult&);
}
inline HasDefaultResult::~HasDefaultResult() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_uresult_ugolden_x0000003a_x0000003aHasDefaultResult(
          *this);
}
inline ::result::HasDefaultResult::HasDefaultResult(::crubit::UnsafeRelocateTag,
                                                    HasDefaultResult&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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
extern "C" void
__crubit_thunk_Drop_udrop_uresult_ugolden_x0000003a_x0000003aHasNoDefault(
    ::result::HasNoDefault&);
}
inline HasNoDefault::~HasNoDefault() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_uresult_ugolden_x0000003a_x0000003aHasNoDefault(
          *this);
}
inline ::result::HasNoDefault::HasNoDefault(::crubit::UnsafeRelocateTag,
                                            HasNoDefault&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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
extern "C" void
__crubit_thunk_Drop_udrop_uresult_ugolden_x0000003a_x0000003aHasNoDefaultResult(
    ::result::HasNoDefaultResult&);
}
inline HasNoDefaultResult::~HasNoDefaultResult() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_uresult_ugolden_x0000003a_x0000003aHasNoDefaultResult(
          *this);
}
inline ::result::HasNoDefaultResult::HasNoDefaultResult(
    ::crubit::UnsafeRelocateTag, HasNoDefaultResult&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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
inline ::result::NestedResult::NestedResult(::crubit::UnsafeRelocateTag,
                                            NestedResult&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

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
inline ::result::ResultWithSizeTypes::ResultWithSizeTypes(
    ::crubit::UnsafeRelocateTag, ResultWithSizeTypes&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ResultWithSizeTypes::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ResultWithSizeTypes, uval_in_ok));
  static_assert(16 == offsetof(ResultWithSizeTypes, uval_in_err));
  static_assert(32 == offsetof(ResultWithSizeTypes, ival_in_ok));
  static_assert(48 == offsetof(ResultWithSizeTypes, ival_in_err));
}
static_assert(
    sizeof(ZStream) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(ZStream) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<ZStream>);
static_assert(::std::is_trivially_move_constructible_v<::result::ZStream>);
static_assert(::std::is_trivially_move_assignable_v<::result::ZStream>);
inline ::result::ZStream::ZStream(::crubit::UnsafeRelocateTag,
                                  ZStream&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ZStream::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ZStream, zfree));
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
extern "C" void __crubit_thunk_return_uresult_uunit_uerr(
    bool, rs_std::Result<::std::uint8_t, rs_std::unit_t>* __ret_ptr);
}
inline rs_std::Result<::std::uint8_t, rs_std::unit_t> return_result_unit_err(
    bool has_err) {
  crubit::Slot<rs_std::Result<::std::uint8_t, rs_std::unit_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_return_uresult_uunit_uerr(
      has_err, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uresult_uunit_uok(
    bool, rs_std::Result<rs_std::unit_t, ::std::uint8_t>* __ret_ptr);
}
inline rs_std::Result<rs_std::unit_t, ::std::uint8_t> return_result_unit_ok(
    bool has_val) {
  crubit::Slot<rs_std::Result<rs_std::unit_t, ::std::uint8_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_return_uresult_uunit_uok(
      has_val, __return_value_storage);
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

namespace __crubit_internal {
extern "C" bool __crubit_thunk_take_uresult_uunit_uerr(
    rs_std::Result<::std::uint8_t, rs_std::unit_t>*);
}
inline bool take_result_unit_err(
    rs_std::Result<::std::uint8_t, rs_std::unit_t> val) {
  return __crubit_internal::__crubit_thunk_take_uresult_uunit_uerr(&val);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_take_uresult_uunit_uok(
    rs_std::Result<rs_std::unit_t, ::std::uint8_t>*);
}
inline bool take_result_unit_ok(
    rs_std::Result<rs_std::unit_t, ::std::uint8_t> val) {
  return __crubit_internal::__crubit_thunk_take_uresult_uunit_uok(&val);
}

}  // namespace result

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Result<rs_std::unit_t, ::std::uint8_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Result<rs_std::unit_t, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<rs_std::unit_t, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<rs_std::unit_t, ::std::uint8_t>>);
inline rs_std::Result<rs_std::unit_t, ::std::uint8_t>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Result<rs_std::unit_t, ::std::uint8_t>>);
inline constexpr ::std::uint8_t
rs_std::Result<rs_std::unit_t, ::std::uint8_t>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs_std::Result<rs_std::unit_t, ::std::uint8_t>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<rs_std::unit_t, ::std::uint8_t>, rs_std::unit_t, U>)
inline constexpr rs_std::Result<rs_std::unit_t, ::std::uint8_t>::Result(
    U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<rs_std::unit_t, ::std::uint8_t>, rs_std::unit_t, U>)
inline constexpr rs_std::Result<rs_std::unit_t, ::std::uint8_t>&
rs_std::Result<rs_std::unit_t, ::std::uint8_t>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs_std::Result<rs_std::unit_t, ::std::uint8_t>::Result(
    rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs_std::Result<rs_std::unit_t, ::std::uint8_t>&
rs_std::Result<rs_std::unit_t, ::std::uint8_t>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<rs_std::unit_t, ::std::uint8_t>::Result(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<rs_std::unit_t, ::std::uint8_t>::Result(
    rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Result<::std::int8_t, ::std::int64_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Result<::std::int8_t, ::std::int64_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<::std::int8_t, ::std::int64_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<::std::int8_t, ::std::int64_t>>);
inline rs_std::Result<::std::int8_t, ::std::int64_t>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Result<::std::int8_t, ::std::int64_t>>);
inline constexpr ::std::uint8_t
rs_std::Result<::std::int8_t, ::std::int64_t>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs_std::Result<::std::int8_t, ::std::int64_t>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::int8_t, ::std::int64_t>, ::std::int8_t, U>)
inline constexpr rs_std::Result<::std::int8_t, ::std::int64_t>::Result(
    U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::int8_t, ::std::int64_t>, ::std::int8_t, U>)
inline constexpr rs_std::Result<::std::int8_t, ::std::int64_t>&
rs_std::Result<::std::int8_t, ::std::int64_t>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::int64_t, F>)
inline constexpr rs_std::Result<::std::int8_t, ::std::int64_t>::Result(
    rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::int64_t, F>)
inline constexpr rs_std::Result<::std::int8_t, ::std::int64_t>&
rs_std::Result<::std::int8_t, ::std::int64_t>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<::std::int8_t, ::std::int64_t>::Result(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<::std::int8_t, ::std::int64_t>::Result(
    rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Result<::std::int64_t, ::std::int8_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Result<::std::int64_t, ::std::int8_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<::std::int64_t, ::std::int8_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<::std::int64_t, ::std::int8_t>>);
inline rs_std::Result<::std::int64_t, ::std::int8_t>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Result<::std::int64_t, ::std::int8_t>>);
inline constexpr ::std::uint8_t
rs_std::Result<::std::int64_t, ::std::int8_t>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs_std::Result<::std::int64_t, ::std::int8_t>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::int64_t, ::std::int8_t>, ::std::int64_t, U>)
inline constexpr rs_std::Result<::std::int64_t, ::std::int8_t>::Result(
    U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::int64_t, ::std::int8_t>, ::std::int64_t, U>)
inline constexpr rs_std::Result<::std::int64_t, ::std::int8_t>&
rs_std::Result<::std::int64_t, ::std::int8_t>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::int8_t, F>)
inline constexpr rs_std::Result<::std::int64_t, ::std::int8_t>::Result(
    rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::int8_t, F>)
inline constexpr rs_std::Result<::std::int64_t, ::std::int8_t>&
rs_std::Result<::std::int64_t, ::std::int8_t>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<::std::int64_t, ::std::int8_t>::Result(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<::std::int64_t, ::std::int8_t>::Result(
    rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cresult_ugolden_x0000003a_x0000003aCloneNoDefault_x0000002c_x00000020u8_x0000003e(
    rs_std::Result<::result::CloneNoDefault, ::std::uint8_t> const&,
    rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cresult_ugolden_x0000003a_x0000003aCloneNoDefault_x0000002c_x00000020u8_x0000003e(
    rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>&,
    rs_std::Result<::result::CloneNoDefault, ::std::uint8_t> const&);
}
inline rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::Result(
    const Result& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cresult_ugolden_x0000003a_x0000003aCloneNoDefault_x0000002c_x00000020u8_x0000003e(
          other, this);
}
inline rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>& rs_std::Result<
    ::result::CloneNoDefault, ::std::uint8_t>::operator=(const Result& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cresult_ugolden_x0000003a_x0000003aCloneNoDefault_x0000002c_x00000020u8_x0000003e(
            *this, other);
  }
  return *this;
}
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>>);
inline rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>,
           ::result::CloneNoDefault, U>)
inline constexpr rs_std::Result<::result::CloneNoDefault,
                                ::std::uint8_t>::Result(U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>,
           ::result::CloneNoDefault, U>)
inline constexpr rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>&
rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::operator=(
    U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::
    Result(rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>&
rs_std::Result<::result::CloneNoDefault, ::std::uint8_t>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<::result::CloneNoDefault,
                                ::std::uint8_t>::Result(::std::in_place_t ip,
                                                        Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<::result::CloneNoDefault,
                                ::std::uint8_t>::Result(rs_std::unexpect_t u,
                                                        Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

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
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>,
           ::result::CopyNoDefault, U>)
inline constexpr rs_std::Result<::result::CopyNoDefault,
                                ::std::uint8_t>::Result(U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>,
           ::result::CopyNoDefault, U>)
inline constexpr rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>&
rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>::operator=(
    U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>::
    Result(rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>&
rs_std::Result<::result::CopyNoDefault, ::std::uint8_t>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<::result::CopyNoDefault,
                                ::std::uint8_t>::Result(::std::in_place_t ip,
                                                        Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<::result::CopyNoDefault,
                                ::std::uint8_t>::Result(rs_std::unexpect_t u,
                                                        Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
inline rs_std::Result<::result::HasDefault, ::std::uint8_t>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Result<::result::HasDefault,
                      ::std::uint8_t>::~Result() noexcept {
  this->Reset();
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

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::result::HasDefault, ::std::uint8_t>,
           ::result::HasDefault, U>)
inline constexpr rs_std::Result<::result::HasDefault, ::std::uint8_t>::Result(
    U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::result::HasDefault, ::std::uint8_t>,
           ::result::HasDefault, U>)
inline constexpr rs_std::Result<::result::HasDefault, ::std::uint8_t>&
rs_std::Result<::result::HasDefault, ::std::uint8_t>::operator=(
    U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs_std::Result<::result::HasDefault, ::std::uint8_t>::Result(
    rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs_std::Result<::result::HasDefault, ::std::uint8_t>&
rs_std::Result<::result::HasDefault, ::std::uint8_t>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<::result::HasDefault, ::std::uint8_t>::Result(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<::result::HasDefault, ::std::uint8_t>::Result(
    rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
inline rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Result<::result::HasNoDefault,
                      ::std::uint8_t>::~Result() noexcept {
  this->Reset();
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

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::result::HasNoDefault, ::std::uint8_t>,
           ::result::HasNoDefault, U>)
inline constexpr rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::Result(
    U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::result::HasNoDefault, ::std::uint8_t>,
           ::result::HasNoDefault, U>)
inline constexpr rs_std::Result<::result::HasNoDefault, ::std::uint8_t>&
rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::operator=(
    U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::Result(
    rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs_std::Result<::result::HasNoDefault, ::std::uint8_t>&
rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::Result(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<::result::HasNoDefault, ::std::uint8_t>::Result(
    rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

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
                      ::std::uint32_t>::Result(::crubit::UnsafeRelocateTag,
                                               Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
                          ::std::uint32_t>,
           rs_std::Result<::std::uint32_t, ::std::uint32_t>, U>)
inline constexpr rs_std::Result<
    rs_std::Result<::std::uint32_t, ::std::uint32_t>,
    ::std::uint32_t>::Result(U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
                          ::std::uint32_t>,
           rs_std::Result<::std::uint32_t, ::std::uint32_t>, U>)
inline constexpr rs_std::Result<
    rs_std::Result<::std::uint32_t, ::std::uint32_t>, ::std::uint32_t>&
rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
               ::std::uint32_t>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint32_t, F>)
inline constexpr rs_std::Result<
    rs_std::Result<::std::uint32_t, ::std::uint32_t>,
    ::std::uint32_t>::Result(rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint32_t, F>)
inline constexpr rs_std::Result<
    rs_std::Result<::std::uint32_t, ::std::uint32_t>, ::std::uint32_t>&
rs_std::Result<rs_std::Result<::std::uint32_t, ::std::uint32_t>,
               ::std::uint32_t>::operator=(rs_std::unexpected<F>&&
                                               err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<
    rs_std::Result<::std::uint32_t, ::std::uint32_t>,
    ::std::uint32_t>::Result(::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<
    rs_std::Result<::std::uint32_t, ::std::uint32_t>,
    ::std::uint32_t>::Result(rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

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
    Result(::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::uint32_t,
                          rs_std::Result<::std::uint32_t, ::std::uint32_t>>,
           ::std::uint32_t, U>)
inline constexpr rs_std::Result<
    ::std::uint32_t,
    rs_std::Result<::std::uint32_t, ::std::uint32_t>>::Result(U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::uint32_t,
                          rs_std::Result<::std::uint32_t, ::std::uint32_t>>,
           ::std::uint32_t, U>)
inline constexpr rs_std::Result<
    ::std::uint32_t, rs_std::Result<::std::uint32_t, ::std::uint32_t>>&
rs_std::Result<::std::uint32_t,
               rs_std::Result<::std::uint32_t,
                              ::std::uint32_t>>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<
           rs_std::Result<::std::uint32_t, ::std::uint32_t>, F>)
inline constexpr rs_std::Result<
    ::std::uint32_t, rs_std::Result<::std::uint32_t, ::std::uint32_t>>::
    Result(rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<
           rs_std::Result<::std::uint32_t, ::std::uint32_t>, F>)
inline constexpr rs_std::Result<
    ::std::uint32_t, rs_std::Result<::std::uint32_t, ::std::uint32_t>>&
rs_std::Result<::std::uint32_t,
               rs_std::Result<::std::uint32_t, ::std::uint32_t>>::
operator=(rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<
    ::std::uint32_t, rs_std::Result<::std::uint32_t, ::std::uint32_t>>::
    Result(::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<
    ::std::uint32_t, rs_std::Result<::std::uint32_t, ::std::uint32_t>>::
    Result(rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

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
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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

template <typename U>
  requires(
      rs_std::ResultForwardConstructible<
          rs_std::Result<::std::uint32_t, ::std::uint32_t>, ::std::uint32_t, U>)
inline constexpr rs_std::Result<::std::uint32_t, ::std::uint32_t>::Result(
    U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(
      rs_std::ResultForwardConstructible<
          rs_std::Result<::std::uint32_t, ::std::uint32_t>, ::std::uint32_t, U>)
inline constexpr rs_std::Result<::std::uint32_t, ::std::uint32_t>&
rs_std::Result<::std::uint32_t, ::std::uint32_t>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint32_t, F>)
inline constexpr rs_std::Result<::std::uint32_t, ::std::uint32_t>::Result(
    rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint32_t, F>)
inline constexpr rs_std::Result<::std::uint32_t, ::std::uint32_t>&
rs_std::Result<::std::uint32_t, ::std::uint32_t>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<::std::uint32_t, ::std::uint32_t>::Result(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<::std::uint32_t, ::std::uint32_t>::Result(
    rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Result<::std::uint8_t, rs_std::unit_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Result<::std::uint8_t, rs_std::unit_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<::std::uint8_t, rs_std::unit_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<::std::uint8_t, rs_std::unit_t>>);
inline rs_std::Result<::std::uint8_t, rs_std::unit_t>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Result<::std::uint8_t, rs_std::unit_t>>);
inline constexpr ::std::uint8_t
rs_std::Result<::std::uint8_t, rs_std::unit_t>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs_std::Result<::std::uint8_t, rs_std::unit_t>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::uint8_t, rs_std::unit_t>, ::std::uint8_t, U>)
inline constexpr rs_std::Result<::std::uint8_t, rs_std::unit_t>::Result(
    U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::uint8_t, rs_std::unit_t>, ::std::uint8_t, U>)
inline constexpr rs_std::Result<::std::uint8_t, rs_std::unit_t>&
rs_std::Result<::std::uint8_t, rs_std::unit_t>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<rs_std::unit_t, F>)
inline constexpr rs_std::Result<::std::uint8_t, rs_std::unit_t>::Result(
    rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<rs_std::unit_t, F>)
inline constexpr rs_std::Result<::std::uint8_t, rs_std::unit_t>&
rs_std::Result<::std::uint8_t, rs_std::unit_t>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<::std::uint8_t, rs_std::unit_t>::Result(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<::std::uint8_t, rs_std::unit_t>::Result(
    rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020CloneNoDefault_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cu8_x0000002c_x00000020result_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e(
    rs_std::Result<::std::uint8_t, ::result::CloneNoDefault> const&,
    rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cu8_x0000002c_x00000020result_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e(
    rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>&,
    rs_std::Result<::std::uint8_t, ::result::CloneNoDefault> const&);
}
inline rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::Result(
    const Result& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cu8_x0000002c_x00000020result_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e(
          other, this);
}
inline rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>& rs_std::Result<
    ::std::uint8_t, ::result::CloneNoDefault>::operator=(const Result& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cu8_x0000002c_x00000020result_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e(
            *this, other);
  }
  return *this;
}
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>>);
inline rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>,
           ::std::uint8_t, U>)
inline constexpr rs_std::Result<
    ::std::uint8_t, ::result::CloneNoDefault>::Result(U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>,
           ::std::uint8_t, U>)
inline constexpr rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>&
rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::operator=(
    U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::result::CloneNoDefault, F>)
inline constexpr rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::
    Result(rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::result::CloneNoDefault, F>)
inline constexpr rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>&
rs_std::Result<::std::uint8_t, ::result::CloneNoDefault>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<
    ::std::uint8_t, ::result::CloneNoDefault>::Result(::std::in_place_t ip,
                                                      Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<
    ::std::uint8_t, ::result::CloneNoDefault>::Result(rs_std::unexpect_t u,
                                                      Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

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
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>,
           ::std::uint8_t, U>)
inline constexpr rs_std::Result<
    ::std::uint8_t, ::result::CopyNoDefault>::Result(U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>,
           ::std::uint8_t, U>)
inline constexpr rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>&
rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>::operator=(
    U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::result::CopyNoDefault, F>)
inline constexpr rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>::
    Result(rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::result::CopyNoDefault, F>)
inline constexpr rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>&
rs_std::Result<::std::uint8_t, ::result::CopyNoDefault>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<
    ::std::uint8_t, ::result::CopyNoDefault>::Result(::std::in_place_t ip,
                                                     Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<
    ::std::uint8_t, ::result::CopyNoDefault>::Result(rs_std::unexpect_t u,
                                                     Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasDefault_x00000020_x0000003e
inline rs_std::Result<::std::uint8_t, ::result::HasDefault>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Result<::std::uint8_t,
                      ::result::HasDefault>::~Result() noexcept {
  this->Reset();
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

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::uint8_t, ::result::HasDefault>, ::std::uint8_t,
           U>)
inline constexpr rs_std::Result<::std::uint8_t, ::result::HasDefault>::Result(
    U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::uint8_t, ::result::HasDefault>, ::std::uint8_t,
           U>)
inline constexpr rs_std::Result<::std::uint8_t, ::result::HasDefault>&
rs_std::Result<::std::uint8_t, ::result::HasDefault>::operator=(
    U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::result::HasDefault, F>)
inline constexpr rs_std::Result<::std::uint8_t, ::result::HasDefault>::Result(
    rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::result::HasDefault, F>)
inline constexpr rs_std::Result<::std::uint8_t, ::result::HasDefault>&
rs_std::Result<::std::uint8_t, ::result::HasDefault>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<::std::uint8_t, ::result::HasDefault>::Result(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<::std::uint8_t, ::result::HasDefault>::Result(
    rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020result_x00000020_x0000003a_x0000003a_x00000020HasNoDefault_x00000020_x0000003e
inline rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Result<::std::uint8_t,
                      ::result::HasNoDefault>::~Result() noexcept {
  this->Reset();
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

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::uint8_t, ::result::HasNoDefault>,
           ::std::uint8_t, U>)
inline constexpr rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::Result(
    U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::uint8_t, ::result::HasNoDefault>,
           ::std::uint8_t, U>)
inline constexpr rs_std::Result<::std::uint8_t, ::result::HasNoDefault>&
rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::operator=(
    U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::result::HasNoDefault, F>)
inline constexpr rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::Result(
    rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::result::HasNoDefault, F>)
inline constexpr rs_std::Result<::std::uint8_t, ::result::HasNoDefault>&
rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::Result(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<::std::uint8_t, ::result::HasNoDefault>::Result(
    rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

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
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::uint8_t, ::std::uint8_t>, ::std::uint8_t, U>)
inline constexpr rs_std::Result<::std::uint8_t, ::std::uint8_t>::Result(
    U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::uint8_t, ::std::uint8_t>, ::std::uint8_t, U>)
inline constexpr rs_std::Result<::std::uint8_t, ::std::uint8_t>&
rs_std::Result<::std::uint8_t, ::std::uint8_t>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs_std::Result<::std::uint8_t, ::std::uint8_t>::Result(
    rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs_std::Result<::std::uint8_t, ::std::uint8_t>&
rs_std::Result<::std::uint8_t, ::std::uint8_t>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<::std::uint8_t, ::std::uint8_t>::Result(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<::std::uint8_t, ::std::uint8_t>::Result(
    rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Result<::std::uint8_t, ::std::uint64_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Result<::std::uint8_t, ::std::uint64_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<::std::uint8_t, ::std::uint64_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<::std::uint8_t, ::std::uint64_t>>);
inline rs_std::Result<::std::uint8_t, ::std::uint64_t>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Result<::std::uint8_t, ::std::uint64_t>>);
inline constexpr ::std::uint8_t
rs_std::Result<::std::uint8_t, ::std::uint64_t>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs_std::Result<::std::uint8_t, ::std::uint64_t>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::uint8_t, ::std::uint64_t>, ::std::uint8_t, U>)
inline constexpr rs_std::Result<::std::uint8_t, ::std::uint64_t>::Result(
    U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::uint8_t, ::std::uint64_t>, ::std::uint8_t, U>)
inline constexpr rs_std::Result<::std::uint8_t, ::std::uint64_t>&
rs_std::Result<::std::uint8_t, ::std::uint64_t>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint64_t, F>)
inline constexpr rs_std::Result<::std::uint8_t, ::std::uint64_t>::Result(
    rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint64_t, F>)
inline constexpr rs_std::Result<::std::uint8_t, ::std::uint64_t>&
rs_std::Result<::std::uint8_t, ::std::uint64_t>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<::std::uint8_t, ::std::uint64_t>::Result(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<::std::uint8_t, ::std::uint64_t>::Result(
    rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000002c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000002c_x00000020crubit_x00000020_x0000003a_x0000003a_x00000020type_uidentity_ut_x00000020_x0000003c_x00000020void_x00000020_x00000028void_x00000020_x0000002a_x00000020_x0000002c_x00000020void_x00000020_x0000002a_x00000029_x00000020_x0000003e_x00000020_x0000002a_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Result<crubit::type_identity_t<void(void*, void*)>*,
                             crubit::type_identity_t<void(void*, void*)>*>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Result<crubit::type_identity_t<void(void*, void*)>*,
                             crubit::type_identity_t<void(void*, void*)>*>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<crubit::type_identity_t<void(void*, void*)>*,
                             crubit::type_identity_t<void(void*, void*)>*>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<crubit::type_identity_t<void(void*, void*)>*,
                             crubit::type_identity_t<void(void*, void*)>*>>);
inline rs_std::Result<crubit::type_identity_t<void(void*, void*)>*,
                      crubit::type_identity_t<void(void*, void*)>*>::
    Result(::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Result<crubit::type_identity_t<void(void*, void*)>*,
                             crubit::type_identity_t<void(void*, void*)>*>>);
inline constexpr ::std::uint64_t rs_std::Result<
    crubit::type_identity_t<void(void*, void*)>*,
    crubit::type_identity_t<void(void*, void*)>*>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void rs_std::Result<
    crubit::type_identity_t<void(void*, void*)>*,
    crubit::type_identity_t<void(void*, void*)>*>::set_tag(::std::uint64_t
                                                               tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint64_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<crubit::type_identity_t<void(void*, void*)>*,
                          crubit::type_identity_t<void(void*, void*)>*>,
           crubit::type_identity_t<void(void*, void*)>*, U>)
inline constexpr rs_std::Result<
    crubit::type_identity_t<void(void*, void*)>*,
    crubit::type_identity_t<void(void*, void*)>*>::Result(U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<crubit::type_identity_t<void(void*, void*)>*,
                          crubit::type_identity_t<void(void*, void*)>*>,
           crubit::type_identity_t<void(void*, void*)>*, U>)
inline constexpr rs_std::Result<crubit::type_identity_t<void(void*, void*)>*,
                                crubit::type_identity_t<void(void*, void*)>*>&
rs_std::Result<
    crubit::type_identity_t<void(void*, void*)>*,
    crubit::type_identity_t<void(void*, void*)>*>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<
           crubit::type_identity_t<void(void*, void*)>*, F>)
inline constexpr rs_std::Result<crubit::type_identity_t<void(void*, void*)>*,
                                crubit::type_identity_t<void(void*, void*)>*>::
    Result(rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<
           crubit::type_identity_t<void(void*, void*)>*, F>)
inline constexpr rs_std::Result<crubit::type_identity_t<void(void*, void*)>*,
                                crubit::type_identity_t<void(void*, void*)>*>&
rs_std::Result<crubit::type_identity_t<void(void*, void*)>*,
               crubit::type_identity_t<void(void*, void*)>*>::
operator=(rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<crubit::type_identity_t<void(void*, void*)>*,
                                crubit::type_identity_t<void(void*, void*)>*>::
    Result(::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<crubit::type_identity_t<void(void*, void*)>*,
                                crubit::type_identity_t<void(void*, void*)>*>::
    Result(rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint64_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Result<::std::uint64_t, ::std::uint8_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Result<::std::uint64_t, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<::std::uint64_t, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<::std::uint64_t, ::std::uint8_t>>);
inline rs_std::Result<::std::uint64_t, ::std::uint8_t>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Result<::std::uint64_t, ::std::uint8_t>>);
inline constexpr ::std::uint8_t
rs_std::Result<::std::uint64_t, ::std::uint8_t>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs_std::Result<::std::uint64_t, ::std::uint8_t>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::uint64_t, ::std::uint8_t>, ::std::uint64_t, U>)
inline constexpr rs_std::Result<::std::uint64_t, ::std::uint8_t>::Result(
    U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::std::uint64_t, ::std::uint8_t>, ::std::uint64_t, U>)
inline constexpr rs_std::Result<::std::uint64_t, ::std::uint8_t>&
rs_std::Result<::std::uint64_t, ::std::uint8_t>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs_std::Result<::std::uint64_t, ::std::uint8_t>::Result(
    rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs_std::Result<::std::uint64_t, ::std::uint8_t>&
rs_std::Result<::std::uint64_t, ::std::uint8_t>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<::std::uint64_t, ::std::uint8_t>::Result(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<::std::uint64_t, ::std::uint8_t>::Result(
    rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_RESULT_GOLDEN
