// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unsafe_attrs_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[inline(always)]
pub fn ReturnsTrue() -> bool {
    unsafe { crate::detail::__rust_thunk___ZL11ReturnsTruev() }
}

#[inline(always)]
pub fn ReturnsFalse() -> bool {
    unsafe { crate::detail::__rust_thunk___ZL12ReturnsFalsev() }
}

#[inline(always)]
pub fn TotallySafe() {
    unsafe { crate::detail::__rust_thunk___Z11TotallySafev() }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `__param_0`: raw pointer
#[inline(always)]
pub unsafe fn TotallyUnsafe(__param_0: *mut ::ffi_11::c_void) {
    crate::detail::__rust_thunk___Z13TotallyUnsafePv(__param_0)
}

/// # Safety
///
/// The C++ function is explicitly annotated as unsafe. Ensure that its safety requirements are upheld.
#[inline(always)]
pub unsafe fn SafeSignatureButAnnotatedUnsafe() {
    crate::detail::__rust_thunk___Z31SafeSignatureButAnnotatedUnsafev()
}

#[inline(always)]
pub fn SafeSignatureButAnnotatedSafe() {
    unsafe { crate::detail::__rust_thunk___Z29SafeSignatureButAnnotatedSafev() }
}

/// # Safety
///
/// The C++ function is explicitly annotated as unsafe. Ensure that its safety requirements are upheld.
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `__param_0`: raw pointer
#[inline(always)]
pub unsafe fn UnsafeSignatureButAnnotatedUnsafe(__param_0: *mut ::ffi_11::c_void) {
    crate::detail::__rust_thunk___Z33UnsafeSignatureButAnnotatedUnsafePv(__param_0)
}

#[inline(always)]
pub fn UnsafeSignatureButAnnotatedSafe(__param_0: *mut ::ffi_11::c_void) {
    unsafe { crate::detail::__rust_thunk___Z31UnsafeSignatureButAnnotatedSafePv(__param_0) }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZL11ReturnsTruev() -> bool;
        pub(crate) unsafe fn __rust_thunk___ZL12ReturnsFalsev() -> bool;
        #[link_name = "_Z11TotallySafev"]
        pub(crate) unsafe fn __rust_thunk___Z11TotallySafev();
        #[link_name = "_Z13TotallyUnsafePv"]
        pub(crate) unsafe fn __rust_thunk___Z13TotallyUnsafePv(__param_0: *mut ::ffi_11::c_void);
        #[link_name = "_Z31SafeSignatureButAnnotatedUnsafev"]
        pub(crate) unsafe fn __rust_thunk___Z31SafeSignatureButAnnotatedUnsafev();
        #[link_name = "_Z29SafeSignatureButAnnotatedSafev"]
        pub(crate) unsafe fn __rust_thunk___Z29SafeSignatureButAnnotatedSafev();
        #[link_name = "_Z33UnsafeSignatureButAnnotatedUnsafePv"]
        pub(crate) unsafe fn __rust_thunk___Z33UnsafeSignatureButAnnotatedUnsafePv(
            __param_0: *mut ::ffi_11::c_void,
        );
        #[link_name = "_Z31UnsafeSignatureButAnnotatedSafePv"]
        pub(crate) unsafe fn __rust_thunk___Z31UnsafeSignatureButAnnotatedSafePv(
            __param_0: *mut ::ffi_11::c_void,
        );
    }
}
