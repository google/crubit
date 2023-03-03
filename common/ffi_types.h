// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_COMMON_FFI_TYPES_H_
#define CRUBIT_COMMON_FFI_TYPES_H_

#include <cstddef>

#include "absl/strings/string_view.h"

namespace crubit {

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

// Returns an `FfiU8Slice` referencing the same data as `s`.
FfiU8Slice MakeFfiU8Slice(absl::string_view s);

// Returns a `string_view` referencing the same data as `ffi_u8_slice`.
absl::string_view StringViewFromFfiU8Slice(FfiU8Slice ffi_u8_slice);

// Returns an `FfiU8SliceBox` containing a copy of the data in `ffi_u8_slice`.
// The returned `FfiU8SliceBox` must be freed by calling `FreeFfiU8SliceBox()`.
// Implemented in Rust.
extern "C" FfiU8SliceBox AllocFfiU8SliceBox(FfiU8Slice ffi_u8_slice);

// Frees the memory associated with an `FfiU8SliceBox`.
// Implemented in Rust.
extern "C" void FreeFfiU8SliceBox(FfiU8SliceBox);

// Whether or not the generated binding will have doc comments indicating their
// source location.
enum SourceLocationDocComment {
  Disabled,
  Enabled,
};

}  // namespace crubit

#endif  // CRUBIT_COMMON_FFI_TYPES_H_
