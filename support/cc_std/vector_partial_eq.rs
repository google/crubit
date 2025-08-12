// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
extern crate alloc;
use crate::std::vector;
use alloc::vec::Vec;
use core::alloc::Allocator;

// The implementation of PartialEq is similar to the one for the std::vec::Vec.
// https://doc.rust-lang.org/src/alloc/vec/partial_eq.rs.html

macro_rules! __impl_partial_eq {
  ([$($vars:tt)*] $lhs:ty, $rhs:ty $(where $ty:ty: $bound:ident)?) => {
      impl<T, U, $($vars)*> PartialEq<$rhs> for $lhs
      where
          T: PartialEq<U>,
          $($ty: $bound)?
      {
          #[inline]
          fn eq(&self, other: &$rhs) -> bool { self[..] == other[..] }
      }
  }
}

// The struct to compare are chosen from the list of types that are known to be
// comparable with the std::vec::Vec.
// https://doc.rust-lang.org/src/alloc/vec/partial_eq.rs.html#36
__impl_partial_eq! {[] vector<T>, vector<U>}
__impl_partial_eq! {[A: Allocator] vector<T>, Vec<U, A>}
__impl_partial_eq! {[A: Allocator] Vec<T, A>, vector<U>}
__impl_partial_eq! {[] vector<T>, [U]}
__impl_partial_eq! {[] [T], vector<U>}
__impl_partial_eq! {[] vector<T>, &[U]}
__impl_partial_eq! {[] &[T], vector<U>}
__impl_partial_eq! {[] vector<T>, &mut [U]}
__impl_partial_eq! {[] &mut [T], vector<U>}
__impl_partial_eq! {[const N: usize] vector<T>, [U; N]}
__impl_partial_eq! {[const N: usize] vector<T>, &[U; N]}
