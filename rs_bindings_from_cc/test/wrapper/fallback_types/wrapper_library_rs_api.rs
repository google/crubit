// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/wrapper/fallback_types:wrapper_library
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported, wrapper

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/wrapper/fallback_types/wrapper_library.h;l=10
#[inline(always)]
pub(crate) fn GetGlobalUnsupportedType(
) -> *mut ::forward_declare::Incomplete<::forward_declare::symbol!("UnsupportedType"), ()> {
    unsafe { crate::detail::__rust_thunk___Z24GetGlobalUnsupportedTypev() }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `x`: raw pointer
///
/// Generated from: rs_bindings_from_cc/test/wrapper/fallback_types/wrapper_library.h;l=15
#[inline(always)]
pub(crate) unsafe fn SetValue(
    x: *mut ::forward_declare::Incomplete<::forward_declare::symbol!("UnsupportedType"), ()>,
    value: ::ffi_11::c_int,
) {
    crate::detail::__rust_thunk___Z8SetValueR15UnsupportedTypei(x, value)
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `x`: raw pointer
///
/// Generated from: rs_bindings_from_cc/test/wrapper/fallback_types/wrapper_library.h;l=17
#[inline(always)]
pub(crate) unsafe fn GetValue(
    x: *const ::forward_declare::Incomplete<::forward_declare::symbol!("UnsupportedType"), ()>,
) -> ::ffi_11::c_int {
    crate::detail::__rust_thunk___Z8GetValueRK15UnsupportedType(x)
}

#[path = "rs_bindings_from_cc/test/wrapper/fallback_types/wrapper_library_extra.rs"]
mod __crubit_mod_0;
#[allow(unused_imports)]
pub use __crubit_mod_0::*;

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z24GetGlobalUnsupportedTypev(
        ) -> *mut ::forward_declare::Incomplete<::forward_declare::symbol!("UnsupportedType"), ()>;
        pub(crate) unsafe fn __rust_thunk___Z8SetValueR15UnsupportedTypei(
            x: *mut ::forward_declare::Incomplete<
                ::forward_declare::symbol!("UnsupportedType"),
                (),
            >,
            value: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___Z8GetValueRK15UnsupportedType(
            x: *const ::forward_declare::Incomplete<
                ::forward_declare::symbol!("UnsupportedType"),
                (),
            >,
        ) -> ::ffi_11::c_int;
    }
}
