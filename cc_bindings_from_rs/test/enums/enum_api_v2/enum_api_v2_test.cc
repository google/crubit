// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/enums/enum_api_v2/enum_api_v2.h"

#include <type_traits>
#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "support/rs_std/enum.h"

namespace {

using ::enum_api_v2::ClonedEnum;
using ::enum_api_v2::Command;
using ::enum_api_v2::CustomDropEnum;
using ::enum_api_v2::MaybeRef;
using ::enum_api_v2::MaybeTracked;
using ::enum_api_v2::MoveOnlyOuter;
using ::enum_api_v2::NestedMaybeTracked;
using ::enum_api_v2::PayloadTracker;
using ::enum_api_v2::Shape;
using ::enum_api_v2::Sign;
using ::enum_api_v2::TaggedEntry;
using ::enum_api_v2::TrackedPayload;
using ::enum_api_v2::Wrapper;
using ::testing::AllOf;
using ::testing::Field;
using ::testing::Pointee;

template <auto F>
concept ConstantEvaluable =
    requires { typename std::integral_constant<bool, (F(), true)>; };

static_assert(std::is_trivially_copyable_v<Shape>);
static_assert(std::is_trivially_copyable_v<Command>);
static_assert(std::is_trivially_copyable_v<TaggedEntry>);
static_assert(std::is_trivially_copyable_v<MaybeRef>);

static_assert(std::is_copy_constructible_v<ClonedEnum>);
static_assert(!std::is_trivially_copy_constructible_v<ClonedEnum>);
static_assert(std::is_trivially_move_constructible_v<ClonedEnum>);
static_assert(std::is_trivially_destructible_v<ClonedEnum>);

static_assert(!std::is_copy_constructible_v<MoveOnlyOuter>);
static_assert(std::is_trivially_move_constructible_v<MoveOnlyOuter>);
static_assert(std::is_trivially_destructible_v<MoveOnlyOuter>);

static_assert(std::is_copy_constructible_v<MaybeTracked>);
static_assert(std::is_move_constructible_v<MaybeTracked>);
static_assert(!std::is_trivially_destructible_v<MaybeTracked>);

TEST(EnumApiV2Test, ConstructInCppReadInRust) {
  EXPECT_EQ(enum_api_v2::area(Shape::Point::Make()), 0);
  EXPECT_EQ(enum_api_v2::area(Shape::Circle::Make(2)), 12);
  EXPECT_EQ(enum_api_v2::area(Shape::Rect::Make(3, 4)), 12);
  EXPECT_EQ(enum_api_v2::area(Shape(std::in_place_type<Shape::Rect>, 3, 4)),
            12);
}

TEST(EnumApiV2Test, ConstructInRustReadInCpp) {
  Shape rect = enum_api_v2::make_rect(3, 4);
  EXPECT_TRUE(rs::holds_alternative<Shape::Rect>(rect));
  EXPECT_FALSE(rs::holds_alternative<Shape::Circle>(rect));
  EXPECT_EQ(rs::get_if<Shape::Circle>(&rect), nullptr);
  EXPECT_THAT(
      rs::get_if<Shape::Rect>(&rect),
      Pointee(AllOf(Field(&Shape::Rect::w, 3), Field(&Shape::Rect::h, 4))));

  Shape circle = enum_api_v2::make_circle(5);
  EXPECT_EQ(rs::get_unchecked<Shape::Circle>(&circle)->__field0, 5);
}

TEST(EnumApiV2Test, Constexpr) {
  constexpr Shape kCircle = Shape::Circle::Make(7);
  static_assert(rs::get_unchecked<Shape::Circle>(&kCircle)->__field0 == 7);
  constexpr Shape kCopy = kCircle;
  static_assert(rs::get_unchecked<Shape::Circle>(&kCopy)->__field0 == 7);
  EXPECT_EQ(enum_api_v2::area(kCopy), 147);
}

TEST(EnumApiV2Test, Discriminant) {
  EXPECT_EQ(rs::discriminant(Shape::Circle::Make(1)),
            rs::discriminant(enum_api_v2::make_circle(2)));
  EXPECT_NE(rs::discriminant(Shape::Point::Make()),
            rs::discriminant(Shape::Circle::Make(1)));
}

TEST(EnumApiV2Test, NegativeDiscriminants) {
  EXPECT_EQ(enum_api_v2::sign_value(Sign::Negative::Make()), -1);
  EXPECT_EQ(enum_api_v2::sign_value(Sign::Zero::Make()), 0);
  EXPECT_EQ(enum_api_v2::sign_value(Sign::Positive::Make()), 1);
  EXPECT_TRUE(rs::holds_alternative<Sign::Negative>(enum_api_v2::sign_of(-5)));
  EXPECT_TRUE(rs::holds_alternative<Sign::Zero>(enum_api_v2::sign_of(0)));
  EXPECT_TRUE(rs::holds_alternative<Sign::Positive>(enum_api_v2::sign_of(5)));
}

TEST(EnumApiV2Test, SingleVariant) {
  EXPECT_EQ(enum_api_v2::unwrap(Wrapper::Only::Make(42)), 42);
  Wrapper wrapper = Wrapper::Only::Make(1);
  EXPECT_TRUE(rs::holds_alternative<Wrapper::Only>(wrapper));
  EXPECT_EQ(rs::get_unchecked<Wrapper::Only>(&wrapper)->__field0, 1);
}

TEST(EnumApiV2Test, NicheMultiVariant) {
  EXPECT_EQ(enum_api_v2::describe_command(Command::SetFlag::Make(false)), 0);
  EXPECT_EQ(enum_api_v2::describe_command(Command::SetFlag::Make(true)), 1);
  EXPECT_EQ(enum_api_v2::describe_command(Command::Start::Make()), 2);
  EXPECT_EQ(enum_api_v2::describe_command(Command::Stop::Make()), 3);

  Command set_true = enum_api_v2::make_set_flag(true);
  EXPECT_TRUE(rs::holds_alternative<Command::SetFlag>(set_true));
  EXPECT_FALSE(rs::holds_alternative<Command::Start>(set_true));
  EXPECT_THAT(rs::get_if<Command::SetFlag>(&set_true),
              Pointee(Field(&Command::SetFlag::__field0, true)));

  Command start = enum_api_v2::make_start();
  EXPECT_TRUE(rs::holds_alternative<Command::Start>(start));
  EXPECT_FALSE(rs::holds_alternative<Command::SetFlag>(start));
  EXPECT_FALSE(rs::holds_alternative<Command::Stop>(start));

  Command stop = enum_api_v2::make_stop();
  EXPECT_TRUE(rs::holds_alternative<Command::Stop>(stop));

  EXPECT_EQ(rs::discriminant(Command::SetFlag::Make(false)),
            rs::discriminant(Command::SetFlag::Make(true)));
  EXPECT_NE(rs::discriminant(Command::SetFlag::Make(false)),
            rs::discriminant(start));
}

TEST(EnumApiV2Test, NicheNonZeroOffset) {
  EXPECT_EQ(enum_api_v2::describe_entry(TaggedEntry::Entry::Make(7, 3, true)),
            10);
  EXPECT_EQ(enum_api_v2::describe_entry(TaggedEntry::Entry::Make(7, 3, false)),
            -10);
  EXPECT_EQ(enum_api_v2::describe_entry(TaggedEntry::Empty::Make(5)), 1005);
  EXPECT_EQ(enum_api_v2::describe_entry(TaggedEntry::Tombstone::Make()), -999);

  TaggedEntry entry = enum_api_v2::make_entry(10, 2, false);
  EXPECT_THAT(rs::get_if<TaggedEntry::Entry>(&entry),
              Pointee(AllOf(Field(&TaggedEntry::Entry::count, 10),
                            Field(&TaggedEntry::Entry::extra, 2),
                            Field(&TaggedEntry::Entry::flag, false))));

  TaggedEntry empty = enum_api_v2::make_empty_entry(42);
  EXPECT_THAT(rs::get_if<TaggedEntry::Empty>(&empty),
              Pointee(Field(&TaggedEntry::Empty::__field0, 42)));

  TaggedEntry tombstone = enum_api_v2::make_tombstone();
  EXPECT_TRUE(rs::holds_alternative<TaggedEntry::Tombstone>(tombstone));
}

TEST(EnumApiV2Test, NichePointer) {
  static constexpr int32_t kValue = 123;
  constexpr MaybeRef kNone = MaybeRef::None::Make();
  constexpr MaybeRef kSome = MaybeRef::Some::Make(&kValue);
  static_assert(*rs::get_unchecked<MaybeRef::Some>(&kSome)->__field0 == 123);

  EXPECT_EQ(enum_api_v2::read_maybe_ref(kNone), -1);
  EXPECT_EQ(enum_api_v2::read_maybe_ref(kSome), 123);

  MaybeRef from_rust_some = enum_api_v2::make_some_ref();
  EXPECT_THAT(rs::get_if<MaybeRef::Some>(&from_rust_some),
              Pointee(Field(&MaybeRef::Some::__field0, Pointee(99))));

  MaybeRef from_rust_none = enum_api_v2::make_none_ref();
  EXPECT_TRUE(rs::holds_alternative<MaybeRef::None>(from_rust_none));
  EXPECT_EQ(rs::get_if<MaybeRef::Some>(&from_rust_none), nullptr);
}

TEST(EnumApiV2Test, CloneWithoutCopy) {
  ClonedEnum original = enum_api_v2::make_cloned_tracked(7);
  EXPECT_EQ(enum_api_v2::read_cloned_enum(original), 700);

  // Copy construction invokes Rust's `Clone::clone`, incrementing `clones`.
  ClonedEnum copy1 = original;
  EXPECT_EQ(enum_api_v2::read_cloned_enum(copy1), 701);

  // Copy assignment invokes Rust's `Clone::clone_from`.
  ClonedEnum copy2 = ClonedEnum::Empty::Make();
  copy2 = copy1;
  EXPECT_EQ(enum_api_v2::read_cloned_enum(copy2), 702);

  // Move is trivial and does not clone.
  MoveOnlyOuter outer = MoveOnlyOuter::Nested::Make(std::move(copy2));
  MoveOnlyOuter moved_outer = std::move(outer);
  EXPECT_EQ(enum_api_v2::consume_move_only_outer(std::move(moved_outer)), 702);
}

TEST(EnumApiV2Test, DropGlueAndConstexprTrivialVariant) {
  // `MaybeTracked::None` is trivially destructible, so it can be built at
  // compile time even though `MaybeTracked` itself has drop glue.
  constexpr MaybeTracked kNone = MaybeTracked::None::Make();
  EXPECT_TRUE(rs::holds_alternative<MaybeTracked::None>(kNone));

  // `NestedMaybeTracked::OuterNone` is trivially destructible, whereas
  // `OuterSome(MaybeTracked::None::Make())` holds a non-trivially-destructible
  // `MaybeTracked` and is rejected during constant evaluation.
  static_assert(ConstantEvaluable<[] {
    NestedMaybeTracked m = NestedMaybeTracked::OuterNone::Make();
    return 0;
  }>);
  static_assert(!ConstantEvaluable<[] {
    NestedMaybeTracked m =
        NestedMaybeTracked::OuterSome::Make(MaybeTracked::None::Make());
    return 0;
  }>);

  PayloadTracker tracker = PayloadTracker::new_();
  {
    MaybeTracked m = MaybeTracked::Some::Make(tracker.track(42));
    EXPECT_EQ(tracker.live_payloads(), 1);
    EXPECT_THAT(rs::get_if<MaybeTracked::Some>(&m),
                Pointee(Field(&MaybeTracked::Some::__field0,
                              Field(&TrackedPayload::id, 42))));

    // Copy construction clones the payload in Rust.
    MaybeTracked copy = m;
    EXPECT_EQ(tracker.live_payloads(), 2);
    EXPECT_THAT(rs::get_if<MaybeTracked::Some>(&copy),
                Pointee(Field(&MaybeTracked::Some::__field0,
                              Field(&TrackedPayload::id, 42))));

    // Copy assignment drops the old payload and clones the new one in Rust.
    copy = MaybeTracked::None::Make();
    EXPECT_EQ(tracker.live_payloads(), 1);
    EXPECT_TRUE(rs::holds_alternative<MaybeTracked::None>(copy));

    // Move construction swaps with `Default::default()` (`MaybeTracked::None`),
    // so no new `TrackedPayload` is created.
    MaybeTracked moved = std::move(m);
    EXPECT_EQ(tracker.live_payloads(), 1);
    EXPECT_EQ(enum_api_v2::consume_maybe_tracked(std::move(moved)), 42);
    EXPECT_EQ(tracker.live_payloads(), 0);
  }
  EXPECT_EQ(tracker.live_payloads(), 0);

  // Returned from Rust and dropped in C++.
  {
    MaybeTracked from_rust = enum_api_v2::make_some_tracked(tracker, 99);
    EXPECT_EQ(tracker.live_payloads(), 1);
    EXPECT_THAT(rs::get_if<MaybeTracked::Some>(&from_rust),
                Pointee(Field(&MaybeTracked::Some::__field0,
                              Field(&TrackedPayload::id, 99))));
  }
  EXPECT_EQ(tracker.live_payloads(), 0);
}

TEST(EnumApiV2Test, CustomDropImpl) {
  // `CustomDropEnum` implements `Drop` directly, so even its unit variant runs
  // `Drop::drop` and cannot be constructed in a constant expression.
  static_assert(!ConstantEvaluable<[] {
    CustomDropEnum e = CustomDropEnum::Empty::Make();
    return 0;
  }>);
}

}  // namespace
