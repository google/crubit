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
#include "support/bridge.h"
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

// CRUBIT_ANNOTATE: must_bind=
rs::Result<::struct_with_const_field, ::std::uint8_t>
return_struct_with_const_field_by_value_in_result();

}  // namespace const_field

#ifndef _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020struct_uwith_uconst_ufield_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020struct_uwith_uconst_ufield_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: cc_struct :: struct_with_const_field , u8 >")
    rs::Result<::struct_with_const_field, ::std::uint8_t>
    : public rs::ResultBase<
          rs::Result<::struct_with_const_field, ::std::uint8_t>,
          ::struct_with_const_field, ::std::uint8_t> {
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
      rs::ResultBase<rs::Result<::struct_with_const_field, ::std::uint8_t>,
                     ::struct_with_const_field, ::std::uint8_t>;
  template <typename U>
    requires(
        rs::ResultForwardConstructible<Result, ::struct_with_const_field, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(
        rs::ResultForwardConstructible<Result, ::struct_with_const_field, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs::ResultUnexpectedConstructible<::std::uint8_t, F>)
  explicit constexpr Result(rs::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs::ResultUnexpectedConstructible<::std::uint8_t, F>)
  constexpr Result& operator=(rs::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  ::struct_with_const_field* ok_ptr() noexcept {
    return reinterpret_cast<::struct_with_const_field*>(__storage + 8);
  }
  ::struct_with_const_field const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::struct_with_const_field const*>(__storage + 8);
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

namespace const_field {

namespace __crubit_internal {
extern "C" void
__crubit_thunk_return_ustruct_uwith_uconst_ufield_uby_uvalue_uin_uoption(
    unsigned char* __ret_ptr);
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
    rs::Result<::struct_with_const_field, ::std::uint8_t>* __ret_ptr);
}
inline rs::Result<::struct_with_const_field, ::std::uint8_t>
return_struct_with_const_field_by_value_in_result() {
  crubit::Slot<rs::Result<::struct_with_const_field, ::std::uint8_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_return_ustruct_uwith_uconst_ufield_uby_uvalue_uin_uresult(
          __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace const_field

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020struct_uwith_uconst_ufield_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020struct_uwith_uconst_ufield_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs::Result<::struct_with_const_field, ::std::uint8_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs::Result<::struct_with_const_field, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs::Result<::struct_with_const_field, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs::Result<::struct_with_const_field, ::std::uint8_t>>);
inline rs::Result<::struct_with_const_field, ::std::uint8_t>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              rs::Result<::struct_with_const_field, ::std::uint8_t>>);
inline constexpr ::std::uint8_t
rs::Result<::struct_with_const_field, ::std::uint8_t>::tag() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return ::std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void
rs::Result<::struct_with_const_field, ::std::uint8_t>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

template <typename U>
  requires(rs::ResultForwardConstructible<
           rs::Result<::struct_with_const_field, ::std::uint8_t>,
           ::struct_with_const_field, U>)
inline constexpr rs::Result<::struct_with_const_field, ::std::uint8_t>::Result(
    U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs::ResultForwardConstructible<
           rs::Result<::struct_with_const_field, ::std::uint8_t>,
           ::struct_with_const_field, U>)
inline constexpr rs::Result<::struct_with_const_field, ::std::uint8_t>&
rs::Result<::struct_with_const_field, ::std::uint8_t>::operator=(
    U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs::Result<::struct_with_const_field, ::std::uint8_t>::Result(
    rs::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs::Result<::struct_with_const_field, ::std::uint8_t>&
rs::Result<::struct_with_const_field, ::std::uint8_t>::operator=(
    rs::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs::Result<::struct_with_const_field, ::std::uint8_t>::Result(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs::Result<::struct_with_const_field, ::std::uint8_t>::Result(
    rs::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_CONST_FIELD_CONST_FIELD_GOLDEN
