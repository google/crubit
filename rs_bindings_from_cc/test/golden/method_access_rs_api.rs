// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:method_access_cc

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
///CRUBIT_ANNOTATE: cpp_type=Struct
pub struct Struct {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Struct {}
impl !Sync for Struct {}
unsafe impl ::cxx::ExternType for Struct {
    type Id = ::cxx::type_id!("Struct");
    type Kind = ::cxx::kind::Trivial;
}
impl Struct {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn AccessNone(__this: *mut Self) {
        crate::detail::__rust_thunk___ZN6Struct10AccessNoneEv(__this)
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn AccessPublic(__this: *mut Self) {
        crate::detail::__rust_thunk___ZN6Struct12AccessPublicEv(__this)
    }
}

impl Default for Struct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN6StructC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Class
pub struct Class {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Class {}
impl !Sync for Class {}
unsafe impl ::cxx::ExternType for Class {
    type Id = ::cxx::type_id!("Class");
    type Kind = ::cxx::kind::Trivial;
}
impl Class {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn AccessPublic(__this: *mut Self) {
        crate::detail::__rust_thunk___ZN5Class12AccessPublicEv(__this)
    }
}

impl Default for Class {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5ClassC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN6StructC1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZN6Struct10AccessNoneEv"]
        pub(crate) unsafe fn __rust_thunk___ZN6Struct10AccessNoneEv(__this: *mut crate::Struct);
        #[link_name = "_ZN6Struct12AccessPublicEv"]
        pub(crate) unsafe fn __rust_thunk___ZN6Struct12AccessPublicEv(__this: *mut crate::Struct);
        pub(crate) unsafe fn __rust_thunk___ZN5ClassC1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZN5Class12AccessPublicEv"]
        pub(crate) unsafe fn __rust_thunk___ZN5Class12AccessPublicEv(__this: *mut crate::Class);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Struct>() == 1);
    assert!(::core::mem::align_of::<crate::Struct>() == 1);
    static_assertions::assert_impl_all!(crate::Struct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Struct: Drop);

    assert!(::core::mem::size_of::<crate::Class>() == 1);
    assert!(::core::mem::align_of::<crate::Class>() == 1);
    static_assertions::assert_impl_all!(crate::Class: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Class: Drop);
};
