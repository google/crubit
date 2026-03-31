// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:assumed_c9
// Features: assume_lifetimes, assume_this_lifetimes, callables, check_default_initialized, experimental, fmt, supported, types, unsafe_view, wrapper

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/assume_lifetimes/assumed_c9.h;l=10
#[inline(always)]
pub fn CoReturnReference() -> ::co::Co<'static, *mut ::ffi_11::c_int> {
    unsafe {
        ::bridge_rust::unstable_return!(@::co::internal_crubit::CoCrubitAbi::new(|consume_result_into_buffer: ::co::internal_crubit::ConsumeResultIntoBufferFn,context: *mut::core::ffi::c_void|->*mut::ffi_11::c_int{ ::bridge_rust::unstable_return!(@::bridge_rust::transmute_abi::<*mut::ffi_11::c_int>(),::bridge_rust::TransmuteAbi<*mut::ffi_11::c_int>,|buffer: *mut u8|{ (consume_result_into_buffer.unwrap())(context,buffer,<::bridge_rust::TransmuteAbi<*mut::ffi_11::c_int>as::bridge_rust::CrubitAbi>::SIZE,); }) }),::co::internal_crubit::CoCrubitAbi<*mut::ffi_11::c_int>,|__return_abi_buffer|{ crate::detail::__rust_thunk___Z17CoReturnReferencev(__return_abi_buffer,); })
    }
}

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_ASSUMED_C9_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z17CoReturnReferencev(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
    }
}
