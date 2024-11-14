// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/bridging/rust_type.h"

namespace crubit {
namespace {

TEST(TypeBridging, StructToStructTest) {
  crubit::test::TheCppType cpp_type = rust_type::create_new(1);

  EXPECT_EQ(rust_type::get_x(cpp_type), 1);
}

}  // namespace
}  // namespace crubit