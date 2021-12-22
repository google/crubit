#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls)]

use memoffset_unstable_const::offset_of;

#[repr(C)]
pub struct PolymorphicClass {
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: std::mem::MaybeUninit<u8>,
}

impl !Unpin for PolymorphicClass {}

// rs_bindings_from_cc/test/golden/polymorphic.h;l=4
// Error while generating bindings for item 'PolymorphicClass::PolymorphicClass':
// Nested classes are not supported yet

impl Drop for PolymorphicClass {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe { crate::detail::__rust_thunk___ZN16PolymorphicClassD1Ev(self) }
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_POLYMORPHIC_H_

mod detail {
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN16PolymorphicClassD1Ev(__this: *mut PolymorphicClass);
    }
}

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<PolymorphicClass>() == 8usize);
const _: () = assert!(std::mem::align_of::<PolymorphicClass>() == 8usize);
