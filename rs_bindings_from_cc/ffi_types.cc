// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ffi_types.h"

#include "third_party/absl/strings/string_view.h"

namespace rs_bindings_from_cc {

FfiU8Slice MakeFfiU8Slice(absl::string_view s) {
  FfiU8Slice result;
  result.ptr = s.data();
  result.size = s.size();
  return result;
}

}  // namespace rs_bindings_from_cc
