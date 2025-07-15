// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// rust_api_golden
// Features: infer_operator_lifetimes, supported, unsafe_types

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_SUPPORT_FFI_11_TESTS_TYPE_IDENTITY_RUST_API_GOLDEN
#define THIRD_PARTY_CRUBIT_SUPPORT_FFI_11_TESTS_TYPE_IDENTITY_RUST_API_GOLDEN

#include "support/internal/slot.h"

#include <cstdint>
#include <utility>

#include "support/ffi_11/ffi_11.h"

namespace rust_api {

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=5
decltype(char(0)) c_char();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=8
std::uint8_t c_uchar();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=11
std::int8_t c_schar();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=15
std::int16_t c_short();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=18
std::uint16_t c_ushort();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=21
std::int32_t c_int();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=24
std::uint32_t c_uint();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=27
std::int64_t c_long();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=30
std::uint64_t c_ulong();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=43
float c_float();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=46
double c_double();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=50
decltype(nullptr) c_nullptr_t();

// Error generating bindings for `c_wchar_t` defined at
// support/ffi_11/tests/type_identity/rust_api.rs;l=54:
// Error formatting function return type `ffi_11::wchar_type::c_wchar_t`: Not a
// public or a supported reexported type (b/262052635).

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=58
decltype(char8_t(0)) c_char8_t();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=62
decltype(char16_t(0)) c_char16_t();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=66
decltype(char32_t(0)) c_char32_t();

namespace __crubit_internal {
extern "C" void __crubit_thunk_c_uchar(decltype(char(0))* __ret_ptr);
}
inline decltype(char(0)) c_char() {
  crubit::Slot<decltype(char(0))> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_c_uchar(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" std::uint8_t __crubit_thunk_c_uuchar();
}
inline std::uint8_t c_uchar() {
  return __crubit_internal::__crubit_thunk_c_uuchar();
}

namespace __crubit_internal {
extern "C" std::int8_t __crubit_thunk_c_uschar();
}
inline std::int8_t c_schar() {
  return __crubit_internal::__crubit_thunk_c_uschar();
}

namespace __crubit_internal {
extern "C" std::int16_t __crubit_thunk_c_ushort();
}
inline std::int16_t c_short() {
  return __crubit_internal::__crubit_thunk_c_ushort();
}

namespace __crubit_internal {
extern "C" std::uint16_t __crubit_thunk_c_uushort();
}
inline std::uint16_t c_ushort() {
  return __crubit_internal::__crubit_thunk_c_uushort();
}

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_c_uint();
}
inline std::int32_t c_int() {
  return __crubit_internal::__crubit_thunk_c_uint();
}

namespace __crubit_internal {
extern "C" std::uint32_t __crubit_thunk_c_uuint();
}
inline std::uint32_t c_uint() {
  return __crubit_internal::__crubit_thunk_c_uuint();
}

namespace __crubit_internal {
extern "C" std::int64_t __crubit_thunk_c_ulong();
}
inline std::int64_t c_long() {
  return __crubit_internal::__crubit_thunk_c_ulong();
}

namespace __crubit_internal {
extern "C" std::uint64_t __crubit_thunk_c_uulong();
}
inline std::uint64_t c_ulong() {
  return __crubit_internal::__crubit_thunk_c_uulong();
}

namespace __crubit_internal {
extern "C" float __crubit_thunk_c_ufloat();
}
inline float c_float() { return __crubit_internal::__crubit_thunk_c_ufloat(); }

namespace __crubit_internal {
extern "C" double __crubit_thunk_c_udouble();
}
inline double c_double() {
  return __crubit_internal::__crubit_thunk_c_udouble();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_c_unullptr_ut(decltype(nullptr)* __ret_ptr);
}
inline decltype(nullptr) c_nullptr_t() {
  crubit::Slot<decltype(nullptr)> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_c_unullptr_ut(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_c_uchar8_ut(decltype(char8_t(0))* __ret_ptr);
}
inline decltype(char8_t(0)) c_char8_t() {
  crubit::Slot<decltype(char8_t(0))> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_c_uchar8_ut(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_c_uchar16_ut(decltype(char16_t(0))* __ret_ptr);
}
inline decltype(char16_t(0)) c_char16_t() {
  crubit::Slot<decltype(char16_t(0))> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_c_uchar16_ut(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_c_uchar32_ut(decltype(char32_t(0))* __ret_ptr);
}
inline decltype(char32_t(0)) c_char32_t() {
  crubit::Slot<decltype(char32_t(0))> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_c_uchar32_ut(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace rust_api
#endif  // THIRD_PARTY_CRUBIT_SUPPORT_FFI_11_TESTS_TYPE_IDENTITY_RUST_API_GOLDEN
