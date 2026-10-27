// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Ensure `::std` is root-qualified inside `namespace rs` when `rs::std` exists.
namespace rs::std {}

#include "support/rs/box.h"

#include <memory>
#include <type_traits>
#include <utility>

#include "support/internal/slot.h"
#include "support/movable.h"
#include "support/unsafe_relocate_tag.h"
#include "gtest/gtest.h"
#include "support/rs/pin_box.h"

namespace rs {
namespace {

struct Tracked {
  ::std::shared_ptr<int> tracker;
  Tracked() = default;
  explicit Tracked(::std::shared_ptr<int> t) : tracker(::std::move(t)) {}
};

struct MultiArg {
  int a;
  int b;
  MultiArg(int a, int b) : a(a), b(b) {}
};

struct Immovable {
  ::std::shared_ptr<int> tracker;
  explicit Immovable(::std::shared_ptr<int> t) : tracker(::std::move(t)) {}
  Immovable(const Immovable&) = delete;
  Immovable& operator=(const Immovable&) = delete;
  Immovable(Immovable&&) = delete;
  Immovable& operator=(Immovable&&) = delete;
};

// Mirrors a non-C++-movable Rust type bound by `cc_bindings_from_rs`: deleted
// C++ move/copy constructors, relocatable via `crubit::UnsafeRelocateTag`.
struct NonCppMovableRustType {
  int value;
  bool relocated_from = false;
  static inline int relocate_calls = 0;
  static inline int live_count = 0;

  explicit NonCppMovableRustType(int v) : value(v) { ++live_count; }
  ~NonCppMovableRustType() {
    EXPECT_FALSE(relocated_from);
    --live_count;
  }

  NonCppMovableRustType(const NonCppMovableRustType&) = delete;
  NonCppMovableRustType& operator=(const NonCppMovableRustType&) = delete;
  NonCppMovableRustType(NonCppMovableRustType&&) = delete;
  NonCppMovableRustType& operator=(NonCppMovableRustType&&) = delete;

