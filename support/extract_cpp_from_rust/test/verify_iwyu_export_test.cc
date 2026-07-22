// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gtest/gtest.h"
#include "support/extract_cpp_from_rust/test/library_with_embedded_cpp.h"

namespace {

TEST(IwyuExportTest, CanUseEmbeddedCppDirectly) {
  // If library_with_embedded_cpp_cc_api.h correctly IWYU exports the embedded
  // C++ header, we should be able to call this inline C++ function seamlessly.
  EXPECT_EQ(add_two_ints(10, 20), 30);
  EXPECT_EQ(my_test_namespace::TestClass::StaticMethod(), 3);
}

}  // namespace
