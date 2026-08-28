// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/unwinding/panic_function.h"

// Wrapper around EXPECT_DEATH_IF_SUPPORTED accommodating Android (where panics
// route to logcat) and iOS (where fork is prohibited in the sandbox).
#ifdef __ANDROID__
#define CRUBIT_EXPECT_DEATH(statement, regex) \
  EXPECT_DEATH_IF_SUPPORTED(statement, "")
#else
#define CRUBIT_EXPECT_DEATH(statement, regex) \
  EXPECT_DEATH_IF_SUPPORTED(statement, regex)
#endif

namespace {

using ::testing::AllOf;
using ::testing::HasSubstr;

TEST(PanicTest, PanicRust) {
  CRUBIT_EXPECT_DEATH(
      panic_function::panic_rust(),
      AllOf(HasSubstr("this is a panic"),
            HasSubstr("panic in a function that cannot unwind")));
}

TEST(PanicTest, PanicC) {
  CRUBIT_EXPECT_DEATH(
      panic_function::panic_c(),
      AllOf(HasSubstr("this is a panic"),
            HasSubstr("panic in a function that cannot unwind")));
}

}  // namespace
