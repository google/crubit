// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:escaping_keywords_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=type
pub struct r#type {
    pub r#dyn: ::core::ffi::c_int,
}
impl !Send for r#type {}
impl !Sync for r#type {}
forward_declare::unsafe_define!(forward_declare::symbol!("type"), crate::r#type);

impl Default for r#type {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN4typeC1Ev(&raw mut tmp as *mut ::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for r#type {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN4typeC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for r#type {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for r#type {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN4typeaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for r#type {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN4typeaSEOS_(self, __param_0);
        }
    }
}

#[inline(always)]
pub fn r#impl(r#match: ::core::ffi::c_int) {
    unsafe { crate::detail::__rust_thunk___Z4impli(r#match) }
}

// Error while generating bindings for item 'await':
// Class templates are not supported yet

// Error while generating bindings for item 'become':
// Function templates are not supported yet

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN4typeC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN4typeC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::r#type>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4typeaSERKS_<'a, 'b>(
            __this: &'a mut crate::r#type,
            __param_0: &'b crate::r#type,
        ) -> &'a mut crate::r#type;
        pub(crate) unsafe fn __rust_thunk___ZN4typeaSEOS_<'a, 'b>(
            __this: &'a mut crate::r#type,
            __param_0: ::ctor::RvalueReference<'b, crate::r#type>,
        ) -> &'a mut crate::r#type;
        #[link_name = "_Z4impli"]
        pub(crate) unsafe fn __rust_thunk___Z4impli(r#match: ::core::ffi::c_int);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::r#type>() == 4);
    assert!(::core::mem::align_of::<crate::r#type>() == 4);
    static_assertions::assert_impl_all!(crate::r#type: Clone);
    static_assertions::assert_impl_all!(crate::r#type: Copy);
    static_assertions::assert_not_impl_any!(crate::r#type: Drop);
    assert!(::core::mem::offset_of!(crate::r#type, r#dyn) == 0);
};
