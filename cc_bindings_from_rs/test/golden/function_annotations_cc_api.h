// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// function_annotations_rust_golden
// Features: custom_ffi_types, experimental, non_unpin_ctor, std_unique_ptr,
// std_vector, supported, wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_FUNCTION_ANNOTATIONS_RUST_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_FUNCTION_ANNOTATIONS_RUST_GOLDEN

#ifdef KYTHE_IS_RUNNING
#pragma kythe_inline_metadata "This file contains Kythe metadata."
#endif
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#include <cstdint>

namespace function_annotations_rust {

// Generated from:
// cc_bindings_from_rs/test/golden/function_annotations.rs;l=5
std::int32_t add_two_integers(std::int32_t x, std::int32_t y);

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_add_utwo_uintegers(std::int32_t,
                                                          std::int32_t);
}
inline std::int32_t add_two_integers(std::int32_t x, std::int32_t y) {
  return __crubit_internal::__crubit_thunk_add_utwo_uintegers(x, y);
}

}  // namespace function_annotations_rust
#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_FUNCTION_ANNOTATIONS_RUST_GOLDEN

// This file contains Kythe metadata. eyJ0eXBlIjoia3l0aGUwIiwibWV0YSI6W3sidHlwZSI6ImFuY2hvcl9hbmNob3IiLCJzb3VyY2VfYmVnaW4iOjI0NzYyNiwic291cmNlX2VuZCI6MjQ3NjQyLCJ0YXJnZXRfYmVnaW4iOjEwNDAsInRhcmdldF9lbmQiOjEwNTYsImVkZ2UiOiIva3l0aGUvZWRnZS9pbXB1dGVzIiwic291cmNlX3ZuYW1lIjp7ImNvcnB1cyI6ImNvcnB1cyIsInBhdGgiOiJ0aGlyZF9wYXJ0eS9jcnViaXQvY2NfYmluZGluZ3NfZnJvbV9ycy90ZXN0L2dvbGRlbi9mdW5jdGlvbl9hbm5vdGF0aW9ucy5ycyIsImxhbmd1YWdlIjoicnVzdCJ9fV19
