// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// rust_api_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_SUPPORT_FFI_11_TESTS_TYPE_IDENTITY_RUST_API_GOLDEN
#define THIRD_PARTY_CRUBIT_SUPPORT_FFI_11_TESTS_TYPE_IDENTITY_RUST_API_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/internal/slot.h"

#include <cstdint>
#include <utility>

#include "support/ffi_11/ffi_11.h"

namespace rust_api {

decltype(char(0)) c_char();

decltype(char16_t(0)) c_char16_t();

decltype(char32_t(0)) c_char32_t();

decltype(char8_t(0)) c_char8_t();

double c_double();

float c_float();

::std::int32_t c_int();

::std::int64_t c_long();

long long c_longlong();

decltype(nullptr) c_nullptr_t();

::std::int8_t c_schar();

::std::int16_t c_short();

::std::uint8_t c_uchar();

::std::uint32_t c_uint();

::std::uint64_t c_ulong();

unsigned long long c_ulonglong();

::std::uint16_t c_ushort();

// Error generating bindings for function `rust_api_golden::c_wchar_t` defined
// at
// support/ffi_11/tests/type_identity/rust_api.rs;l=52:
// Error formatting function return type `ffi_11::wchar_type::c_wchar_t`: Failed
// to format type for the definition of `ffi_11::wchar_type::c_wchar_t`: Error
// formatting the fully-qualified C++ name of `c_wchar_t`: `wchar_t` is a C++
// reserved word and can't be used as a C++ identifier

namespace __crubit_internal {
extern "C" void __crubit_thunk_c_uchar(decltype(char(0))* __ret_ptr);
}
inline decltype(char(0)) c_char() {
  crubit::Slot<decltype(char(0))> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_c_uchar(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_c_uchar16_ut(decltype(char16_t(0))* __ret_ptr);
}
inline decltype(char16_t(0)) c_char16_t() {
  crubit::Slot<decltype(char16_t(0))> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_c_uchar16_ut(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_c_uchar32_ut(decltype(char32_t(0))* __ret_ptr);
}
inline decltype(char32_t(0)) c_char32_t() {
  crubit::Slot<decltype(char32_t(0))> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_c_uchar32_ut(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_c_uchar8_ut(decltype(char8_t(0))* __ret_ptr);
}
inline decltype(char8_t(0)) c_char8_t() {
  crubit::Slot<decltype(char8_t(0))> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_c_uchar8_ut(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" double __crubit_thunk_c_udouble();
}
inline double c_double() {
  return __crubit_internal::__crubit_thunk_c_udouble();
}

namespace __crubit_internal {
extern "C" float __crubit_thunk_c_ufloat();
}
inline float c_float() { return __crubit_internal::__crubit_thunk_c_ufloat(); }

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_c_uint();
}
inline ::std::int32_t c_int() {
  return __crubit_internal::__crubit_thunk_c_uint();
}

namespace __crubit_internal {
extern "C" ::std::int64_t __crubit_thunk_c_ulong();
}
inline ::std::int64_t c_long() {
  return __crubit_internal::__crubit_thunk_c_ulong();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_c_ulonglong(long long* __ret_ptr);
}
inline long long c_longlong() {
  crubit::Slot<long long> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_c_ulonglong(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_c_unullptr_ut(decltype(nullptr)* __ret_ptr);
}
inline decltype(nullptr) c_nullptr_t() {
  crubit::Slot<decltype(nullptr)> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_c_unullptr_ut(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int8_t __crubit_thunk_c_uschar();
}
inline ::std::int8_t c_schar() {
  return __crubit_internal::__crubit_thunk_c_uschar();
}

namespace __crubit_internal {
extern "C" ::std::int16_t __crubit_thunk_c_ushort();
}
inline ::std::int16_t c_short() {
  return __crubit_internal::__crubit_thunk_c_ushort();
}

namespace __crubit_internal {
extern "C" ::std::uint8_t __crubit_thunk_c_uuchar();
}
inline ::std::uint8_t c_uchar() {
  return __crubit_internal::__crubit_thunk_c_uuchar();
}

namespace __crubit_internal {
extern "C" ::std::uint32_t __crubit_thunk_c_uuint();
}
inline ::std::uint32_t c_uint() {
  return __crubit_internal::__crubit_thunk_c_uuint();
}

namespace __crubit_internal {
extern "C" ::std::uint64_t __crubit_thunk_c_uulong();
}
inline ::std::uint64_t c_ulong() {
  return __crubit_internal::__crubit_thunk_c_uulong();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_c_uulonglong(unsigned long long* __ret_ptr);
}
inline unsigned long long c_ulonglong() {
  crubit::Slot<unsigned long long> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_c_uulonglong(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::uint16_t __crubit_thunk_c_uushort();
}
inline ::std::uint16_t c_ushort() {
  return __crubit_internal::__crubit_thunk_c_uushort();
}

}  // namespace rust_api

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_SUPPORT_FFI_11_TESTS_TYPE_IDENTITY_RUST_API_GOLDEN
