// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/lifetime_symbol_table.h"

#include "testing/base/public/gunit.h"

namespace devtools_rust {
namespace {

TEST(LifetimeSymbolTableTest, Static) {
  LifetimeSymbolTable table;
  EXPECT_EQ(table.LookupName("static"), Lifetime::Static());
  EXPECT_EQ(table.LookupNameAndMaybeDeclare("static"), Lifetime::Static());
}

TEST(LifetimeSymbolTableTest, LookupName) {
  LifetimeSymbolTable table;

  EXPECT_EQ(table.LookupName("a"), std::nullopt);
  Lifetime a = table.LookupNameAndMaybeDeclare("a");
  EXPECT_EQ(table.LookupName("a"), a);
  EXPECT_EQ(table.LookupNameAndMaybeDeclare("a"), a);

  EXPECT_EQ(table.LookupName("b"), std::nullopt);
  Lifetime b = table.LookupNameAndMaybeDeclare("b");
  EXPECT_NE(a, b);
  EXPECT_EQ(table.LookupName("b"), b);
  EXPECT_EQ(table.LookupNameAndMaybeDeclare("b"), b);
}

TEST(LifetimeSymbolTableTest, LookupLifetime) {
  LifetimeSymbolTable table;

  Lifetime a = table.LookupNameAndMaybeDeclare("a");
  Lifetime b = table.LookupNameAndMaybeDeclare("b");

  EXPECT_EQ(table.LookupLifetime(a), "a");
  EXPECT_EQ(table.LookupLifetime(b), "b");
}

}  // namespace
}  // namespace devtools_rust
