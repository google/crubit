// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:item_order_cc
// Features: experimental, extern_c, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![deny(warnings)]

#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cc_type = "FirstStruct")]
pub struct FirstStruct {
    pub field: ::core::ffi::c_int,
}
impl !Send for FirstStruct {}
impl !Sync for FirstStruct {}
forward_declare::unsafe_define!(forward_declare::symbol!("FirstStruct"), crate::FirstStruct);

impl Default for FirstStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11FirstStructC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for FirstStruct {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11FirstStructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for FirstStruct {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN11FirstStructaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for FirstStruct {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN11FirstStructaSEOS_(self, __param_0);
        }
    }
}

#[inline(always)]
pub fn first_func() -> ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___Z10first_funcv() }
}

#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cc_type = "SecondStruct")]
pub struct SecondStruct {
    pub field: ::core::ffi::c_int,
}
impl !Send for SecondStruct {}
impl !Sync for SecondStruct {}
forward_declare::unsafe_define!(forward_declare::symbol!("SecondStruct"), crate::SecondStruct);

impl Default for SecondStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN12SecondStructC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for SecondStruct {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN12SecondStructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for SecondStruct {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN12SecondStructaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for SecondStruct {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN12SecondStructaSEOS_(self, __param_0);
        }
    }
}

#[inline(always)]
pub fn second_func() -> ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___Z11second_funcv() }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN11FirstStructC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::FirstStruct>,
        );
        pub(crate) fn __rust_thunk___ZN11FirstStructC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::FirstStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::FirstStruct>,
        );
        pub(crate) fn __rust_thunk___ZN11FirstStructaSERKS_<'a, 'b>(
            __this: &'a mut crate::FirstStruct,
            __param_0: &'b crate::FirstStruct,
        ) -> &'a mut crate::FirstStruct;
        pub(crate) fn __rust_thunk___ZN11FirstStructaSEOS_<'a, 'b>(
            __this: &'a mut crate::FirstStruct,
            __param_0: ::ctor::RvalueReference<'b, crate::FirstStruct>,
        ) -> &'a mut crate::FirstStruct;
        pub(crate) fn __rust_thunk___Z10first_funcv() -> ::core::ffi::c_int;
        pub(crate) fn __rust_thunk___ZN12SecondStructC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SecondStruct>,
        );
        pub(crate) fn __rust_thunk___ZN12SecondStructC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SecondStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::SecondStruct>,
        );
        pub(crate) fn __rust_thunk___ZN12SecondStructaSERKS_<'a, 'b>(
            __this: &'a mut crate::SecondStruct,
            __param_0: &'b crate::SecondStruct,
        ) -> &'a mut crate::SecondStruct;
        pub(crate) fn __rust_thunk___ZN12SecondStructaSEOS_<'a, 'b>(
            __this: &'a mut crate::SecondStruct,
            __param_0: ::ctor::RvalueReference<'b, crate::SecondStruct>,
        ) -> &'a mut crate::SecondStruct;
        pub(crate) fn __rust_thunk___Z11second_funcv() -> ::core::ffi::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::FirstStruct>() == 4);
    assert!(::core::mem::align_of::<crate::FirstStruct>() == 4);
    static_assertions::assert_impl_all!(crate::FirstStruct: Clone);
    static_assertions::assert_impl_all!(crate::FirstStruct: Copy);
    static_assertions::assert_not_impl_any!(crate::FirstStruct: Drop);
    assert!(memoffset::offset_of!(crate::FirstStruct, field) == 0);

    assert!(::core::mem::size_of::<crate::SecondStruct>() == 4);
    assert!(::core::mem::align_of::<crate::SecondStruct>() == 4);
    static_assertions::assert_impl_all!(crate::SecondStruct: Clone);
    static_assertions::assert_impl_all!(crate::SecondStruct: Copy);
    static_assertions::assert_not_impl_any!(crate::SecondStruct: Drop);
    assert!(memoffset::offset_of!(crate::SecondStruct, field) == 0);
};
