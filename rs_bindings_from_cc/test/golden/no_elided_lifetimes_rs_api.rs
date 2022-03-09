// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:no_elided_lifetimes_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use memoffset_unstable_const::offset_of;

pub type __builtin_ms_va_list = *mut u8;

#[inline(always)]
pub unsafe fn free_function(p1: *mut i32) -> *mut i32 {
    crate::detail::__rust_thunk___Z13free_functionRi(p1)
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct S {
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: std::mem::MaybeUninit<u8>,
}

// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=6
// Error while generating bindings for item 'S::S':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=6
// Error while generating bindings for item 'S::S':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=6
// Error while generating bindings for item 'S::S':
// Parameter #0 is not supported: Unsupported type 'struct S &&': Unsupported clang::Type class 'RValueReference'

// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=6
// Error while generating bindings for item 'S::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=6
// Error while generating bindings for item 'S::operator=':
// Parameter #0 is not supported: Unsupported type 'struct S &&': Unsupported clang::Type class 'RValueReference'

impl S {
    #[inline(always)]
    pub unsafe fn const_method(__this: *const S, p1: *mut i32, p2: *mut i32) -> *mut i32 {
        crate::detail::__rust_thunk___ZNK1S12const_methodERiS0_(__this, p1, p2)
    }
}

impl S {
    #[inline(always)]
    pub unsafe fn method(__this: *mut S, p1: *mut i32, p2: *mut i32) -> *mut i32 {
        crate::detail::__rust_thunk___ZN1S6methodERiS0_(__this, p1, p2)
    }
}

#[inline(always)]
pub unsafe fn take_pointer(p: *mut i32) {
    crate::detail::__rust_thunk___Z12take_pointerPi(p)
}

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NO_ELIDED_LIFETIMES_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        #[link_name = "_Z13free_functionRi"]
        pub(crate) fn __rust_thunk___Z13free_functionRi(p1: *mut i32) -> *mut i32;
        #[link_name = "_ZNK1S12const_methodERiS0_"]
        pub(crate) fn __rust_thunk___ZNK1S12const_methodERiS0_(
            __this: *const S,
            p1: *mut i32,
            p2: *mut i32,
        ) -> *mut i32;
        #[link_name = "_ZN1S6methodERiS0_"]
        pub(crate) fn __rust_thunk___ZN1S6methodERiS0_(
            __this: *mut S,
            p1: *mut i32,
            p2: *mut i32,
        ) -> *mut i32;
        #[link_name = "_Z12take_pointerPi"]
        pub(crate) fn __rust_thunk___Z12take_pointerPi(p: *mut i32);
    }
}

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<S>() == 1usize);
const _: () = assert!(std::mem::align_of::<S>() == 1usize);
