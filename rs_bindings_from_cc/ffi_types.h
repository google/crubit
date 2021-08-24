// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_FFI_TYPES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_FFI_TYPES_H_

#include <cstddef>

#include "third_party/absl/strings/string_view.h"

namespace rs_bindings_from_cc {

// Owned, Rust-allocated byte array. Call `FreeFfiU8SliceBox` to
// deallocate.
struct FfiU8SliceBox {
  const char* ptr;
  size_t size;
};

// Borrowed byte array.
struct FfiU8Slice {
  const char* ptr;
  size_t size;
};

FfiU8Slice MakeFfiU8Slice(absl::string_view s);

// This function is implemented in Rust.
extern "C" void FreeFfiU8SliceBox(FfiU8SliceBox);

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_FFI_TYPES_H_
