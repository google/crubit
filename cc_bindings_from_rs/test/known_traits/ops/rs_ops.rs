// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `rs_ops_test.cc`.

use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MyInt {
    pub value: i32,
}

impl MyInt {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

impl Add for MyInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

impl AddAssign for MyInt {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl BitAnd for MyInt {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.value & rhs.value)
    }
}

impl BitAndAssign for MyInt {
    fn bitand_assign(&mut self, rhs: Self) {
        self.value &= rhs.value;
    }
}

impl BitOr for MyInt {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new(self.value | rhs.value)
    }
}

impl BitOrAssign for MyInt {
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

impl BitXor for MyInt {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::new(self.value ^ rhs.value)
    }
}

impl BitXorAssign for MyInt {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.value ^= rhs.value;
    }
}

impl Div for MyInt {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.value / rhs.value)
    }
}

impl DivAssign for MyInt {
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
    }
}

impl Mul for MyInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.value * rhs.value)
    }
}

impl MulAssign for MyInt {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
    }
}

impl Neg for MyInt {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.value)
    }
}

impl Not for MyInt {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self::new(!self.value)
    }
}

impl Rem for MyInt {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self::new(self.value % rhs.value)
    }
}

impl RemAssign for MyInt {
    fn rem_assign(&mut self, rhs: Self) {
        self.value %= rhs.value;
    }
}

impl Shl<i32> for MyInt {
    type Output = Self;
    fn shl(self, rhs: i32) -> Self::Output {
        Self::new(self.value << rhs)
    }
}

impl ShlAssign<i32> for MyInt {
    fn shl_assign(&mut self, rhs: i32) {
        self.value <<= rhs;
    }
}

impl Shr<i32> for MyInt {
    type Output = Self;
    fn shr(self, rhs: i32) -> Self::Output {
        Self::new(self.value >> rhs)
    }
}

impl ShrAssign<i32> for MyInt {
    fn shr_assign(&mut self, rhs: i32) {
        self.value >>= rhs;
    }
}

impl Sub for MyInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.value - rhs.value)
    }
}

impl SubAssign for MyInt {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

/// Exercises operator impls whose `Self` type is a *reference* to the ADT.
///
/// `MyBorrowedInt` does intentially no derive `Copy`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MyBorrowedInt {
    pub value: i32,
}

impl MyBorrowedInt {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

impl Add<&MyBorrowedInt> for &MyBorrowedInt {
    type Output = MyBorrowedInt;
    fn add(self, rhs: &MyBorrowedInt) -> MyBorrowedInt {
        MyBorrowedInt::new(self.value + rhs.value)
    }
}

impl Neg for &MyBorrowedInt {
    type Output = MyBorrowedInt;
    fn neg(self) -> MyBorrowedInt {
        MyBorrowedInt::new(-self.value)
    }
}

impl Shl<i32> for &MyBorrowedInt {
    type Output = MyBorrowedInt;
    fn shl(self, rhs: i32) -> MyBorrowedInt {
        MyBorrowedInt::new(self.value << rhs)
    }
}

// Rust's `forward_ref_binop!`/`forward_ref_unop!` pattern: the same operator implemented on both
// `MyInt` and `&MyInt`. The self type only decides how the C++ receiver is qualified, so both map
// to the same C++ overload and Crubit must emit exactly one of them -- otherwise the generated
// header does not compile. The impl on the ADT wins, so the bindings for `MyInt` are unaffected
// by the two impls below.

impl Add<MyInt> for &MyInt {
    type Output = MyInt;
    fn add(self, rhs: MyInt) -> MyInt {
        MyInt::new(self.value + rhs.value)
    }
}

impl Not for &MyInt {
    type Output = MyInt;
    fn not(self) -> MyInt {
        MyInt::new(!self.value)
    }
}
