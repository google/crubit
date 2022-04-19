// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "lifetime_annotations/lifetime_substitutions.h"

#include "testing/base/public/gmock.h"
#include "testing/base/public/gunit.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

TEST(LifetimeSubstitutions, EmptySubstitution) {
  LifetimeSubstitutions r;

  Lifetime l1 = Lifetime::CreateVariable();

  EXPECT_EQ(r.Substitute(l1), l1);
}

TEST(LifetimeSubstitutions, SingleSubstitution) {
  LifetimeSubstitutions s;

  Lifetime l1 = Lifetime::CreateVariable();
  Lifetime l2 = Lifetime::CreateVariable();

  s.Add(l1, l2);

  EXPECT_EQ(s.Substitute(l1), l2);
  EXPECT_EQ(s.Substitute(l2), l2);
}

TEST(LifetimeSubstitutions, Constants) {
  LifetimeSubstitutions s;
  EXPECT_EQ(s.Substitute(Lifetime::Static()), Lifetime::Static());
  Lifetime local = Lifetime::CreateLocal();
  EXPECT_EQ(s.Substitute(local), local);
}

TEST(LifetimeSubstitutions, SelfSubstitution) {
  LifetimeSubstitutions s;

  Lifetime l1 = Lifetime::CreateVariable();

  s.Add(l1, l1);

  EXPECT_EQ(s.Substitute(l1), l1);
}

TEST(LifetimeSubstitutions, Cycle) {
  LifetimeSubstitutions s;

  Lifetime l1 = Lifetime::CreateVariable();
  Lifetime l2 = Lifetime::CreateVariable();

  s.Add(l1, l2);
  s.Add(l2, l1);

  EXPECT_EQ(s.Substitute(l1), l2);
  EXPECT_EQ(s.Substitute(l2), l2);
}

TEST(LifetimeSubstitutions, LargerCycle) {
  LifetimeSubstitutions s;

  Lifetime l1 = Lifetime::CreateVariable();
  Lifetime l2 = Lifetime::CreateVariable();
  Lifetime l3 = Lifetime::CreateVariable();

  s.Add(l1, l2);
  s.Add(l2, l3);
  s.Add(l3, l1);

  EXPECT_EQ(s.Substitute(l1), l3);
  EXPECT_EQ(s.Substitute(l2), l3);
  EXPECT_EQ(s.Substitute(l3), l3);
}

TEST(LifetimeSubstitutions, SubstituteIsTransitive) {
  // Check that Substitute() performs transitive substitutions by adding
  // substitutions in an order that will prevent Add() from preemptively
  // performing all of those transitive substitutions.

  Lifetime l1 = Lifetime::CreateVariable();
  Lifetime l2 = Lifetime::CreateVariable();
  Lifetime l3 = Lifetime::CreateVariable();
  Lifetime l4 = Lifetime::CreateVariable();

  LifetimeSubstitutions s;
  s.Add(l1, l2);
  s.Add(l3, l4);
  s.Add(l2, l3);

  EXPECT_EQ(s.Substitute(l1), l4);
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
