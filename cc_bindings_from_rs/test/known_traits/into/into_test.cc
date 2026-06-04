// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/known_traits/into/into.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(IntoTest, ConvertConversionOperators) {
  into::Convert convert(1563);
  EXPECT_EQ(static_cast<int32_t>(convert), 1563);
  EXPECT_EQ(static_cast<int64_t>(convert), 1563);
  EXPECT_EQ(static_cast<rs_std::StrRef>(convert), "Convert");
  EXPECT_EQ(static_cast<int16_t>(convert), 1563);
}

TEST(IntoTest, ConvertRefConversionOperators) {
  into::ConvertRef convert_ref =
      into::ConvertRef::create(rs_std::StrRef("Hello, World!"));
  EXPECT_EQ(static_cast<rs_std::StrRef>(convert_ref), "Hello, World!");
  EXPECT_EQ(static_cast<into::Convert>(convert_ref).__field0, 42);
}

}  // namespace
}  // namespace crubit
