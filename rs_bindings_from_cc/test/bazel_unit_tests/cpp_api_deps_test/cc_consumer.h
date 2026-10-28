// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_UNIT_TESTS_CPP_API_DEPS_TEST_CC_CONSUMER_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_UNIT_TESTS_CPP_API_DEPS_TEST_CC_CONSUMER_H_

#include <cstdint>

namespace cpp_api_deps_test {

int32_t CallDirectRustApi(int32_t x);
int32_t CallTransitiveRustApi(int32_t x);

}  // namespace cpp_api_deps_test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_UNIT_TESTS_CPP_API_DEPS_TEST_CC_CONSUMER_H_
