// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_KNOWN_TYPES_MAP_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_KNOWN_TYPES_MAP_H_

#include <optional>

#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Type.h"

namespace crubit {

// Converts C++ types to an already-existing Rust type, instead of generating
// bindings for the C++ type.
//
// The return value is a fully-qualified Rust name, including builtin type
// names.
//
// For example, C++ `int64_t` becomes Rust `i64`.
//
// To create a new type mapping, add the type to the hardcoded list
// of types.
std::optional<MappedType> GetTypeMapOverride(const clang::Type& cpp_type);

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_KNOWN_TYPES_MAP_H_
