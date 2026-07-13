// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/move_semantics/move.h"

#include <type_traits>
#include <utility>

#include "gtest/gtest.h"
#include "support/internal/slot.h"
#include "support/value.h"

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

static_assert(!std::is_move_constructible_v<move::UnmovableFoo>);
static_assert(
    std::is_move_constructible_v<rs::RelocatableValue<move::UnmovableFoo>>);
static_assert(!std::is_constructible_v<rs::RelocatableValue<move::UnmovableFoo>,
                                       move::UnmovableFoo>);

TEST(MoveTest, MoveUnmovableFooViaValue) {
  rs::RelocatableValue<move::UnmovableFoo> v2;
  {
    crubit::Slot<move::UnmovableFoo> slot;
    move::initialize_unmovable_foo(slot.Get(), 42);

    rs::RelocatableValue<move::UnmovableFoo> v1(std::move(slot));
    EXPECT_TRUE(v1.has_value());
    EXPECT_EQ(v1->read_byte(), 42);

    v2 = std::move(v1);
    EXPECT_TRUE(v2.has_value());
    EXPECT_FALSE(v1.has_value());
    EXPECT_EQ(v2->read_byte(), 42);
  }
  EXPECT_EQ(v2->read_byte(), 42);
}

TEST(MoveTest, WrapReturnValueInValue) {
  rs::RelocatableValue<move::UnmovableFoo> v;
  {
    crubit::Slot<move::UnmovableFoo> slot;
    new (slot.Get()) move::UnmovableFoo(move::new_unmovable_foo(42));
    v = rs::RelocatableValue<move::UnmovableFoo>(std::move(slot));
  }
  EXPECT_TRUE(v.has_value());
  EXPECT_EQ(v->read_byte(), 42);
}

}  // namespace
