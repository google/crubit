// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gtest/gtest.h"
#include "support/extract_cpp_from_rust/test/test_extracted.h"

namespace {

TEST(GlobalCppExtractionTest, ExtractedFunctionIsCallable) {
  EXPECT_EQ(MyTestFunction(10), 15);
}

TEST(GlobalCppExtractionTest, HandlesNamespacesAndNestedBraces) {
  EXPECT_EQ(my_test_namespace::TestClass::StaticMethod(), 3);
}

TEST(GlobalCppExtractionTest, HandlesStringsWithBraces) {
  EXPECT_STREQ(my_test_namespace::GetStringWithBrace(),
               "String with { and } in it!");
}

TEST(GlobalCppExtractionTest, HandlesCharWithBraces) {
  EXPECT_EQ(my_test_namespace::GetCharWithBrace(), '{');
}

}  // namespace
