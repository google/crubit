// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/known_traits/partial_eq/partial_eq.h"

#include <cstdint>

#include "gtest/gtest.h"
#include "support/rs_std/str_ref.h"

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

TEST(PartialEqTest, TestBoolAndStrRhs) {
  auto s = partial_eq::bool_and_str_rhs::MyStruct::new_(true, 5, 42);

  EXPECT_TRUE(s == true);     // NOLINT(readability/check)
  EXPECT_FALSE(s == false);   // NOLINT(readability/check)
  EXPECT_TRUE(s == "hello");  // NOLINT(readability/check)
  EXPECT_FALSE(s == "hi");    // NOLINT(readability/check)
  EXPECT_TRUE(s == 42);       // NOLINT(readability/check)
  EXPECT_FALSE(s == 99);      // NOLINT(readability/check)
}

TEST(PartialEqTest, TestStrAndRefStrRhs) {
  auto s = partial_eq::str_and_ref_str_rhs::MyStruct::new_(5);

  EXPECT_TRUE(s == "hello");  // NOLINT(readability/check)
  EXPECT_FALSE(s == "hi");    // NOLINT(readability/check)

  // Verify that gTest EXPECT_EQ/EXPECT_NE macros also compile and work.  The
  // `StrRef` has to be spelled out: `StrRef`'s constructor from a string
  // literal is `consteval`, so it cannot run on gTest's runtime parameter.
  EXPECT_EQ(s, rs_std::StrRef("hello"));
  EXPECT_NE(s, rs_std::StrRef("hi"));
}

}  // namespace
}  // namespace crubit
