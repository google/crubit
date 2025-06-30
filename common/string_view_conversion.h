// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Utilities for converting between `absl::string_view` and `llvm::StringRef`.
//
// These are needed for platforms where `absl::string_view` is not an alias for
// `std::string_view`.

#ifndef THIRD_PARTY_CRUBIT_COMMON_STRING_VIEW_CONVERSION_H_
#define THIRD_PARTY_CRUBIT_COMMON_STRING_VIEW_CONVERSION_H_

#include "absl/strings/string_view.h"
#include "llvm/ADT/StringRef.h"

// Converts an `absl::string_view` to a `llvm::StringRef`.
inline llvm::StringRef StringRefFromStringView(absl::string_view str) {
  return llvm::StringRef(str.data(), str.size());
}

// Converts a `llvm::StringRef` to an `absl::string_view`.
inline absl::string_view StringViewFromStringRef(llvm::StringRef str) {
  // Avoid passing a null pointer to the `string_view` constructor (which
  // isn't legal if `absl::string_view` is an alias for `std::string_view`).
  if (str.empty()) {
    return absl::string_view();
  }
  return absl::string_view(str.data(), str.size());
}

#endif  // THIRD_PARTY_CRUBIT_COMMON_STRING_VIEW_CONVERSION_H_
