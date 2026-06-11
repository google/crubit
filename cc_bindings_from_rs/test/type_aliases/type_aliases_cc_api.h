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
#include "support/internal/check.h"
#include "support/internal/move_assign.h"
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
        rs_std::Result<::std::int32_t, ::std::int32_t> {
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
  Result(::std::int32_t&& ok) noexcept;
  Result& operator=(::std::int32_t&& ok) noexcept;
  Result(rs_std::unexpected<::std::int32_t>&& err) noexcept;
  Result& operator=(rs_std::unexpected<::std::int32_t>&& err) noexcept;
  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs_std::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  ::std::int32_t& value() &;
  ::std::int32_t&& value() &&;
  ::std::int32_t& err() &;
  ::std::int32_t&& err() &&;
  ::std::int32_t& operator*() &;
  std::add_const_t<::std::int32_t>& operator*() const&;
  ::std::int32_t&& operator*() &&;
  ::std::int32_t* operator->();
  std::add_const_t<::std::int32_t>* operator->() const;
  ~Result() noexcept = default;

 private:
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;
  void check_has_ok() const;
  void check_has_err() const;

 private:
  unsigned char __storage[8];
};
#endif

namespace type_aliases::test_generics_matching {

using SpecializedAlias CRUBIT_INTERNAL_RUST_TYPE(
    ":: type_aliases_golden :: test_generics_matching :: SpecializedAlias") =
    rs_std::Result<::std::int32_t, ::std::int32_t>;

rs_std::Result<::std::int32_t, ::std::int32_t> returns_matching_alias();

rs_std::Result<::std::int32_t, ::std::int32_t> returns_specialized();

}  // namespace type_aliases::test_generics_matching

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int8_ut_x00000020_x0000003e
template <>
struct alignas(4)
    CRUBIT_INTERNAL_RUST_TYPE("std :: result :: Result < u32 , i8 >")
        rs_std::Result<::std::uint32_t, ::std::int8_t> {
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
  Result(rs_std::unexpected<::std::int8_t>&& err) noexcept;
  Result& operator=(rs_std::unexpected<::std::int8_t>&& err) noexcept;
  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs_std::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  ::std::uint32_t& value() &;
  ::std::uint32_t&& value() &&;
  ::std::int8_t& err() &;
  ::std::int8_t&& err() &&;
  ::std::uint32_t& operator*() &;
  std::add_const_t<::std::uint32_t>& operator*() const&;
  ::std::uint32_t&& operator*() &&;
  ::std::uint32_t* operator->();
  std::add_const_t<::std::uint32_t>* operator->() const;
  ~Result() noexcept = default;

 private:
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;
  void check_has_ok() const;
  void check_has_err() const;

 private:
  unsigned char __storage[8];
};
#endif

namespace type_aliases::test_generics_matching {

rs_std::Result<::std::uint32_t, ::std::int8_t> returns_flipped_alias();

}

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
inline rs_std::Result<::std::int32_t, ::std::int32_t>::Result(
    ::std::int32_t&& ok) noexcept {
  set_tag(0);
  ::std::construct_at(reinterpret_cast<::std::int32_t*>(__storage + 4),
                      ::std::move(ok));
}
inline rs_std::Result<::std::int32_t, ::std::int32_t>& rs_std::Result<
    ::std::int32_t, ::std::int32_t>::operator=(::std::int32_t&& ok) noexcept {
  if (!has_value()) {
    ::std::destroy_at(reinterpret_cast<::std::int32_t*>(__storage + 4));
    set_tag(0);
    ::std::construct_at(reinterpret_cast<::std::int32_t*>(__storage + 4),
                        ::std::move(ok));
  } else {
    set_tag(0);
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::int32_t*>(__storage + 4), ::std::move(ok));
  }
  return *this;
}

