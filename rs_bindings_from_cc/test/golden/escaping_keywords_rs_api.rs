// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:escaping_keywords_cc
// Features: experimental, extern_c, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cc_type = "type")]
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
            crate::detail::__rust_thunk___ZN4typeC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for r#type {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN4typeC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
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

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_ESCAPING_KEYWORDS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN4typeC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::r#type>,
        );
        pub(crate) fn __rust_thunk___ZN4typeC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::r#type>,
            __param_0: ::ctor::RvalueReference<'b, crate::r#type>,
        );
        pub(crate) fn __rust_thunk___ZN4typeaSERKS_<'a, 'b>(
            __this: &'a mut crate::r#type,
            __param_0: &'b crate::r#type,
        ) -> &'a mut crate::r#type;
        pub(crate) fn __rust_thunk___ZN4typeaSEOS_<'a, 'b>(
            __this: &'a mut crate::r#type,
            __param_0: ::ctor::RvalueReference<'b, crate::r#type>,
        ) -> &'a mut crate::r#type;
        #[link_name = "_Z4impli"]
        pub(crate) fn __rust_thunk___Z4impli(r#match: ::core::ffi::c_int);
    }
}

const _: () = assert!(::core::mem::size_of::<crate::r#type>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::r#type>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::r#type:Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::r#type:Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::r#type:Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::r#type, r#dyn) == 0);
