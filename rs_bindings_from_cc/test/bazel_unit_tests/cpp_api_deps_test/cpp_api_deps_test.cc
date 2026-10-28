// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gtest/gtest.h"
#include "rs_bindings_from_cc/test/bazel_unit_tests/cpp_api_deps_test/cc_consumer.h"

namespace {

TEST(CppApiDepsTest, DirectAndTransitiveRustConsumersExportCppApiDeps) {
  EXPECT_EQ(cpp_api_deps_test::CallDirectRustApi(42), 42);
  EXPECT_EQ(cpp_api_deps_test::CallTransitiveRustApi(42), 43);
}

}  // namespace
