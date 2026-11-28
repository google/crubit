// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// never_golden

// clang-format off
#ifndef CC_BINDINGS_FROM_RS_TEST_NEVER_NEVER_GOLDEN
#define CC_BINDINGS_FROM_RS_TEST_NEVER_NEVER_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#pragma clang diagnostic ignored "-Wshadow"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/rs/never.h"
#include "support/rs/option.h"
#include "support/rs/result.h"
#include "support/rs/unit.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <optional>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_alloc.h"

#pragma clang assume_nonnull begin

namespace never {

struct CRUBIT_INTERNAL_RUST_TYPE(":: never_golden :: NeverStruct") alignas(4)
    [[clang::trivial_abi]] NeverStruct final {
 public:
  // Default::default
  NeverStruct();

  // No custom `Drop` impl and no custom "drop glue" required
  ~NeverStruct() = default;
  NeverStruct(NeverStruct&&) = default;
  NeverStruct& operator=(NeverStruct&&) = default;

  // `never_golden::NeverStruct` doesn't implement the `Clone` trait
  NeverStruct(const NeverStruct&) = delete;
  NeverStruct& operator=(const NeverStruct&) = delete;
  NeverStruct(::crubit::UnsafeRelocateTag, NeverStruct&& value);

  [[noreturn]] static void associated_fn_never_return();

  [[noreturn]] void method_never_return() const;

 private:
  // Field type has been replaced with a blob of bytes: Field is private
  ::std::array<unsigned char, 4> _non_zst_field;

 private:
  static void __crubit_field_offset_assertions();
};

extern "C" [[noreturn]] void extern_never_return();

[[noreturn]] void never_return();

::rs::Option<::rs::never_t> option_never();

::rs::Result<::std::int32_t, ::rs::never_t> result_i32_never();

::rs::Result<::rs::never_t, ::std::int32_t> result_never_i32();

::rs::Result<::rs::never_t, ::rs::never_t> result_never_never();

::rs::Result<::rs::never_t, ::rs::unit_t> result_never_unit();

::rs::Result<::rs::alloc::string::String, ::rs::never_t> result_string_never();

::rs::Result<::rs::unit_t, ::rs::never_t> result_unit_never();

}

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < ! >")::rs::Option<::rs::never_t>
    : public ::rs::OptionBase<::rs::Option<::rs::never_t>, ::rs::never_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      ::rs::OptionBase<::rs::Option<::rs::never_t>, ::rs::never_t>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::rs::never_t, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::rs::never_t, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::rs::never_t, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::rs::never_t, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = unsigned char;
  static constexpr tag_type kNoneVal = 0;
  ::rs::never_t* some_ptr() noexcept {
    return reinterpret_cast<::rs::never_t*>(this);
  }
  ::rs::never_t const* some_const_ptr() const noexcept {
    return reinterpret_cast<::rs::never_t const*>(this);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr unsigned char tag() const& noexcept { return 0; }
  constexpr void set_tag(unsigned char) noexcept {}
};
static_assert(sizeof(::rs::Option<::rs::never_t>) == 1);

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
template <>
struct alignas(1)
    CRUBIT_INTERNAL_RUST_TYPE("core :: result :: Result < ! , ! >")::rs::Result<
        ::rs::never_t, ::rs::never_t>
    : public ::rs::ResultBase<::rs::Result<::rs::never_t, ::rs::never_t>,
                              ::rs::never_t, ::rs::never_t> {
 public:
  Result() = delete;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Result(const Result&) = default;
  Result& operator=(const Result&) = default;
  Result(Result&&) = default;
  Result& operator=(Result&&) = default;

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type = ::rs::ResultBase<::rs::Result<::rs::never_t, ::rs::never_t>,
                                     ::rs::never_t, ::rs::never_t>;
  template <typename U>
    requires(::rs::ResultForwardConstructible<Result, ::rs::never_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(::rs::ResultForwardConstructible<Result, ::rs::never_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(::rs::ResultUnexpectedConstructible<::rs::never_t, F>)
  explicit constexpr Result(::rs::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(::rs::ResultUnexpectedConstructible<::rs::never_t, F>)
  constexpr Result& operator=(::rs::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(::rs::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return false; }
  ::rs::never_t* ok_ptr() noexcept {
    return reinterpret_cast<::rs::never_t*>(this);
  }
  ::rs::never_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::rs::never_t const*>(this);
  }
  ::rs::never_t* err_ptr() noexcept {
    return reinterpret_cast<::rs::never_t*>(this);
  }
  ::rs::never_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::rs::never_t const*>(this);
  }
  void set_ok_tag() noexcept {}
  void set_err_tag() noexcept {}

 private:
};
static_assert(sizeof(::rs::Result<::rs::never_t, ::rs::never_t>) == 1);

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: result :: Result < ! , () >")::rs::Result<::rs::never_t,
                                                       ::rs::unit_t>
    : public ::rs::ResultBase<::rs::Result<::rs::never_t, ::rs::unit_t>,
                              ::rs::never_t, ::rs::unit_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Result(const Result&) = default;
  Result& operator=(const Result&) = default;
  Result(Result&&) = default;
  Result& operator=(Result&&) = default;

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type = ::rs::ResultBase<::rs::Result<::rs::never_t, ::rs::unit_t>,
                                     ::rs::never_t, ::rs::unit_t>;
  template <typename U>
    requires(::rs::ResultForwardConstructible<Result, ::rs::never_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(::rs::ResultForwardConstructible<Result, ::rs::never_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(::rs::ResultUnexpectedConstructible<::rs::unit_t, F>)
  explicit constexpr Result(::rs::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(::rs::ResultUnexpectedConstructible<::rs::unit_t, F>)
  constexpr Result& operator=(::rs::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(::rs::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return false; }
  ::rs::never_t* ok_ptr() noexcept {
    return reinterpret_cast<::rs::never_t*>(this);
  }
  ::rs::never_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::rs::never_t const*>(this);
  }
  ::rs::unit_t* err_ptr() noexcept {
    return reinterpret_cast<::rs::unit_t*>(this);
  }
  ::rs::unit_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::rs::unit_t const*>(this);
  }
  void set_ok_tag() noexcept {}
  void set_err_tag() noexcept {}

 private:
};
static_assert(sizeof(::rs::Result<::rs::never_t, ::rs::unit_t>) == 1);

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: result :: Result < ! , i32 >")::rs::Result<::rs::never_t,
                                                        ::std::int32_t>
    : public ::rs::ResultBase<::rs::Result<::rs::never_t, ::std::int32_t>,
                              ::rs::never_t, ::std::int32_t> {
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
      ::rs::ResultBase<::rs::Result<::rs::never_t, ::std::int32_t>,
                       ::rs::never_t, ::std::int32_t>;
  template <typename U>
    requires(::rs::ResultForwardConstructible<Result, ::rs::never_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(::rs::ResultForwardConstructible<Result, ::rs::never_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(::rs::ResultUnexpectedConstructible<::std::int32_t, F>)
  explicit constexpr Result(::rs::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(::rs::ResultUnexpectedConstructible<::std::int32_t, F>)
  constexpr Result& operator=(::rs::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(::rs::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return false; }
  ::rs::never_t* ok_ptr() noexcept {
    return reinterpret_cast<::rs::never_t*>(__storage + 0);
  }
  ::rs::never_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::rs::never_t const*>(__storage + 0);
  }
  ::std::int32_t* err_ptr() noexcept {
    return reinterpret_cast<::std::int32_t*>(__storage + 0);
  }
  ::std::int32_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::std::int32_t const*>(__storage + 0);
  }
  void set_ok_tag() noexcept {}
  void set_err_tag() noexcept {}

 private:
  unsigned char __storage[4];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
template <>
struct alignas(1) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: result :: Result < () , ! >")::rs::Result<::rs::unit_t,
                                                       ::rs::never_t>
    : public ::rs::ResultBase<::rs::Result<::rs::unit_t, ::rs::never_t>,
                              ::rs::unit_t, ::rs::never_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Result(const Result&) = default;
  Result& operator=(const Result&) = default;
  Result(Result&&) = default;
  Result& operator=(Result&&) = default;

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type = ::rs::ResultBase<::rs::Result<::rs::unit_t, ::rs::never_t>,
                                     ::rs::unit_t, ::rs::never_t>;
  template <typename U>
    requires(::rs::ResultForwardConstructible<Result, ::rs::unit_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(::rs::ResultForwardConstructible<Result, ::rs::unit_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(::rs::ResultUnexpectedConstructible<::rs::never_t, F>)
  explicit constexpr Result(::rs::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(::rs::ResultUnexpectedConstructible<::rs::never_t, F>)
  constexpr Result& operator=(::rs::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(::rs::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return true; }
  ::rs::unit_t* ok_ptr() noexcept {
    return reinterpret_cast<::rs::unit_t*>(this);
  }
  ::rs::unit_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::rs::unit_t const*>(this);
  }
  ::rs::never_t* err_ptr() noexcept {
    return reinterpret_cast<::rs::never_t*>(this);
  }
  ::rs::never_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::rs::never_t const*>(this);
  }
  void set_ok_tag() noexcept {}
  void set_err_tag() noexcept {}

 private:
};
static_assert(sizeof(::rs::Result<::rs::unit_t, ::rs::never_t>) == 1);

#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: result :: Result < i32 , ! >")::rs::Result<::std::int32_t,
                                                        ::rs::never_t>
    : public ::rs::ResultBase<::rs::Result<::std::int32_t, ::rs::never_t>,
                              ::std::int32_t, ::rs::never_t> {
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
      ::rs::ResultBase<::rs::Result<::std::int32_t, ::rs::never_t>,
                       ::std::int32_t, ::rs::never_t>;
  template <typename U>
    requires(::rs::ResultForwardConstructible<Result, ::std::int32_t, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(::rs::ResultForwardConstructible<Result, ::std::int32_t, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(::rs::ResultUnexpectedConstructible<::rs::never_t, F>)
  explicit constexpr Result(::rs::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(::rs::ResultUnexpectedConstructible<::rs::never_t, F>)
  constexpr Result& operator=(::rs::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(::rs::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return true; }
  ::std::int32_t* ok_ptr() noexcept {
    return reinterpret_cast<::std::int32_t*>(__storage + 0);
  }
  ::std::int32_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::std::int32_t const*>(__storage + 0);
  }
  ::rs::never_t* err_ptr() noexcept {
    return reinterpret_cast<::rs::never_t*>(__storage + 0);
  }
  ::rs::never_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::rs::never_t const*>(__storage + 0);
  }
  void set_ok_tag() noexcept {}
  void set_err_tag() noexcept {}

 private:
  unsigned char __storage[4];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: result :: Result < :: alloc :: string :: String , ! >")::rs::
    Result<::rs::alloc::string::String, ::rs::never_t>
    : public ::rs::ResultBase<
          ::rs::Result<::rs::alloc::string::String, ::rs::never_t>,
          ::rs::alloc::string::String, ::rs::never_t> {
 public:
  // Clone::clone
  Result(const Result&) noexcept;

  // Clone::clone_from
  ::rs::Result<::rs::alloc::string::String, ::rs::never_t>& operator=(
      const Result&) noexcept;

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type =
      ::rs::ResultBase<::rs::Result<::rs::alloc::string::String, ::rs::never_t>,
                       ::rs::alloc::string::String, ::rs::never_t>;
  template <typename U>
    requires(::rs::ResultForwardConstructible<Result,
                                              ::rs::alloc::string::String, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(::rs::ResultForwardConstructible<Result,
                                              ::rs::alloc::string::String, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(::rs::ResultUnexpectedConstructible<::rs::never_t, F>)
  explicit constexpr Result(::rs::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(::rs::ResultUnexpectedConstructible<::rs::never_t, F>)
  constexpr Result& operator=(::rs::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(::rs::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return true; }
  ::rs::alloc::string::String* ok_ptr() noexcept {
    return reinterpret_cast<::rs::alloc::string::String*>(__storage + 0);
  }
  ::rs::alloc::string::String const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::rs::alloc::string::String const*>(__storage + 0);
  }
  ::rs::never_t* err_ptr() noexcept {
    return reinterpret_cast<::rs::never_t*>(__storage + 0);
  }
  ::rs::never_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::rs::never_t const*>(__storage + 0);
  }
  void set_ok_tag() noexcept {}
  void set_err_tag() noexcept {}

 private:
  unsigned char __storage[12];
};
#endif

namespace never {

static_assert(
    sizeof(::never::NeverStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::never::NeverStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_unever_ugolden_x0000003a_x0000003aNeverStruct(
    ::never::NeverStruct* __ret_ptr);
}
inline ::never::NeverStruct::NeverStruct() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_unever_ugolden_x0000003a_x0000003aNeverStruct(
          this);
}
static_assert(::std::is_trivially_destructible_v<::never::NeverStruct>);
static_assert(::std::is_trivially_move_constructible_v<::never::NeverStruct>);
static_assert(::std::is_trivially_move_assignable_v<::never::NeverStruct>);
inline ::never::NeverStruct::NeverStruct(::crubit::UnsafeRelocateTag,
                                         NeverStruct&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" [[noreturn]] void __crubit_thunk_associated_ufn_unever_ureturn();
}
inline void(NeverStruct::associated_fn_never_return)() {
  __crubit_internal::__crubit_thunk_associated_ufn_unever_ureturn();
}

namespace __crubit_internal {
extern "C" [[noreturn]] void __crubit_thunk_method_unever_ureturn(
    ::never::NeverStruct const&);
}
inline void(NeverStruct::method_never_return)() const {
  auto&& self = *this;
  __crubit_internal::__crubit_thunk_method_unever_ureturn(self);
}
inline void ::never::NeverStruct::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::never::NeverStruct;
  static_assert(0 == offsetof(__crubit_assert_type, _non_zst_field));
}
namespace __crubit_internal {
extern "C" [[noreturn]] void __crubit_thunk_never_ureturn();
}
inline void never_return() {
  __crubit_internal::__crubit_thunk_never_ureturn();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_option_unever(
    ::rs::Option<::rs::never_t>* __ret_ptr);
}
inline ::rs::Option<::rs::never_t> option_never() {
  crubit::Slot<::rs::Option<::rs::never_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_option_unever(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_result_ui32_unever(
    ::rs::Result<::std::int32_t, ::rs::never_t>* __ret_ptr);
}
inline ::rs::Result<::std::int32_t, ::rs::never_t> result_i32_never() {
  crubit::Slot<::rs::Result<::std::int32_t, ::rs::never_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_result_ui32_unever(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_result_unever_ui32(
    ::rs::Result<::rs::never_t, ::std::int32_t>* __ret_ptr);
}
inline ::rs::Result<::rs::never_t, ::std::int32_t> result_never_i32() {
  crubit::Slot<::rs::Result<::rs::never_t, ::std::int32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_result_unever_ui32(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_result_unever_unever(
    ::rs::Result<::rs::never_t, ::rs::never_t>* __ret_ptr);
}
inline ::rs::Result<::rs::never_t, ::rs::never_t> result_never_never() {
  crubit::Slot<::rs::Result<::rs::never_t, ::rs::never_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_result_unever_unever(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_result_unever_uunit(
    ::rs::Result<::rs::never_t, ::rs::unit_t>* __ret_ptr);
}
inline ::rs::Result<::rs::never_t, ::rs::unit_t> result_never_unit() {
  crubit::Slot<::rs::Result<::rs::never_t, ::rs::unit_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_result_unever_uunit(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_result_ustring_unever(
    ::rs::Result<::rs::alloc::string::String, ::rs::never_t>* __ret_ptr);
}
inline ::rs::Result<::rs::alloc::string::String, ::rs::never_t>
result_string_never() {
  crubit::Slot<::rs::Result<::rs::alloc::string::String, ::rs::never_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_result_ustring_unever(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_result_uunit_unever(
    ::rs::Result<::rs::unit_t, ::rs::never_t>* __ret_ptr);
}
inline ::rs::Result<::rs::unit_t, ::rs::never_t> result_unit_never() {
  crubit::Slot<::rs::Result<::rs::unit_t, ::rs::never_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_result_uunit_unever(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<::rs::Option<::rs::never_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::rs::Option<::rs::never_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<::rs::Option<::rs::never_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<::rs::Option<::rs::never_t>>);
inline ::rs::Option<::rs::never_t>::Option(::crubit::UnsafeRelocateTag,
                                           Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<::rs::Option<::rs::never_t>>);

inline constexpr ::rs::Option<::rs::never_t>::Option(::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::rs::never_t>& ::rs::Option<
    ::rs::never_t>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::rs::never_t>,
                                            ::rs::never_t, U>
inline ::rs::Option<::rs::never_t>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::rs::never_t>,
                                            ::rs::never_t, U>
inline ::rs::Option<::rs::never_t>& ::rs::Option<::rs::never_t>::operator=(
    U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::rs::never_t, Opt>
inline ::rs::Option<::rs::never_t>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::rs::never_t, Opt>
inline ::rs::Option<::rs::never_t>& ::rs::Option<::rs::never_t>::operator=(
    Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::rs::never_t>::Option(::std::in_place_t ip,
                                           Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs::Result<::rs::never_t, ::rs::never_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs::Result<::rs::never_t, ::rs::never_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs::Result<::rs::never_t, ::rs::never_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs::Result<::rs::never_t, ::rs::never_t>>);
inline ::rs::Result<::rs::never_t, ::rs::never_t>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              ::rs::Result<::rs::never_t, ::rs::never_t>>);

template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::rs::never_t, ::rs::never_t>, ::rs::never_t, U>)
inline constexpr ::rs::Result<::rs::never_t, ::rs::never_t>::Result(
    U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::rs::never_t, ::rs::never_t>, ::rs::never_t, U>)
inline constexpr ::rs::Result<::rs::never_t, ::rs::never_t>& ::rs::Result<
    ::rs::never_t, ::rs::never_t>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<::rs::never_t, F>)
inline constexpr ::rs::Result<::rs::never_t, ::rs::never_t>::Result(
    ::rs::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<::rs::never_t, F>)
inline constexpr ::rs::Result<::rs::never_t, ::rs::never_t>& ::rs::Result<
    ::rs::never_t, ::rs::never_t>::operator=(::rs::unexpected<F>&&
                                                 err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr ::rs::Result<::rs::never_t, ::rs::never_t>::Result(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr ::rs::Result<::rs::never_t, ::rs::never_t>::Result(
    ::rs::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs::Result<::rs::never_t, ::rs::unit_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs::Result<::rs::never_t, ::rs::unit_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs::Result<::rs::never_t, ::rs::unit_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs::Result<::rs::never_t, ::rs::unit_t>>);
inline ::rs::Result<::rs::never_t, ::rs::unit_t>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              ::rs::Result<::rs::never_t, ::rs::unit_t>>);

template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::rs::never_t, ::rs::unit_t>, ::rs::never_t, U>)
inline constexpr ::rs::Result<::rs::never_t, ::rs::unit_t>::Result(
    U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::rs::never_t, ::rs::unit_t>, ::rs::never_t, U>)
inline constexpr ::rs::Result<::rs::never_t, ::rs::unit_t>& ::rs::Result<
    ::rs::never_t, ::rs::unit_t>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<::rs::unit_t, F>)
inline constexpr ::rs::Result<::rs::never_t, ::rs::unit_t>::Result(
    ::rs::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<::rs::unit_t, F>)
inline constexpr ::rs::Result<::rs::never_t, ::rs::unit_t>& ::rs::Result<
    ::rs::never_t, ::rs::unit_t>::operator=(::rs::unexpected<F>&&
                                                err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr ::rs::Result<::rs::never_t, ::rs::unit_t>::Result(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr ::rs::Result<::rs::never_t, ::rs::unit_t>::Result(
    ::rs::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs::Result<::rs::never_t, ::std::int32_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs::Result<::rs::never_t, ::std::int32_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs::Result<::rs::never_t, ::std::int32_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs::Result<::rs::never_t, ::std::int32_t>>);
inline ::rs::Result<::rs::never_t, ::std::int32_t>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              ::rs::Result<::rs::never_t, ::std::int32_t>>);

template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::rs::never_t, ::std::int32_t>, ::rs::never_t, U>)
inline constexpr ::rs::Result<::rs::never_t, ::std::int32_t>::Result(
    U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::rs::never_t, ::std::int32_t>, ::rs::never_t, U>)
inline constexpr ::rs::Result<::rs::never_t, ::std::int32_t>& ::rs::Result<
    ::rs::never_t, ::std::int32_t>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<::std::int32_t, F>)
inline constexpr ::rs::Result<::rs::never_t, ::std::int32_t>::Result(
    ::rs::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<::std::int32_t, F>)
inline constexpr ::rs::Result<::rs::never_t, ::std::int32_t>& ::rs::Result<
    ::rs::never_t, ::std::int32_t>::operator=(::rs::unexpected<F>&&
                                                  err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr ::rs::Result<::rs::never_t, ::std::int32_t>::Result(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr ::rs::Result<::rs::never_t, ::std::int32_t>::Result(
    ::rs::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020unit_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs::Result<::rs::unit_t, ::rs::never_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs::Result<::rs::unit_t, ::rs::never_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs::Result<::rs::unit_t, ::rs::never_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs::Result<::rs::unit_t, ::rs::never_t>>);
inline ::rs::Result<::rs::unit_t, ::rs::never_t>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              ::rs::Result<::rs::unit_t, ::rs::never_t>>);

template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::rs::unit_t, ::rs::never_t>, ::rs::unit_t, U>)
inline constexpr ::rs::Result<::rs::unit_t, ::rs::never_t>::Result(
    U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::rs::unit_t, ::rs::never_t>, ::rs::unit_t, U>)
inline constexpr ::rs::Result<::rs::unit_t, ::rs::never_t>& ::rs::Result<
    ::rs::unit_t, ::rs::never_t>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<::rs::never_t, F>)
inline constexpr ::rs::Result<::rs::unit_t, ::rs::never_t>::Result(
    ::rs::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<::rs::never_t, F>)
inline constexpr ::rs::Result<::rs::unit_t, ::rs::never_t>& ::rs::Result<
    ::rs::unit_t, ::rs::never_t>::operator=(::rs::unexpected<F>&&
                                                err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr ::rs::Result<::rs::unit_t, ::rs::never_t>::Result(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr ::rs::Result<::rs::unit_t, ::rs::never_t>::Result(
    ::rs::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs::Result<::std::int32_t, ::rs::never_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs::Result<::std::int32_t, ::rs::never_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs::Result<::std::int32_t, ::rs::never_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs::Result<::std::int32_t, ::rs::never_t>>);
inline ::rs::Result<::std::int32_t, ::rs::never_t>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              ::rs::Result<::std::int32_t, ::rs::never_t>>);

template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::std::int32_t, ::rs::never_t>, ::std::int32_t, U>)
inline constexpr ::rs::Result<::std::int32_t, ::rs::never_t>::Result(
    U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::std::int32_t, ::rs::never_t>, ::std::int32_t, U>)
inline constexpr ::rs::Result<::std::int32_t, ::rs::never_t>& ::rs::Result<
    ::std::int32_t, ::rs::never_t>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<::rs::never_t, F>)
inline constexpr ::rs::Result<::std::int32_t, ::rs::never_t>::Result(
    ::rs::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<::rs::never_t, F>)
inline constexpr ::rs::Result<::std::int32_t, ::rs::never_t>& ::rs::Result<
    ::std::int32_t, ::rs::never_t>::operator=(::rs::unexpected<F>&&
                                                  err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr ::rs::Result<::std::int32_t, ::rs::never_t>::Result(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr ::rs::Result<::std::int32_t, ::rs::never_t>::Result(
    ::rs::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020never_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000002c_x00000020_x00000021_x0000003e(
    ::rs::Result<::rs::alloc::string::String, ::rs::never_t> const&,
    ::rs::Result<::rs::alloc::string::String, ::rs::never_t>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000002c_x00000020_x00000021_x0000003e(
    ::rs::Result<::rs::alloc::string::String, ::rs::never_t>&,
    ::rs::Result<::rs::alloc::string::String, ::rs::never_t> const&);
}
inline ::rs::Result<::rs::alloc::string::String, ::rs::never_t>::Result(
    const Result& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000002c_x00000020_x00000021_x0000003e(
          other, this);
}
inline ::rs::Result<::rs::alloc::string::String, ::rs::never_t>& ::rs::Result<
    ::rs::alloc::string::String, ::rs::never_t>::operator=(const Result&
                                                               other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000002c_x00000020_x00000021_x0000003e(
            *this, other);
  }
  return *this;
}
inline ::rs::Result<::rs::alloc::string::String, ::rs::never_t>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline ::rs::Result<::rs::alloc::string::String,
                    ::rs::never_t>::~Result() noexcept {
  this->Reset();
}

template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::rs::alloc::string::String, ::rs::never_t>,
           ::rs::alloc::string::String, U>)
inline constexpr ::rs::Result<::rs::alloc::string::String,
                              ::rs::never_t>::Result(U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(::rs::ResultForwardConstructible<
           ::rs::Result<::rs::alloc::string::String, ::rs::never_t>,
           ::rs::alloc::string::String, U>)
inline constexpr ::rs::Result<::rs::alloc::string::String, ::rs::never_t>& ::
rs::Result<::rs::alloc::string::String, ::rs::never_t>::operator=(
    U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<::rs::never_t, F>)
inline constexpr ::rs::Result<::rs::alloc::string::String, ::rs::never_t>::
    Result(::rs::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(::rs::ResultUnexpectedConstructible<::rs::never_t, F>)
inline constexpr ::rs::Result<::rs::alloc::string::String, ::rs::never_t>& ::
rs::Result<::rs::alloc::string::String, ::rs::never_t>::operator=(
    ::rs::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr ::rs::Result<::rs::alloc::string::String,
                              ::rs::never_t>::Result(::std::in_place_t ip,
                                                     Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr ::rs::Result<::rs::alloc::string::String,
                              ::rs::never_t>::Result(::rs::unexpect_t u,
                                                     Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#pragma clang assume_nonnull end

#pragma clang diagnostic pop
#endif  // CC_BINDINGS_FROM_RS_TEST_NEVER_NEVER_GOLDEN
