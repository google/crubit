// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/lifetimes/lifetimes.h"

#include <cstdint>

#include "gtest/gtest.h"

namespace {

using ::lifetimes::function_with_trivial_unnamed_lifetime_param;
using ::lifetimes::StructWithLifetime;
using ::lifetimes::StructWithLifetimeAndDropGlue;

TEST(LifetimesTest, ReferencesWithNamedLifetimesBecomePointerArguments) {
  const int32_t v = 42;
  StructWithLifetime s = StructWithLifetime::from_ref(&v);
  EXPECT_EQ(s.value(), v);
}

TEST(LifetimesTest, StructWithStaticLifetimeCanBeReturnedFromFunction) {
  StructWithLifetime s = StructWithLifetime::make_static_42();
  EXPECT_EQ(s.value(), 42);
}

TEST(LifetimesTest, ReferencesWithStaticLifetimesBecomePointerArguments) {
  const int32_t v = 42;
  StructWithLifetime s = StructWithLifetime::from_static_ref(&v);
  EXPECT_EQ(s.value(), v);
}

TEST(LifetimesTest,
     ReferencesWithStaticBoundedLifetimesBecomePointerArguments) {
  const int32_t v = 42;
  StructWithLifetime s = StructWithLifetime::from_static_ref_where_bound(&v);
  EXPECT_EQ(s.value(), v);
}

TEST(LifetimesTest, LongReferencesToSelfRemainReferences) {
  StructWithLifetime s = StructWithLifetime::make_static_42();
  s.borrow_from_self();
}

TEST(LifetimesTest, StaticReferencesToSelfRemainReferences) {
  StructWithLifetime s = StructWithLifetime::make_static_42();
  s.borrow_from_static_self();
}

TEST(LifetimesTest, ReferencesWithTrivialUnnamedLifetimesRemainReferences) {
  function_with_trivial_unnamed_lifetime_param(42);
}

TEST(LifetimesTest, StructWithLifetimesAndDropGlueExists) {
  StructWithLifetimeAndDropGlue s =
      StructWithLifetimeAndDropGlue::make_static_42();
}

}  // namespace