inline rs_std::Result<::std::int32_t, ::std::int32_t>::Result(
    rs_std::unexpected<::std::int32_t>&& err) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::std::int32_t*>(__storage + 4),
                      ::std::move(err.error()));
}
inline rs_std::Result<::std::int32_t, ::std::int32_t>&
rs_std::Result<::std::int32_t, ::std::int32_t>::operator=(
    rs_std::unexpected<::std::int32_t>&& err) noexcept {
  if (has_value()) {
    ::std::destroy_at(__storage + 4);
    set_tag(1);
    ::std::construct_at(reinterpret_cast<::std::int32_t*>(__storage + 4),
                        ::std::move(err.error()));
  } else {
    set_tag(1);
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::int32_t*>(__storage + 4),
        ::std::move(err.error()));
  }
  return *this;
}

template <typename... Args>
inline rs_std::Result<::std::int32_t, ::std::int32_t>::Result(std::in_place_t,
                                                              Args&&... args) {
  set_tag(0);
  std::construct_at(__storage + 4, std::forward<Args>(args)...);
}
template <typename... Args>
inline rs_std::Result<::std::int32_t, ::std::int32_t>::Result(
    rs_std::unexpect_t, Args&&... args) {
  set_tag(1);
  std::construct_at(__storage + 4, std::forward<Args>(args)...);
}
inline constexpr rs_std::Result<::std::int32_t, ::std::int32_t>::operator bool()
    const noexcept {
  return has_value();
}
inline constexpr bool
rs_std::Result<::std::int32_t, ::std::int32_t>::has_value() const noexcept {
  return tag() == 0;
}
inline ::std::int32_t&
rs_std::Result<::std::int32_t, ::std::int32_t>::value() & {
  check_has_ok();
  return *reinterpret_cast<::std::int32_t*>(__storage + 4);
}
inline ::std::int32_t&&
rs_std::Result<::std::int32_t, ::std::int32_t>::value() && {
  check_has_ok();
  return ::std::move(*reinterpret_cast<::std::int32_t*>(__storage + 4));
}
inline ::std::int32_t& rs_std::Result<::std::int32_t, ::std::int32_t>::err() & {
  check_has_err();
  return *reinterpret_cast<::std::int32_t*>(__storage + 4);
}
inline ::std::int32_t&&
rs_std::Result<::std::int32_t, ::std::int32_t>::err() && {
  check_has_err();
  return ::std::move(*reinterpret_cast<::std::int32_t*>(__storage + 4));
}
inline ::std::int32_t&
rs_std::Result<::std::int32_t, ::std::int32_t>::operator*() & {
  check_has_ok();
  return *reinterpret_cast<::std::int32_t*>(__storage + 4);
}
inline std::add_const_t<::std::int32_t>&
rs_std::Result<::std::int32_t, ::std::int32_t>::operator*() const& {
  check_has_ok();
  return *reinterpret_cast<std::add_const_t<::std::int32_t>*>(__storage + 4);
}
inline ::std::int32_t&&
rs_std::Result<::std::int32_t, ::std::int32_t>::operator*() && {
  check_has_ok();
  return ::std::move(*reinterpret_cast<::std::int32_t*>(__storage + 4));
}
inline ::std::int32_t*
rs_std::Result<::std::int32_t, ::std::int32_t>::operator->() {
  check_has_ok();
  return reinterpret_cast<::std::int32_t*>(__storage + 4);
}
inline std::add_const_t<::std::int32_t>*
rs_std::Result<::std::int32_t, ::std::int32_t>::operator->() const {
  check_has_ok();
  return reinterpret_cast<std::add_const_t<::std::int32_t>*>(__storage + 4);
}
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

