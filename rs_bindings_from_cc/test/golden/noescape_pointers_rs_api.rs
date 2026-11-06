// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:noescape_pointers_cc

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
pub fn takes_noescape_nonnull(p: &mut ::ffi_11::c_int) {
    unsafe { crate::detail::__rust_thunk___Z22takes_noescape_nonnullPi(p) }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `p`: raw pointer
#[inline(always)]
pub unsafe fn takes_noescape_nullable(p: *mut ::ffi_11::c_int) {
    unsafe { crate::detail::__rust_thunk___Z23takes_noescape_nullablePi(p) }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `p`: raw pointer
#[inline(always)]
pub unsafe fn takes_nonnull_only(p: *mut ::ffi_11::c_int) {
    unsafe { crate::detail::__rust_thunk___Z18takes_nonnull_onlyPi(p) }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `p`: raw pointer
#[inline(always)]
pub unsafe fn takes_noescape_void(p: *mut ::ffi_11::c_void) {
    unsafe { crate::detail::__rust_thunk___Z19takes_noescape_voidPv(p) }
}

forward_declare::forward_declare!(pub Incomplete = forward_declare::symbol!("Incomplete"));

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `p`: raw pointer
#[inline(always)]
pub unsafe fn takes_noescape_incomplete(p: *mut crate::Incomplete) {
    unsafe { crate::detail::__rust_thunk___Z25takes_noescape_incompleteP10Incomplete(p) }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `p`: raw pointer
#[inline(always)]
pub unsafe fn takes_noescape_pointer_to_pointer(p: &mut *mut ::ffi_11::c_int) {
    unsafe { crate::detail::__rust_thunk___Z33takes_noescape_pointer_to_pointerPPi(p) }
}

#[inline(always)]
pub fn takes_noescape_const_pointee(p: &::ffi_11::c_int) {
    unsafe { crate::detail::__rust_thunk___Z28takes_noescape_const_pointeePKi(p) }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_Z22takes_noescape_nonnullPi"]
        pub(crate) unsafe fn __rust_thunk___Z22takes_noescape_nonnullPi(p: &mut ::ffi_11::c_int);
        #[link_name = "_Z23takes_noescape_nullablePi"]
        pub(crate) unsafe fn __rust_thunk___Z23takes_noescape_nullablePi(p: *mut ::ffi_11::c_int);
        #[link_name = "_Z18takes_nonnull_onlyPi"]
        pub(crate) unsafe fn __rust_thunk___Z18takes_nonnull_onlyPi(p: *mut ::ffi_11::c_int);
        #[link_name = "_Z19takes_noescape_voidPv"]
        pub(crate) unsafe fn __rust_thunk___Z19takes_noescape_voidPv(p: *mut ::ffi_11::c_void);
        #[link_name = "_Z25takes_noescape_incompleteP10Incomplete"]
        pub(crate) unsafe fn __rust_thunk___Z25takes_noescape_incompleteP10Incomplete(
            p: *mut crate::Incomplete,
        );
        #[link_name = "_Z33takes_noescape_pointer_to_pointerPPi"]
        pub(crate) unsafe fn __rust_thunk___Z33takes_noescape_pointer_to_pointerPPi(
            p: &mut *mut ::ffi_11::c_int,
        );
        #[link_name = "_Z28takes_noescape_const_pointeePKi"]
        pub(crate) unsafe fn __rust_thunk___Z28takes_noescape_const_pointeePKi(p: &::ffi_11::c_int);
    }
}
