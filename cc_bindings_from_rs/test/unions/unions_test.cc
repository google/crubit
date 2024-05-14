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

TEST(UnionsTest, ReprRustUnionFieldSmokeTest) {
  unions::repr_rust::U my_union = unions::repr_rust::create();
  my_union.x.value = 1;
  my_union.y.value = 2;

  EXPECT_EQ(my_union.y.value, 2);
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
  my_union.x.value = 3;
  unions::repr_rust_clone::U my_clone = my_union;
  my_union.x.value = 2;
  unions::repr_rust_clone::U my_clone_from =
      unions::repr_rust_clone::U(my_union);
  my_union.x.value = 1;

  EXPECT_EQ(my_clone.x.value, 3);
  EXPECT_EQ(my_clone_from.x.value, 2);
  EXPECT_EQ(my_union.x.value, 1);
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
  int32_t drops = 0;
  {
    unions::repr_rust_drop::U my_union;
    my_union.x.value = &drops;
  }
  EXPECT_EQ(drops, 1);
}

}  // namespace
}  // namespace crubit
