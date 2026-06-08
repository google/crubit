// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/uses_reexport/uses_reexport.h"

#include <type_traits>

#include "gtest/gtest.h"

namespace {

TEST(UsesReexportTest, GlobImports) {
  using namespace uses_reexport;
  EXPECT_EQ(f1(), 42);
  EXPECT_EQ(f2(), 43);
}

TEST(UsesReexportTest, PrivateModuleReexport) {
  using namespace uses_reexport;
  Foo foo = Foo::create();
  Bar bar = Foo::bar();
  (void)foo;
  (void)bar;
}

TEST(UsesReexportTest, NestedReexport) {
  using namespace uses_reexport;
  static_assert(sizeof(InnerX) == 4);
}

TEST(UsesReexportTest, AliasReexport) {
  using namespace uses_reexport;
  static_assert(sizeof(G) == 4);
  static_assert(std::is_same_v<G, test_mod::S>);
}

}  // namespace
