// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:string_view

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
#[inline(always)]
pub fn string_view_sink<'s>(s: ::cc_std::std::string_view<'s>) {
    let mut s = ::core::mem::MaybeUninit::new(s);
    unsafe {
        crate::detail::__rust_thunk___Z16string_view_sinkNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(s.as_mut_ptr())
    }
}

#[inline(always)]
pub fn string_view_return<'s>(s: ::cc_std::std::string_view<'s>) -> ::cc_std::std::string_view<'s> {
    let mut s = ::core::mem::MaybeUninit::new(s);
    unsafe {
        let mut __crubit_return =
            ::core::mem::MaybeUninit::<::cc_std::std::string_view<'s>>::uninit();
        crate::detail::__rust_thunk___Z18string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(&raw mut __crubit_return as*mut::core::ffi::c_void,s.as_mut_ptr());
        __crubit_return.assume_init()
    }
}

#[inline(always)]
pub fn ambiguous_string_view_return<'a, 'b>(
    a: ::cc_std::std::string_view<'a>,
    b: ::cc_std::std::string_view<'b>,
) -> ::cc_std::std::__u::raw_string_view {
    let mut a = ::core::mem::MaybeUninit::new(a);
    let mut b = ::core::mem::MaybeUninit::new(b);
    unsafe {
        let mut __crubit_return =
            ::core::mem::MaybeUninit::<::cc_std::std::__u::raw_string_view>::uninit();
        crate::detail::__rust_thunk___Z28ambiguous_string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_(&raw mut __crubit_return as*mut::core::ffi::c_void,a.as_mut_ptr(),b.as_mut_ptr());
        __crubit_return.assume_init()
    }
}

#[inline(always)]
pub fn explicit_lifetime_string_view<'a>(x: ::cc_std::std::string_view<'a>) {
    let mut x = ::core::mem::MaybeUninit::new(x);
    unsafe {
        crate::detail::__rust_thunk___Z29explicit_lifetime_string_viewNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(x.as_mut_ptr())
    }
}

#[inline(always)]
pub fn unambiguous_string_view_return_annotated<'a>(
    x: ::cc_std::std::string_view<'a>,
    y: ::cc_std::std::string_view<'a>,
) -> ::cc_std::std::string_view<'a> {
    let mut x = ::core::mem::MaybeUninit::new(x);
    let mut y = ::core::mem::MaybeUninit::new(y);
    unsafe {
        let mut __crubit_return =
            ::core::mem::MaybeUninit::<::cc_std::std::string_view<'a>>::uninit();
        crate::detail::__rust_thunk___Z40unambiguous_string_view_return_annotatedNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_(&raw mut __crubit_return as*mut::core::ffi::c_void,x.as_mut_ptr(),y.as_mut_ptr());
        __crubit_return.assume_init()
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z16string_view_sinkNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE<
            's,
        >(
            s: *mut ::cc_std::std::string_view<'s>,
        );
        pub(crate) unsafe fn __rust_thunk___Z18string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE<
            's,
        >(
            __return: *mut ::core::ffi::c_void,
            s: *mut ::cc_std::std::string_view<'s>,
        );
        pub(crate) unsafe fn __rust_thunk___Z28ambiguous_string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_<
            'a,
            'b,
        >(
            __return: *mut ::core::ffi::c_void,
            a: *mut ::cc_std::std::string_view<'a>,
            b: *mut ::cc_std::std::string_view<'b>,
        );
        pub(crate) unsafe fn __rust_thunk___Z29explicit_lifetime_string_viewNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE<
            'a,
        >(
            x: *mut ::cc_std::std::string_view<'a>,
        );
        pub(crate) unsafe fn __rust_thunk___Z40unambiguous_string_view_return_annotatedNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_<
            'a,
        >(
            __return: *mut ::core::ffi::c_void,
            x: *mut ::cc_std::std::string_view<'a>,
            y: *mut ::cc_std::std::string_view<'a>,
        );
    }
}
