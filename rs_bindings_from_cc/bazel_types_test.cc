// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/bazel_types.h"

#include <string>

#include "gtest/gtest.h"
#include "absl/strings/string_view.h"

namespace crubit {

TEST(BazelTypesTest,
     ConvertBazelLabelToCcIdentifier_AlphanumericNotTransformed) {
  constexpr absl::string_view kTestInputs[] = {
      "abc",
      "foo123",
  };
  for (absl::string_view kTestInput : kTestInputs) {
    EXPECT_EQ(kTestInput,
              ConvertToCcIdentifier(BazelLabel(std::string(kTestInput))));
  }
}

TEST(BazelTypesTest, ConvertBazelLabelToCcIdentifier_SimpleTargets) {
  EXPECT_EQ(
      "_2f_2ffoo_2fbar_3abaz_5fabc",
      ConvertToCcIdentifier(BazelLabel(std::string("//foo/bar:baz_abc"))));
}

TEST(BazelTypesTest, ConvertBazelLabelToCcIdentifier_Conflict) {
  EXPECT_NE(ConvertToCcIdentifier(BazelLabel(std::string("//foo_bar:baz"))),
            ConvertToCcIdentifier(BazelLabel(std::string("//foo/bar:baz"))));
}

}  // namespace crubit
