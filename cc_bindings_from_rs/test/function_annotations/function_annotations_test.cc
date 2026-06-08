// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/function_annotations/function_annotations.h"

#include "gtest/gtest.h"

namespace {

TEST(FunctionAnnotationsTest, CallAdd) {
  EXPECT_EQ(function_annotations::add_two_integers(2, 3), 5);
}

}  // namespace
