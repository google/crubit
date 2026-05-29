// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/known_traits/partial_eq/partial_eq.h"

#include <cstdint>

#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(PartialEqTest, TestBasic) {
  auto one1 = partial_eq::basic_test::MyStruct::new_(1);
  auto one2 = partial_eq::basic_test::MyStruct::new_(1);
  auto two = partial_eq::basic_test::MyStruct::new_(2);

  // Direct testing of `operator==` is preferable, but this requires
  // suppressing the lint that wants us to use `EXPECT_EQ` instead.
  EXPECT_TRUE(one1 == one2);  // NOLINT(readability/check)
  EXPECT_FALSE(one1 == two);  // NOLINT(readability/check)

  // Verify that gTest EXPECT_EQ/EXPECT_NE macros also compile and work
  EXPECT_EQ(one1, one2);
  EXPECT_NE(one1, two);
}

TEST(PartialEqTest, TestUsizeRhs) {
  auto one = partial_eq::usize_rhs::MyStruct::new_(1);

  // Verify direct comparison with std::uintptr_t
  EXPECT_EQ(one, std::uintptr_t{1});
  EXPECT_NE(one, std::uintptr_t{2});

  // Verify that implicit casts work from a C++ integer literal
  EXPECT_EQ(one, 1);
  EXPECT_NE(one, 2);
}

}  // namespace
}  // namespace crubit
