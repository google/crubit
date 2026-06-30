// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:simple_string_view

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: SV
pub struct SV<'a> {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    __marker_a: ::core::marker::PhantomData<&'a ()>,
}
impl<'a> !Send for SV<'a> {}
impl<'a> !Sync for SV<'a> {}
unsafe impl<'a> ::cxx::ExternType for SV<'a> {
    type Id = ::cxx::type_id!(":: SV");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!(":: SV"), crate::SV<'_>);

impl<'a> Default for SV<'a> {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN2SVC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

/// TODO(zarko): We should mark 'unknowns (or equivalent) as unsafe.
#[inline(always)]
pub fn sv_ident<'s>(mut s: crate::SV<'s>) -> crate::SV<'s> {
    unsafe {
        let mut __crubit_return = ::core::mem::MaybeUninit::<crate::SV<'s>>::uninit();
        crate::detail::__rust_thunk___Z8sv_ident2SV(
            &raw mut __crubit_return as *mut ::core::ffi::c_void,
            &mut s,
        );
        __crubit_return.assume_init()
    }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `s`: type SV has 1 lifetime parameter, but 0 were provided; callers must ensure that arguments have the appropriate lifetime
#[inline(always)]
pub unsafe fn sv_ident_unknown(mut s: crate::SV<'static>) -> crate::SV<'static> {
    unsafe {
        let mut __crubit_return = ::core::mem::MaybeUninit::<crate::SV<'static>>::uninit();
        crate::detail::__rust_thunk___Z16sv_ident_unknown2SV(
            &raw mut __crubit_return as *mut ::core::ffi::c_void,
            &mut s,
        );
        __crubit_return.assume_init()
    }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `s`: type SV has 1 lifetime parameter, but 0 were provided; callers must ensure that arguments have the appropriate lifetime
#[inline(always)]
pub unsafe fn sv_ident_unknown_elided(mut s: crate::SV<'static>) -> crate::SV<'static> {
    unsafe {
        let mut __crubit_return = ::core::mem::MaybeUninit::<crate::SV<'static>>::uninit();
        crate::detail::__rust_thunk___Z23sv_ident_unknown_elided2SV(
            &raw mut __crubit_return as *mut ::core::ffi::c_void,
            &mut s,
        );
        __crubit_return.assume_init()
    }
}

#[inline(always)]
pub fn sv_make_raw() -> crate::SV<'static> {
    unsafe {
        let mut __crubit_return = ::core::mem::MaybeUninit::<crate::SV<'static>>::uninit();
        crate::detail::__rust_thunk___Z11sv_make_rawv(
            &raw mut __crubit_return as *mut ::core::ffi::c_void,
        );
        __crubit_return.assume_init()
    }
}

pub type SVA<'__alias0> = crate::SV<'__alias0>;

#[inline(always)]
pub fn sva_lb<'__rv>(mut s: crate::SVA<'__rv>) -> crate::SVA<'__rv> {
    unsafe {
        let mut __crubit_return = ::core::mem::MaybeUninit::<crate::SVA<'__rv>>::uninit();
        crate::detail::__rust_thunk___Z6sva_lb2SV(
            &raw mut __crubit_return as *mut ::core::ffi::c_void,
            &mut s,
        );
        __crubit_return.assume_init()
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN2SVC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___Z8sv_ident2SV<'s>(
            __return: *mut ::core::ffi::c_void,
            s: &mut crate::SV<'s>,
        );
        pub(crate) unsafe fn __rust_thunk___Z16sv_ident_unknown2SV(
            __return: *mut ::core::ffi::c_void,
            s: &mut crate::SV<'static>,
        );
        pub(crate) unsafe fn __rust_thunk___Z23sv_ident_unknown_elided2SV(
            __return: *mut ::core::ffi::c_void,
            s: &mut crate::SV<'static>,
        );
        pub(crate) unsafe fn __rust_thunk___Z11sv_make_rawv(__return: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___Z6sva_lb2SV<'__rv>(
            __return: *mut ::core::ffi::c_void,
            s: &mut crate::SVA<'__rv>,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::SV>() == 1);
    assert!(::core::mem::align_of::<crate::SV>() == 1);
    static_assertions::assert_impl_all!(crate::SV<'static>: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SV<'static>: Drop);
};
