// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_ABSL_HASH_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_ABSL_HASH_H_

#include <cstdint>

#include "absl/hash/hash.h"

namespace crubit {

// Returns the `absl::Hash` of `value`, for use in the Rust `Hash` impls that
// Crubit generates for C++ types that provide `AbslHashValue`.
//
// The result is only stable within one process (`absl::Hash` uses a per-process
// seed).
template <typename T>
uint64_t AbslHash(const T& value) {
  return absl::HashOf(value);
}

}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_ABSL_HASH_H_
