// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/known_traits/ops/rs_ops.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace {

using ::rs_ops::MyInt;

TEST(OpsTest, Add) {
  MyInt a = MyInt::new_(10);
  MyInt b = MyInt::new_(20);
  MyInt c = a + b;
  EXPECT_EQ(c.value, 30);
}

TEST(OpsTest, AddAssign) {
  MyInt a = MyInt::new_(10);
  MyInt b = MyInt::new_(20);
  a += b;
  EXPECT_EQ(a.value, 30);
}

TEST(OpsTest, BitAnd) {
  MyInt a = MyInt::new_(0b1100);
  MyInt b = MyInt::new_(0b1010);
  MyInt c = a & b;
  EXPECT_EQ(c.value, 0b1000);
}

TEST(OpsTest, BitAndAssign) {
  MyInt a = MyInt::new_(0b1100);
  MyInt b = MyInt::new_(0b1010);
  a &= b;
  EXPECT_EQ(a.value, 0b1000);
}

TEST(OpsTest, BitOr) {
  MyInt a = MyInt::new_(0b1100);
  MyInt b = MyInt::new_(0b1010);
  MyInt c = a | b;
  EXPECT_EQ(c.value, 0b1110);
}

TEST(OpsTest, BitOrAssign) {
  MyInt a = MyInt::new_(0b1100);
  MyInt b = MyInt::new_(0b1010);
  a |= b;
  EXPECT_EQ(a.value, 0b1110);
}

TEST(OpsTest, BitXor) {
  MyInt a = MyInt::new_(0b1100);
  MyInt b = MyInt::new_(0b1010);
  MyInt c = a ^ b;
  EXPECT_EQ(c.value, 0b0110);
}

TEST(OpsTest, BitXorAssign) {
  MyInt a = MyInt::new_(0b1100);
  MyInt b = MyInt::new_(0b1010);
  a ^= b;
  EXPECT_EQ(a.value, 0b0110);
}

TEST(OpsTest, Div) {
  MyInt a = MyInt::new_(20);
  MyInt b = MyInt::new_(5);
  MyInt c = a / b;
  EXPECT_EQ(c.value, 4);
}

TEST(OpsTest, DivAssign) {
  MyInt a = MyInt::new_(20);
  MyInt b = MyInt::new_(5);
  a /= b;
  EXPECT_EQ(a.value, 4);
}

TEST(OpsTest, Mul) {
  MyInt a = MyInt::new_(5);
  MyInt b = MyInt::new_(4);
  MyInt c = a * b;
  EXPECT_EQ(c.value, 20);
}

TEST(OpsTest, MulAssign) {
  MyInt a = MyInt::new_(5);
  MyInt b = MyInt::new_(4);
  a *= b;
  EXPECT_EQ(a.value, 20);
}

TEST(OpsTest, Neg) {
  MyInt a = MyInt::new_(5);
  MyInt b = -a;
  EXPECT_EQ(b.value, -5);
}

TEST(OpsTest, Not) {
  MyInt a = MyInt::new_(5);
  MyInt b = !a;
  EXPECT_EQ(b.value, ~5);
}

TEST(OpsTest, Rem) {
  MyInt a = MyInt::new_(22);
  MyInt b = MyInt::new_(5);
  MyInt c = a % b;
  EXPECT_EQ(c.value, 2);
}

TEST(OpsTest, RemAssign) {
  MyInt a = MyInt::new_(22);
  MyInt b = MyInt::new_(5);
  a %= b;
  EXPECT_EQ(a.value, 2);
}

TEST(OpsTest, Shl) {
  MyInt a = MyInt::new_(5);
  MyInt b = a << 2;
  EXPECT_EQ(b.value, 20);
}

TEST(OpsTest, ShlAssign) {
  MyInt a = MyInt::new_(5);
  a <<= 2;
  EXPECT_EQ(a.value, 20);
}

TEST(OpsTest, Shr) {
  MyInt a = MyInt::new_(20);
  MyInt b = a >> 2;
  EXPECT_EQ(b.value, 5);
}

TEST(OpsTest, ShrAssign) {
  MyInt a = MyInt::new_(20);
  a >>= 2;
  EXPECT_EQ(a.value, 5);
}

TEST(OpsTest, Sub) {
  MyInt a = MyInt::new_(20);
  MyInt b = MyInt::new_(5);
  MyInt c = a - b;
  EXPECT_EQ(c.value, 15);
}

TEST(OpsTest, SubAssign) {
  MyInt a = MyInt::new_(20);
  MyInt b = MyInt::new_(5);
  a -= b;
  EXPECT_EQ(a.value, 15);
}

}  // namespace
