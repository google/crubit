// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// rust_api_golden
// Features: infer_operator_lifetimes, non_unpin_ctor, std_unique_ptr,
// std_vector, supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_SUPPORT_FFI_11_TESTS_TYPE_IDENTITY_RUST_API_GOLDEN
#define THIRD_PARTY_CRUBIT_SUPPORT_FFI_11_TESTS_TYPE_IDENTITY_RUST_API_GOLDEN

#include "support/ffi_11/ffi_11.h"

namespace rust_api {

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=5
decltype(char(0)) c_char();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=8
unsigned char c_uchar();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=11
signed char c_schar();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=15
decltype(short(0)) c_short();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=18
unsigned short c_ushort();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=21
decltype(int(0)) c_int();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=24
unsigned int c_uint();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=27
decltype(long(0)) c_long();

// Generated from:
// support/ffi_11/tests/type_identity/rust_api.rs;l=30
unsigned long c_ulong();

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
// Error formatting function return type `ffi_11::wchar_type::c_wchar_t`: Failed
// to format type for the definition of `ffi_11::wchar_type::c_wchar_t`: Error
// formatting the fully-qualified C++ name of `c_wchar_t`: `wchar_t` is a C++
// reserved keyword and can't be used as a C++ identifier

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
extern "C" decltype(char(0)) __crubit_thunk_c_uchar();
}
inline decltype(char(0)) c_char() {
  return __crubit_internal::__crubit_thunk_c_uchar();
}

namespace __crubit_internal {
extern "C" unsigned char __crubit_thunk_c_uuchar();
}
inline unsigned char c_uchar() {
  return __crubit_internal::__crubit_thunk_c_uuchar();
}

namespace __crubit_internal {
extern "C" signed char __crubit_thunk_c_uschar();
}
inline signed char c_schar() {
  return __crubit_internal::__crubit_thunk_c_uschar();
}

namespace __crubit_internal {
extern "C" decltype(short(0)) __crubit_thunk_c_ushort();
}
inline decltype(short(0)) c_short() {
  return __crubit_internal::__crubit_thunk_c_ushort();
}

namespace __crubit_internal {
extern "C" unsigned short __crubit_thunk_c_uushort();
}
inline unsigned short c_ushort() {
  return __crubit_internal::__crubit_thunk_c_uushort();
}

namespace __crubit_internal {
extern "C" decltype(int(0)) __crubit_thunk_c_uint();
}
inline decltype(int(0)) c_int() {
  return __crubit_internal::__crubit_thunk_c_uint();
}

namespace __crubit_internal {
extern "C" unsigned int __crubit_thunk_c_uuint();
}
inline unsigned int c_uint() {
  return __crubit_internal::__crubit_thunk_c_uuint();
}

namespace __crubit_internal {
extern "C" decltype(long(0)) __crubit_thunk_c_ulong();
}
inline decltype(long(0)) c_long() {
  return __crubit_internal::__crubit_thunk_c_ulong();
}

namespace __crubit_internal {
extern "C" unsigned long __crubit_thunk_c_uulong();
}
inline unsigned long c_ulong() {
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
extern "C" decltype(nullptr) __crubit_thunk_c_unullptr_ut();
}
inline decltype(nullptr) c_nullptr_t() {
  return __crubit_internal::__crubit_thunk_c_unullptr_ut();
}

namespace __crubit_internal {
extern "C" decltype(char8_t(0)) __crubit_thunk_c_uchar8_ut();
}
inline decltype(char8_t(0)) c_char8_t() {
  return __crubit_internal::__crubit_thunk_c_uchar8_ut();
}

namespace __crubit_internal {
extern "C" decltype(char16_t(0)) __crubit_thunk_c_uchar16_ut();
}
inline decltype(char16_t(0)) c_char16_t() {
  return __crubit_internal::__crubit_thunk_c_uchar16_ut();
}

namespace __crubit_internal {
extern "C" decltype(char32_t(0)) __crubit_thunk_c_uchar32_ut();
}
inline decltype(char32_t(0)) c_char32_t() {
  return __crubit_internal::__crubit_thunk_c_uchar32_ut();
}

}  // namespace rust_api
#endif  // THIRD_PARTY_CRUBIT_SUPPORT_FFI_11_TESTS_TYPE_IDENTITY_RUST_API_GOLDEN
