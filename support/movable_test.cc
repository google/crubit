// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <type_traits>
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wglobal-constructors"

#include <utility>

#include "support/internal/slot.h"
#include "gtest/gtest.h"
#include "support/movable.h"

namespace {

// A normal C++ movable type.
struct CppMovable {
  int value;
  explicit CppMovable(int v) : value(v) {}
  CppMovable(CppMovable&& other) : value(other.value) { other.value = -1; }
  CppMovable& operator=(CppMovable&& other) {
    value = other.value;
    other.value = -1;
    return *this;
  }
  CppMovable(const CppMovable&) = delete;
  CppMovable& operator=(const CppMovable&) = delete;
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

struct DefaultConstructible {
  int value = 42;
};

TEST(MovableTest, DefaultConstructor) {
  static_assert(
      std::is_default_constructible_v<rs::Movable<DefaultConstructible>>);
  static_assert(!std::is_default_constructible_v<rs::Movable<CppMovable>>);

  rs::Movable<DefaultConstructible> v;
  EXPECT_FALSE(v.valueless_after_move());
  EXPECT_TRUE(v);
  EXPECT_EQ(v->value, 42);
}

TEST(MovableTest, ConstructFromValue) {
  rs::Movable<CppMovable> v(CppMovable(42));
  EXPECT_FALSE(v.valueless_after_move());
  EXPECT_EQ(v->value, 42);
}

TEST(MovableTest, ConstructInPlace) {
  rs::Movable<CppMovable> v(std::in_place, 42);
  EXPECT_FALSE(v.valueless_after_move());
  EXPECT_EQ(v->value, 42);
}

TEST(MovableTest, MoveNormalType) {
  rs::Movable<CppMovable> v1(CppMovable(42));
  rs::Movable<CppMovable> v2(std::move(v1));

  EXPECT_FALSE(v2.valueless_after_move());
  EXPECT_EQ(v2->value, 42);
  EXPECT_TRUE(v1.valueless_after_move());  // NOLINT(bugprone-use-after-move)
  EXPECT_FALSE(v1);                        // NOLINT(bugprone-use-after-move)
}

TEST(MovableTest, ConstructFromSlotRustMovable) {
  static_assert(!std::is_constructible_v<rs::Movable<RustMovable>,
                                         crubit::Slot<RustMovable>&&>);

  RustMovable::relocate_calls = 0;
  RustMovable::destructor_calls = 0;
  {
    crubit::Slot<RustMovable> slot;
    new (slot.Get()) RustMovable(42);

    auto v = rs::Movable<RustMovable>::TakeFromSlot(std::move(slot));
    EXPECT_FALSE(v.valueless_after_move());
    EXPECT_EQ(v->value, 42);
    EXPECT_EQ(RustMovable::relocate_calls, 1);
    EXPECT_EQ(RustMovable::destructor_calls,
              0);  // Slot destructor shouldn't destroy it
  }
  // v went out of scope, should destroy RustMovable
  EXPECT_EQ(RustMovable::destructor_calls, 1);
}

TEST(MovableTest, MoveRustMovable) {
  RustMovable::relocate_calls = 0;
  RustMovable::destructor_calls = 0;
  {
    crubit::Slot<RustMovable> slot;
    new (slot.Get()) RustMovable(42);
    auto v1 = rs::Movable<RustMovable>::TakeFromSlot(std::move(slot));
    EXPECT_EQ(RustMovable::relocate_calls, 1);

    rs::Movable<RustMovable> v2(std::move(v1));
    EXPECT_FALSE(v2.valueless_after_move());
    EXPECT_EQ(v2->value, 42);
    EXPECT_TRUE(v1.valueless_after_move());  // NOLINT(bugprone-use-after-move)
    EXPECT_FALSE(v1);                        // NOLINT(bugprone-use-after-move)

    EXPECT_EQ(RustMovable::relocate_calls, 2);
    EXPECT_EQ(RustMovable::destructor_calls,
              0);  // v1 should not destroy its value
  }
  // v2 went out of scope
  EXPECT_EQ(RustMovable::destructor_calls, 1);
}

TEST(MovableTest, MoveAssignmentRustMovable) {
  RustMovable::relocate_calls = 0;
  RustMovable::destructor_calls = 0;
  {
    crubit::Slot<RustMovable> slot1;
    new (slot1.Get()) RustMovable(42);
    auto v1 = rs::Movable<RustMovable>::TakeFromSlot(std::move(slot1));

    crubit::Slot<RustMovable> slot2;
    new (slot2.Get()) RustMovable(100);
    auto v2 = rs::Movable<RustMovable>::TakeFromSlot(std::move(slot2));

    EXPECT_EQ(RustMovable::relocate_calls, 2);
    EXPECT_EQ(RustMovable::destructor_calls, 0);

    v2 = std::move(
        v1);  // should destroy v2's old value, and relocate v1 into v2

    EXPECT_FALSE(v2.valueless_after_move());
    EXPECT_EQ(v2->value, 42);
    EXPECT_TRUE(v1.valueless_after_move());  // NOLINT(bugprone-use-after-move)
    EXPECT_FALSE(v1);                        // NOLINT(bugprone-use-after-move)

    EXPECT_EQ(RustMovable::relocate_calls, 3);
    EXPECT_EQ(RustMovable::destructor_calls,
              1);  // v2's old value (100) destroyed
  }
  // v2 went out of scope (contains 42)
  EXPECT_EQ(RustMovable::destructor_calls, 2);
}

TEST(MovableTest, MoveToSlot) {
  RustMovable::relocate_calls = 0;
  RustMovable::destructor_calls = 0;
  {
    crubit::Slot<RustMovable> slot1;
    new (slot1.Get()) RustMovable(42);
    auto v = rs::Movable<RustMovable>::TakeFromSlot(std::move(slot1));
    EXPECT_EQ(RustMovable::relocate_calls, 1);
    EXPECT_EQ(RustMovable::destructor_calls, 0);

    crubit::Slot<RustMovable> slot2;
    std::move(v).MoveToSlot(slot2);
    EXPECT_TRUE(v.valueless_after_move());  // NOLINT(bugprone-use-after-move)
    EXPECT_EQ(RustMovable::relocate_calls, 2);
    EXPECT_EQ(RustMovable::destructor_calls, 0);

    // Take out of slot2 and destroy
    auto v2 = rs::Movable<RustMovable>::TakeFromSlot(std::move(slot2));
    EXPECT_EQ(RustMovable::relocate_calls, 3);
    EXPECT_EQ(RustMovable::destructor_calls, 0);
  }
  EXPECT_EQ(RustMovable::destructor_calls, 1);
}

struct Copyable {
  int value;
};

TEST(MovableTest, CopySemantics) {
  static_assert(std::is_copy_constructible_v<rs::Movable<Copyable>>);
  static_assert(std::is_copy_assignable_v<rs::Movable<Copyable>>);
  static_assert(!std::is_copy_constructible_v<rs::Movable<CppMovable>>);
  static_assert(!std::is_copy_constructible_v<rs::Movable<RustMovable>>);

  rs::Movable<Copyable> v1(Copyable{42});
  rs::Movable<Copyable> v2(v1);

  EXPECT_FALSE(v1.valueless_after_move());
  EXPECT_FALSE(v2.valueless_after_move());
  EXPECT_EQ(v1->value, 42);
  EXPECT_EQ(v2->value, 42);

  rs::Movable<Copyable> v3(Copyable{100});
  v3 = v1;
  EXPECT_FALSE(v3.valueless_after_move());
  EXPECT_EQ(v3->value, 42);
}

rs::Movable<RustMovable> MakeRustMovable(int value) {
  crubit::Slot<RustMovable> slot;
  new (slot.Get()) RustMovable(value);
  return rs::Movable<RustMovable>::TakeFromSlot(std::move(slot));
}

int ReadRustMovable(const RustMovable& v) { return v.value; }

TEST(MovableTest, ImplicitConversionToValue) {
  // Only rvalues convert, so that an lvalue is never silently emptied.
  static_assert(std::is_convertible_v<rs::Movable<RustMovable>, RustMovable>);
  static_assert(std::is_convertible_v<rs::Movable<CppMovable>, CppMovable>);
  static_assert(!std::is_convertible_v<rs::Movable<RustMovable>&, RustMovable>);
  static_assert(
      !std::is_convertible_v<const rs::Movable<RustMovable>&, RustMovable>);

  RustMovable::relocate_calls = 0;
  RustMovable::destructor_calls = 0;
  {
    // Copy-initialization from a prvalue `Movable<T>` initializes the `T`
    // directly, even though `T` has no C++ move constructor.
    RustMovable v = MakeRustMovable(42);
    EXPECT_EQ(v.value, 42);
    // One relocation into the `Movable`, and one out of it. The emptied
    // `Movable` temporary did not run `T`'s destructor.
    EXPECT_EQ(RustMovable::relocate_calls, 2);
    EXPECT_EQ(RustMovable::destructor_calls, 0);
  }
  EXPECT_EQ(RustMovable::destructor_calls, 1);
}

TEST(MovableTest, ImplicitConversionBindsToConstRef) {
  EXPECT_EQ(ReadRustMovable(MakeRustMovable(42)), 42);
}

TEST(MovableTest, ImplicitConversionWithLifetimeExtension) {
  // `const T& x = f();` binds `x` to the `T` produced by the conversion
  // function, which is a temporary bound directly to the reference and so
  // lives as long as `x`. The `Movable` temporary dies at the end of the
  // full-expression, but it is valueless by then and does not destroy the `T`.
  RustMovable::relocate_calls = 0;
  RustMovable::destructor_calls = 0;
  {
    const RustMovable& x = MakeRustMovable(42);
    EXPECT_EQ(x.value, 42);
    EXPECT_EQ(RustMovable::relocate_calls, 2);
    EXPECT_EQ(RustMovable::destructor_calls, 0);

    RustMovable&& y = MakeRustMovable(7);
    EXPECT_EQ(y.value, 7);
    EXPECT_EQ(RustMovable::destructor_calls, 0);
  }
  EXPECT_EQ(RustMovable::destructor_calls, 2);
}

TEST(MovableTest, ImplicitConversionFromLvalueWithStdMove) {
  RustMovable::relocate_calls = 0;
  RustMovable::destructor_calls = 0;
  {
    rs::Movable<RustMovable> m = MakeRustMovable(42);
    RustMovable v = std::move(m);
    EXPECT_EQ(v.value, 42);
    EXPECT_TRUE(m.valueless_after_move());  // NOLINT(bugprone-use-after-move)
    EXPECT_FALSE(m);                        // NOLINT(bugprone-use-after-move)
    EXPECT_EQ(RustMovable::destructor_calls, 0);
  }
  // Only `v` is destroyed; the valueless `m` is not.
  EXPECT_EQ(RustMovable::destructor_calls, 1);
}

TEST(MovableTest, ImplicitConversionOfCppMovable) {
  rs::Movable<CppMovable> m(std::in_place, 42);
  CppMovable v = std::move(m);
  EXPECT_EQ(v.value, 42);
  EXPECT_TRUE(m.valueless_after_move());  // NOLINT(bugprone-use-after-move)
}

TEST(MovableDeathTest, ImplicitConversionOfValuelessAborts) {
  rs::Movable<CppMovable> m(std::in_place, 42);
  CppMovable v = std::move(m);
  EXPECT_EQ(v.value, 42);
  EXPECT_DEATH(
      {
        CppMovable again = std::move(m);  // NOLINT(bugprone-use-after-move)
        (void)again;
      },
      "valueless");
}

}  // namespace

#pragma clang diagnostic pop
