// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/type_aliases/type_aliases.h"

#include <cstdint>
#include <type_traits>

#include "gtest/gtest.h"

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

TEST(TypeAliasesTest, GenericsMatching) {
  using namespace type_aliases::test_generics_matching;

  auto r1 = returns_matching_alias();
  EXPECT_TRUE(r1.has_value());
  EXPECT_EQ(r1.value(), 0);

  auto r2 = returns_flipped_alias();
  EXPECT_TRUE(r2.has_value());
  EXPECT_EQ(r2.value(), 0u);

  auto r3 = returns_specialized();
  EXPECT_TRUE(r3.has_value());
  EXPECT_EQ(r3.value(), 0);
}

}  // namespace
