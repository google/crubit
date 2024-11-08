// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate
// Features: <none>

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_RUST_LIBRARY_CONFIG_EXAMPLE_CRATE
#define THIRD_PARTY_CRUBIT_EXAMPLES_RUST_LIBRARY_CONFIG_EXAMPLE_CRATE

#include <cstdint>

namespace my::library {

// Generated from:
// examples/rust/library_config/example.rs;l=5
std::int32_t add_two_integers(std::int32_t x, std::int32_t y);

namespace __crubit_internal {
extern "C" std::int32_t
__crubit_thunk__uRNvCseF4PRLPR6bH_u13example_ucrate16add_utwo_uintegers(
    std::int32_t, std::int32_t);
}
inline std::int32_t add_two_integers(std::int32_t x, std::int32_t y) {
  return __crubit_internal::
      __crubit_thunk__uRNvCseF4PRLPR6bH_u13example_ucrate16add_utwo_uintegers(
          x, y);
}

}  // namespace my::library
#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_RUST_LIBRARY_CONFIG_EXAMPLE_CRATE
