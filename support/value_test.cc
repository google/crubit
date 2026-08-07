// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/value.h"

#include <utility>

#include "gtest/gtest.h"
#include "support/internal/slot.h"

namespace rs {
namespace {

// A normal C++ movable type.
struct Movable {
  int value;
  explicit Movable(int v) : value(v) {}
  Movable(Movable&& other) : value(other.value) { other.value = -1; }
  Movable& operator=(Movable&& other) {
    value = other.value;
    other.value = -1;
    return *this;
  }
  Movable(const Movable&) = delete;
  Movable& operator=(const Movable&) = delete;
};

// A type that is NOT C++ movable, but has UnsafeRelocateTag constructor.
struct RustMovable {
  int value;
  bool relocated_from = false;
  static inline int relocate_calls = 0;
  static inline int destructor_calls = 0;

  explicit RustMovable(int v) : value(v) {}
  ~RustMovable() {
    destructor_calls++;
    // If it was relocated from, we shouldn't be destroying it (except if it was
    // a temporary, but in our Slot/Value tests we should avoid destroying
    // relocated-from objects).
    if (relocated_from) {
      // In a real app, this might crash or be a no-op depending on how
      // "defused" it is. For test, we can check if we shouldn't have been
      // destroyed. But we can't easily assert here without failing the test if
      // C++ forces destructor. However, Slot and Value should prevent
      // destructor from running on relocated-from.
    }
  }

  // No C++ move/copy
  RustMovable(RustMovable&&) = delete;
  RustMovable& operator=(RustMovable&&) = delete;
  RustMovable(const RustMovable&) = delete;
  RustMovable& operator=(const RustMovable&) = delete;

  // Rust move constructor
  RustMovable(crubit::UnsafeRelocateTag, RustMovable&& other)
      : value(other.value) {
    other.relocated_from = true;
    relocate_calls++;
  }
};

TEST(RelocatableValueTest, DefaultConstructor) {
  RelocatableValue<Movable> v;
  EXPECT_FALSE(v.has_value());
  EXPECT_FALSE(v);
}

TEST(RelocatableValueTest, ConstructFromValue) {
  RelocatableValue<Movable> v(Movable(42));
  EXPECT_TRUE(v.has_value());
  EXPECT_EQ(v->value, 42);
}

TEST(RelocatableValueTest, ConstructInPlace) {
  RelocatableValue<Movable> v(std::in_place, 42);
  EXPECT_TRUE(v.has_value());
  EXPECT_EQ(v->value, 42);
}

TEST(RelocatableValueTest, MoveNormalType) {
  RelocatableValue<Movable> v1(Movable(42));
  RelocatableValue<Movable> v2(std::move(v1));

  EXPECT_TRUE(v2.has_value());
  EXPECT_EQ(v2->value, 42);
  EXPECT_FALSE(v1.has_value());  // NOLINT(bugprone-use-after-move)
}

TEST(RelocatableValueTest, ConstructFromSlotRustMovable) {
  RustMovable::relocate_calls = 0;
  RustMovable::destructor_calls = 0;
  {
    crubit::Slot<RustMovable> slot;
    new (slot.Get()) RustMovable(42);

    RelocatableValue<RustMovable> v(std::move(slot));
    EXPECT_TRUE(v.has_value());
    EXPECT_EQ(v->value, 42);
    EXPECT_EQ(RustMovable::relocate_calls, 1);
    EXPECT_EQ(RustMovable::destructor_calls,
              0);  // Slot destructor shouldn't destroy it
  }
  // v went out of scope, should destroy RustMovable
  EXPECT_EQ(RustMovable::destructor_calls, 1);
}

TEST(RelocatableValueTest, MoveRustMovable) {
  RustMovable::relocate_calls = 0;
  RustMovable::destructor_calls = 0;
  {
    crubit::Slot<RustMovable> slot;
    new (slot.Get()) RustMovable(42);
    RelocatableValue<RustMovable> v1(std::move(slot));
    EXPECT_EQ(RustMovable::relocate_calls, 1);

    RelocatableValue<RustMovable> v2(std::move(v1));
    EXPECT_TRUE(v2.has_value());
    EXPECT_EQ(v2->value, 42);
    EXPECT_FALSE(v1.has_value());  // NOLINT(bugprone-use-after-move)

    EXPECT_EQ(RustMovable::relocate_calls, 2);
    EXPECT_EQ(RustMovable::destructor_calls,
              0);  // v1 should not destroy its value
  }
  // v2 went out of scope
  EXPECT_EQ(RustMovable::destructor_calls, 1);
}

TEST(RelocatableValueTest, MoveAssignmentRustMovable) {
  RustMovable::relocate_calls = 0;
  RustMovable::destructor_calls = 0;
  {
    crubit::Slot<RustMovable> slot1;
    new (slot1.Get()) RustMovable(42);
    RelocatableValue<RustMovable> v1(std::move(slot1));

    crubit::Slot<RustMovable> slot2;
    new (slot2.Get()) RustMovable(100);
    RelocatableValue<RustMovable> v2(std::move(slot2));

    EXPECT_EQ(RustMovable::relocate_calls, 2);
    EXPECT_EQ(RustMovable::destructor_calls, 0);

    v2 = std::move(
        v1);  // should destroy v2's old value, and relocate v1 into v2

    EXPECT_TRUE(v2.has_value());
    EXPECT_EQ(v2->value, 42);
    EXPECT_FALSE(v1.has_value());  // NOLINT(bugprone-use-after-move)

    EXPECT_EQ(RustMovable::relocate_calls, 3);
    EXPECT_EQ(RustMovable::destructor_calls,
              1);  // v2's old value (100) destroyed
  }
  // v2 went out of scope (contains 42)
  EXPECT_EQ(RustMovable::destructor_calls, 2);
}

}  // namespace
}  // namespace rs
