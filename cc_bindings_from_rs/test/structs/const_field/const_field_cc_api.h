// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// const_field_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_CONST_FIELD_CONST_FIELD_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_CONST_FIELD_CONST_FIELD_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/bridge.h"
#include "support/internal/check.h"
#include "support/internal/move_assign.h"
#include "support/internal/slot.h"
#include "support/rs_std/result.h"

#include <bit>
#include <cstdint>
#include <cstring>
#include <optional>
#include <type_traits>
#include <utility>

#include "cc_bindings_from_rs/test/structs/const_field/cc_struct.h"

namespace const_field {

// CRUBIT_ANNOTATE: must_bind=
::std::optional<::struct_with_const_field>
return_struct_with_const_field_by_value_in_option();

}  // namespace const_field

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020struct_uwith_uconst_ufield_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020struct_uwith_uconst_ufield_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: cc_struct :: struct_with_const_field , u8 >")
    rs_std::Result<::struct_with_const_field, ::std::uint8_t> {
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
  Result(::struct_with_const_field&& ok) noexcept;
  Result& operator=(::struct_with_const_field&& ok) noexcept;
  Result(rs_std::unexpected<::std::uint8_t>&& err) noexcept;
  Result& operator=(rs_std::unexpected<::std::uint8_t>&& err) noexcept;
  template <typename... Args>
  Result(::std::in_place_t, Args&&... args);
  template <typename... Args>
  Result(rs_std::unexpect_t, Args&&... args);
  explicit constexpr operator bool() const noexcept;
  constexpr bool has_value() const noexcept;
  ::struct_with_const_field& value() &;
  ::struct_with_const_field&& value() &&;
  ::std::uint8_t& err() &;
  ::std::uint8_t&& err() &&;
  ::struct_with_const_field& operator*() &;
  ::struct_with_const_field const& operator*() const&;
  ::struct_with_const_field&& operator*() &&;
  ::struct_with_const_field* operator->();
  ::struct_with_const_field const* operator->() const;
  ~Result() noexcept = default;

 private:
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;
  void check_has_ok() const;
  void check_has_err() const;

 private:
  unsigned char __storage[16];
};
#endif

