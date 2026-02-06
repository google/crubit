// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate_golden
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector,
// supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_RUST_LIBRARY_CONFIG_EXAMPLE_CRATE_GOLDEN
#define THIRD_PARTY_CRUBIT_EXAMPLES_RUST_LIBRARY_CONFIG_EXAMPLE_CRATE_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#include <cstdint>

namespace example_crate {

// Generated from:
// examples/rust/library_config/example.rs;l=5
std::int32_t add_two_integers(std::int32_t x, std::int32_t y);

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_add_utwo_uintegers(std::int32_t,
                                                          std::int32_t);
}
inline std::int32_t add_two_integers(std::int32_t x, std::int32_t y) {
  return __crubit_internal::__crubit_thunk_add_utwo_uintegers(x, y);
}

}  // namespace example_crate

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_RUST_LIBRARY_CONFIG_EXAMPLE_CRATE_GOLDEN
