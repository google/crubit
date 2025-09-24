// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate_golden
// Features: supported, unsafe_types

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_RUST_CPP_ENUM_EXAMPLE_CRATE_GOLDEN
#define THIRD_PARTY_CRUBIT_EXAMPLES_RUST_CPP_ENUM_EXAMPLE_CRATE_GOLDEN

#include "support/annotations_internal.h"

#include <cstdint>

namespace example_crate {

// CRUBIT_ANNOTATE: cpp_enum=enum class
//
// Generated from:
// examples/rust/cpp_enum/example.rs;l=13
enum class CRUBIT_INTERNAL_RUST_TYPE(
    ":: example_crate_golden :: Color") Color : std::int32_t {
  Red = INT32_C(0),
  Blue = INT32_C(1),
  Green = INT32_C(5),
  Gray = INT32_C(5),
  Magenta = INT32_C(7),
};

}  // namespace example_crate
#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_RUST_CPP_ENUM_EXAMPLE_CRATE_GOLDEN
