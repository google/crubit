// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// rs_ops_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_OPS_RS_OPS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_OPS_RS_OPS_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/rs_std/traits.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_core.h"

namespace rs_ops {

struct CRUBIT_INTERNAL_RUST_TYPE(":: rs_ops_golden :: MyInt") alignas(4)
    [[clang::trivial_abi]] MyInt final {
 public:
  // `rs_ops_golden::MyInt` doesn't implement the `Default` trait
  MyInt() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyInt() = default;
  MyInt(MyInt&&) = default;
  MyInt& operator=(MyInt&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  MyInt(const MyInt&) = default;
  MyInt& operator=(const MyInt&) = default;
  MyInt(::crubit::UnsafeRelocateTag, MyInt&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  static ::rs_ops::MyInt new_(::std::int32_t value);

  bool operator==(::rs_ops::MyInt const& other) const;

  ::rs_ops::MyInt operator+(::rs_ops::MyInt rhs) const;

  void operator+=(::rs_ops::MyInt rhs);

  ::rs_ops::MyInt operator&(::rs_ops::MyInt rhs) const;

  void operator&=(::rs_ops::MyInt rhs);

  ::rs_ops::MyInt operator|(::rs_ops::MyInt rhs) const;

  void operator|=(::rs_ops::MyInt rhs);

  ::rs_ops::MyInt operator^(::rs_ops::MyInt rhs) const;

  void operator^=(::rs_ops::MyInt rhs);

  ::rs_ops::MyInt operator/(::rs_ops::MyInt rhs) const;

  void operator/=(::rs_ops::MyInt rhs);

  ::rs_ops::MyInt operator*(::rs_ops::MyInt rhs) const;

  void operator*=(::rs_ops::MyInt rhs);

  ::rs_ops::MyInt operator-() const;

  ::rs_ops::MyInt operator!() const;

  ::rs_ops::MyInt operator%(::rs_ops::MyInt rhs) const;

  void operator%=(::rs_ops::MyInt rhs);

  ::rs_ops::MyInt operator<<(::std::int32_t rhs) const;

  void operator<<=(::std::int32_t rhs);

  ::rs_ops::MyInt operator>>(::std::int32_t rhs) const;

  void operator>>=(::std::int32_t rhs);

  ::rs_ops::MyInt operator-(::rs_ops::MyInt rhs) const;

  void operator-=(::rs_ops::MyInt rhs);

  union {
    ::std::int32_t value;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace rs_ops

template <>
struct rs_std::impl<::rs_ops::MyInt, ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct rs_std::impl<::rs_ops::MyInt, ::rs::core::fmt::Debug> {
  static constexpr bool kIsImplemented = true;

  // Error generating bindings for associated function `<rs_ops_golden::MyInt as
  // std::fmt::Debug>::fmt` defined at
  // cc_bindings_from_rs/test/known_traits/ops/rs_ops.rs;l=13:
  // Error formatting function return type `std::result::Result<(),
  // std::fmt::Error>`: Generic types are not supported yet (b/259749095)
};

namespace rs_ops {

static_assert(
    sizeof(MyInt) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyInt) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<MyInt>);
static_assert(::std::is_trivially_move_constructible_v<::rs_ops::MyInt>);
static_assert(::std::is_trivially_move_assignable_v<::rs_ops::MyInt>);
static_assert(::std::is_trivially_copy_constructible_v<::rs_ops::MyInt>);
static_assert(::std::is_trivially_copy_assignable_v<::rs_ops::MyInt>);
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_new(::std::int32_t, ::rs_ops::MyInt* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::rs_ops::MyInt MyInt::new_(::std::int32_t value) {
  crubit::Slot<::rs_ops::MyInt> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" bool
__crubit_thunk_PartialEq_ueq_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    ::rs_ops::MyInt const&, ::rs_ops::MyInt const&);
/// \endcond
}  // namespace __crubit_internal
inline bool MyInt::operator==(::rs_ops::MyInt const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
          self, other);
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_Add_uadd_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    ::rs_ops::MyInt*, ::rs_ops::MyInt*, ::rs_ops::MyInt* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::rs_ops::MyInt MyInt::operator+(::rs_ops::MyInt rhs) const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  crubit::Slot<::rs_ops::MyInt> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Add_uadd_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
          &self, &rhs, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_AddAssign_uadd_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    ::rs_ops::MyInt&, ::rs_ops::MyInt*);
/// \endcond
}  // namespace __crubit_internal
inline void MyInt::operator+=(::rs_ops::MyInt rhs) {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_AddAssign_uadd_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
          self, &rhs);
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_BitAnd_ubitand_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    ::rs_ops::MyInt*, ::rs_ops::MyInt*, ::rs_ops::MyInt* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::rs_ops::MyInt MyInt::operator&(::rs_ops::MyInt rhs) const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  crubit::Slot<::rs_ops::MyInt> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_BitAnd_ubitand_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
          &self, &rhs, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_BitAndAssign_ubitand_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    ::rs_ops::MyInt&, ::rs_ops::MyInt*);
/// \endcond
}  // namespace __crubit_internal
inline void MyInt::operator&=(::rs_ops::MyInt rhs) {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_BitAndAssign_ubitand_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
          self, &rhs);
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_BitOr_ubitor_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    ::rs_ops::MyInt*, ::rs_ops::MyInt*, ::rs_ops::MyInt* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::rs_ops::MyInt MyInt::operator|(::rs_ops::MyInt rhs) const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  crubit::Slot<::rs_ops::MyInt> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_BitOr_ubitor_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
          &self, &rhs, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_BitOrAssign_ubitor_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    ::rs_ops::MyInt&, ::rs_ops::MyInt*);
