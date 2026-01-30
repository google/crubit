// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:lifetimes_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[inline(always)]
pub fn AddHook(__param_0: Option<extern "C" fn()>) {
    unsafe { crate::detail::__rust_thunk___Z7AddHookPFvvE(__param_0) }
}

pub type FunctionPointer = Option<extern "C" fn()>;

#[inline(always)]
pub fn AddHookWithTypedef(hook: Option<extern "C" fn()>) {
    unsafe { crate::detail::__rust_thunk___Z18AddHookWithTypedefPFvvE(hook) }
}

#[inline(always)]
pub fn AddAnotherHook(__param_0: extern "C" fn()) {
    unsafe { crate::detail::__rust_thunk___Z14AddAnotherHookRFvvE(__param_0) }
}

pub type FunctionReference = extern "C" fn();

#[inline(always)]
pub fn AddAnotherHookWithTypedef(hook: extern "C" fn()) {
    unsafe { crate::detail::__rust_thunk___Z25AddAnotherHookWithTypedefRFvvE(hook) }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `pair`: raw pointer
#[inline(always)]
pub unsafe fn ConsumeArray(pair: *mut ::ffi_11::c_int) {
    crate::detail::__rust_thunk___Z12ConsumeArrayPi(pair)
}

// Error while generating bindings for type alias 'Arr':
// Unsupported type 'int[2]': Unsupported clang::Type class 'ConstantArray'

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `__param_0`: raw pointer
#[inline(always)]
pub unsafe fn ConsumeArrayWithTypedef(__param_0: *mut ::ffi_11::c_int) {
    crate::detail::__rust_thunk___Z23ConsumeArrayWithTypedefPi(__param_0)
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_Z7AddHookPFvvE"]
        pub(crate) unsafe fn __rust_thunk___Z7AddHookPFvvE(__param_0: Option<extern "C" fn()>);
        #[link_name = "_Z18AddHookWithTypedefPFvvE"]
        pub(crate) unsafe fn __rust_thunk___Z18AddHookWithTypedefPFvvE(
            hook: Option<extern "C" fn()>,
        );
        #[link_name = "_Z14AddAnotherHookRFvvE"]
        pub(crate) unsafe fn __rust_thunk___Z14AddAnotherHookRFvvE(__param_0: extern "C" fn());
        #[link_name = "_Z25AddAnotherHookWithTypedefRFvvE"]
        pub(crate) unsafe fn __rust_thunk___Z25AddAnotherHookWithTypedefRFvvE(
            hook: extern "C" fn(),
        );
        #[link_name = "_Z12ConsumeArrayPi"]
        pub(crate) unsafe fn __rust_thunk___Z12ConsumeArrayPi(pair: *mut ::ffi_11::c_int);
        #[link_name = "_Z23ConsumeArrayWithTypedefPi"]
        pub(crate) unsafe fn __rust_thunk___Z23ConsumeArrayWithTypedefPi(
            __param_0: *mut ::ffi_11::c_int,
        );
    }
}
