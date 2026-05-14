// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/templates/named_instantiation:named_instantiation
// Features: assume_lifetimes, assume_this_lifetimes, callables, check_default_initialized, experimental, fmt, leading_colons_for_cpp_type, supported, types, unsafe_view, wrapper

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

// Generated from: rs_bindings_from_cc/test/templates/named_instantiation/named_instantiation.h;l=11
// error: class `Ni` could not be bound
//   Class templates are not yet supported

/// Generated from: rs_bindings_from_cc/test/templates/named_instantiation/named_instantiation.h;l=16
pub type NiIF = crate::__CcTemplateInst2NiIifE;

/// Generated from: rs_bindings_from_cc/test/templates/named_instantiation/named_instantiation.h;l=17
#[inline(always)]
pub fn SomeApi<'i>(i: &'i crate::NiIF) {
    unsafe { crate::detail::__rust_thunk___Z7SomeApiRK2NiIifE(i) }
}

/// Generated from: rs_bindings_from_cc/test/templates/named_instantiation/named_instantiation.h;l=12
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: Ni < int , float >
pub struct __CcTemplateInst2NiIifE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst2NiIifE {}
impl !Sync for __CcTemplateInst2NiIifE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!(":: Ni < int , float >"),
    crate::__CcTemplateInst2NiIifE
);

/// Generated from: rs_bindings_from_cc/test/templates/named_instantiation/named_instantiation.h;l=13
impl From<(::ffi_11::c_int, f32)> for __CcTemplateInst2NiIifE {
    #[inline(always)]
    fn from(args: (::ffi_11::c_int, f32)) -> Self {
        let (mut t, mut s) = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN2NiIifEC1Eif__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fnamed_5finstantiation_3anamed_5finstantiation(&raw mut tmp as*mut _,t,s);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int, f32)> for __CcTemplateInst2NiIifE {
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
        #[link_name = "_Z7SomeApiRK2NiIifE"]
        pub(crate) unsafe fn __rust_thunk___Z7SomeApiRK2NiIifE<'i>(i: &'i crate::NiIF);
        pub(crate) unsafe fn __rust_thunk___ZN2NiIifEC1Eif__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fnamed_5finstantiation_3anamed_5finstantiation(
            __this: *mut ::core::ffi::c_void,
            t: ::ffi_11::c_int,
            s: f32,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::__CcTemplateInst2NiIifE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst2NiIifE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst2NiIifE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst2NiIifE: Drop);
};
