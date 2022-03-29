// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:private_members_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use memoffset_unstable_const::offset_of;
use static_assertions::{assert_impl_all, assert_not_impl_all};

pub type __builtin_ms_va_list = *mut u8;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SomeClass {
    pub public_member_variable_: i32,
    private_member_variable_: i32,
}

impl Default for SomeClass {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, SomeClass>> for SomeClass {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, SomeClass>) -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/private_members.h;l=10
// Error while generating bindings for item 'SomeClass::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/private_members.h;l=10
// Error while generating bindings for item 'SomeClass::operator=':
// Bindings for this kind of operator are not supported

impl SomeClass {
    #[inline(always)]
    pub fn public_method<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN9SomeClass13public_methodEv(self) }
    }
}

impl SomeClass {
    #[inline(always)]
    pub fn public_static_method() {
        unsafe { crate::detail::__rust_thunk___ZN9SomeClass20public_static_methodEv() }
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_PRIVATE_MEMBERS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN9SomeClassC1Ev<'a>(
            __this: &'a mut std::mem::MaybeUninit<SomeClass>,
        );
        pub(crate) fn __rust_thunk___ZN9SomeClassC1EOS_<'a, 'b>(
            __this: &'a mut std::mem::MaybeUninit<SomeClass>,
            __param_0: ctor::RvalueReference<'b, SomeClass>,
        );
        #[link_name = "_ZN9SomeClass13public_methodEv"]
        pub(crate) fn __rust_thunk___ZN9SomeClass13public_methodEv<'a>(__this: &'a mut SomeClass);
        #[link_name = "_ZN9SomeClass20public_static_methodEv"]
        pub(crate) fn __rust_thunk___ZN9SomeClass20public_static_methodEv();
    }
}

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<SomeClass>() == 8usize);
const _: () = assert!(std::mem::align_of::<SomeClass>() == 4usize);
const _: () = {
    assert_impl_all!(SomeClass: Clone);
};
const _: () = {
    assert_impl_all!(SomeClass: Copy);
};
const _: () = {
    assert_not_impl_all!(SomeClass: Drop);
};
const _: () = assert!(offset_of!(SomeClass, public_member_variable_) * 8 == 0usize);
const _: () = assert!(offset_of!(SomeClass, private_member_variable_) * 8 == 32usize);
