// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //examples/cpp/trivial_struct:example_lib
// Features: infer_operator_lifetimes, supported, unsafe_types

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// Generated from: examples/cpp/trivial_struct/example.h;l=8
#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Position
pub struct Position {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
}
impl !Send for Position {}
impl !Sync for Position {}
unsafe impl ::cxx::ExternType for Position {
    type Id = ::cxx::type_id!("Position");
    type Kind = ::cxx::kind::Trivial;
}

/// Generated from: examples/cpp/trivial_struct/example.h;l=8
impl Default for Position {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN8PositionC1Ev(&raw mut tmp as *mut ::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

/// Generated from: examples/cpp/trivial_struct/example.h;l=8
impl From<::ctor::RvalueReference<'_, Self>> for Position {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN8PositionC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for Position {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

/// Generated from: examples/cpp/trivial_struct/example.h;l=8
impl ::ctor::UnpinAssign<&Self> for Position {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN8PositionaSERKS_(self, __param_0);
        }
    }
}

/// Generated from: examples/cpp/trivial_struct/example.h;l=8
impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for Position {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN8PositionaSEOS_(self, __param_0);
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN8PositionC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN8PositionC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::Position>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN8PositionaSERKS_<'__return_lifetime>(
            __this: &mut crate::Position,
            __param_0: &crate::Position,
        ) -> &'__return_lifetime mut crate::Position;
        pub(crate) unsafe fn __rust_thunk___ZN8PositionaSEOS_<'__return_lifetime>(
            __this: &mut crate::Position,
            __param_0: ::ctor::RvalueReference<'_, crate::Position>,
        ) -> &'__return_lifetime mut crate::Position;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Position>() == 8);
    assert!(::core::mem::align_of::<crate::Position>() == 4);
    static_assertions::assert_impl_all!(crate::Position: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Position: Drop);
    assert!(::core::mem::offset_of!(crate::Position, x) == 0);
    assert!(::core::mem::offset_of!(crate::Position, y) == 4);
};
