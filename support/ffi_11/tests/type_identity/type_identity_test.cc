// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <type_traits>

#include "support/ffi_11/tests/type_identity/rust_api.h"

// NOLINTBEGIN(runtime/int) - this is the point!
namespace {

static_assert(std::is_same_v<decltype(rust_api::c_char()), char>);
static_assert(std::is_same_v<decltype(rust_api::c_uchar()), unsigned char>);
static_assert(std::is_same_v<decltype(rust_api::c_schar()), signed char>);

static_assert(std::is_same_v<decltype(rust_api::c_short()), short>);
static_assert(std::is_same_v<decltype(rust_api::c_ushort()), unsigned short>);
static_assert(std::is_same_v<decltype(rust_api::c_int()), int>);
static_assert(std::is_same_v<decltype(rust_api::c_uint()), unsigned int>);
static_assert(std::is_same_v<decltype(rust_api::c_long()), long>);
static_assert(std::is_same_v<decltype(rust_api::c_ulong()), unsigned long>);
static_assert(std::is_same_v<decltype(rust_api::c_longlong()), long long>);
static_assert(
    std::is_same_v<decltype(rust_api::c_ulonglong()), unsigned long long>);

static_assert(std::is_same_v<decltype(rust_api::c_float()), float>);
static_assert(std::is_same_v<decltype(rust_api::c_double()), double>);

static_assert(std::is_same_v<decltype(rust_api::c_nullptr_t()), nullptr_t>);

// TODO(b/b/262052635): uncomment this once the re-export works.
// static_assert(std::is_same_v<decltype(rust_api::c_wchar_t()), wchar_t>);
static_assert(std::is_same_v<decltype(rust_api::c_char8_t()), char8_t>);
static_assert(std::is_same_v<decltype(rust_api::c_char16_t()), char16_t>);
static_assert(std::is_same_v<decltype(rust_api::c_char32_t()), char32_t>);
}  // namespace

int main() {}
// NOLINTEND(runtime/int)
