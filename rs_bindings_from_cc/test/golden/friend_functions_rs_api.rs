// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:friend_functions_cc
// Features: experimental, non_extern_c_functions, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cpp_type = "SomeClass")]
pub struct SomeClass {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for SomeClass {}
impl !Sync for SomeClass {}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeClass"), crate::SomeClass);

impl Default for SomeClass {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for SomeClass {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for SomeClass {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for SomeClass {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassaSEOS_(self, __param_0);
        }
    }
}

/// Friend functions that are visible via ADL.
#[inline(always)]
pub fn visible_val(mut __param_0: crate::SomeClass) {
    unsafe { crate::detail::__rust_thunk___Z11visible_val9SomeClass(&mut __param_0) }
}

#[inline(always)]
pub fn visible_ref<'a>(__param_0: &'a mut crate::SomeClass) {
    unsafe { crate::detail::__rust_thunk___Z11visible_refR9SomeClass(__param_0) }
}

#[inline(always)]
pub fn visible_cref<'a>(__param_0: &'a crate::SomeClass) {
    unsafe { crate::detail::__rust_thunk___Z12visible_crefRK9SomeClass(__param_0) }
}

#[inline(always)]
pub fn visible_rref<'a>(__param_0: ::ctor::RvalueReference<'a, crate::SomeClass>) {
    unsafe { crate::detail::__rust_thunk___Z12visible_rrefO9SomeClass(__param_0) }
}

/// A function can be declared multiple times - e.g. once in a friend
/// declaration below + in a definition below.  This example mimics
/// Uint128Low64 declarations from absl/numeric/int128.h.  This is a
/// regression test for b/244311755.
#[inline(always)]
pub fn multiple_declarations<'a>(__param_0: &'a crate::SomeClass) -> ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___Z21multiple_declarationsRK9SomeClass(__param_0) }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN9SomeClassC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SomeClass>,
        );
        pub(crate) fn __rust_thunk___ZN9SomeClassC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SomeClass>,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeClass>,
        );
        pub(crate) fn __rust_thunk___ZN9SomeClassaSERKS_<'a, 'b>(
            __this: &'a mut crate::SomeClass,
            __param_0: &'b crate::SomeClass,
        ) -> &'a mut crate::SomeClass;
        pub(crate) fn __rust_thunk___ZN9SomeClassaSEOS_<'a, 'b>(
            __this: &'a mut crate::SomeClass,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeClass>,
        ) -> &'a mut crate::SomeClass;
        pub(crate) fn __rust_thunk___Z11visible_val9SomeClass(__param_0: &mut crate::SomeClass);
        #[link_name = "_Z11visible_refR9SomeClass"]
        pub(crate) fn __rust_thunk___Z11visible_refR9SomeClass<'a>(
            __param_0: &'a mut crate::SomeClass,
        );
        #[link_name = "_Z12visible_crefRK9SomeClass"]
        pub(crate) fn __rust_thunk___Z12visible_crefRK9SomeClass<'a>(
            __param_0: &'a crate::SomeClass,
        );
        #[link_name = "_Z12visible_rrefO9SomeClass"]
        pub(crate) fn __rust_thunk___Z12visible_rrefO9SomeClass<'a>(
            __param_0: ::ctor::RvalueReference<'a, crate::SomeClass>,
        );
        pub(crate) fn __rust_thunk___Z21multiple_declarationsRK9SomeClass<'a>(
            __param_0: &'a crate::SomeClass,
        ) -> ::core::ffi::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::SomeClass>() == 1);
    assert!(::core::mem::align_of::<crate::SomeClass>() == 1);
    static_assertions::assert_impl_all!(crate::SomeClass: Clone);
    static_assertions::assert_impl_all!(crate::SomeClass: Copy);
    static_assertions::assert_not_impl_any!(crate::SomeClass: Drop);
};
