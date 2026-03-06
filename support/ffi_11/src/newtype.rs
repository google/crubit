// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Implement `From<WrappedT> for unwrapped_t` by reusing the `From<WrappedT.0>` impl.
macro_rules! wrapped_to_primitive {
    ($(impl From<$from:ty> for $to:ty);* $(;)?) => {
      $(
        impl From<$from> for $to {
          fn from(x: $from) -> $to {
            x.0.into()
          }
        }
      )*
  }
}

/// Implement `From<unwrapped_t> for WrappedT` by reusing the `WrappedT.0 : From<unwrapped_t>` impl.
macro_rules! primitive_to_wrapped {
    ($(impl From<$from:ty> for $to:path);* $(;)?) => {
      $(
        impl From<$from> for $to {
          fn from(x: $from) -> $to {
            $to(x.into())
          }
        }
      )*
  }
}

/// Implement `From<WrappedU> for WrappedT` by reusing the `From<wrapped_type.0>`
/// impl for T.0.
macro_rules! wrapped_to_wrapped {
  ($(impl From<$from:ty> for $to:ident);* $(;)?) => {
    $(
      impl From<$from> for $to {
        fn from(x: $from) -> $to {
          $to(x.0.into())
        }
      }
    )*
  }
}

/// `new_integer` defines an integer newtype, which is interconvertible with all
/// other known integer types in ffi_11.
macro_rules! new_integer {
    (
        $(#[$($attr:tt)*])*
        pub struct $IntegerType:ident($underlying:ident);
        pub const fn $new_fn:ident;
    ) => {
      $(#[$($attr)*])*
      #[repr(transparent)]
      #[derive(Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
      pub struct $IntegerType($underlying);
      $crate::newtype::new_integer!(@__from, $IntegerType, $underlying);

      pub const fn $new_fn(inner: $underlying) -> $IntegerType { $IntegerType(inner) }

      impl core::fmt::Debug for $IntegerType {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
          <$underlying as core::fmt::Debug>::fmt(&self.0, f)
        }
      }
      impl core::fmt::Display for $IntegerType {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
          <$underlying as core::fmt::Display>::fmt(&self.0, f)
        }
      }
      impl core::cmp::PartialEq<$underlying> for $IntegerType {
        fn eq(&self, rhs: &$underlying) -> bool {
          self.0 == *rhs
        }
      }
      impl core::cmp::PartialEq<$IntegerType> for $underlying {
        fn eq(&self, rhs: &$IntegerType) -> bool {
          *self == rhs.0
        }
      }
      impl core::cmp::PartialOrd<$underlying> for $IntegerType {
        fn partial_cmp(&self, rhs: &$underlying) -> Option<core::cmp::Ordering> {
          self.0.partial_cmp(rhs)
        }
      }
      impl core::cmp::PartialOrd<$IntegerType> for $underlying {
        fn partial_cmp(&self, rhs: &$IntegerType) -> Option<core::cmp::Ordering> {
          self.partial_cmp(&rhs.0)
        }
      }
      impl core::ops::Add<$IntegerType> for $IntegerType {
        type Output = $IntegerType;
        fn add(self, rhs: Self) -> Self {
          $IntegerType(self.0 + rhs.0)
        }
      }
      impl core::ops::Add<$underlying> for $IntegerType {
        type Output = $underlying;
        fn add(self, rhs: $underlying) -> $underlying {
          self.0 + rhs
        }
      }
      impl core::ops::Add<$IntegerType> for $underlying {
        type Output = $underlying;
        fn add(self, rhs: $IntegerType) -> $underlying {
          self + rhs.0
        }
      }
      impl core::ops::AddAssign<$IntegerType> for $IntegerType {
        fn add_assign(&mut self, rhs: Self) {
          self.0 += rhs.0;
        }
      }
      impl core::ops::AddAssign<$underlying> for $IntegerType {
        fn add_assign(&mut self, rhs: $underlying) {
          self.0 += rhs;
        }
      }
      impl core::ops::AddAssign<$IntegerType> for $underlying {
        fn add_assign(&mut self, rhs: $IntegerType) {
          *self += rhs.0;
        }
      }
      impl core::ops::Sub<$IntegerType> for $IntegerType {
        type Output = $IntegerType;
        fn sub(self, rhs: $IntegerType) -> Self {
          $IntegerType(self.0 - rhs.0)
        }
      }
      impl core::ops::Sub<$underlying> for $IntegerType {
        type Output = $underlying;
        fn sub(self, rhs: $underlying) -> $underlying {
          self.0 - rhs
        }
      }
      impl core::ops::Sub<$IntegerType> for $underlying {
        type Output = $underlying;
        fn sub(self, rhs: $IntegerType) -> Self {
          self - rhs.0
        }
      }
      impl core::ops::SubAssign<$IntegerType> for $IntegerType {
        fn sub_assign(&mut self, rhs: Self) {
          self.0 -= rhs.0;
        }
      }
      impl core::ops::SubAssign<$underlying> for $IntegerType {
        fn sub_assign(&mut self, rhs: $underlying) {
          self.0 -= rhs;
        }
      }
      impl core::ops::SubAssign<$IntegerType> for $underlying {
        fn sub_assign(&mut self, rhs: $IntegerType) {
          *self -= rhs.0;
        }
      }
      impl core::ops::Mul<$IntegerType> for $IntegerType {
        type Output = $IntegerType;
        fn mul(self, rhs: Self) -> Self {
          $IntegerType(self.0 * rhs.0)
        }
      }
      impl core::ops::Mul<$underlying> for $IntegerType {
        type Output = $underlying;
        fn mul(self, rhs: $underlying) -> $underlying {
          self.0 * rhs
        }
      }
      impl core::ops::Mul<$IntegerType> for $underlying {
        type Output = $underlying;
        fn mul(self, rhs: $IntegerType) -> Self {
          self * rhs.0
        }
      }
      impl core::ops::MulAssign<$IntegerType> for $IntegerType {
        fn mul_assign(&mut self, rhs: Self) {
          self.0 *= rhs.0;
        }
      }
      impl core::ops::MulAssign<$underlying> for $IntegerType {
        fn mul_assign(&mut self, rhs: $underlying) {
          self.0 *= rhs;
        }
      }
      impl core::ops::MulAssign<$IntegerType> for $underlying {
        fn mul_assign(&mut self, rhs: $IntegerType) {
          *self *= rhs.0;
        }
      }
      impl core::ops::Div<$IntegerType> for $IntegerType {
        type Output = $IntegerType;
        fn div(self, rhs: Self) -> Self {
          $IntegerType(self.0 / rhs.0)
        }
      }
      impl core::ops::Div<$underlying> for $IntegerType {
        type Output = $underlying;
        fn div(self, rhs: $underlying) -> $underlying {
          self.0 / rhs
        }
      }
      impl core::ops::Div<$IntegerType> for $underlying {
        type Output = $underlying;
        fn div(self, rhs: $IntegerType) -> $underlying {
          self / rhs.0
        }
      }
      impl core::ops::DivAssign<$IntegerType> for $IntegerType {
        fn div_assign(&mut self, rhs: Self) {
          self.0 /= rhs.0;
        }
      }
      impl core::ops::DivAssign<$IntegerType> for $underlying {
        fn div_assign(&mut self, rhs: $IntegerType) {
          *self /= rhs.0;
        }
      }
      impl core::ops::DivAssign<$underlying> for $IntegerType {
        fn div_assign(&mut self, rhs: $underlying) {
          self.0 /= rhs;
        }
      }
    };
    // @__into: define <new integer type>.into() for all known integer types with known sizes.
    (@__from, $IntegerType:ident, u8) => {
      $crate::newtype::wrapped_to_primitive!{
        impl From<$IntegerType> for u8;
        impl From<$IntegerType> for i16;
        impl From<$IntegerType> for u16;
        impl From<$IntegerType> for i32;
        impl From<$IntegerType> for u32;
        impl From<$IntegerType> for i64;
        impl From<$IntegerType> for u64;
        impl From<$IntegerType> for i128;
        impl From<$IntegerType> for u128;
        impl From<$IntegerType> for usize;
        impl From<$IntegerType> for ::core::sync::atomic::AtomicU8;
        impl From<$IntegerType> for f32;
        impl From<$IntegerType> for f64;
      }
      $crate::newtype::primitive_to_wrapped!{
        impl From<u8> for $IntegerType;
      }
    };
    (@__from, $IntegerType:ident, u16) => {
      $crate::newtype::wrapped_to_primitive!{
        impl From<$IntegerType> for u16;
        impl From<$IntegerType> for i32;
        impl From<$IntegerType> for u32;
        impl From<$IntegerType> for i64;
        impl From<$IntegerType> for u64;
        impl From<$IntegerType> for i128;
        impl From<$IntegerType> for u128;
        impl From<$IntegerType> for usize;
        impl From<$IntegerType> for ::core::sync::atomic::AtomicU16;
        impl From<$IntegerType> for f32;
        impl From<$IntegerType> for f64;
      }
      $crate::newtype::primitive_to_wrapped!{
        impl From<u8> for $IntegerType;
        impl From<u16> for $IntegerType;
      }
    };
    (@__from, $IntegerType:ident, i32) => {
      $crate::newtype::wrapped_to_primitive!{
        impl From<$IntegerType> for i32;
        impl From<$IntegerType> for i64;
        impl From<$IntegerType> for u64;
        impl From<$IntegerType> for i128;
        impl From<$IntegerType> for u128;
        impl From<$IntegerType> for ::core::sync::atomic::AtomicI32;
        impl From<$IntegerType> for f64;
      }
      $crate::newtype::primitive_to_wrapped!{
        impl From<i8> for $IntegerType;
        impl From<i16> for $IntegerType;
      }
    };
    (@__from, $IntegerType:ident, u32) => {
      $crate::newtype::wrapped_to_primitive!{
        impl From<$IntegerType> for u32;
        impl From<$IntegerType> for i64;
        impl From<$IntegerType> for u64;
        impl From<$IntegerType> for i128;
        impl From<$IntegerType> for u128;
        impl From<$IntegerType> for ::core::sync::atomic::AtomicU32;
        impl From<$IntegerType> for f64;
      }
    };
    (@__from, $IntegerType:ident, i64) => {
      $crate::newtype::wrapped_to_primitive!{
        impl From<$IntegerType> for i64;
        impl From<$IntegerType> for i128;
        impl From<$IntegerType> for ::core::sync::atomic::AtomicI64;
      }
      $crate::newtype::primitive_to_wrapped!{
        impl From<u8> for $IntegerType;
        impl From<i8> for $IntegerType;
        impl From<i16> for $IntegerType;
        impl From<i32> for $IntegerType;
        impl From<i64> for $IntegerType;
      }
    };
    (@__from, $IntegerType:ident, u64) => {
      $crate::newtype::wrapped_to_primitive!{
        impl From<$IntegerType> for u64;
        impl From<$IntegerType> for u128;
        impl From<$IntegerType> for ::core::sync::atomic::AtomicU64;
      }
      $crate::newtype::primitive_to_wrapped!{
        impl From<u8> for $IntegerType;
        impl From<u16> for $IntegerType;
        impl From<u32> for $IntegerType;
        impl From<u64> for $IntegerType;
      }
    };
}

pub(crate) use new_integer;
pub(crate) use primitive_to_wrapped;
pub(crate) use wrapped_to_primitive;
pub(crate) use wrapped_to_wrapped;
