// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //examples/cpp/unsafe_attributes:example_lib
// Features: supported

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

/// Generated from: examples/cpp/unsafe_attributes/example.h;l=10
#[inline(always)]
pub fn SafeSignatureWithoutAnnotation() {
    unsafe { crate::detail::__rust_thunk___Z30SafeSignatureWithoutAnnotationv() }
}

/// # Safety
///
/// The C++ function is explicitly annotated as unsafe. Ensure that its safety requirements are upheld.
///
/// Generated from: examples/cpp/unsafe_attributes/example.h;l=11
#[inline(always)]
pub unsafe fn SafeSignatureButAnnotatedUnsafe() {
    crate::detail::__rust_thunk___Z31SafeSignatureButAnnotatedUnsafev()
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `__param_0`: raw pointer
///
/// Generated from: examples/cpp/unsafe_attributes/example.h;l=13
#[inline(always)]
pub unsafe fn UnsafeSignatureWithoutAnnotation(__param_0: *mut ::ffi_11::c_void) {
    crate::detail::__rust_thunk___Z32UnsafeSignatureWithoutAnnotationPv(__param_0)
}

/// Generated from: examples/cpp/unsafe_attributes/example.h;l=14
#[inline(always)]
pub fn UnsafeSignatureButAnnotatedSafe(__param_0: *mut ::ffi_11::c_void) {
    unsafe { crate::detail::__rust_thunk___Z31UnsafeSignatureButAnnotatedSafePv(__param_0) }
}

// is_unsafe=

/// Generated from: examples/cpp/unsafe_attributes/example.h;l=16
#[inline(always)]
pub fn SafeBasedOnBoolean() {
    unsafe { crate::detail::__rust_thunk___Z18SafeBasedOnBooleanv() }
}

// is_unsafe=

/// # Safety
///
/// The C++ function is explicitly annotated as unsafe. Ensure that its safety requirements are upheld.
///
/// Generated from: examples/cpp/unsafe_attributes/example.h;l=17
#[inline(always)]
pub unsafe fn UnsafeBasedOnBoolean() {
    crate::detail::__rust_thunk___Z20UnsafeBasedOnBooleanv()
}

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// error: struct `std::integral_constant<bool, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// error: struct `std::integral_constant<bool, true>` could not be bound
//   template instantiation is not yet supported

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z30SafeSignatureWithoutAnnotationv();
        pub(crate) unsafe fn __rust_thunk___Z31SafeSignatureButAnnotatedUnsafev();
        pub(crate) unsafe fn __rust_thunk___Z32UnsafeSignatureWithoutAnnotationPv(
            __param_0: *mut ::ffi_11::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___Z31UnsafeSignatureButAnnotatedSafePv(
            __param_0: *mut ::ffi_11::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___Z18SafeBasedOnBooleanv();
        pub(crate) unsafe fn __rust_thunk___Z20UnsafeBasedOnBooleanv();
    }
}
