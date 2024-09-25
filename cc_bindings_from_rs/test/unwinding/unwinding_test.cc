// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/unwinding/panic_function.h"

namespace {

using ::testing::AllOf;
using ::testing::HasSubstr;

TEST(PanicTest, PanicRust) {
  EXPECT_DEATH(panic_function::panic_rust(),
               AllOf(HasSubstr("this is a panic"),
                     HasSubstr("panic in a function that cannot unwind")));
}

TEST(PanicTest, PanicC) {
  EXPECT_DEATH(panic_function::panic_c(),
               AllOf(HasSubstr("this is a panic"),
                     HasSubstr("panic in a function that cannot unwind")));
}

}  // namespace
