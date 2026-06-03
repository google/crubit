// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/known_traits/into/into.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(IntoTest, ConvertConversionOperators) {
  into::Convert convert(1563);
  EXPECT_EQ(static_cast<int32_t>(convert), 1563);
  EXPECT_EQ(static_cast<int64_t>(convert), 1563);
  EXPECT_EQ(static_cast<rs_std::StrRef>(convert), "Convert");
  EXPECT_EQ(static_cast<int16_t>(convert), 1563);
}

TEST(IntoTest, ConvertRefConversionOperators) {
  into::ConvertRef convert_ref =
      into::ConvertRef::create(rs_std::StrRef("Hello, World!"));
  EXPECT_EQ(static_cast<rs_std::StrRef>(convert_ref), "Hello, World!");
  EXPECT_EQ(static_cast<into::Convert>(std::move(convert_ref)).__field0, 42);
}

TEST(IntoTest, CloneCopyConversionAndConstructor) {
  into::CloneCopyType src(42);
  auto target = static_cast<into::CloneCopyTarget>(src);
  EXPECT_EQ(target.__field0, 42);

  into::CloneCopyTarget target2(src);
  EXPECT_EQ(target2.__field0, 42);
}

TEST(IntoTest, CloneAllocConversionAndConstructor) {
  into::CloneAllocType src =
      into::CloneAllocType::create(rs_std::StrRef("hello into alloc"));
  auto target = static_cast<into::CloneAllocTarget>(std::move(src));
  EXPECT_EQ(target.get_value().to_string_view(), "hello into alloc");

  into::CloneAllocType src2 =
      into::CloneAllocType::create(rs_std::StrRef("hello into alloc 2"));
  into::CloneAllocTarget target2(std::move(src2));
  EXPECT_EQ(target2.get_value().to_string_view(), "hello into alloc 2");
}

TEST(IntoTest, NoCloneDefaultConversionAndConstructor) {
  into::NoCloneDefaultType src(100);
  auto target = static_cast<into::NoCloneDefaultTarget>(std::move(src));
  EXPECT_EQ(target.__field0, 100);

  into::NoCloneDefaultType src2(200);
  into::NoCloneDefaultTarget target2(std::move(src2));
  EXPECT_EQ(target2.__field0, 200);
}

TEST(IntoTest, NoCloneCopyDropConversionAndConstructor) {
  into::NoCloneCopyDropType src(500);
  auto target = static_cast<into::NoCloneCopyDropTarget>(std::move(src));
  EXPECT_EQ(target.__field0, 500);

  into::NoCloneCopyDropType src2(600);
  into::NoCloneCopyDropTarget target2(std::move(src2));
  EXPECT_EQ(target2.__field0, 600);
}

TEST(IntoTest, IntoLoop) {
  into::LoopA a(1);
  into::LoopB b(std::move(a));
  EXPECT_EQ(b.__field0, 1);

  into::LoopB b2(2);
  into::LoopA a2(std::move(b2));
  EXPECT_EQ(a2.__field0, 2);

  into::LoopA a3(3);
  EXPECT_EQ(static_cast<into::LoopB>(std::move(a3)).__field0, 3);

  into::LoopB b3(4);
  EXPECT_EQ(static_cast<into::LoopA>(std::move(b3)).__field0, 4);
}

}  // namespace
}  // namespace crubit
