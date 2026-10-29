// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/templates/allowlist:allowlist_specific_instance

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
// This test checks that we generate complete bindings for a specific
// template instance.

// error: class `Ts` could not be bound
//   Class templates are not yet supported

// error: namespace `(anonymous)` could not be bound
//   Anonymous namespaces are not yet supported

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst2TsIifE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Ts < int , float >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct TsIntFloat {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for TsIntFloat {}
impl !Sync for TsIntFloat {}
forward_declare::unsafe_define!(forward_declare::symbol!("Ts < int , float >"), crate::TsIntFloat);
impl TsIntFloat {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn Member(__this: *mut Self) {
        unsafe { self::ts_int_float::Member(__this) }
    }
}

impl From<(::ffi_11::c_int, f32)> for TsIntFloat {
    #[inline(always)]
    fn from(args: (::ffi_11::c_int, f32)) -> Self {
        let (mut t, mut s) = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__ca694370__ZN2TsIifEC1Eif(&raw mut tmp as *mut _, t, s);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int, f32)> for TsIntFloat {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int, f32)) -> Self::CtorType {
        <Self as From<(::ffi_11::c_int, f32)>>::from(args)
    }
}

pub mod ts_int_float {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn Member(__this: *mut crate::TsIntFloat) {
        unsafe { crate::detail::__rust_thunk__f7010ef7__ZN2TsIifE6MemberEv(__this) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst2TsIsdE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Ts < short , double >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct TsShortDouble {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for TsShortDouble {}
impl !Sync for TsShortDouble {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("Ts < short , double >"),
    crate::TsShortDouble
);

impl From<(::ffi_11::c_short, f64)> for TsShortDouble {
    #[inline(always)]
    fn from(args: (::ffi_11::c_short, f64)) -> Self {
        let (mut t, mut s) = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__ca694370__ZN2TsIsdEC1Esd(&raw mut tmp as *mut _, t, s);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_short, f64)> for TsShortDouble {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_short, f64)) -> Self::CtorType {
        <Self as From<(::ffi_11::c_short, f64)>>::from(args)
    }
}

#[inline(always)]
pub fn IntFloatCaller(i: crate::TsIntFloat) {
    let mut i = ::core::mem::MaybeUninit::new(i);
    unsafe { crate::detail::__rust_thunk___Z14IntFloatCaller2TsIifE(i.as_mut_ptr()) }
}

#[inline(always)]
pub fn ShortDoubleCaller(i: crate::TsShortDouble) {
    let mut i = ::core::mem::MaybeUninit::new(i);
    unsafe { crate::detail::__rust_thunk___Z17ShortDoubleCaller2TsIsdE(i.as_mut_ptr()) }
}

/// Class template instantiations that aren't named by a C++ type alias.
pub mod zz_template_instantiations {
    #[doc(hidden)]
    pub type __CcTemplateInst2TsIifE = crate::TsIntFloat;

    #[doc(hidden)]
    pub type __CcTemplateInst2TsIsdE = crate::TsShortDouble;
}
// Keeps the instantiations available at their old path.
#[doc(hidden)]
pub use self::zz_template_instantiations::*;

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk__ca694370__ZN2TsIifEC1Eif(
            __this: *mut ::core::ffi::c_void,
            t: ::ffi_11::c_int,
            s: f32,
        );
        pub(crate) unsafe fn __rust_thunk__f7010ef7__ZN2TsIifE6MemberEv(
            __this: *mut crate::TsIntFloat,
        );
        pub(crate) unsafe fn __rust_thunk__ca694370__ZN2TsIsdEC1Esd(
            __this: *mut ::core::ffi::c_void,
            t: ::ffi_11::c_short,
            s: f64,
        );
        pub(crate) unsafe fn __rust_thunk___Z14IntFloatCaller2TsIifE(i: *mut crate::TsIntFloat);
        pub(crate) unsafe fn __rust_thunk___Z17ShortDoubleCaller2TsIsdE(
            i: *mut crate::TsShortDouble,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::TsIntFloat>() == 1);
    assert!(::core::mem::align_of::<crate::TsIntFloat>() == 1);
    static_assertions::assert_impl_all!(crate::TsIntFloat: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TsIntFloat: Drop);

    assert!(::core::mem::size_of::<crate::TsShortDouble>() == 1);
    assert!(::core::mem::align_of::<crate::TsShortDouble>() == 1);
    static_assertions::assert_impl_all!(crate::TsShortDouble: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TsShortDouble: Drop);
};
