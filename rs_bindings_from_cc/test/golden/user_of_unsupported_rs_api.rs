// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:user_of_unsupported_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, impl_trait_in_assoc_type)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[inline(always)]
pub fn UseNontrivialCustomType(
    non_trivial_custom_type: ::ctor::Ctor![::unsupported_cc::NontrivialCustomType],
) {
    unsafe {
        crate::detail::__rust_thunk___Z23UseNontrivialCustomType20NontrivialCustomType(
            ::core::pin::Pin::into_inner_unchecked(::ctor::emplace!(non_trivial_custom_type)),
        )
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z23UseNontrivialCustomType20NontrivialCustomType(
            non_trivial_custom_type: &mut ::unsupported_cc::NontrivialCustomType,
        );
    }
}
