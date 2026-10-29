// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/templates/named_instantiation:named_instantiation

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
// This file contains definitions for a very simple named implicit template
// instantiation test.

// error: class `Ni` could not be bound
//   Class templates are not yet supported

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst2NiIifE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Ni < int , float >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct NiIF {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NiIF {}
impl !Sync for NiIF {}
forward_declare::unsafe_define!(forward_declare::symbol!("Ni < int , float >"), crate::NiIF);

impl From<(::ffi_11::c_int, f32)> for NiIF {
    #[inline(always)]
    fn from(args: (::ffi_11::c_int, f32)) -> Self {
        let (mut t, mut s) = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__426ccee6__ZN2NiIifEC1Eif(&raw mut tmp as *mut _, t, s);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int, f32)> for NiIF {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int, f32)) -> Self::CtorType {
        <Self as From<(::ffi_11::c_int, f32)>>::from(args)
    }
}

#[inline(always)]
pub fn SomeApi<'i>(i: &'i crate::NiIF) {
    unsafe { crate::detail::__rust_thunk___Z7SomeApiRK2NiIifE(i) }
}

#[doc(hidden)]
pub type __CcTemplateInst2NiIifE = crate::NiIF;

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk__426ccee6__ZN2NiIifEC1Eif(
            __this: *mut ::core::ffi::c_void,
            t: ::ffi_11::c_int,
            s: f32,
        );
        #[link_name = "_Z7SomeApiRK2NiIifE"]
        pub(crate) unsafe fn __rust_thunk___Z7SomeApiRK2NiIifE<'i>(i: &'i crate::NiIF);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::NiIF>() == 1);
    assert!(::core::mem::align_of::<crate::NiIF>() == 1);
    static_assertions::assert_impl_all!(crate::NiIF: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NiIF: Drop);
};
