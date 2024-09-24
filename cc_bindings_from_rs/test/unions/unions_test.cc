// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <bit>
#include <cstdint>

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/unions/unions_cc_api.h"

namespace crubit {
namespace {

TEST(UnionsTest, ReprCUnionFieldSmokeTest) {
  unions::repr_c::U my_union = unions::repr_c::create();
  my_union.x = 1;
  my_union.y = 2;

  uint32_t union_value = std::bit_cast<uint32_t>(my_union);
  EXPECT_EQ(union_value, 2);
}

template <class T>
concept HasXField = requires(T u) { u.x; };
template <class T>
concept HasYField = requires(T u) { u.x; };

TEST(UnionsTest, ReprRustUnionFieldSmokeTest) {
  unions::repr_rust::U my_union = unions::repr_rust::create();
  EXPECT_FALSE(HasXField<unions::repr_rust::U>);
  EXPECT_FALSE(HasYField<unions::repr_rust::U>);
  my_union.set_x(1);
  my_union.set_y(2);

  EXPECT_EQ(my_union.get_y(), 2);
}

TEST(UnionsTest, ReprCUnionPacked) {
  unions::repr_c_packed::U my_union_packed = unions::repr_c_packed::create();
  unions::repr_c::U my_union = unions::repr_c::create();

  EXPECT_EQ(alignof(my_union_packed), 1);
  EXPECT_EQ(alignof(my_union), 4);
}

TEST(UnionsTest, ReprRustUnionPacked) {
  unions::repr_rust_packed::U my_union_packed =
      unions::repr_rust_packed::create();
  unions::repr_rust::U my_union = unions::repr_rust::create();
  EXPECT_FALSE(HasXField<unions::repr_rust_packed::U>);
  EXPECT_FALSE(HasYField<unions::repr_rust_packed::U>);

  EXPECT_EQ(alignof(my_union_packed), 1);
  EXPECT_EQ(alignof(my_union), 4);
}

TEST(UnionsTest, ReprCUnionCloneTest) {
  unions::repr_c_clone::U my_union = unions::repr_c_clone::create();
  my_union.x = 3;
  unions::repr_c_clone::U my_clone = my_union;
  my_union.x = 2;
  unions::repr_c_clone::U my_clone_from = unions::repr_c_clone::U(my_union);
  my_union.x = 1;

  EXPECT_EQ(my_clone.x, 3);
  EXPECT_EQ(my_clone_from.x, 2);
  EXPECT_EQ(my_union.x, 1);
}

TEST(UnionsTest, ReprRustUnionCloneTest) {
  unions::repr_rust_clone::U my_union = unions::repr_rust_clone::create();
  EXPECT_FALSE(HasXField<unions::repr_rust_clone::U>);
  EXPECT_FALSE(HasYField<unions::repr_rust_clone::U>);
  my_union.set_x(3);
  unions::repr_rust_clone::U my_clone = my_union;
  my_union.set_x(2);
  unions::repr_rust_clone::U my_clone_from =
      unions::repr_rust_clone::U(my_union);
  my_union.set_x(1);

  EXPECT_EQ(my_clone.get_x(), 3);
  EXPECT_EQ(my_clone_from.get_x(), 2);
  EXPECT_EQ(my_union.get_x(), 1);
}

TEST(UnionsTest, ReprCUnionDropTest) {
  int32_t drops = 0;
  {
    unions::repr_c_drop::U my_union;
    my_union.x = &drops;
  }
  EXPECT_EQ(drops, 1);
}

TEST(UnionsTest, ReprRustUnionDropTest) {
  EXPECT_FALSE(HasXField<unions::repr_rust_drop::U>);
  int32_t drops = 0;
  {
    unions::repr_rust_drop::U my_union;
    my_union.set_x(&drops);
  }
  EXPECT_EQ(drops, 1);
}

}  // namespace
}  // namespace crubit
