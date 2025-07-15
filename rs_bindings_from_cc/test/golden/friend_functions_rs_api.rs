// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:friend_functions_cc

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
///CRUBIT_ANNOTATE: cpp_type=SomeClass
pub struct SomeClass {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for SomeClass {}
impl !Sync for SomeClass {}
unsafe impl ::cxx::ExternType for SomeClass {
    type Id = ::cxx::type_id!("SomeClass");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeClass"), crate::SomeClass);

impl Default for SomeClass {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

/// Friend functions that are visible via ADL.
#[inline(always)]
pub fn visible_val(mut __param_0: crate::SomeClass) {
    unsafe { crate::detail::__rust_thunk___Z11visible_val9SomeClass(&mut __param_0) }
}

#[inline(always)]
pub fn visible_ref<'a>(__param_0: &'a mut crate::SomeClass) {
    unsafe { crate::detail::__rust_thunk___Z11visible_refR9SomeClass(__param_0) }
}

#[inline(always)]
pub fn visible_cref<'a>(__param_0: &'a crate::SomeClass) {
    unsafe { crate::detail::__rust_thunk___Z12visible_crefRK9SomeClass(__param_0) }
}

#[inline(always)]
pub fn visible_rref<'a>(__param_0: ::ctor::RvalueReference<'a, crate::SomeClass>) {
    unsafe { crate::detail::__rust_thunk___Z12visible_rrefO9SomeClass(__param_0) }
}

/// A function can be declared multiple times - e.g. once in a friend
/// declaration below + in a definition below.  This example mimics
/// Uint128Low64 declarations from absl/numeric/int128.h.  This is a
/// regression test for b/244311755.
#[inline(always)]
pub fn multiple_declarations<'a>(__param_0: &'a crate::SomeClass) -> ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___Z21multiple_declarationsRK9SomeClass(__param_0) }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClassC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___Z11visible_val9SomeClass(
            __param_0: &mut crate::SomeClass,
        );
        #[link_name = "_Z11visible_refR9SomeClass"]
        pub(crate) unsafe fn __rust_thunk___Z11visible_refR9SomeClass<'a>(
            __param_0: &'a mut crate::SomeClass,
        );
        #[link_name = "_Z12visible_crefRK9SomeClass"]
        pub(crate) unsafe fn __rust_thunk___Z12visible_crefRK9SomeClass<'a>(
            __param_0: &'a crate::SomeClass,
        );
        #[link_name = "_Z12visible_rrefO9SomeClass"]
        pub(crate) unsafe fn __rust_thunk___Z12visible_rrefO9SomeClass<'a>(
            __param_0: ::ctor::RvalueReference<'a, crate::SomeClass>,
        );
        pub(crate) unsafe fn __rust_thunk___Z21multiple_declarationsRK9SomeClass<'a>(
            __param_0: &'a crate::SomeClass,
        ) -> ::core::ffi::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::SomeClass>() == 1);
    assert!(::core::mem::align_of::<crate::SomeClass>() == 1);
    static_assertions::assert_impl_all!(crate::SomeClass: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SomeClass: Drop);
};
