// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// dep_crate_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_SPECIALIZATION_DEDUPLICATION_DEP_CRATE_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_SPECIALIZATION_DEDUPLICATION_DEP_CRATE_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/rs_std/result.h"

#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

namespace dep_crate {

struct CRUBIT_INTERNAL_RUST_TYPE(":: dep_crate_golden :: ExpectedName") alignas(
    4) [[clang::trivial_abi]] ExpectedName final {
 public:
  ::std::int32_t x = {};

 private:
  static void __crubit_field_offset_assertions();
};

rs_std::Result<::dep_crate::ExpectedName, ::std::int32_t> use_s();

}  // namespace dep_crate

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020dep_ucrate_x00000020_x0000003a_x0000003a_x00000020ExpectedName_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020dep_ucrate_x00000020_x0000003a_x0000003a_x00000020ExpectedName_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: dep_crate_golden :: ExpectedName , i32 >")
    rs_std::Result<::dep_crate::ExpectedName, ::std::int32_t>
    : public rs_std::ResultBase<
          rs_std::Result<::dep_crate::ExpectedName, ::std::int32_t>,
          ::dep_crate::ExpectedName, ::std::int32_t> {
 public:
  // `core::result::Result` doesn't implement the `Clone` trait
  Result(const Result&) = delete;
  Result& operator=(const Result&) = delete;
  Result(Result&&) = default;
  Result& operator=(Result&&) = default;

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type = rs_std::ResultBase<
      rs_std::Result<::dep_crate::ExpectedName, ::std::int32_t>,
      ::dep_crate::ExpectedName, ::std::int32_t>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result,
                                                ::dep_crate::ExpectedName, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<Result,
                                                ::dep_crate::ExpectedName, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::int32_t, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::int32_t, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept = default;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  ::dep_crate::ExpectedName* ok_ptr() noexcept {
    return reinterpret_cast<::dep_crate::ExpectedName*>(__storage + 4);
  }
  ::dep_crate::ExpectedName const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::dep_crate::ExpectedName const*>(__storage + 4);
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

namespace dep_crate {

static_assert(
    sizeof(ExpectedName) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(ExpectedName) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<ExpectedName>);
static_assert(
    ::std::is_trivially_move_constructible_v<::dep_crate::ExpectedName>);
static_assert(::std::is_trivially_move_assignable_v<::dep_crate::ExpectedName>);
inline void ExpectedName::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ExpectedName, x));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_use_us(
    rs_std::Result<::dep_crate::ExpectedName, ::std::int32_t>* __ret_ptr);
}
inline rs_std::Result<::dep_crate::ExpectedName, ::std::int32_t> use_s() {
  crubit::Slot<rs_std::Result<::dep_crate::ExpectedName, ::std::int32_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_use_us(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace dep_crate

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020dep_ucrate_x00000020_x0000003a_x0000003a_x00000020ExpectedName_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020dep_ucrate_x00000020_x0000003a_x0000003a_x00000020ExpectedName_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Result<::dep_crate::ExpectedName, ::std::int32_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Result<::dep_crate::ExpectedName, ::std::int32_t>>);
inline rs_std::Result<::dep_crate::ExpectedName, ::std::int32_t>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Result<::dep_crate::ExpectedName, ::std::int32_t>>);
inline constexpr ::std::uint32_t rs_std::Result<
    ::dep_crate::ExpectedName, ::std::int32_t>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void
rs_std::Result<::dep_crate::ExpectedName, ::std::int32_t>::set_tag(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint32_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::dep_crate::ExpectedName, ::std::int32_t>,
           ::dep_crate::ExpectedName, U>)
inline constexpr rs_std::Result<::dep_crate::ExpectedName,
                                ::std::int32_t>::Result(U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::dep_crate::ExpectedName, ::std::int32_t>,
           ::dep_crate::ExpectedName, U>)
inline constexpr rs_std::Result<::dep_crate::ExpectedName, ::std::int32_t>&
rs_std::Result<::dep_crate::ExpectedName, ::std::int32_t>::operator=(
    U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::int32_t, F>)
inline constexpr rs_std::Result<::dep_crate::ExpectedName, ::std::int32_t>::
    Result(rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::int32_t, F>)
inline constexpr rs_std::Result<::dep_crate::ExpectedName, ::std::int32_t>&
rs_std::Result<::dep_crate::ExpectedName, ::std::int32_t>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<::dep_crate::ExpectedName,
                                ::std::int32_t>::Result(::std::in_place_t ip,
                                                        Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<::dep_crate::ExpectedName,
                                ::std::int32_t>::Result(rs_std::unexpect_t u,
                                                        Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_SPECIALIZATION_DEDUPLICATION_DEP_CRATE_GOLDEN
