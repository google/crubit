// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate
// Features: <none>

// clang-format off
#pragma once

#include <cstdint>

namespace example_crate {

// Generated from:
// examples/rust/function/example.rs;l=5
std::int32_t add_two_integers(std::int32_t x, std::int32_t y);

namespace __crubit_internal {
extern "C" std::int32_t
__crubit_thunk__uRNvCsh2mUQSogkZl_u13example_ucrate16add_utwo_uintegers(
    std::int32_t, std::int32_t);
}
inline std::int32_t add_two_integers(std::int32_t x, std::int32_t y) {
  return __crubit_internal::
      __crubit_thunk__uRNvCsh2mUQSogkZl_u13example_ucrate16add_utwo_uintegers(
          x, y);
}

}  // namespace example_crate
