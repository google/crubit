// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:escaping_keywords_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use memoffset_unstable_const::offset_of;

pub type __builtin_ms_va_list = *mut u8;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct r#type {
    pub r#dyn: i32,
}

impl Default for r#type {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN4typeC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/escaping_keywords.h;l=6
// Error while generating bindings for item 'type::type':
// Parameter #0 is not supported: Unsupported type 'struct type &&'

// rs_bindings_from_cc/test/golden/escaping_keywords.h;l=6
// Error while generating bindings for item 'type::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/escaping_keywords.h;l=6
// Error while generating bindings for item 'type::operator=':
// Parameter #0 is not supported: Unsupported type 'struct type &&'

#[inline(always)]
pub fn r#impl(r#match: i32) {
    unsafe { crate::detail::__rust_thunk___Z4impli(r#match) }
}

// rs_bindings_from_cc/test/golden/escaping_keywords.h;l=12
// Error while generating bindings for item 'await':
// Class templates are not supported yet

// rs_bindings_from_cc/test/golden/escaping_keywords.h;l=17
// Error while generating bindings for item 'become':
// Function templates are not supported yet

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_ESCAPING_KEYWORDS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN4typeC1Ev<'a>(__this: &'a mut std::mem::MaybeUninit<r#type>);
        #[link_name = "_Z4impli"]
        pub(crate) fn __rust_thunk___Z4impli(r#match: i32);
    }
}

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<r#type>() == 4usize);
const _: () = assert!(std::mem::align_of::<r#type>() == 4usize);
const _: () = assert!(offset_of!(r#type, r#dyn) * 8 == 0usize);
