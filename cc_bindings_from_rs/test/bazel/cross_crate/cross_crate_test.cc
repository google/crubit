// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/bazel/cross_crate/test_api.h"

namespace crubit {
namespace {

TEST(CrossCrateTests, BasicEndToEndTest) {
  other_crate::SomeStruct s = test_api::create_struct(123);
  int i = test_api::extract_int(std::move(s));
  EXPECT_EQ(123, i);
}

// b/292231336
// TEST(CrossCrateTests, RustToolchainCrate) {
//   ::alloc::string::String s =
//   test_api::return_a_type_from_a_rust_toolchain_crate();
// }

}  // namespace
}  // namespace crubit