  NonCppMovableRustType(::crubit::UnsafeRelocateTag,
                        NonCppMovableRustType&& other) noexcept
      : value(other.value) {
    other.relocated_from = true;
    ++relocate_calls;
    ++live_count;
  }
};

// Simulates a Rust function returning a non-C++-movable type under
// `movable_return_values`.
::rs::Movable<NonCppMovableRustType> ReturnNonCppMovableRustType(int v) {
  crubit::Slot<NonCppMovableRustType> slot;
  new (slot.Get()) NonCppMovableRustType(v);
  return ::rs::Movable<NonCppMovableRustType>::TakeFromSlot(::std::move(slot));
}

TEST(BoxTest, StaticAsserts) {
  static_assert(sizeof(::rs::Box<int>) == sizeof(int*));
  static_assert(sizeof(::rs::Movable<::rs::Box<int>>) == sizeof(int*));
  static_assert(!::std::is_copy_constructible_v<Box<int>>);
  static_assert(!::std::is_move_constructible_v<Box<int>>);
  static_assert(!::std::is_copy_assignable_v<Box<int>>);
  static_assert(!::std::is_move_assignable_v<Box<int>>);
  static_assert(::std::is_constructible_v<Box<int>, ::crubit::UnsafeRelocateTag,
                                          Box<int>&&>);
  static_assert(
      ::std::is_constructible_v<Box<NonCppMovableRustType>,
                                ::rs::Movable<NonCppMovableRustType>>);
  static_assert(!::std::is_default_constructible_v<::rs::Movable<Box<int>>>);
  static_assert(
      !::std::is_constructible_v<::rs::Movable<Box<int>>, ::std::nullptr_t>);
  static_assert(!::std::is_copy_constructible_v<::rs::Movable<Box<int>>>);
  static_assert(!::std::is_copy_assignable_v<::rs::Movable<Box<int>>>);
  static_assert(
      ::std::is_nothrow_move_constructible_v<::rs::Movable<Box<int>>>);
  static_assert(::std::is_nothrow_move_assignable_v<::rs::Movable<Box<int>>>);
  static_assert(::std::is_convertible_v<::rs::Movable<Box<int>>, Box<int>>);
  static_assert(!::std::is_convertible_v<::rs::Movable<Box<int>>&, Box<int>>);
  static_assert(
      !::std::is_convertible_v<const ::rs::Movable<Box<int>>&, Box<int>>);
  static_assert(![]<typename M = ::rs::Movable<Box<int>>>() {
    return requires(M m) { ::std::move(m).as_box(); };
  }());
}

TEST(PinBoxTest, StaticAsserts) {
  static_assert(sizeof(::rs::PinBox<int>) == sizeof(int*));
  static_assert(sizeof(::rs::Movable<::rs::PinBox<int>>) == sizeof(int*));
  static_assert(!::std::is_copy_constructible_v<PinBox<int>>);
  static_assert(!::std::is_move_constructible_v<PinBox<int>>);
  static_assert(!::std::is_copy_assignable_v<PinBox<int>>);
  static_assert(!::std::is_move_assignable_v<PinBox<int>>);
  static_assert(
      ::std::is_constructible_v<PinBox<int>, ::crubit::UnsafeRelocateTag,
                                PinBox<int>&&>);
  static_assert(
      ::std::is_constructible_v<PinBox<NonCppMovableRustType>,
                                ::rs::Movable<NonCppMovableRustType>>);
  static_assert(!::std::is_default_constructible_v<::rs::Movable<PinBox<int>>>);
  static_assert(
      !::std::is_constructible_v<::rs::Movable<PinBox<int>>, ::std::nullptr_t>);
  static_assert(!::std::is_copy_constructible_v<::rs::Movable<PinBox<int>>>);
  static_assert(!::std::is_copy_assignable_v<::rs::Movable<PinBox<int>>>);
  static_assert(
      ::std::is_nothrow_move_constructible_v<::rs::Movable<PinBox<int>>>);
  static_assert(
      ::std::is_nothrow_move_assignable_v<::rs::Movable<PinBox<int>>>);
  static_assert(
      ::std::is_convertible_v<::rs::Movable<PinBox<int>>, PinBox<int>>);
  static_assert(
      !::std::is_convertible_v<::rs::Movable<PinBox<int>>&, PinBox<int>>);
  static_assert(
      !::std::is_convertible_v<const ::rs::Movable<PinBox<int>>&, PinBox<int>>);
  static_assert(![]<typename M = ::rs::Movable<PinBox<int>>>() {
    return requires(M m) { ::std::move(m).as_pin_box(); };
  }());
}

TEST(BoxTest, ConstructInPlace) {
  auto tracker = ::std::make_shared<int>(42);
  EXPECT_EQ(tracker.use_count(), 1);
  {
    Box<Tracked> b(::std::in_place, tracker);
    EXPECT_EQ(tracker.use_count(), 2);
    EXPECT_EQ(b->tracker, tracker);
    EXPECT_EQ((*b).tracker, tracker);
    EXPECT_EQ(b.get()->tracker, tracker);
  }
  EXPECT_EQ(tracker.use_count(), 1);
}

TEST(PinBoxTest, ConstructInPlace) {
  auto tracker = ::std::make_shared<int>(42);
  EXPECT_EQ(tracker.use_count(), 1);
  {
    PinBox<Tracked> b(::std::in_place, tracker);
    EXPECT_EQ(tracker.use_count(), 2);
    EXPECT_EQ(b->tracker, tracker);
    EXPECT_EQ((*b).tracker, tracker);
    EXPECT_EQ(b.get()->tracker, tracker);
  }
  EXPECT_EQ(tracker.use_count(), 1);
}

TEST(BoxTest, ConstructInPlaceImmovable) {
  auto tracker = ::std::make_shared<int>(42);
  EXPECT_EQ(tracker.use_count(), 1);
  {
    Box<Immovable> b(::std::in_place, tracker);
    EXPECT_EQ(tracker.use_count(), 2);
  }
  EXPECT_EQ(tracker.use_count(), 1);
}

TEST(BoxTest, ConstructValue) {
  Box<int> b(42);
  EXPECT_EQ(*b, 42);
}

TEST(BoxTest, MakeBox) {
  ::rs::Movable<Box<int>> m = ::rs::make_box<int>(42);
  EXPECT_EQ(*m, 42);
  EXPECT_EQ(*m.get(), 42);
  EXPECT_EQ(*m.as_box(), 42);

  // Direct single-dereference on a temporary `Movable<Box<T>>`.
  EXPECT_EQ(::rs::make_box<MultiArg>(10, 20)->a, 10);
  EXPECT_EQ(*::rs::make_box<int>(55), 55);

  Box<MultiArg> multi = ::rs::make_box<MultiArg>(10, 20);
  EXPECT_EQ(multi->a, 10);
  EXPECT_EQ(multi->b, 20);

  auto deduced = ::rs::make_box(99);
  static_assert(::std::is_same_v<decltype(deduced), ::rs::Movable<Box<int>>>);
  EXPECT_EQ(*deduced, 99);
  *deduced = 100;
  const auto& const_deduced = deduced;
  static_assert(::std::is_same_v<decltype(*const_deduced), const int&>);
  static_assert(::std::is_same_v<decltype(const_deduced.get()), const int*>);
  static_assert(
      ::std::is_same_v<decltype(const_deduced.as_box()), const Box<int>&>);
  EXPECT_EQ(*const_deduced, 100);

  auto tracker = ::std::make_shared<int>(7);
  {
    Box<Immovable> imm = ::rs::make_box<Immovable>(tracker);
    EXPECT_EQ(tracker.use_count(), 2);
    EXPECT_EQ(*imm->tracker, 7);
  }
  EXPECT_EQ(tracker.use_count(), 1);
}

TEST(PinBoxTest, MakePinBox) {
  ::rs::Movable<PinBox<int>> m = ::rs::make_pin_box<int>(42);
  EXPECT_EQ(*m, 42);
  EXPECT_EQ(*m.get(), 42);
  EXPECT_EQ(*m.as_pin_box(), 42);

  // Direct single-dereference on a temporary `Movable<PinBox<T>>`.
  EXPECT_EQ(::rs::make_pin_box<MultiArg>(10, 20)->a, 10);
  EXPECT_EQ(*::rs::make_pin_box<int>(55), 55);

  PinBox<MultiArg> multi = ::rs::make_pin_box<MultiArg>(10, 20);
  EXPECT_EQ(multi->a, 10);
  EXPECT_EQ(multi->b, 20);

  auto deduced = ::rs::make_pin_box(99);
  static_assert(
      ::std::is_same_v<decltype(deduced), ::rs::Movable<PinBox<int>>>);
  EXPECT_EQ(*deduced, 99);
  *deduced = 100;
  const auto& const_deduced = deduced;
  static_assert(::std::is_same_v<decltype(*const_deduced), const int&>);
  static_assert(::std::is_same_v<decltype(const_deduced.get()), const int*>);
  static_assert(::std::is_same_v<decltype(const_deduced.as_pin_box()),
                                 const PinBox<int>&>);
  EXPECT_EQ(*const_deduced, 100);

  auto tracker = ::std::make_shared<int>(7);
  {
    PinBox<Immovable> imm = ::rs::make_pin_box<Immovable>(tracker);
    EXPECT_EQ(tracker.use_count(), 2);
    EXPECT_EQ(*imm->tracker, 7);
  }
  EXPECT_EQ(tracker.use_count(), 1);
}

TEST(BoxTest, MakeBoxFromNonCppMovableRustType) {
  NonCppMovableRustType::relocate_calls = 0;
  NonCppMovableRustType::live_count = 0;
  {
    // Deduced `T` from `rs::Movable<T>`.
    auto boxed = ::rs::make_box(ReturnNonCppMovableRustType(42));
    static_assert(::std::is_same_v<decltype(boxed),
                                   ::rs::Movable<Box<NonCppMovableRustType>>>);
    EXPECT_EQ(boxed->value, 42);
    EXPECT_EQ((*boxed).value, 42);

    // Explicit `<T>` with `rs::Movable<T>`.
    Box<NonCppMovableRustType> explicit_boxed =
        ::rs::make_box<NonCppMovableRustType>(ReturnNonCppMovableRustType(77));
    EXPECT_EQ(explicit_boxed->value, 77);

    // Direct `Box(rs::Movable<T>)` constructor.
    Box<NonCppMovableRustType> direct_boxed(ReturnNonCppMovableRustType(88));
    EXPECT_EQ(direct_boxed->value, 88);

    // Nested `rs::make_box` (since `make_box` itself returns `rs::Movable`).
    auto nested = ::rs::make_box(::rs::make_box<int>(123));
    static_assert(
        ::std::is_same_v<decltype(nested), ::rs::Movable<Box<Box<int>>>>);
    EXPECT_EQ(**nested, 123);
  }
  // Every relocated-from temporary was suppressed; only the 3 heap instances
  // ran their destructors, leaving live_count == 0 (accounting for relocated
  // increments: each relocation increments live_count without destroying src).
  EXPECT_EQ(NonCppMovableRustType::live_count,
            NonCppMovableRustType::relocate_calls);
}

TEST(PinBoxTest, MakePinBoxFromNonCppMovableRustType) {
  {
    auto boxed = ::rs::make_pin_box(ReturnNonCppMovableRustType(42));
    static_assert(
        ::std::is_same_v<decltype(boxed),
                         ::rs::Movable<PinBox<NonCppMovableRustType>>>);
    EXPECT_EQ(boxed->value, 42);
    EXPECT_EQ((*boxed).value, 42);

    PinBox<NonCppMovableRustType> explicit_boxed =
        ::rs::make_pin_box<NonCppMovableRustType>(
            ReturnNonCppMovableRustType(77));
    EXPECT_EQ(explicit_boxed->value, 77);
  }
}

TEST(BoxTest, Relocation) {
  auto tracker = ::std::make_shared<int>(42);
  {
    alignas(Box<Tracked>) char buf[sizeof(Box<Tracked>)];
    Box<Tracked>* b1 = new (buf) Box<Tracked>(::std::in_place, tracker);
    EXPECT_EQ(tracker.use_count(), 2);
    Box<Tracked> b2(::crubit::UnsafeRelocateTag{}, ::std::move(*b1));
    EXPECT_EQ(tracker.use_count(), 2);
  }
  EXPECT_EQ(tracker.use_count(), 1);
}

TEST(BoxTest, MovableIntegration) {
  auto tracker1 = ::std::make_shared<int>(42);
  auto tracker2 = ::std::make_shared<int>(99);
  {
    ::rs::Movable<Box<Tracked>> m = ::rs::make_box<Tracked>(tracker1);
    EXPECT_FALSE(m.valueless_after_move());
    EXPECT_TRUE(static_cast<bool>(m));
    EXPECT_EQ(tracker1.use_count(), 2);

    ::rs::Movable<Box<Tracked>> m2 = ::std::move(m);
    EXPECT_TRUE(m.valueless_after_move());  // NOLINT(bugprone-use-after-move)
    EXPECT_FALSE(static_cast<bool>(m));     // NOLINT(bugprone-use-after-move)
    EXPECT_FALSE(m2.valueless_after_move());
    EXPECT_EQ(tracker1.use_count(), 2);
    EXPECT_EQ(m2->tracker, tracker1);

    // Move-constructing from an already-valueless `Movable` produces a
    // valueless `Movable`.
    ::rs::Movable<Box<Tracked>> m_empty(
        ::std::move(m));  // NOLINT(bugprone-use-after-move)
    EXPECT_TRUE(m_empty.valueless_after_move());

    // Move-assigning a live `Movable` over another live `Movable` destroys the
    // old box immediately.
    ::rs::Movable<Box<Tracked>> m3 = ::rs::make_box<Tracked>(tracker2);
    EXPECT_EQ(tracker2.use_count(), 2);
    m2 = ::std::move(m3);
    EXPECT_EQ(tracker1.use_count(), 1);
    EXPECT_EQ(tracker2.use_count(), 2);
    EXPECT_TRUE(m3.valueless_after_move());  // NOLINT(bugprone-use-after-move)
    EXPECT_EQ(m2->tracker, tracker2);

    // Move-assigning a valueless `Movable` over a live `Movable` destroys the
    // live box and leaves the target valueless.
    m2 = ::std::move(m_empty);
    EXPECT_EQ(tracker2.use_count(), 1);
    EXPECT_TRUE(m2.valueless_after_move());

    // Lifetime extension when binding `const Box<T>&` to a temporary `Movable`.
    const Box<Tracked>& ref = ::rs::make_box<Tracked>(tracker1);
    EXPECT_EQ(tracker1.use_count(), 2);
    EXPECT_EQ(ref->tracker, tracker1);
  }
  EXPECT_EQ(tracker1.use_count(), 1);
  EXPECT_EQ(tracker2.use_count(), 1);
}

TEST(PinBoxTest, MovableIntegration) {
  auto tracker = ::std::make_shared<int>(42);
  {
    ::rs::Movable<PinBox<Tracked>> m = ::rs::make_pin_box<Tracked>(tracker);
    EXPECT_FALSE(m.valueless_after_move());
    EXPECT_EQ(tracker.use_count(), 2);

    ::crubit::Slot<PinBox<Tracked>> slot;
    ::std::move(m).MoveToSlot(slot);
    EXPECT_TRUE(m.valueless_after_move());  // NOLINT(bugprone-use-after-move)
    EXPECT_EQ(tracker.use_count(), 2);

    auto m2 = ::rs::Movable<PinBox<Tracked>>::TakeFromSlot(::std::move(slot));
    EXPECT_FALSE(m2.valueless_after_move());
    EXPECT_EQ(m2->tracker, tracker);

    const PinBox<Tracked>& ref = ::std::move(m2);
    EXPECT_TRUE(m2.valueless_after_move());  // NOLINT(bugprone-use-after-move)
    EXPECT_EQ(ref->tracker, tracker);
    EXPECT_EQ(tracker.use_count(), 2);
  }
  EXPECT_EQ(tracker.use_count(), 1);
}

TEST(BoxTest, SlotIntegration) {
  auto tracker = ::std::make_shared<int>(42);
  {
    crubit::Slot<Box<Tracked>> slot;
    new (slot.Get()) Box<Tracked>(::std::in_place, tracker);
    EXPECT_EQ(tracker.use_count(), 2);
    auto m = ::rs::Movable<Box<Tracked>>::TakeFromSlot(::std::move(slot));
    EXPECT_EQ(tracker.use_count(), 2);

    crubit::Slot<Box<Tracked>> slot2;
    ::std::move(m).MoveToSlot(slot2);
    EXPECT_TRUE(m.valueless_after_move());  // NOLINT(bugprone-use-after-move)
    Box<Tracked> b = ::std::move(slot2).AssumeInitAndTakeValue();
    EXPECT_EQ(tracker.use_count(), 2);
    EXPECT_EQ(b->tracker, tracker);
  }
  EXPECT_EQ(tracker.use_count(), 1);
}

}  // namespace
}  // namespace rs

