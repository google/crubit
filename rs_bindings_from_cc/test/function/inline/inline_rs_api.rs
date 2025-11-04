// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/function/inline:inline
// Features: infer_operator_lifetimes, non_unpin_ctor, std_unique_ptr, std_vector, supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/function/inline/inline.h;l=8
#[inline(always)]
pub fn hello_world_inline() -> ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___Z18hello_world_inlinev() }
}

/// This testcase helps verify that thunks correctly work with const-ref
/// parameters. Using an 'inline' method forces generation of a C++ thunk.
///
/// Generated from: rs_bindings_from_cc/test/function/inline/inline.h;l=12
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=SomeStruct
pub struct SomeStruct {
    pub int_field: ::core::ffi::c_int,
}
impl !Send for SomeStruct {}
impl !Sync for SomeStruct {}
unsafe impl ::cxx::ExternType for SomeStruct {
    type Id = ::cxx::type_id!("SomeStruct");
    type Kind = ::cxx::kind::Trivial;
}

/// Generated from: rs_bindings_from_cc/test/function/inline/inline.h;l=12
impl Default for SomeStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/function/inline/inline.h;l=15
#[inline(always)]
pub unsafe fn take_struct_by_const_ptr(s: *const crate::SomeStruct) -> ::core::ffi::c_int {
    crate::detail::__rust_thunk___Z24take_struct_by_const_ptrPK10SomeStruct(s)
}

/// This testcase helps verify that thunks correctly work with primitive types
/// that have multi-word type names (e.g. `unsigned int`). Using an 'inline'
/// method forces generation of a C++ thunk.
///
/// Generated from: rs_bindings_from_cc/test/function/inline/inline.h;l=22
#[inline(always)]
pub fn double_unsigned_int(i: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    unsafe { crate::detail::__rust_thunk___Z19double_unsigned_intj(i) }
}

// namespace namespaced

pub mod namespaced {
    /// Generated from: rs_bindings_from_cc/test/function/inline/inline.h;l=28
    #[inline(always)]
    pub fn forward_declared_doubler(x: ::core::ffi::c_int) -> ::core::ffi::c_int {
        unsafe { crate::detail::__rust_thunk___ZN10namespaced24forward_declared_doublerEi(x) }
    }
}

// namespace namespaced

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z18hello_world_inlinev() -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN10SomeStructC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___Z24take_struct_by_const_ptrPK10SomeStruct(
            s: *const crate::SomeStruct,
        ) -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___Z19double_unsigned_intj(
            i: ::core::ffi::c_uint,
        ) -> ::core::ffi::c_uint;
        pub(crate) unsafe fn __rust_thunk___ZN10namespaced24forward_declared_doublerEi(
            x: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::SomeStruct>() == 4);
    assert!(::core::mem::align_of::<crate::SomeStruct>() == 4);
    static_assertions::assert_impl_all!(crate::SomeStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SomeStruct: Drop);
    assert!(::core::mem::offset_of!(crate::SomeStruct, int_field) == 0);
};
