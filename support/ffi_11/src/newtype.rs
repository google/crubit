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
///
/// TODO(jeanpierreda): Also define arithmetic, etc.
macro_rules! new_integer {
    (
      $(
        $(#[$($attr:tt)*])*
        pub struct $IntegerType:ident($underlying:ident);)*
    ) => {
      $(
        $(#[$($attr)*])*
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
        pub struct $IntegerType($underlying);
        $crate::newtype::new_integer!(@__from, $IntegerType, $underlying);
      )*
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