namespace {

TEST(BoxDeathTest, ValuelessMovableBoxAccessAborts) {
  auto m = ::rs::make_box<int>(42);
  ::rs::Box<int> b = ::std::move(m);
  EXPECT_EQ(*b, 42);
  EXPECT_DEATH({ (void)*m; }, "valueless");          // NOLINT
  EXPECT_DEATH({ (void)m.get(); }, "valueless");     // NOLINT
  EXPECT_DEATH({ (void)m.as_box(); }, "valueless");  // NOLINT
  EXPECT_DEATH(
      {
        ::rs::Box<int> b2 = ::std::move(m);  // NOLINT
        (void)b2;
      },
      "valueless");

  auto pm = ::rs::make_pin_box<int>(7);
  ::rs::PinBox<int> pb = ::std::move(pm);
  EXPECT_EQ(*pb, 7);
  EXPECT_DEATH({ (void)*pm; }, "valueless");              // NOLINT
  EXPECT_DEATH({ (void)pm.get(); }, "valueless");         // NOLINT
  EXPECT_DEATH({ (void)pm.as_pin_box(); }, "valueless");  // NOLINT
}

TEST(BoxDeathTest, BoxingValuelessMovableAborts) {
  ::rs::Movable<::rs::NonCppMovableRustType> m =
      ::rs::ReturnNonCppMovableRustType(1);
  ::rs::Box<::rs::NonCppMovableRustType> b(::std::move(m));
  EXPECT_EQ(b->value, 1);
  EXPECT_TRUE(m.valueless_after_move());  // NOLINT
  EXPECT_DEATH(
      {
        ::rs::Box<::rs::NonCppMovableRustType> b2(::std::move(m));  // NOLINT
        (void)b2;
      },
      "valueless");
  EXPECT_DEATH(
      { (void)::rs::make_box(::std::move(m)); },  // NOLINT
      "valueless");
  EXPECT_DEATH(
      {
        ::rs::PinBox<::rs::NonCppMovableRustType> pb(::std::move(m));  // NOLINT
        (void)pb;
      },
      "valueless");
  EXPECT_DEATH(
      { (void)::rs::make_pin_box(::std::move(m)); },  // NOLINT
      "valueless");
}

}  // namespace
