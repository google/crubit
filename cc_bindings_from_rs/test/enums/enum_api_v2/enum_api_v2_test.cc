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

using ::enum_api_v2::Command;
using ::enum_api_v2::MaybeRef;
using ::enum_api_v2::Shape;
using ::enum_api_v2::Sign;
using ::enum_api_v2::TaggedEntry;
using ::enum_api_v2::Wrapper;
using ::testing::AllOf;
using ::testing::Field;
using ::testing::Pointee;

static_assert(std::is_trivially_copyable_v<Shape>);
static_assert(std::is_trivially_copyable_v<Command>);
static_assert(std::is_trivially_copyable_v<TaggedEntry>);
static_assert(std::is_trivially_copyable_v<MaybeRef>);

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

}  // namespace
