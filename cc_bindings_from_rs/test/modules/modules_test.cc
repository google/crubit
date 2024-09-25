// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/modules/modules.h"

#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(ModulesTest, BasicModule) {
  ASSERT_EQ(123 + 456, modules::basic_module::add_i32(123, 456));
}

TEST(ModulesTest, ImplInSeparatePrivateModule) {
  namespace test = modules::impl_in_separate_private_module;
  test::Foo foo = test::Foo::create(123);
  ASSERT_EQ(123, test::Foo::into_i32(std::move(foo)));
}

}  // namespace
}  // namespace crubit
