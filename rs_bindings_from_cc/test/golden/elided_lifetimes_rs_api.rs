#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(const_ptr_offset_from, custom_inner_attributes)]

use memoffset_unstable_const::offset_of;

#[inline(always)]
pub fn free_function<'a>(p1: &'a mut i32) -> &'a mut i32 {
    unsafe { crate::detail::__rust_thunk___Z13free_functionRi(p1) }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct S {
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: core::mem::MaybeUninit<u8>,
}

// rs_bindings_from_cc/test/golden/elided_lifetimes.h;l=8
// Error while generating bindings for item 'S::S':
// Nested classes are not supported yet

impl S {
    #[inline(always)]
    pub fn method<'a, 'b, 'c>(__this: &'c mut S, p1: &'a mut i32, p2: &'b mut i32) -> &'c mut i32 {
        unsafe { crate::detail::__rust_thunk___ZN1S6methodERiS0_(__this, p1, p2) }
    }
}

// rs_bindings_from_cc/test/golden/elided_lifetimes.h;l=8
// Error while generating bindings for item 'S::S':
// Parameter type 'struct S &&' is not supported

// rs_bindings_from_cc/test/golden/elided_lifetimes.h;l=8
// Error while generating bindings for item 'S::operator=':
// Parameter type 'struct S &&' is not supported

#[inline(always)]
pub fn take_pointer<'a>(p: Option<&'a mut i32>) {
    unsafe { crate::detail::__rust_thunk___Z12take_pointerPi(p) }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_ELIDED_LIFETIMES_H_

mod detail {
    use super::*;
    extern "C" {
        #[link_name = "_Z13free_functionRi"]
        pub(crate) fn __rust_thunk___Z13free_functionRi<'a>(p1: &'a mut i32) -> &'a mut i32;
        #[link_name = "_ZN1S6methodERiS0_"]
        pub(crate) fn __rust_thunk___ZN1S6methodERiS0_<'a, 'b, 'c>(
            __this: &'c mut S,
            p1: &'a mut i32,
            p2: &'b mut i32,
        ) -> &'c mut i32;
        pub(crate) fn __rust_thunk___ZN1SC1Ev<'a>(__this: &'a mut S);
        pub(crate) fn __rust_thunk___ZN1SC1ERKS_<'a, 'b>(__this: &'b mut S, __param_0: &'a S);
        #[link_name = "_Z12take_pointerPi"]
        pub(crate) fn __rust_thunk___Z12take_pointerPi<'a>(p: Option<&'a mut i32>);
    }
}

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<S>() == 1usize);
const _: () = assert!(std::mem::align_of::<S>() == 1usize);
