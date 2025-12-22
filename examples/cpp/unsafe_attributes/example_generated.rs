// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //examples/cpp/unsafe_attributes:example_lib
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// Generated from: examples/cpp/unsafe_attributes/example.h;l=10
#[inline(always)]
pub fn SafeSignatureWithoutAnnotation() {
    unsafe { crate::detail::__rust_thunk___Z30SafeSignatureWithoutAnnotationv() }
}

/// Generated from: examples/cpp/unsafe_attributes/example.h;l=11
#[inline(always)]
pub unsafe fn SafeSignatureButAnnotatedUnsafe() {
    crate::detail::__rust_thunk___Z31SafeSignatureButAnnotatedUnsafev()
}

/// Generated from: examples/cpp/unsafe_attributes/example.h;l=13
#[inline(always)]
pub unsafe fn UnsafeSignatureWithoutAnnotation(__param_0: *mut ::core::ffi::c_void) {
    crate::detail::__rust_thunk___Z32UnsafeSignatureWithoutAnnotationPv(__param_0)
}

/// Generated from: examples/cpp/unsafe_attributes/example.h;l=14
#[inline(always)]
pub fn UnsafeSignatureButAnnotatedSafe(__param_0: *mut ::core::ffi::c_void) {
    unsafe { crate::detail::__rust_thunk___Z31UnsafeSignatureButAnnotatedSafePv(__param_0) }
}

// is_unsafe=

/// Generated from: examples/cpp/unsafe_attributes/example.h;l=16
#[inline(always)]
pub fn SafeBasedOnBoolean() {
    unsafe { crate::detail::__rust_thunk___Z18SafeBasedOnBooleanv() }
}

// is_unsafe=

/// Generated from: examples/cpp/unsafe_attributes/example.h;l=17
#[inline(always)]
pub unsafe fn UnsafeBasedOnBoolean() {
    crate::detail::__rust_thunk___Z20UnsafeBasedOnBooleanv()
}

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, false>':
// Can't generate bindings for std::integral_constant<bool, false>, because of missing required features (crubit.rs-features):
// //examples/cpp/unsafe_attributes:example_lib needs [//features:wrapper] for std::integral_constant<bool, false> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, true>':
// Can't generate bindings for std::integral_constant<bool, true>, because of missing required features (crubit.rs-features):
// //examples/cpp/unsafe_attributes:example_lib needs [//features:wrapper] for std::integral_constant<bool, true> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE is a template instantiation)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z30SafeSignatureWithoutAnnotationv();
        pub(crate) unsafe fn __rust_thunk___Z31SafeSignatureButAnnotatedUnsafev();
        pub(crate) unsafe fn __rust_thunk___Z32UnsafeSignatureWithoutAnnotationPv(
            __param_0: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___Z31UnsafeSignatureButAnnotatedSafePv(
            __param_0: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___Z18SafeBasedOnBooleanv();
        pub(crate) unsafe fn __rust_thunk___Z20UnsafeBasedOnBooleanv();
    }
}
