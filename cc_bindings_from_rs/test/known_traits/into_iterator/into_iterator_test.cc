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
using ::into_iterator_rust::make_iterator;
using ::into_iterator_rust::make_move_only_iterator;
using ::into_iterator_rust::MoveOnlyIterator;
using ::into_iterator_rust::MoveOnlyPayload;
using ::into_iterator_rust::MyContainer;
using ::into_iterator_rust::MyIterator;

static_assert(std::ranges::range<MyContainer>);
static_assert(std::ranges::range<const MyContainer&>);
static_assert(std::ranges::range<MyContainer&>);

static_assert(std::ranges::range<MyIterator>);
static_assert(!std::ranges::range<const MyIterator&>);
static_assert(std::ranges::range<MyIterator&>);

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

TEST(IntoIteratorTest, DirectIteratorForLoop) {
  MyIterator iter = make_iterator(42);
  int count = 0;
  for (int x : iter) {
    EXPECT_EQ(x, 42);
    if (++count >= 3) break;
  }
  EXPECT_EQ(count, 3);
}

TEST(IntoIteratorTest, MoveOnlyIteratorForLoop) {
  MoveOnlyIterator iter = make_move_only_iterator(21, 3);
  int count = 0;
  for (MoveOnlyPayload&& payload : iter) {
    EXPECT_EQ(payload.mutating_method(), 42);
    count++;
  }
  EXPECT_EQ(count, 3);
}

}  // namespace
