// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:member_function
// Features: assume_lifetimes, fmt, supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/assume_lifetimes/member_function.h;l=8
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=S
pub struct S {
    pub int_field: ::ffi_11::c_int,
}
impl !Send for S {}
impl !Sync for S {}
unsafe impl ::cxx::ExternType for S {
    type Id = ::cxx::type_id!("S");
    type Kind = ::cxx::kind::Trivial;
}
impl S {
    /// Generated from: rs_bindings_from_cc/test/assume_lifetimes/member_function.h;l=9
    #[inline(always)]
    pub fn int_accessor<'__this>(&'__this self) -> &'__this ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZNK1S12int_accessorEv(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    ///
    /// Generated from: rs_bindings_from_cc/test/assume_lifetimes/member_function.h;l=10
    #[inline(always)]
    pub unsafe fn me(__this: *mut Self) -> *mut crate::S {
        crate::detail::__rust_thunk___ZN1S2meEv(__this)
    }
}

/// Generated from: rs_bindings_from_cc/test/assume_lifetimes/member_function.h;l=8
impl Default for S {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN1SC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN1SC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZNK1S12int_accessorEv<'__this>(
            __this: &'__this crate::S,
        ) -> &'__this ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN1S2meEv(__this: *mut crate::S) -> *mut crate::S;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::S>() == 4);
    assert!(::core::mem::align_of::<crate::S>() == 4);
    static_assertions::assert_impl_all!(crate::S: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::S: Drop);
    assert!(::core::mem::offset_of!(crate::S, int_field) == 0);
};
