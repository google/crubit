// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:item_order_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=FirstStruct
pub struct FirstStruct {
    pub field: ::core::ffi::c_int,
}
impl !Send for FirstStruct {}
impl !Sync for FirstStruct {}
unsafe impl ::cxx::ExternType for FirstStruct {
    type Id = ::cxx::type_id!("FirstStruct");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("FirstStruct"), crate::FirstStruct);

impl Default for FirstStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11FirstStructC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

#[inline(always)]
pub fn first_func() -> ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___Z10first_funcv() }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=SecondStruct
pub struct SecondStruct {
    pub field: ::core::ffi::c_int,
}
impl !Send for SecondStruct {}
impl !Sync for SecondStruct {}
unsafe impl ::cxx::ExternType for SecondStruct {
    type Id = ::cxx::type_id!("SecondStruct");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("SecondStruct"), crate::SecondStruct);

impl Default for SecondStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN12SecondStructC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

#[inline(always)]
pub fn second_func() -> ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___Z11second_funcv() }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN11FirstStructC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___Z10first_funcv() -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN12SecondStructC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___Z11second_funcv() -> ::core::ffi::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::FirstStruct>() == 4);
    assert!(::core::mem::align_of::<crate::FirstStruct>() == 4);
    static_assertions::assert_impl_all!(crate::FirstStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::FirstStruct: Drop);
    assert!(::core::mem::offset_of!(crate::FirstStruct, field) == 0);
    assert!(::core::mem::size_of::<crate::SecondStruct>() == 4);
    assert!(::core::mem::align_of::<crate::SecondStruct>() == 4);
    static_assertions::assert_impl_all!(crate::SecondStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SecondStruct: Drop);
    assert!(::core::mem::offset_of!(crate::SecondStruct, field) == 0);
};
