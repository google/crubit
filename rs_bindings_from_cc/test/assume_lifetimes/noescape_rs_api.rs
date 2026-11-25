// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:noescape

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
// `[[clang::noescape]]` on parameters, under `assume_lifetimes`.
//
// The importer records the attribute in the IR (`FuncParam::clang_noescape`),
// but nothing reads it yet: it is also still an unknown attribute, so a
// function with such a parameter is not bound. This golden pins that
// behaviour, so that the CL that starts reading the attribute shows up as a
// diff here.

/// Bound: the control, without the attribute.
#[inline(always)]
pub fn Plain<'x>(x: &'x mut ::ffi_11::c_int) {
    unsafe { crate::detail::__rust_thunk___Z5PlainRi(x) }
}

// error: function `NoEscapeRef` could not be bound
//   crubit.rs/errors/unknown_attribute: param x has unknown attribute(s): clang::noescape

// error: function `NoEscapePtr` could not be bound
//   crubit.rs/errors/unknown_attribute: param p has unknown attribute(s): clang::noescape

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_Z5PlainRi"]
        pub(crate) unsafe fn __rust_thunk___Z5PlainRi<'x>(x: &'x mut ::ffi_11::c_int);
    }
}
