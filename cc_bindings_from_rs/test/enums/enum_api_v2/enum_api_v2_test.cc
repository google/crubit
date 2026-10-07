// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/enums/enum_api_v2/enum_api_v2.h"

#include <type_traits>
#include <utility>

#include "gtest/gtest.h"
#include "support/rs_std/enum.h"

namespace {

using ::enum_api_v2::Shape;
using ::enum_api_v2::Sign;
using ::enum_api_v2::Wrapper;

static_assert(std::is_trivially_copyable_v<Shape>);

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
  const Shape::Rect* r = rs::get_if<Shape::Rect>(&rect);
  ASSERT_NE(r, nullptr);
  EXPECT_EQ(r->w, 3);
  EXPECT_EQ(r->h, 4);

  Shape circle = enum_api_v2::make_circle(5);
  EXPECT_EQ(rs::get<Shape::Circle>(&circle)->__field0, 5);
}

TEST(EnumApiV2Test, Constexpr) {
  constexpr Shape kCircle = Shape::Circle::Make(7);
  static_assert(rs::get<Shape::Circle>(&kCircle)->__field0 == 7);
  constexpr Shape kCopy = kCircle;
  static_assert(rs::get<Shape::Circle>(&kCopy)->__field0 == 7);
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
  EXPECT_EQ(rs::get<Wrapper::Only>(&wrapper)->__field0, 1);
}

}  // namespace