inline void rs_std::Result<::std::int32_t, ::std::int32_t>::check_has_ok()
    const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs_std::Result";
}
inline void rs_std::Result<::std::int32_t, ::std::int32_t>::check_has_err()
    const {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs_std::Result";
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
inline rs_std::Result<::std::uint32_t, ::std::int8_t>::Result(
    ::std::uint32_t&& ok) noexcept {
  set_tag(0);
  ::std::construct_at(reinterpret_cast<::std::uint32_t*>(__storage + 4),
                      ::std::move(ok));
}
inline rs_std::Result<::std::uint32_t, ::std::int8_t>& rs_std::Result<
    ::std::uint32_t, ::std::int8_t>::operator=(::std::uint32_t&& ok) noexcept {
  if (!has_value()) {
    ::std::destroy_at(reinterpret_cast<::std::int8_t*>(__storage + 1));
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

inline rs_std::Result<::std::uint32_t, ::std::int8_t>::Result(
    rs_std::unexpected<::std::int8_t>&& err) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::std::int8_t*>(__storage + 1),
                      ::std::move(err.error()));
}
inline rs_std::Result<::std::uint32_t, ::std::int8_t>&
rs_std::Result<::std::uint32_t, ::std::int8_t>::operator=(
    rs_std::unexpected<::std::int8_t>&& err) noexcept {
  if (has_value()) {
    ::std::destroy_at(__storage + 4);
    set_tag(1);
    ::std::construct_at(reinterpret_cast<::std::int8_t*>(__storage + 1),
                        ::std::move(err.error()));
  } else {
    set_tag(1);
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::std::int8_t*>(__storage + 1),
        ::std::move(err.error()));
  }
  return *this;
}

template <typename... Args>
inline rs_std::Result<::std::uint32_t, ::std::int8_t>::Result(std::in_place_t,
                                                              Args&&... args) {
  set_tag(0);
  std::construct_at(__storage + 4, std::forward<Args>(args)...);
}
template <typename... Args>
inline rs_std::Result<::std::uint32_t, ::std::int8_t>::Result(
    rs_std::unexpect_t, Args&&... args) {
  set_tag(1);
  std::construct_at(__storage + 1, std::forward<Args>(args)...);
}
inline constexpr rs_std::Result<::std::uint32_t, ::std::int8_t>::operator bool()
    const noexcept {
  return has_value();
}
inline constexpr bool
rs_std::Result<::std::uint32_t, ::std::int8_t>::has_value() const noexcept {
  return tag() == 0;
}
inline ::std::uint32_t&
rs_std::Result<::std::uint32_t, ::std::int8_t>::value() & {
  check_has_ok();
  return *reinterpret_cast<::std::uint32_t*>(__storage + 4);
}
inline ::std::uint32_t&&
rs_std::Result<::std::uint32_t, ::std::int8_t>::value() && {
  check_has_ok();
  return ::std::move(*reinterpret_cast<::std::uint32_t*>(__storage + 4));
}
inline ::std::int8_t& rs_std::Result<::std::uint32_t, ::std::int8_t>::err() & {
  check_has_err();
  return *reinterpret_cast<::std::int8_t*>(__storage + 1);
}
inline ::std::int8_t&&
rs_std::Result<::std::uint32_t, ::std::int8_t>::err() && {
  check_has_err();
  return ::std::move(*reinterpret_cast<::std::int8_t*>(__storage + 1));
}
inline ::std::uint32_t&
rs_std::Result<::std::uint32_t, ::std::int8_t>::operator*() & {
  check_has_ok();
  return *reinterpret_cast<::std::uint32_t*>(__storage + 4);
}
inline std::add_const_t<::std::uint32_t>&
rs_std::Result<::std::uint32_t, ::std::int8_t>::operator*() const& {
  check_has_ok();
  return *reinterpret_cast<std::add_const_t<::std::uint32_t>*>(__storage + 4);
}
inline ::std::uint32_t&&
rs_std::Result<::std::uint32_t, ::std::int8_t>::operator*() && {
  check_has_ok();
  return ::std::move(*reinterpret_cast<::std::uint32_t*>(__storage + 4));
}
inline ::std::uint32_t*
rs_std::Result<::std::uint32_t, ::std::int8_t>::operator->() {
  check_has_ok();
  return reinterpret_cast<::std::uint32_t*>(__storage + 4);
}
inline std::add_const_t<::std::uint32_t>*
rs_std::Result<::std::uint32_t, ::std::int8_t>::operator->() const {
  check_has_ok();
  return reinterpret_cast<std::add_const_t<::std::uint32_t>*>(__storage + 4);
}
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

inline void rs_std::Result<::std::uint32_t, ::std::int8_t>::check_has_ok()
    const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs_std::Result";
}
inline void rs_std::Result<::std::uint32_t, ::std::int8_t>::check_has_err()
    const {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs_std::Result";
}
#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TYPE_ALIASES_TYPE_ALIASES_GOLDEN
