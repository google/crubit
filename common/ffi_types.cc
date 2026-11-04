// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/ffi_types.h"

#include <string_view>

namespace crubit {

FfiU8Slice MakeFfiU8Slice(std::string_view s) {
  FfiU8Slice result;
  result.ptr = s.data();
  result.size = s.size();
  return result;
}

std::string_view StringViewFromFfiU8Slice(FfiU8Slice ffi_u8_slice) {
  return std::string_view(ffi_u8_slice.ptr, ffi_u8_slice.size);
}

}  // namespace crubit
