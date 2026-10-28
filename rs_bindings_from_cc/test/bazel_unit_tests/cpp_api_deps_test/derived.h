// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_UNIT_TESTS_CPP_API_DEPS_TEST_DERIVED_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_UNIT_TESTS_CPP_API_DEPS_TEST_DERIVED_H_

#include <cstdint>

#include "rs_bindings_from_cc/test/bazel_unit_tests/cpp_api_deps_test/base.h"

namespace cpp_api_deps_test {

struct DerivedWrapper {
  int32_t value;
  BaseValue ToBase() const { return BaseValue{value}; }
};

}  // namespace cpp_api_deps_test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_UNIT_TESTS_CPP_API_DEPS_TEST_DERIVED_H_
