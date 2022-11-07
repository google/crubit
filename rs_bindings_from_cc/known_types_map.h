// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_KNOWN_TYPES_MAP_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_KNOWN_TYPES_MAP_H_

#include <optional>

#include "absl/strings/string_view.h"

namespace crubit {

// Converts primitive types like `std::usize` or `int64_t` into their Rust
// equivalents.
std::optional<absl::string_view> MapKnownCcTypeToRsType(
    absl::string_view cc_type);

}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_KNOWN_TYPES_MAP_H_
