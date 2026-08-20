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
#include "support/internal/slot.h"
#include "support/rs_std/option.h"
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
rs_std::Option<::struct_with_const_field>
return_struct_with_const_field_by_value_in_option();

// CRUBIT_ANNOTATE: must_bind=
rs_std::Result<::struct_with_const_field, ::std::uint8_t>
return_struct_with_const_field_by_value_in_result();

}  // namespace const_field

#ifndef _CRUBIT_BINDINGS_FOR_core_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ccc_ustruct_x0000003a_x0000003astruct_uwith_uconst_ufield_x0000003e
#define _CRUBIT_BINDINGS_FOR_core_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ccc_ustruct_x0000003a_x0000003astruct_uwith_uconst_ufield_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: cc_struct :: struct_with_const_field >")
    rs_std::Option<::struct_with_const_field>
    : public rs_std::OptionBase<rs_std::Option<::struct_with_const_field>,
                                ::struct_with_const_field> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      rs_std::OptionBase<rs_std::Option<::struct_with_const_field>,
                         ::struct_with_const_field>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option,
                                                ::struct_with_const_field, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option,
                                                ::struct_with_const_field, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::struct_with_const_field, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::struct_with_const_field, Opt>)
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint64_t;
  static constexpr tag_type kNoneVal = 0;
  ::struct_with_const_field* some_ptr() noexcept {
    return reinterpret_cast<::struct_with_const_field*>(storage_ + 8);
  }
  ::struct_with_const_field const* some_const_ptr() const noexcept {
    return reinterpret_cast<::struct_with_const_field const*>(storage_ + 8);
  }
  void set_some_tag() noexcept { set_tag(1); }
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char storage_[16];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_core_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ccc_ustruct_x0000003a_x0000003astruct_uwith_uconst_ufield_x0000002c_x00000020u8_x0000003e
#define _CRUBIT_BINDINGS_FOR_core_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ccc_ustruct_x0000003a_x0000003astruct_uwith_uconst_ufield_x0000002c_x00000020u8_x0000003e
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

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type = rs_std::ResultBase<
      rs_std::Result<::struct_with_const_field, ::std::uint8_t>,
      ::struct_with_const_field, ::std::uint8_t>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result,
                                                ::struct_with_const_field, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result,
                                                ::struct_with_const_field, U>)
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
    rs_std::Option<::struct_with_const_field>* __ret_ptr);
}
inline rs_std::Option<::struct_with_const_field>
return_struct_with_const_field_by_value_in_option() {
  crubit::Slot<rs_std::Option<::struct_with_const_field>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_return_ustruct_uwith_uconst_ufield_uby_uvalue_uin_uoption(
          __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
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

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_core_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ccc_ustruct_x0000003a_x0000003astruct_uwith_uconst_ufield_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_core_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ccc_ustruct_x0000003a_x0000003astruct_uwith_uconst_ufield_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Option<::struct_with_const_field>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Option<::struct_with_const_field>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Option<::struct_with_const_field>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Option<::struct_with_const_field>>);
inline rs_std::Option<::struct_with_const_field>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Option<::struct_with_const_field>>);
inline constexpr ::std::uint64_t
rs_std::Option<::struct_with_const_field>::tag() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void rs_std::Option<::struct_with_const_field>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<::struct_with_const_field>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<::struct_with_const_field>& rs_std::Option<
    ::struct_with_const_field>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::struct_with_const_field>, ::struct_with_const_field,
           U>)
inline rs_std::Option<::struct_with_const_field>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::struct_with_const_field>, ::struct_with_const_field,
           U>)
inline rs_std::Option<::struct_with_const_field>&
rs_std::Option<::struct_with_const_field>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::struct_with_const_field, Opt>)
inline rs_std::Option<::struct_with_const_field>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::struct_with_const_field, Opt>)
inline rs_std::Option<::struct_with_const_field>&
rs_std::Option<::struct_with_const_field>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<::struct_with_const_field>::Option(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_core_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ccc_ustruct_x0000003a_x0000003astruct_uwith_uconst_ufield_x0000002c_x00000020u8_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_core_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ccc_ustruct_x0000003a_x0000003astruct_uwith_uconst_ufield_x0000002c_x00000020u8_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Result<::struct_with_const_field, ::std::uint8_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Result<::struct_with_const_field, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<::struct_with_const_field, ::std::uint8_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<::struct_with_const_field, ::std::uint8_t>>);
inline rs_std::Result<::struct_with_const_field, ::std::uint8_t>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::struct_with_const_field, ::std::uint8_t>,
           ::struct_with_const_field, U>)
inline constexpr rs_std::Result<::struct_with_const_field,
                                ::std::uint8_t>::Result(U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::struct_with_const_field, ::std::uint8_t>,
           ::struct_with_const_field, U>)
inline constexpr rs_std::Result<::struct_with_const_field, ::std::uint8_t>&
rs_std::Result<::struct_with_const_field, ::std::uint8_t>::operator=(
    U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs_std::Result<::struct_with_const_field, ::std::uint8_t>::
    Result(rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs_std::Result<::struct_with_const_field, ::std::uint8_t>&
rs_std::Result<::struct_with_const_field, ::std::uint8_t>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<::struct_with_const_field,
                                ::std::uint8_t>::Result(::std::in_place_t ip,
                                                        Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<::struct_with_const_field,
                                ::std::uint8_t>::Result(rs_std::unexpect_t u,
                                                        Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_CONST_FIELD_CONST_FIELD_GOLDEN
