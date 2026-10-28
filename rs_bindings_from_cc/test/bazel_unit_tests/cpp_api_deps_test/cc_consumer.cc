// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/bazel_unit_tests/cpp_api_deps_test/cc_consumer.h"

#include <cstdint>

#include "rs_bindings_from_cc/test/bazel_unit_tests/cpp_api_deps_test/reexport_consumer_rust_api.h"
#include "rs_bindings_from_cc/test/bazel_unit_tests/cpp_api_deps_test/rust_api.h"

namespace cpp_api_deps_test {

int32_t CallDirectRustApi(int32_t x) { return rust_api::read_derived({x}); }

int32_t CallTransitiveRustApi(int32_t x) {
  return reexport_consumer_rust_api::read_derived_transitively({x});
}

}  // namespace cpp_api_deps_test
