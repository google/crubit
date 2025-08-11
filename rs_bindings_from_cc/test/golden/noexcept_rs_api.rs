// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:noexcept_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=SomeClass
pub struct SomeClass {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for SomeClass {}
impl !Sync for SomeClass {}
unsafe impl ::cxx::ExternType for SomeClass {
    type Id = ::cxx::type_id!("SomeClass");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for SomeClass {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl SomeClass {
    #[inline(always)]
    pub fn create(i: ::core::ffi::c_int, c: ::core::ffi::c_char) {
        unsafe { crate::detail::__rust_thunk___ZN9SomeClass6createEic(i, c) }
    }
}

impl SomeClass {
    #[inline(always)]
    pub unsafe fn no_except_member(__this: *mut Self) {
        crate::detail::__rust_thunk___ZN9SomeClass16no_except_memberEv(__this)
    }
}

impl SomeClass {
    #[inline(always)]
    pub unsafe fn no_except_true_member(__this: *mut Self) {
        crate::detail::__rust_thunk___ZN9SomeClass21no_except_true_memberEv(__this)
    }
}

impl SomeClass {
    #[inline(always)]
    pub unsafe fn no_except_false_member(__this: *mut Self) {
        crate::detail::__rust_thunk___ZN9SomeClass22no_except_false_memberEv(__this)
    }
}

#[inline(always)]
pub fn no_except() {
    unsafe { crate::detail::__rust_thunk___Z9no_exceptv() }
}

#[inline(always)]
pub fn no_except_true() {
    unsafe { crate::detail::__rust_thunk___Z14no_except_truev() }
}

#[inline(always)]
pub fn no_except_false() {
    unsafe { crate::detail::__rust_thunk___Z15no_except_falsev() }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClassC1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZN9SomeClass6createEic"]
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClass6createEic(
            i: ::core::ffi::c_int,
            c: ::core::ffi::c_char,
        );
        #[link_name = "_ZN9SomeClass16no_except_memberEv"]
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClass16no_except_memberEv(
            __this: *mut crate::SomeClass,
        );
        #[link_name = "_ZN9SomeClass21no_except_true_memberEv"]
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClass21no_except_true_memberEv(
            __this: *mut crate::SomeClass,
        );
        #[link_name = "_ZN9SomeClass22no_except_false_memberEv"]
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClass22no_except_false_memberEv(
            __this: *mut crate::SomeClass,
        );
        #[link_name = "_Z9no_exceptv"]
        pub(crate) unsafe fn __rust_thunk___Z9no_exceptv();
        #[link_name = "_Z14no_except_truev"]
        pub(crate) unsafe fn __rust_thunk___Z14no_except_truev();
        #[link_name = "_Z15no_except_falsev"]
        pub(crate) unsafe fn __rust_thunk___Z15no_except_falsev();
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::SomeClass>() == 1);
    assert!(::core::mem::align_of::<crate::SomeClass>() == 1);
    static_assertions::assert_impl_all!(crate::SomeClass: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SomeClass: Drop);
};
