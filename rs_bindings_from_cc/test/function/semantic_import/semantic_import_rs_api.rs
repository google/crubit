// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/function/semantic_import:semantic_import

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "1S"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=S
pub struct S {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) x_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for S {}
impl !Sync for S {}
unsafe impl ::cxx::ExternType for S {
    type Id = ::cxx::type_id!("S");
    type Kind = ::cxx::kind::Trivial;
}
impl S {
    #[inline(always)]
    pub fn x<'__this>(&'__this self) -> ::ffi_11::c_int {
        unsafe { self::s::x(self) }
    }
    #[inline(always)]
    pub fn set_x<'__this>(&'__this mut self, x: ::ffi_11::c_int) {
        unsafe { self::s::set_x(self, x) }
    }
}

impl From<::ffi_11::c_int> for S {
    #[inline(always)]
    fn from(args: ::ffi_11::c_int) -> Self {
        let mut x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN1SC1Ei(&raw mut tmp as *mut _, x);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ffi_11::c_int> for S {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        <Self as From<::ffi_11::c_int>>::from(args)
    }
}

pub mod s {
    #[inline(always)]
    pub(crate) fn x<'__this>(__this: &'__this crate::S) -> ::ffi_11::c_int {
        unsafe { *((__this as *const _ as *const u8).add(0) as *const ::ffi_11::c_int) }
    }
    #[inline(always)]
    pub(crate) fn set_x<'__this>(__this: &'__this mut crate::S, x: ::ffi_11::c_int) {
        unsafe { *((__this as *mut _ as *mut u8).add(0) as *mut ::ffi_11::c_int) = x }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN1SC1Ei(
            __this: *mut ::core::ffi::c_void,
            x: ::ffi_11::c_int,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::S>() == 4);
    assert!(::core::mem::align_of::<crate::S>() == 4);
    static_assertions::assert_impl_all!(crate::S: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::S: Drop);
    assert!(::core::mem::offset_of!(crate::S, x_) == 0);
};
