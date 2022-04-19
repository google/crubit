// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/lifetime.h"

#include "testing/base/public/gmock.h"
#include "testing/base/public/gunit.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

TEST(Lifetime, IsVariable) {
  EXPECT_TRUE(Lifetime::CreateVariable().IsVariable());
  EXPECT_FALSE(Lifetime::Static().IsVariable());
  EXPECT_FALSE(Lifetime::CreateLocal().IsVariable());
}

TEST(Lifetime, IsConstant) {
  EXPECT_TRUE(Lifetime::Static().IsConstant());
  EXPECT_TRUE(Lifetime::CreateLocal().IsConstant());
  EXPECT_FALSE(Lifetime::CreateVariable().IsConstant());
}

TEST(Lifetime, IsLocal) {
  EXPECT_TRUE(Lifetime::CreateLocal().IsLocal());
  EXPECT_FALSE(Lifetime::Static().IsLocal());
}

TEST(Lifetime, Equality) {
  Lifetime l1 = Lifetime::CreateVariable();
  Lifetime l2 = Lifetime::CreateVariable();

  EXPECT_EQ(l1, l1);
  EXPECT_EQ(l2, l2);
  EXPECT_NE(l1, l2);
  EXPECT_NE(l2, l1);

  EXPECT_EQ(Lifetime::Static(), Lifetime::Static());
  EXPECT_NE(l1, Lifetime::Static());

  Lifetime local1 = Lifetime::CreateLocal();
  Lifetime local2 = Lifetime::CreateLocal();
  EXPECT_NE(local1, local2);
  EXPECT_NE(l1, local1);
}

TEST(Lifetime, Copy) {
  Lifetime l1 = Lifetime::CreateVariable();
  Lifetime l2 = l1;

  EXPECT_EQ(l1, l2);

  Lifetime l3 = Lifetime::CreateVariable();
  EXPECT_NE(l1, l3);
  l3 = l1;
  EXPECT_EQ(l1, l3);
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
