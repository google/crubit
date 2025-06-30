// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:c_abi_compatible_type_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Type bindings for struct MyI8 suppressed due to being mapped to an existing Rust type (i8)

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=X
pub struct X {
    pub a: ::core::ffi::c_int,
}
impl !Send for X {}
impl !Sync for X {}
unsafe impl ::cxx::ExternType for X {
    type Id = ::cxx::type_id!("X");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("X"), crate::X);

impl Default for X {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN1XC1Ev(&raw mut tmp as *mut ::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for X {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN1XC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for X {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for X {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN1XaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for X {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN1XaSEOS_(self, __param_0);
        }
    }
}

#[inline(always)]
pub fn ffi(a: i8, mut b: crate::X) -> i8 {
    unsafe { crate::detail::__rust_thunk___Z3ffi4MyI81X(a, &mut b) }
}

pub type MyTypedefDecl = ::core::ffi::c_int;

#[inline(always)]
pub unsafe fn f(a: crate::MyTypedefDecl, b: *mut ::core::ffi::c_void, c: ::core::ffi::c_int) {
    crate::detail::__rust_thunk___Z1fiPvi(a, b, c)
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN1XC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN1XC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::X>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN1XaSERKS_<'__return_lifetime>(
            __this: &mut crate::X,
            __param_0: &crate::X,
        ) -> &'__return_lifetime mut crate::X;
        pub(crate) unsafe fn __rust_thunk___ZN1XaSEOS_<'__return_lifetime>(
            __this: &mut crate::X,
            __param_0: ::ctor::RvalueReference<'_, crate::X>,
        ) -> &'__return_lifetime mut crate::X;
        pub(crate) unsafe fn __rust_thunk___Z3ffi4MyI81X(a: i8, b: &mut crate::X) -> i8;
        pub(crate) unsafe fn __rust_thunk___Z1fiPvi(
            a: crate::MyTypedefDecl,
            b: *mut ::core::ffi::c_void,
            c: ::core::ffi::c_int,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<i8>() == 1);
    assert!(::core::mem::align_of::<i8>() == 1);
    assert!(::core::mem::size_of::<crate::X>() == 4);
    assert!(::core::mem::align_of::<crate::X>() == 4);
    static_assertions::assert_impl_all!(crate::X: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::X: Drop);
    assert!(::core::mem::offset_of!(crate::X, a) == 0);
};
