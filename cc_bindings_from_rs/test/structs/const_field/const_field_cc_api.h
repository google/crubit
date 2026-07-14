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
rs_std::Result<::struct_with_const_field, ::std::uint8_t>
return_struct_with_const_field_by_value_in_result();

}  // namespace const_field

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020struct_uwith_uconst_ufield_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020struct_uwith_uconst_ufield_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: cc_struct :: struct_with_const_field , u8 >")
    rs_std::Result<::struct_with_const_field, ::std::uint8_t>
    : public rs_std::ResultBase<
          rs_std::Result<::struct_with_const_field, ::std::uint8_t>,
          ::struct_with_const_field, ::std::uint8_t> {
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
  using base_type = rs_std::ResultBase<
      rs_std::Result<::struct_with_const_field, ::std::uint8_t>,
      ::struct_with_const_field, ::std::uint8_t>;
  template <typename U>
    requires(!std::is_base_of_v<Result, std::decay_t<U>> &&
             !rs_std::is_unexpected_v<std::decay_t<U>> &&
             !std::is_same_v<std::decay_t<U>, rs_std::unexpect_t> &&
             !std::is_same_v<std::decay_t<U>, ::std::in_place_t> &&
             std::is_constructible_v<::struct_with_const_field, U>)
  explicit constexpr Result(U&& ok) noexcept
      : base_type(::std::forward<U>(ok)) {}
  template <typename U>
    requires(!std::is_base_of_v<Result, std::decay_t<U>> &&
             !rs_std::is_unexpected_v<std::decay_t<U>> &&
             !std::is_same_v<std::decay_t<U>, rs_std::unexpect_t> &&
             std::is_constructible_v<::struct_with_const_field, U>)
  constexpr Result& operator=(U&& ok) noexcept {
    base_type::operator=(::std::forward<U>(ok));
    return *this;
  }
  template <typename F>
    requires(std::is_constructible_v<::std::uint8_t, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept
      : base_type(::std::move(err)) {}
  template <typename F>
    requires(std::is_constructible_v<::std::uint8_t, F>)
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
    rs_std::Result<::struct_with_const_field, ::std::uint8_t>* __ret_ptr);
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

#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_CONST_FIELD_CONST_FIELD_GOLDEN
