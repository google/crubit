// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <concepts>
#include <ranges>

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/known_traits/into_iterator/into_iterator_rust.h"

namespace {

using ::into_iterator_rust::ContainerWithInherentBegin;
using ::into_iterator_rust::ContainerWithRefIntoIter;
using ::into_iterator_rust::make_container;
using ::into_iterator_rust::make_inherent_container;
using ::into_iterator_rust::MyContainer;

static_assert(std::ranges::range<MyContainer>);
static_assert(std::ranges::range<const MyContainer&>);
static_assert(std::ranges::range<MyContainer&>);

static_assert(std::ranges::input_range<MyContainer>);
static_assert(std::ranges::input_range<const MyContainer&>);
static_assert(std::ranges::input_range<MyContainer&>);

static_assert(!std::ranges::range<ContainerWithInherentBegin>);
static_assert(!std::ranges::range<ContainerWithRefIntoIter>);

TEST(IntoIteratorTest, ValueContainer) {
  MyContainer c = make_container(10, 20, 30);
  int count = 0;
  int sum = 0;
  for (int x : std::move(c)) {
    count++;
    sum += x;
  }
  EXPECT_EQ(count, 3);
  EXPECT_EQ(sum, 60);
}

TEST(IntoIteratorTest, SharedRefContainer) {
  MyContainer c = make_container(10, 20, 30);
  const MyContainer& const_c = c;
  int count = 0;
  int sum = 0;
  for (const int& x : const_c) {
    count++;
    sum += x;
  }
  EXPECT_EQ(count, 3);
  EXPECT_EQ(sum, 60);
}

TEST(IntoIteratorTest, MutRefContainer) {
  MyContainer c = make_container(10, 20, 30);
  for (int& x : c) {
    x *= 2;
  }

  int sum = 0;
  for (int x : std::move(c)) {
    sum += x;
  }
  EXPECT_EQ(sum, 120);
}

TEST(IntoIteratorTest, InherentBeginConflict) {
  ContainerWithInherentBegin c = make_inherent_container();
  EXPECT_EQ(c.begin(), 42);
}

}  // namespace
