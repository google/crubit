// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/modules/modules_cc_api.h"

namespace crubit {
namespace {

TEST(ModulesTest, BasicModule) {
  ASSERT_EQ(123 + 456, modules::basic_module::add_i32(123, 456));
}

}  // namespace
}  // namespace crubit
