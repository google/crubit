// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/enums/enums.h"

#include <type_traits>

#include "gtest/gtest.h"

namespace crubit {
namespace {

using enums::cpp_enum::Color;
using enums::repr_c::MyEnum;
using enums::repr_c_clone_active_variant::CloneActiveVariant;
using enums::repr_c_clone_active_variant::is_a;
using enums::repr_c_clone_active_variant::is_b;
using enums::repr_c_clone_active_variant::is_c;
using enums::repr_c_clone_counter::CloneCount;
using enums::repr_c_drop::DropMe;

TEST(EnumsTest, TestDefault) {
  MyEnum e;
  // The default value is `A(1, 2)`.

  EXPECT_EQ(e.tag, MyEnum::Tag::A);
  EXPECT_EQ(e.A.__field0, 1);
  EXPECT_EQ(e.A.__field1, 2);
}

TEST(EnumsTest, TestModification) {
  MyEnum e;
  // The default value is `A(1, 2)`.

  EXPECT_EQ(e.tag, MyEnum::Tag::A);
  EXPECT_EQ(e.A.__field0, 1);
  EXPECT_EQ(e.A.__field1, 2);

  e.tag = MyEnum::Tag::B;
  e.B.h = true;
  e.B.i = false;

  EXPECT_EQ(e.tag, MyEnum::Tag::B);
  EXPECT_EQ(e.B.h, true);
  EXPECT_EQ(e.B.i, false);
}

TEST(EnumsTest, TestDrop) {
  // See the drop implementation in the Rust file, basically, we increment
  // the value of C.p by 1 when the enum is dropped and C is the active
  // variant.
  int p = 1;
  {
    DropMe d;
    d.tag = DropMe::Tag::C;
    d.C.p = &p;
  }
  EXPECT_EQ(p, 2);

  // Do the same, but now we change the tag.
  int q = 1;
  {
    DropMe d;
    d.tag = DropMe::Tag::C;
    d.C.p = &q;

    d.tag = DropMe::Tag::A;
  }
  EXPECT_EQ(q, 1);
}

TEST(EnumsTest, TestCloneCount) {
  int x = 1;
  CloneCount c;
  c.tag = CloneCount::Tag::A;
  c.A.p = &x;

  // Clone triggers the increment of x.
  CloneCount c2 = c;

  EXPECT_EQ(x, 2);
}

TEST(EnumsTest, TestCloneActiveVariant) {
  // A
  CloneActiveVariant a;
  EXPECT_TRUE(is_a(a));

  // B
  CloneActiveVariant b = a;
  EXPECT_TRUE(is_b(b));

  // C
  CloneActiveVariant c = b;
  EXPECT_TRUE(is_c(c));

  // And back to A
  CloneActiveVariant a2 = c;
  EXPECT_TRUE(is_a(a2));
}

TEST(CppEnumTest, BasicTest) {
  static_assert(std::is_enum_v<Color>);

  Color red = Color::RED;
  Color blue = Color::BLUE;
  EXPECT_EQ(static_cast<int32_t>(red), 0);
  EXPECT_EQ(static_cast<int32_t>(blue), 2);
}

}  // namespace
}  // namespace crubit
