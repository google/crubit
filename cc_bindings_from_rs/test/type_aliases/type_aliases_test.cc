// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/type_aliases/type_aliases.h"

#include <cstdint>
#include <type_traits>

#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(TypeAliasesTest, SimpleTypeAlias) {
  EXPECT_TRUE((std::is_same_v<std::int32_t,
                              type_aliases::test_type_aliases::TypeAlias>));
}

TEST(TypeAliasesTest, MultipleLayerTypeAlias) {
  EXPECT_TRUE((std::is_same_v<std::int32_t,
                              type_aliases::test_type_aliases::TypeAlias2>));
}

TEST(TypeAliasesTest, TypeAliasUsage) {
  EXPECT_TRUE((std::is_same_v<
               std::int32_t,
               decltype(type_aliases::test_type_aliases::func_using_alias())>));
}

// Note: this test verifies that the generated code compiles and gives the
// correct type, but doesn't check that the C++ type is actually deprecated.
TEST(TypeAliasesTest, DeprecatedTypeAlias) {
  EXPECT_TRUE(
      (std::is_same_v<std::int32_t,
                      type_aliases::test_deprecated_type_alias::TypeAlias>));
}

}  // namespace
}  // namespace crubit
