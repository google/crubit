// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/never/never.h"

#include "gtest/gtest.h"

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

TEST(NeverTest, NeverReturnDoesNotReturn) {
  CRUBIT_EXPECT_DEATH(never::never_return(), "You can't do that!");
}

TEST(NeverTest, ExternNeverReturnDoesNotReturn) {
  CRUBIT_EXPECT_DEATH(never::extern_never_return(),
                      "You can't do that directly!");
}

TEST(NeverTest, AssocatedFnNeverReturnDoesNotReturn) {
  CRUBIT_EXPECT_DEATH(never::NeverStruct::associated_fn_never_return(),
                      "You can't do that as an associated fn!");
}

TEST(NeverTest, MethodNeverReturnDoesNotReturn) {
  never::NeverStruct never_struct;
  CRUBIT_EXPECT_DEATH(never_struct.method_never_return(),
                      "You can't do that as a method!");
}

TEST(NeverTest, OptionNever) {
  auto opt = never::option_never();
  EXPECT_FALSE(opt.has_value());
}

TEST(NeverTest, ResultUnitNever) {
  auto res = never::result_unit_never();
  EXPECT_TRUE(res.has_value());
}

TEST(NeverTest, ResultNeverUnit) {
  auto res = never::result_never_unit();
  EXPECT_FALSE(res.has_value());
}

TEST(NeverTest, ResultI32Never) {
  auto res = never::result_i32_never();
  EXPECT_TRUE(res.has_value());
  EXPECT_EQ(res.value(), 42);
}

TEST(NeverTest, ResultNeverI32) {
  auto res = never::result_never_i32();
  EXPECT_FALSE(res.has_value());
  EXPECT_EQ(res.error(), 42);
}

TEST(NeverTest, ResultStringNever) {
  auto res = never::result_string_never();
  EXPECT_TRUE(res.has_value());
  auto value = std::move(res).value();
  (void)value;
}

TEST(NeverTest, OptionNeverHasValue) {
  rs::Option<rs::never_t> opt = std::nullopt;
  EXPECT_FALSE(opt.has_value());
}

}  // namespace
