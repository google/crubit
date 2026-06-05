// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/enums/enums.h"

#include <type_traits>

#include "gtest/gtest.h"

namespace {

using enums::qr_error::QrError;
using enums::repr_128::ReprI128;
using enums::repr_128::ReprU128;
using enums::repr_c::MyEnum;
using enums::repr_c::ReprCWithExtremeDiscriminants;
using enums::repr_c::ReprCWithSingleNoPayloadVariant;
using enums::repr_c_clone_active_variant::CloneActiveVariant;
using enums::repr_c_clone_active_variant::is_a;
using enums::repr_c_clone_active_variant::is_b;
using enums::repr_c_clone_active_variant::is_c;
using enums::repr_c_clone_counter::CloneCount;
using enums::repr_c_drop::DropMe;
using enums::repr_int::IntReprEnumWithNoPayload;
using enums::repr_int::IntReprWithSingleNoPayloadVariant;
using enums::repr_int::NegReprIntEnum;
using enums::repr_rust::RustReprEnum;
using enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods;
using enums::repr_rust::RustReprWithSingleTuplePayloadVariant;

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

TEST(EnumsTest, TestConstruction) {
  // TODO(b/489085607): Make `e` `constexpr` once `constexpr` constructors are
  // supported even for types with drop glue.
  MyEnum e = MyEnum::MakeF();
  EXPECT_EQ(e.tag, MyEnum::Tag::F);
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

TEST(EnumsTest, TestRustReprEnumNoPayloadCtor) {
  // `constexpr` below is load-bearing - it is used to verify that aspect of the
  // generated bindings.
  constexpr auto e1 = RustReprEnum::MakeVariant1();
  EXPECT_EQ(e1.get_variant_number(), 1);

  EXPECT_EQ(RustReprEnum::MakeVariant2().get_variant_number(), 2);
  EXPECT_EQ(RustReprEnum::MakeVariant3().get_variant_number(), 3);
}

TEST(EnumsTest, TestRustReprEnumTuplePayloadCtor) {
  // TODO(b/489085607): Make `e1` variable `constexpr` when possible.
  RustReprEnum e1 = RustReprEnum::MakeTuplePayloadVariant(123, 456);
  ASSERT_TRUE(e1.is_tuple_payload_variant());
  EXPECT_EQ(e1.get_first_item_from_tuple_payload(), 123);
}

TEST(EnumsTest, TestRustReprWithNamingConflictBetweenCtorsAndMethods) {
  using EnumType = RustReprWithNamingConflictBetweenCtorsAndMethods;
  auto e1 = EnumType::MakeNoPayloadVariant();
  EXPECT_EQ(e1.get_variant_number(), 1);

  auto e2 = EnumType::MakeTuplePayloadVariant(100);
  EXPECT_EQ(e2.get_variant_number(), 2);
  EXPECT_EQ(e2.get_value(), 200);

  auto e3 = EnumType::MakeStructPayloadVariant(200);
  EXPECT_EQ(e3.get_variant_number(), 3);
  EXPECT_EQ(e3.get_value(), 600);
}

TEST(EnumsTest, TestIntReprEnumNoPayloadCtor) {
  EXPECT_TRUE(IntReprEnumWithNoPayload::MakeNoPayload1().is_no_payload1());
  EXPECT_TRUE(IntReprEnumWithNoPayload::MakeNoPayload2().is_no_payload2());
}

TEST(EnumsTest, TestIntReprWithSingleNoPayloadVariant) {
  auto e = IntReprWithSingleNoPayloadVariant::MakeSingleVariant();
  EXPECT_TRUE(e.is_single_variant());
}

TEST(EnumsTest, TestReprCWithSingleNoPayloadVariant) {
  auto e = ReprCWithSingleNoPayloadVariant::MakeSingleVariant();
  EXPECT_TRUE(e.is_single_variant());
  EXPECT_EQ(e.tag, ReprCWithSingleNoPayloadVariant::Tag::SingleVariant);
}

TEST(EnumsTest, TestRustReprWithSingleTuplePayloadVariant) {
  auto e = RustReprWithSingleTuplePayloadVariant::MakeSingleVariant(123);
  EXPECT_EQ(e.get_single_item_from_tuple_payload(), 123);
}

TEST(EnumsTest, TestQrError) {
  auto e = QrError::MakeDataTooLong();
  EXPECT_TRUE(e.is_data_too_long());
}

TEST(EnumsTest, TestReprCWithExtremeDiscriminants) {
  constexpr auto e_minus_one = ReprCWithExtremeDiscriminants::MakeMinusOne();
  constexpr auto e_minus_two = ReprCWithExtremeDiscriminants::MakeMinusTwo();
  EXPECT_TRUE(e_minus_one.is_minus_one());
  EXPECT_FALSE(e_minus_one.is_minus_two());
  EXPECT_EQ(e_minus_one.tag, ReprCWithExtremeDiscriminants::Tag::MinusOne);

  EXPECT_TRUE(e_minus_two.is_minus_two());
  EXPECT_FALSE(e_minus_two.is_minus_one());
  EXPECT_EQ(e_minus_two.tag, ReprCWithExtremeDiscriminants::Tag::MinusTwo);

  constexpr auto e_min = ReprCWithExtremeDiscriminants::MakeMinI32();
  constexpr auto e_max = ReprCWithExtremeDiscriminants::MakeMaxI32();
  EXPECT_TRUE(e_min.is_min_i32());
  EXPECT_EQ(e_min.tag, ReprCWithExtremeDiscriminants::Tag::MinI32);
  EXPECT_TRUE(e_max.is_max_i32());
  EXPECT_EQ(e_max.tag, ReprCWithExtremeDiscriminants::Tag::MaxI32);
}

TEST(EnumsTest, TestNegReprIntEnum) {
  auto e_minus_one = NegReprIntEnum::MakeMinusOne();
  auto e_minus_two = NegReprIntEnum::MakeMinusTwo();
  EXPECT_TRUE(e_minus_one.is_minus_one());
  EXPECT_FALSE(e_minus_one.is_minus_two());

  EXPECT_TRUE(e_minus_two.is_minus_two());
  EXPECT_FALSE(e_minus_two.is_minus_one());
}

TEST(EnumsTest, TestReprU128) {
  constexpr auto e_zero = ReprU128::MakeZero();
  constexpr auto e_max = ReprU128::MakeMaxU128();
  EXPECT_TRUE(e_max.is_max_u128());
  EXPECT_FALSE(e_zero.is_max_u128());
}

TEST(EnumsTest, TestReprI128) {
  constexpr auto e_zero = ReprI128::MakeZero();
  constexpr auto e_min = ReprI128::MakeMinI128();
  constexpr auto e_max = ReprI128::MakeMaxI128();
  EXPECT_TRUE(e_min.is_min_i128());
  EXPECT_TRUE(e_max.is_max_i128());
  EXPECT_FALSE(e_zero.is_min_i128());
  EXPECT_FALSE(e_zero.is_max_i128());
}

}  // namespace
