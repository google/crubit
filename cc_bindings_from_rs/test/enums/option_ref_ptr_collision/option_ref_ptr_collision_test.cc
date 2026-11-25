// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/enums/option_ref_ptr_collision/option_ref_ptr_collision.h"

#include "gtest/gtest.h"

namespace {

// Currently fails to compile: the generated header defines
// `rs_std::Option<Value const*>` twice (once for `Option<&Value>`, once for
// `Option<*const Value>`).
TEST(OptionRefPtrCollisionTest, BothSpecializationsUsable) {
  option_ref_ptr_collision::Value v;
  v.x = 42;
  auto by_ref = option_ref_ptr_collision::by_ref(v);
  auto by_ptr = option_ref_ptr_collision::by_ptr(v);
  EXPECT_TRUE(by_ref.has_value());
  EXPECT_TRUE(by_ptr.has_value());
}

}  // namespace