namespace const_field {

// CRUBIT_ANNOTATE: must_bind=
rs_std::Result<::struct_with_const_field, ::std::uint8_t>
return_struct_with_const_field_by_value_in_result();

namespace __crubit_internal {
extern "C" void
__crubit_thunk_return_ustruct_uwith_uconst_ufield_uby_uvalue_uin_uoption(
    unsigned char* crubit_nonnull __ret_ptr);
}
inline ::std::optional<::struct_with_const_field>
return_struct_with_const_field_by_value_in_option() {
  unsigned char __return_value_storage[::crubit::OptionAbi<
      ::crubit::TransmuteAbi<struct_with_const_field>>::kSize];
  __crubit_internal::
      __crubit_thunk_return_ustruct_uwith_uconst_ufield_uby_uvalue_uin_uoption(
          __return_value_storage);
  return ::crubit::internal::Decode<
      ::crubit::OptionAbi<::crubit::TransmuteAbi<struct_with_const_field>>>(
      ::crubit::OptionAbi<::crubit::TransmuteAbi<struct_with_const_field>>(
          ::crubit::TransmuteAbi<struct_with_const_field>()),
      __return_value_storage);
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_return_ustruct_uwith_uconst_ufield_uby_uvalue_uin_uresult(
    rs_std::Result<::struct_with_const_field, ::std::uint8_t>* crubit_nonnull
        __ret_ptr);
}
inline rs_std::Result<::struct_with_const_field, ::std::uint8_t>
return_struct_with_const_field_by_value_in_result() {
  crubit::Slot<rs_std::Result<::struct_with_const_field, ::std::uint8_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_return_ustruct_uwith_uconst_ufield_uby_uvalue_uin_uresult(
          __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace const_field

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020struct_uwith_uconst_ufield_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020struct_uwith_uconst_ufield_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Result<::struct_with_const_field, ::std::uint8_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Result<::struct_with_const_field, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<::struct_with_const_field, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<::struct_with_const_field, ::std::uint8_t>>);
inline rs_std::Result<::struct_with_const_field, ::std::uint8_t>::Result(
    ::struct_with_const_field&& ok) noexcept {
  set_tag(0);
  ::std::construct_at(
      reinterpret_cast<::struct_with_const_field*>(__storage + 8),
      ::std::move(ok));
}
inline rs_std::Result<::struct_with_const_field, ::std::uint8_t>&
rs_std::Result<::struct_with_const_field, ::std::uint8_t>::operator=(
    ::struct_with_const_field&& ok) noexcept {
  if (!has_value()) {
    ::std::destroy_at(reinterpret_cast<::std::uint8_t*>(__storage + 1));
    set_tag(0);
    ::std::construct_at(
        reinterpret_cast<::struct_with_const_field*>(__storage + 8),
        ::std::move(ok));
  } else {
    set_tag(0);
    ::crubit::MoveAssignOrDestroyAndConstruct(
        reinterpret_cast<::struct_with_const_field*>(__storage + 8),
        ::std::move(ok));
  }
  return *this;
}

inline rs_std::Result<::struct_with_const_field, ::std::uint8_t>::Result(
    rs_std::unexpected<::std::uint8_t>&& err) noexcept {
  set_tag(1);
  ::std::construct_at(reinterpret_cast<::std::uint8_t*>(__storage + 1),
                      ::std::move(err.error()));
}
inline rs_std::Result<::struct_with_const_field, ::std::uint8_t>&
rs_std::Result<::struct_with_const_field, ::std::uint8_t>::operator=(
    rs_std::unexpected<::std::uint8_t>&& err) noexcept {
  if (has_value()) {
    ::std::destroy_at(__storage + 8);
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
inline rs_std::Result<::struct_with_const_field, ::std::uint8_t>::Result(
    std::in_place_t, Args&&... args) {
  set_tag(0);
  std::construct_at(__storage + 8, std::forward<Args>(args)...);
}
template <typename... Args>
inline rs_std::Result<::struct_with_const_field, ::std::uint8_t>::Result(
    rs_std::unexpect_t, Args&&... args) {
  set_tag(1);
  std::construct_at(__storage + 1, std::forward<Args>(args)...);
}
inline constexpr rs_std::Result<::struct_with_const_field, ::std::uint8_t>::
operator bool() const noexcept {
  return has_value();
}
inline constexpr bool rs_std::Result<
    ::struct_with_const_field, ::std::uint8_t>::has_value() const noexcept {
  return tag() == 0;
}
inline ::struct_with_const_field&
rs_std::Result<::struct_with_const_field, ::std::uint8_t>::value() & {
  check_has_ok();
  return *reinterpret_cast<::struct_with_const_field*>(__storage + 8);
}
inline ::struct_with_const_field&&
rs_std::Result<::struct_with_const_field, ::std::uint8_t>::value() && {
  check_has_ok();
  return ::std::move(
      *reinterpret_cast<::struct_with_const_field*>(__storage + 8));
}
inline ::std::uint8_t&
rs_std::Result<::struct_with_const_field, ::std::uint8_t>::err() & {
  check_has_err();
  return *reinterpret_cast<::std::uint8_t*>(__storage + 1);
}
inline ::std::uint8_t&&
rs_std::Result<::struct_with_const_field, ::std::uint8_t>::err() && {
  check_has_err();
  return ::std::move(*reinterpret_cast<::std::uint8_t*>(__storage + 1));
}
inline ::struct_with_const_field&
rs_std::Result<::struct_with_const_field, ::std::uint8_t>::operator*() & {
  check_has_ok();
  return *reinterpret_cast<::struct_with_const_field*>(__storage + 8);
}
inline ::struct_with_const_field const&
rs_std::Result<::struct_with_const_field, ::std::uint8_t>::operator*() const& {
  check_has_ok();
  return *reinterpret_cast<::struct_with_const_field const*>(__storage + 8);
}
inline ::struct_with_const_field&&
rs_std::Result<::struct_with_const_field, ::std::uint8_t>::operator*() && {
  check_has_ok();
  return ::std::move(
      *reinterpret_cast<::struct_with_const_field*>(__storage + 8));
}
inline ::struct_with_const_field*
rs_std::Result<::struct_with_const_field, ::std::uint8_t>::operator->() {
  check_has_ok();
  return reinterpret_cast<::struct_with_const_field*>(__storage + 8);
}
inline ::struct_with_const_field const*
rs_std::Result<::struct_with_const_field, ::std::uint8_t>::operator->() const {
  check_has_ok();
  return reinterpret_cast<::struct_with_const_field const*>(__storage + 8);
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Result<::struct_with_const_field, ::std::uint8_t>>);
inline constexpr ::std::uint8_t rs_std::Result<
    ::struct_with_const_field, ::std::uint8_t>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void
rs_std::Result<::struct_with_const_field, ::std::uint8_t>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

inline void rs_std::Result<::struct_with_const_field,
                           ::std::uint8_t>::check_has_ok() const {
  CRUBIT_CHECK(has_value()) << "Bad value access on rs_std::Result";
}
inline void rs_std::Result<::struct_with_const_field,
                           ::std::uint8_t>::check_has_err() const {
  CRUBIT_CHECK(!has_value()) << "Bad error access on rs_std::Result";
}
#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_CONST_FIELD_CONST_FIELD_GOLDEN
