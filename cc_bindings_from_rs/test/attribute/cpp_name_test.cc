// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/attribute/cpp_name.h"

#include <type_traits>
#include <utility>

#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(CppNameTest, RenameStruct) {
  cpp_name::Replaced replaced = cpp_name::Replaced::create();
  EXPECT_EQ(replaced.x, 42);
}

}  // namespace
}  // namespace crubit