/// \endcond
}  // namespace __crubit_internal
inline void MyInt::operator|=(::rs_ops::MyInt rhs) {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_BitOrAssign_ubitor_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
          self, &rhs);
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_BitXor_ubitxor_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    ::rs_ops::MyInt*, ::rs_ops::MyInt*, ::rs_ops::MyInt* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::rs_ops::MyInt MyInt::operator^(::rs_ops::MyInt rhs) const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  crubit::Slot<::rs_ops::MyInt> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_BitXor_ubitxor_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
          &self, &rhs, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_BitXorAssign_ubitxor_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    ::rs_ops::MyInt&, ::rs_ops::MyInt*);
/// \endcond
}  // namespace __crubit_internal
inline void MyInt::operator^=(::rs_ops::MyInt rhs) {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_BitXorAssign_ubitxor_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
          self, &rhs);
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_Div_udiv_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    ::rs_ops::MyInt*, ::rs_ops::MyInt*, ::rs_ops::MyInt* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::rs_ops::MyInt MyInt::operator/(::rs_ops::MyInt rhs) const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  crubit::Slot<::rs_ops::MyInt> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Div_udiv_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
          &self, &rhs, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_DivAssign_udiv_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    ::rs_ops::MyInt&, ::rs_ops::MyInt*);
/// \endcond
}  // namespace __crubit_internal
inline void MyInt::operator/=(::rs_ops::MyInt rhs) {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_DivAssign_udiv_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
          self, &rhs);
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_Mul_umul_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    ::rs_ops::MyInt*, ::rs_ops::MyInt*, ::rs_ops::MyInt* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::rs_ops::MyInt MyInt::operator*(::rs_ops::MyInt rhs) const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  crubit::Slot<::rs_ops::MyInt> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Mul_umul_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
          &self, &rhs, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_MulAssign_umul_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    ::rs_ops::MyInt&, ::rs_ops::MyInt*);
/// \endcond
}  // namespace __crubit_internal
inline void MyInt::operator*=(::rs_ops::MyInt rhs) {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_MulAssign_umul_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
          self, &rhs);
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_Neg_uneg_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    ::rs_ops::MyInt*, ::rs_ops::MyInt* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::rs_ops::MyInt MyInt::operator-() const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  crubit::Slot<::rs_ops::MyInt> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Neg_uneg_urs_uops_ugolden_x0000003a_x0000003aMyInt(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_Not_unot_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    ::rs_ops::MyInt*, ::rs_ops::MyInt* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::rs_ops::MyInt MyInt::operator!() const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  crubit::Slot<::rs_ops::MyInt> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Not_unot_urs_uops_ugolden_x0000003a_x0000003aMyInt(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_Rem_urem_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    ::rs_ops::MyInt*, ::rs_ops::MyInt*, ::rs_ops::MyInt* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::rs_ops::MyInt MyInt::operator%(::rs_ops::MyInt rhs) const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  crubit::Slot<::rs_ops::MyInt> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Rem_urem_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
          &self, &rhs, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_RemAssign_urem_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    ::rs_ops::MyInt&, ::rs_ops::MyInt*);
/// \endcond
}  // namespace __crubit_internal
inline void MyInt::operator%=(::rs_ops::MyInt rhs) {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_RemAssign_urem_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
          self, &rhs);
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_Shl_ushl_urs_uops_ugolden_x0000003a_x0000003aMyInt_ui32(
    ::rs_ops::MyInt*, ::std::int32_t, ::rs_ops::MyInt* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::rs_ops::MyInt MyInt::operator<<(::std::int32_t rhs) const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  crubit::Slot<::rs_ops::MyInt> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Shl_ushl_urs_uops_ugolden_x0000003a_x0000003aMyInt_ui32(
          &self, rhs, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_ShlAssign_ushl_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_ui32(
    ::rs_ops::MyInt&, ::std::int32_t);
/// \endcond
}  // namespace __crubit_internal
inline void MyInt::operator<<=(::std::int32_t rhs) {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_ShlAssign_ushl_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_ui32(
          self, rhs);
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_Shr_ushr_urs_uops_ugolden_x0000003a_x0000003aMyInt_ui32(
    ::rs_ops::MyInt*, ::std::int32_t, ::rs_ops::MyInt* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::rs_ops::MyInt MyInt::operator>>(::std::int32_t rhs) const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  crubit::Slot<::rs_ops::MyInt> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Shr_ushr_urs_uops_ugolden_x0000003a_x0000003aMyInt_ui32(
          &self, rhs, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_ShrAssign_ushr_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_ui32(
    ::rs_ops::MyInt&, ::std::int32_t);
/// \endcond
}  // namespace __crubit_internal
inline void MyInt::operator>>=(::std::int32_t rhs) {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_ShrAssign_ushr_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_ui32(
          self, rhs);
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_Sub_usub_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    ::rs_ops::MyInt*, ::rs_ops::MyInt*, ::rs_ops::MyInt* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::rs_ops::MyInt MyInt::operator-(::rs_ops::MyInt rhs) const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  crubit::Slot<::rs_ops::MyInt> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Sub_usub_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
          &self, &rhs, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_SubAssign_usub_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    ::rs_ops::MyInt&, ::rs_ops::MyInt*);
/// \endcond
}  // namespace __crubit_internal
inline void MyInt::operator-=(::rs_ops::MyInt rhs) {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_SubAssign_usub_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
          self, &rhs);
}
inline void MyInt::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyInt, value));
}
}  // namespace rs_ops

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_OPS_RS_OPS_GOLDEN
