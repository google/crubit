// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/forward_declaration/basic_references:forward_declaration

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![deny(rust_2024_compatibility)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

forward_declare::forward_declare!(pub A = forward_declare::symbol!(":: A"));

#[inline(always)]
pub fn fwd_source() -> ::cref::CMut<'static, crate::A> {
    unsafe { crate::detail::__rust_thunk___Z10fwd_sourcev() }
}

#[inline(always)]
pub fn fwd_ident<'a>(a: ::cref::CMut<'a, crate::A>) -> ::cref::CMut<'a, crate::A> {
    unsafe { crate::detail::__rust_thunk___Z9fwd_identR1A(a) }
}

#[inline(always)]
pub fn fwd_ident_const<'a>(a: ::cref::CRef<'a, crate::A>) -> ::cref::CRef<'a, crate::A> {
    unsafe { crate::detail::__rust_thunk___Z15fwd_ident_constRK1A(a) }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_Z10fwd_sourcev"]
        pub(crate) unsafe fn __rust_thunk___Z10fwd_sourcev() -> ::cref::CMut<'static, crate::A>;
        #[link_name = "_Z9fwd_identR1A"]
        pub(crate) unsafe fn __rust_thunk___Z9fwd_identR1A<'a>(
            a: ::cref::CMut<'a, crate::A>,
        ) -> ::cref::CMut<'a, crate::A>;
        #[link_name = "_Z15fwd_ident_constRK1A"]
        pub(crate) unsafe fn __rust_thunk___Z15fwd_ident_constRK1A<'a>(
            a: ::cref::CRef<'a, crate::A>,
        ) -> ::cref::CRef<'a, crate::A>;
    }
}
