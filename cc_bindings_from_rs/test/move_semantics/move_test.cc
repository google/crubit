// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/move_semantics/move.h"

#include <utility>

#include "gtest/gtest.h"

namespace {

// This tests the current behavior of movable C++ structs generated from Rust
// sources. At the moment Crubit replaces the "moved-from" value with the Rust
// `Default::default()` implementation of the type. Only types implementing
// Default will be movable in C++.

TEST(MoveTest, MoveFullStruct) {
  auto Foo = move::Foo::from_byte(42);
  EXPECT_EQ(Foo.read_byte(), 42);
  move::consume_foo(std::move(Foo));
  EXPECT_EQ(Foo.read_byte(), 0);
}

TEST(MoveTest, MoveViaMember) {
  auto Foo = move::Foo::from_byte(42);
  EXPECT_EQ(Foo.read_byte(), 42);
  EXPECT_EQ(std::move(Foo).into_byte(), 42);
  EXPECT_EQ(Foo.read_byte(), 0);
}

TEST(MoveTest, MoveAssign) {
  auto Foo = move::Foo::from_byte(42);
  EXPECT_EQ(Foo.read_byte(), 42);
  move::Foo Foo2;
  Foo2 = std::move(Foo);
  EXPECT_EQ(Foo.read_byte(), 0);
}

TEST(MoveTest, MoveConstruction) {
  auto Foo = move::Foo::from_byte(42);
  EXPECT_EQ(Foo.read_byte(), 42);
  move::Foo Foo2 = std::move(Foo);
  EXPECT_EQ(Foo.read_byte(), 0);
}

TEST(MoveTest, CopyableBySelfMethodsDontRequireRvalue) {
  move::Copyable copyable = move::Copyable::from_byte(42);
  // Ensure that the `consume_self` method is not rvalue-qualified, that it
  // does not replace `copyable` with a default-constructed value, and  that
  // modifications to `self` are not reflected in the original object.
  EXPECT_EQ(copyable.consume_self(), 42);
  EXPECT_EQ(copyable.consume_self(), 42);
}

}  // namespace
