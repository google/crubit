// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/lifetimes/lifetimes.h"

#include "gtest/gtest.h"

namespace {

using ::lifetimes::StructWithLifetime;
using ::lifetimes::StructWithLifetimeAndDropGlue;

TEST(LifetimesTest, StructWithStaticLifetimeCanBeReturnedFromFunction) {
  StructWithLifetime s = StructWithLifetime::make_static_42();
  EXPECT_EQ(s.value(), 42);
}

TEST(LifetimesTest, TemporariesDontBindToStaticLifetimes) {
  // TODO: b/396735681 - This should fail to compile. Instead, such input
  // references should be converted to pointers.
  (void)StructWithLifetime::from_static_ref_where_bound(42);
}

TEST(LifetimesTest, StructWithLifetimesAndDropGlueExists) {
  StructWithLifetimeAndDropGlue s =
      StructWithLifetimeAndDropGlue::make_static_42();
}

}  // namespace
