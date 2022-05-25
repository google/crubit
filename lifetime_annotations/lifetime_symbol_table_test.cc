// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/lifetime_symbol_table.h"

#include "gtest/gtest.h"

namespace clang {
namespace tidy {
namespace lifetimes {
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

TEST(LifetimeSymbolTableTest, RebindLifetime) {
  LifetimeSymbolTable table;

  Lifetime a = table.LookupNameAndMaybeDeclare("a");
  Lifetime b = Lifetime::CreateVariable();

  EXPECT_EQ(table.LookupLifetime(a), "a");

  table.Rebind("a", b);
  EXPECT_EQ(table.LookupName("a"), b);
  EXPECT_EQ(table.LookupLifetime(a), std::nullopt);
  EXPECT_EQ(table.LookupLifetime(b), "a");
}

TEST(LifetimeSymbolTableTest, LookupLifetimeAndMaybeDeclare) {
  {
    LifetimeSymbolTable table;

    table.LookupNameAndMaybeDeclare("a");
    table.LookupNameAndMaybeDeclare("c");

    EXPECT_EQ(table.LookupLifetimeAndMaybeDeclare(Lifetime::CreateVariable()),
              "b");
    EXPECT_EQ(table.LookupLifetimeAndMaybeDeclare(Lifetime::CreateVariable()),
              "d");
  }

  {
    LifetimeSymbolTable table;
    for (int i = 0; i < 26; ++i) {
      table.LookupLifetimeAndMaybeDeclare(Lifetime::CreateVariable());
    }
    EXPECT_EQ(table.LookupLifetimeAndMaybeDeclare(Lifetime::CreateVariable()),
              "aa");
    EXPECT_EQ(table.LookupLifetimeAndMaybeDeclare(Lifetime::CreateVariable()),
              "ab");
  }
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
