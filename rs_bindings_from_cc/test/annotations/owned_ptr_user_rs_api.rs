// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:owned_ptr_user
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// An example of a C++ file that defines functions that create a
/// CRUBIT_OWNED_PTR type as well as consume it.
///
/// Generated from: rs_bindings_from_cc/test/annotations/owned_ptr_user.h;l=14
#[inline(always)]
pub fn MakeOwnedThing(value: ::ffi_11::c_int) -> ::owned_ptr::OwnedThing {
    unsafe { ::core::mem::transmute(crate::detail::__rust_thunk___Z14MakeOwnedThingi(value)) }
}

/// Generated from: rs_bindings_from_cc/test/annotations/owned_ptr_user.h;l=16
#[inline(always)]
pub fn MakeThing(value: ::ffi_11::c_int) -> *mut ::owned_ptr::RawThing {
    unsafe { crate::detail::__rust_thunk___Z9MakeThingi(value) }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `thingptr`: raw pointer
///
/// Generated from: rs_bindings_from_cc/test/annotations/owned_ptr_user.h;l=18
#[inline(always)]
pub unsafe fn ThingToValue(thingptr: ::owned_ptr::OwnedThing) -> ::ffi_11::c_int {
    crate::detail::__rust_thunk___Z12ThingToValueP5Thing(::core::mem::transmute(thingptr))
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `thingptr`: raw pointer
///
/// Generated from: rs_bindings_from_cc/test/annotations/owned_ptr_user.h;l=20
#[inline(always)]
pub unsafe fn GetThingValue(thingptr: *mut ::owned_ptr::RawThing) -> ::ffi_11::c_int {
    crate::detail::__rust_thunk___Z13GetThingValueP5Thing(thingptr)
}

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, false>':
// Can't generate bindings for std::integral_constant<bool, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/annotations:owned_ptr_user needs [//features:wrapper] for std::integral_constant<bool, false> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, true>':
// Can't generate bindings for std::integral_constant<bool, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/annotations:owned_ptr_user needs [//features:wrapper] for std::integral_constant<bool, true> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE is a template instantiation)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_Z14MakeOwnedThingi"]
        pub(crate) unsafe fn __rust_thunk___Z14MakeOwnedThingi(
            value: ::ffi_11::c_int,
        ) -> *mut ::owned_ptr::RawThing;
        #[link_name = "_Z9MakeThingi"]
        pub(crate) unsafe fn __rust_thunk___Z9MakeThingi(
            value: ::ffi_11::c_int,
        ) -> *mut ::owned_ptr::RawThing;
        #[link_name = "_Z12ThingToValueP5Thing"]
        pub(crate) unsafe fn __rust_thunk___Z12ThingToValueP5Thing(
            thingptr: *mut ::owned_ptr::RawThing,
        ) -> ::ffi_11::c_int;
        #[link_name = "_Z13GetThingValueP5Thing"]
        pub(crate) unsafe fn __rust_thunk___Z13GetThingValueP5Thing(
            thingptr: *mut ::owned_ptr::RawThing,
        ) -> ::ffi_11::c_int;
    }
}
