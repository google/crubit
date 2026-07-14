// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// type_aliases_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TYPE_ALIASES_TYPE_ALIASES_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TYPE_ALIASES_TYPE_ALIASES_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/rs_std/result.h"

#include <bit>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

namespace type_aliases::test_deprecated_type_alias {

using TypeAlias CRUBIT_INTERNAL_RUST_TYPE(
    ":: type_aliases_golden :: test_deprecated_type_alias :: TypeAlias")
    [[deprecated("Use `OtherTypeAlias` instead")]] = ::std::int32_t;
}

namespace type_aliases::test_generics_matching {

using SpecializedAlias CRUBIT_INTERNAL_RUST_TYPE(
    ":: type_aliases_golden :: test_generics_matching :: SpecializedAlias") =
    rs_std::Result<::std::int32_t, ::std::int32_t>;

rs_std::Result<::std::uint32_t, ::std::int8_t> returns_flipped_alias();

rs_std::Result<::std::int32_t, ::std::int32_t> returns_matching_alias();

rs_std::Result<::std::int32_t, ::std::int32_t> returns_specialized();

}  // namespace type_aliases::test_generics_matching

namespace type_aliases::test_type_aliases {

using TypeAlias CRUBIT_INTERNAL_RUST_TYPE(
    ":: type_aliases_golden :: test_type_aliases :: TypeAlias") =
    ::std::int32_t;
using TypeAlias2 CRUBIT_INTERNAL_RUST_TYPE(
    ":: type_aliases_golden :: test_type_aliases :: TypeAlias2") =
    ::std::int32_t;

::std::int32_t func_using_alias();

}  // namespace type_aliases::test_type_aliases

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
template <>
struct alignas(4)
    CRUBIT_INTERNAL_RUST_TYPE("std :: result :: Result < i32 , i32 >")
        rs_std::Result<::std::int32_t, ::std::int32_t>
    : public rs_std::ResultBase<rs_std::Result<::std::int32_t, ::std::int32_t>,
                                ::std::int32_t, ::std::int32_t> {
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

 public:
  using base_type =
      rs_std::ResultBase<rs_std::Result<::std::int32_t, ::std::int32_t>,
                         ::std::int32_t, ::std::int32_t>;
  template <typename U>
    requires(!std::is_base_of_v<Result, std::decay_t<U>> &&
             !rs_std::is_unexpected_v<std::decay_t<U>> &&
             !std::is_same_v<std::decay_t<U>, rs_std::unexpect_t> &&
             !std::is_same_v<std::decay_t<U>, ::std::in_place_t> &&
             std::is_constructible_v<::std::int32_t, U>)
  explicit constexpr Result(U&& ok) noexcept
      : base_type(::std::forward<U>(ok)) {}
  template <typename U>
    requires(!std::is_base_of_v<Result, std::decay_t<U>> &&
             !rs_std::is_unexpected_v<std::decay_t<U>> &&
             !std::is_same_v<std::decay_t<U>, rs_std::unexpect_t> &&
             std::is_constructible_v<::std::int32_t, U>)
  constexpr Result& operator=(U&& ok) noexcept {
    base_type::operator=(::std::forward<U>(ok));
    return *this;
  }
  template <typename F>
    requires(std::is_constructible_v<::std::int32_t, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept
      : base_type(::std::move(err)) {}
  template <typename F>
    requires(std::is_constructible_v<::std::int32_t, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept {
    base_type::operator=(::std::move(err));
    return *this;
  }
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept
      : base_type(ip, ::std::forward<Args>(args)...) {}
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept
      : base_type(u, ::std::forward<Args>(args)...) {}
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  ::std::int32_t* ok_ptr() noexcept {
    return reinterpret_cast<::std::int32_t*>(__storage + 4);
  }
  ::std::int32_t const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::std::int32_t const*>(__storage + 4);
  }
  ::std::int32_t* err_ptr() noexcept {
    return reinterpret_cast<::std::int32_t*>(__storage + 4);
  }
  ::std::int32_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::std::int32_t const*>(__storage + 4);
  }
  void set_ok_tag() noexcept { set_tag(0); }
  void set_err_tag() noexcept { set_tag(1); }
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;

 private:
  unsigned char __storage[8];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000003e
template <>
struct alignas(4)
    CRUBIT_INTERNAL_RUST_TYPE("std :: result :: Result < u32 , i8 >")
        rs_std::Result<::std::uint32_t, ::std::int8_t>
    : public rs_std::ResultBase<rs_std::Result<::std::uint32_t, ::std::int8_t>,
                                ::std::uint32_t, ::std::int8_t> {
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

 public:
  using base_type =
      rs_std::ResultBase<rs_std::Result<::std::uint32_t, ::std::int8_t>,
                         ::std::uint32_t, ::std::int8_t>;
  template <typename U>
    requires(!std::is_base_of_v<Result, std::decay_t<U>> &&
             !rs_std::is_unexpected_v<std::decay_t<U>> &&
             !std::is_same_v<std::decay_t<U>, rs_std::unexpect_t> &&
             !std::is_same_v<std::decay_t<U>, ::std::in_place_t> &&
             std::is_constructible_v<::std::uint32_t, U>)
  explicit constexpr Result(U&& ok) noexcept
      : base_type(::std::forward<U>(ok)) {}
  template <typename U>
    requires(!std::is_base_of_v<Result, std::decay_t<U>> &&
             !rs_std::is_unexpected_v<std::decay_t<U>> &&
             !std::is_same_v<std::decay_t<U>, rs_std::unexpect_t> &&
             std::is_constructible_v<::std::uint32_t, U>)
  constexpr Result& operator=(U&& ok) noexcept {
    base_type::operator=(::std::forward<U>(ok));
    return *this;
  }
  template <typename F>
    requires(std::is_constructible_v<::std::int8_t, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept
      : base_type(::std::move(err)) {}
  template <typename F>
    requires(std::is_constructible_v<::std::int8_t, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept {
    base_type::operator=(::std::move(err));
    return *this;
  }
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept
      : base_type(ip, ::std::forward<Args>(args)...) {}
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept
      : base_type(u, ::std::forward<Args>(args)...) {}
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
  unsigned char __storage[8];
};
#endif

namespace type_aliases::test_generics_matching {

// Error generating bindings for enum `std::result::Result` defined at
// third_party/rust_toolchain/library/core/src/result.rs;l=557:
// The following Rust type is not supported yet: T

// Error generating bindings for enum `std::result::Result` defined at
// third_party/rust_toolchain/library/core/src/result.rs;l=557:
// The following Rust type is not supported yet: T

}  // namespace type_aliases::test_generics_matching

namespace type_aliases::test_generics_matching {

namespace __crubit_internal {
extern "C" void __crubit_thunk_returns_uflipped_ualias(
    rs_std::Result<::std::uint32_t, ::std::int8_t>* __ret_ptr);
}
inline rs_std::Result<::std::uint32_t, ::std::int8_t> returns_flipped_alias() {
  crubit::Slot<rs_std::Result<::std::uint32_t, ::std::int8_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_returns_uflipped_ualias(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_returns_umatching_ualias(
    rs_std::Result<::std::int32_t, ::std::int32_t>* __ret_ptr);
}
inline rs_std::Result<::std::int32_t, ::std::int32_t> returns_matching_alias() {
  crubit::Slot<rs_std::Result<::std::int32_t, ::std::int32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_returns_umatching_ualias(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_returns_uspecialized(
    rs_std::Result<::std::int32_t, ::std::int32_t>* __ret_ptr);
}
inline rs_std::Result<::std::int32_t, ::std::int32_t> returns_specialized() {
  crubit::Slot<rs_std::Result<::std::int32_t, ::std::int32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_returns_uspecialized(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace type_aliases::test_generics_matching

namespace type_aliases::test_type_aliases {

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_func_uusing_ualias();
}
inline ::std::int32_t func_using_alias() {
  return __crubit_internal::__crubit_thunk_func_uusing_ualias();
}

}  // namespace type_aliases::test_type_aliases

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Result<::std::int32_t, ::std::int32_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Result<::std::int32_t, ::std::int32_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<::std::int32_t, ::std::int32_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<::std::int32_t, ::std::int32_t>>);
static_assert(::std::is_trivially_destructible_v<
              rs_std::Result<::std::int32_t, ::std::int32_t>>);
inline constexpr ::std::uint32_t
rs_std::Result<::std::int32_t, ::std::int32_t>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void rs_std::Result<::std::int32_t, ::std::int32_t>::set_tag(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint32_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Result<::std::uint32_t, ::std::int8_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Result<::std::uint32_t, ::std::int8_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<::std::uint32_t, ::std::int8_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<::std::uint32_t, ::std::int8_t>>);
static_assert(::std::is_trivially_destructible_v<
              rs_std::Result<::std::uint32_t, ::std::int8_t>>);
inline constexpr ::std::uint8_t
rs_std::Result<::std::uint32_t, ::std::int8_t>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void rs_std::Result<::std::uint32_t, ::std::int8_t>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TYPE_ALIASES_TYPE_ALIASES_GOLDEN
