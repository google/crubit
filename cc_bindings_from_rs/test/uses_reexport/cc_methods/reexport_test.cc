// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/uses_reexport/cc_methods/reexport.h"

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/uses_reexport/cc_methods/cc_type.h"

namespace {

TEST(ReexportCcMethodsTest, ReexportedTypeIsUsable) {
  cc_ns::CcType value = reexport::make_value(42);
  EXPECT_EQ(value.GetValue(), 42);
  reexport::CcType alias = value;
  EXPECT_EQ(alias.GetValue(), 42);
}

TEST(ReexportCcMethodsTest, MethodFreeFunctionIsCallableFromRust) {
  cc_ns::CcType value = cc_ns::CcType::Make(7);
  EXPECT_EQ(reexport::get_value_via_free_fn(value), 7);
}

}  // namespace
