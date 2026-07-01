// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/templates/allowlist:allowlist_specific_template

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

// This file contains definitions for a very simple named implicit template
// instantiation test.

// error: class `AlwaysBoundTs` could not be bound
//   Class templates are not yet supported

// error: class `NotBoundTs` could not be bound
//   Class templates are not yet supported

#[inline(always)]
pub fn IntFloatCaller(mut i: crate::__CcTemplateInst13AlwaysBoundTsIifE) {
    unsafe { crate::detail::__rust_thunk___Z14IntFloatCaller13AlwaysBoundTsIifE(&mut i) }
}

#[inline(always)]
pub fn NotBoundCaller(mut i: crate::__CcTemplateInst10NotBoundTsIifE) {
    unsafe { crate::detail::__rust_thunk___Z14NotBoundCaller10NotBoundTsIifE(&mut i) }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AlwaysBoundTs < int , float >
pub struct __CcTemplateInst13AlwaysBoundTsIifE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst13AlwaysBoundTsIifE {}
impl !Sync for __CcTemplateInst13AlwaysBoundTsIifE {}
impl __CcTemplateInst13AlwaysBoundTsIifE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn Member(__this: *mut Self) {
        unsafe { self::cc_template_inst13_always_bound_ts_iif_e::Member(__this) }
    }
}

impl From<(::ffi_11::c_int, f32)> for __CcTemplateInst13AlwaysBoundTsIifE {
    #[inline(always)]
    fn from(args: (::ffi_11::c_int, f32)) -> Self {
        let (mut t, mut s) = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13AlwaysBoundTsIifEC1Eif__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fallowlist_3aallowlist_5fspecific_5ftemplate(&raw mut tmp as*mut _,t,s);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int, f32)> for __CcTemplateInst13AlwaysBoundTsIifE {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int, f32)) -> Self::CtorType {
        <Self as From<(::ffi_11::c_int, f32)>>::from(args)
    }
}

pub mod cc_template_inst13_always_bound_ts_iif_e {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn Member(__this: *mut crate::__CcTemplateInst13AlwaysBoundTsIifE) {
        unsafe {
            crate::detail::__rust_thunk___ZN13AlwaysBoundTsIifE6MemberEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fallowlist_3aallowlist_5fspecific_5ftemplate(__this)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NotBoundTs < int , float >
pub struct __CcTemplateInst10NotBoundTsIifE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst10NotBoundTsIifE {}
impl !Sync for __CcTemplateInst10NotBoundTsIifE {}

impl From<(::ffi_11::c_int, f32)> for __CcTemplateInst10NotBoundTsIifE {
    #[inline(always)]
    fn from(args: (::ffi_11::c_int, f32)) -> Self {
        let (mut t, mut s) = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10NotBoundTsIifEC1Eif__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fallowlist_3aallowlist_5fspecific_5ftemplate(&raw mut tmp as*mut _,t,s);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int, f32)> for __CcTemplateInst10NotBoundTsIifE {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int, f32)) -> Self::CtorType {
        <Self as From<(::ffi_11::c_int, f32)>>::from(args)
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z14IntFloatCaller13AlwaysBoundTsIifE(
            i: &mut crate::__CcTemplateInst13AlwaysBoundTsIifE,
        );
        pub(crate) unsafe fn __rust_thunk___Z14NotBoundCaller10NotBoundTsIifE(
            i: &mut crate::__CcTemplateInst10NotBoundTsIifE,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13AlwaysBoundTsIifEC1Eif__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fallowlist_3aallowlist_5fspecific_5ftemplate(
            __this: *mut ::core::ffi::c_void,
            t: ::ffi_11::c_int,
            s: f32,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13AlwaysBoundTsIifE6MemberEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fallowlist_3aallowlist_5fspecific_5ftemplate(
            __this: *mut crate::__CcTemplateInst13AlwaysBoundTsIifE,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10NotBoundTsIifEC1Eif__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fallowlist_3aallowlist_5fspecific_5ftemplate(
            __this: *mut ::core::ffi::c_void,
            t: ::ffi_11::c_int,
            s: f32,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::__CcTemplateInst13AlwaysBoundTsIifE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst13AlwaysBoundTsIifE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst13AlwaysBoundTsIifE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst13AlwaysBoundTsIifE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInst10NotBoundTsIifE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst10NotBoundTsIifE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10NotBoundTsIifE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10NotBoundTsIifE: Drop);
};
