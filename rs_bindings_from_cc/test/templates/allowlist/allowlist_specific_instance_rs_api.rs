// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/templates/allowlist:allowlist_specific_instance

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![deny(rust_2024_compatibility)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

// This test checks that we generate complete bindings for a specific
// template instance.

// error: class `Ts` could not be bound
//   Class templates are not yet supported

pub type crubit_bind_instantiation_0 = crate::__CcTemplateInst2TsIifE;

#[inline(always)]
pub fn IntFloatCaller(mut i: crate::__CcTemplateInst2TsIifE) {
    unsafe { crate::detail::__rust_thunk___Z14IntFloatCaller2TsIifE(&mut i) }
}

#[inline(always)]
pub fn ShortDoubleCaller(mut i: crate::__CcTemplateInst2TsIsdE) {
    unsafe { crate::detail::__rust_thunk___Z17ShortDoubleCaller2TsIsdE(&mut i) }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Ts < int , float >
pub struct __CcTemplateInst2TsIifE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst2TsIifE {}
impl !Sync for __CcTemplateInst2TsIifE {}
impl __CcTemplateInst2TsIifE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn Member(__this: *mut Self) {
        unsafe { self::cc_template_inst2_ts_iif_e::Member(__this) }
    }
}

impl From<(::ffi_11::c_int, f32)> for __CcTemplateInst2TsIifE {
    #[inline(always)]
    fn from(args: (::ffi_11::c_int, f32)) -> Self {
        let (mut t, mut s) = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN2TsIifEC1Eif__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fallowlist_3aallowlist_5fspecific_5finstance(&raw mut tmp as*mut _,t,s);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int, f32)> for __CcTemplateInst2TsIifE {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int, f32)) -> Self::CtorType {
        <Self as From<(::ffi_11::c_int, f32)>>::from(args)
    }
}

pub mod cc_template_inst2_ts_iif_e {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn Member(__this: *mut crate::__CcTemplateInst2TsIifE) {
        unsafe {
            crate::detail::__rust_thunk___ZN2TsIifE6MemberEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fallowlist_3aallowlist_5fspecific_5finstance(__this)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Ts < short , double >
pub struct __CcTemplateInst2TsIsdE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst2TsIsdE {}
impl !Sync for __CcTemplateInst2TsIsdE {}

impl From<(::ffi_11::c_short, f64)> for __CcTemplateInst2TsIsdE {
    #[inline(always)]
    fn from(args: (::ffi_11::c_short, f64)) -> Self {
        let (mut t, mut s) = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN2TsIsdEC1Esd__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fallowlist_3aallowlist_5fspecific_5finstance(&raw mut tmp as*mut _,t,s);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_short, f64)> for __CcTemplateInst2TsIsdE {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_short, f64)) -> Self::CtorType {
        <Self as From<(::ffi_11::c_short, f64)>>::from(args)
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z14IntFloatCaller2TsIifE(
            i: &mut crate::__CcTemplateInst2TsIifE,
        );
        pub(crate) unsafe fn __rust_thunk___Z17ShortDoubleCaller2TsIsdE(
            i: &mut crate::__CcTemplateInst2TsIsdE,
        );
        pub(crate) unsafe fn __rust_thunk___ZN2TsIifEC1Eif__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fallowlist_3aallowlist_5fspecific_5finstance(
            __this: *mut ::core::ffi::c_void,
            t: ::ffi_11::c_int,
            s: f32,
        );
        pub(crate) unsafe fn __rust_thunk___ZN2TsIifE6MemberEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fallowlist_3aallowlist_5fspecific_5finstance(
            __this: *mut crate::__CcTemplateInst2TsIifE,
        );
        pub(crate) unsafe fn __rust_thunk___ZN2TsIsdEC1Esd__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fallowlist_3aallowlist_5fspecific_5finstance(
            __this: *mut ::core::ffi::c_void,
            t: ::ffi_11::c_short,
            s: f64,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::__CcTemplateInst2TsIifE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst2TsIifE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst2TsIifE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst2TsIifE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInst2TsIsdE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst2TsIsdE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst2TsIsdE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst2TsIsdE: Drop);
};
